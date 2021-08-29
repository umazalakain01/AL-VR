mod color_correction;
mod compositing;
mod convert;
mod foveated_rendering;
mod slicing;

pub use convert::*;

use alvr_common::prelude::*;
use alvr_session::{ColorCorrectionDesc, Fov, FoveatedRenderingDesc};
use color_correction::ColorCorrectionPass;
use compositing::{CompositingPass, Layer};
use foveated_rendering::{Direction, FoveatedRenderingPass};
use slicing::{AlignmentDirection, SlicingPass};
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Color,
    ColorTargetState, ColorWrites, CommandEncoder, CommandEncoderDescriptor, Device, Extent3d,
    FilterMode, FragmentState, Instance, LoadOp, MultisampleState, Operations, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    Sampler, SamplerDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, VertexState,
};

pub const TARGET_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;

pub struct Context {
    instance: Instance,
    device: Device,
    queue: Queue,
}

impl Context {
    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}

pub struct Swapchain {
    textures: Vec<Texture>,
    bind_groups: Vec<Vec<BindGroup>>, //[0]: texture index, [1]: array index
    current_index: usize,
}

impl Swapchain {
    pub fn enumerate_images(&self) -> &[Texture] {
        &self.textures
    }

    // This is used in place of acquire_image + wait_image + release_image
    pub fn next_index(&mut self) -> usize {
        self.current_index = (self.current_index + 1) % self.textures.len();

        self.current_index
    }
}

pub struct CompositionLayerView<'a> {
    pub swapchain: &'a Swapchain,
    pub image_rect: openxr_sys::Rect2Di,
    pub image_array_index: usize,
    pub fov: Fov,
}

// Most of the compositor structure cannot be modified after creation. Some parameters like FOV for
// FFR and color correction parameters (if enabled) can be changed on the fly. Enabling/disabling
// FFR and changing the target view size require recreating the compositor completely, which might
// cause a lag spike.
pub struct Compositor {
    context: Context,
    inner: CompositingPass,
    color_corrector: ColorCorrectionPass,
    foveation_encoder: Option<FoveatedRenderingPass>,
    slicer: SlicingPass,

    // todo: move to client
    slicer2: SlicingPass,
    foveation_decoder: Option<FoveatedRenderingPass>,

    output_textures: Vec<Texture>,
    output_texture_views: Vec<TextureView>,
    output_size: (u32, u32),
}

impl Compositor {
    pub fn new(
        context: Context,
        target_view_size: (u32, u32), // expected size of a layer after cropping
        foveation_desc: Option<&FoveatedRenderingDesc>,
        slices_count: usize,
    ) -> Self {
        let inner = CompositingPass::new(&context.device);

        let color_corrector = ColorCorrectionPass::new(&context.device, target_view_size);

        let mut output_size = target_view_size;

        let foveation_encoder = foveation_desc
            .map(|desc| {
                FoveatedRenderingPass::new(
                    Direction::Encoding,
                    target_view_size,
                    desc,
                    Fov {
                        left: 45_f32,
                        right: 45_f32,
                        top: 45_f32,
                        bottom: 45_f32,
                    },
                )
            })
            .map(|(encoder, encoded_size)| {
                output_size = encoded_size;

                encoder
            });

        let foveation_decoder = foveation_desc
            .map(|desc| {
                FoveatedRenderingPass::new(
                    Direction::Decoding,
                    target_view_size,
                    desc,
                    Fov {
                        left: 45_f32,
                        right: 45_f32,
                        top: 45_f32,
                        bottom: 45_f32,
                    },
                )
            })
            .map(|(decoder, _)| decoder);

        let combined_size = (output_size.0 * 2, output_size.1);

        let slicer = SlicingPass::new(
            &context.device,
            combined_size,
            2,
            slices_count,
            AlignmentDirection::Output,
        );

        let slicer2 = SlicingPass::new(
            &context.device,
            combined_size,
            slices_count,
            2,
            AlignmentDirection::Input,
        );

        let output_size = slicer.output_size();

        let output_textures = (0..slices_count)
            .map(|_| {
                context.device.create_texture(&TextureDescriptor {
                    label: None,
                    size: Extent3d {
                        width: output_size.0,
                        height: output_size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TARGET_FORMAT,
                    usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                })
            })
            .collect::<Vec<_>>();
        let output_texture_views = output_textures
            .iter()
            .map(|tex| tex.create_view(&Default::default()))
            .collect();

        Self {
            context,
            inner,
            color_corrector,
            foveation_encoder,
            slicer,
            slicer2,
            foveation_decoder,
            output_textures,
            output_texture_views,
            output_size,
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    fn swapchain(&self, textures: Vec<Texture>, array_size: u32) -> Swapchain {
        let bind_groups = textures
            .iter()
            .map(|texture| {
                (0..array_size)
                    .map(|array_index| {
                        self.inner
                            .create_bind_group(&self.context.device, texture, array_index)
                    })
                    .collect()
            })
            .collect();

        Swapchain {
            textures,
            bind_groups,
            current_index: 0,
        }
    }

    // image size used for encoding
    pub fn output_size(&self) -> (u32, u32) {
        self.output_size
    }

    pub fn output(&self) -> &[Texture] {
        &self.output_textures
    }

    // The function is blocking but it should finish quite fast. Corresponds to xrEndFrame
    pub fn end_frame(
        &self,
        layers: &[&[CompositionLayerView]],
        color_correction: Option<ColorCorrectionDesc>,
    ) {
        for views in &*layers {
            assert_eq!(views.len(), 2);
        }

        let mut encoder = self
            .context
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        for view_index in 0..2 {
            let layers = layers.iter().map(|layer| {
                let view = &layer[view_index];
                let swapchain = &view.swapchain;

                Layer {
                    bind_group: &swapchain.bind_groups[swapchain.current_index]
                        [view.image_array_index],
                    rect: view.image_rect,
                }
            });

            let render_target = if color_correction.is_some() {
                self.color_corrector.input()
            } else if let Some(encoder) = &self.foveation_encoder {
                encoder.input()
            } else {
                &self.slicer.input()[view_index]
            };

            self.inner.draw(&mut encoder, layers, render_target);

            if let Some(desc) = color_correction {
                let render_target = if let Some(encoder) = &self.foveation_encoder {
                    encoder.input()
                } else {
                    &self.slicer.input()[view_index]
                };

                self.color_corrector
                    .draw(&mut encoder, &desc, render_target)
            }

            if let Some(foveation_encoder) = &self.foveation_encoder {
                // todo: get correct fov
                let fov = Fov::default();
                foveation_encoder.draw(&mut encoder, fov);
            }
        }

        for slice_idx in 0..self.output_texture_views.len() {
            self.slicer.draw(
                &mut encoder,
                slice_idx,
                &self.output_texture_views[slice_idx],
            )
        }

        // For the best performance, all compositing work is submitted at once.
        self.context.queue.submit(Some(encoder.finish()));

        pollster::block_on(self.context.queue.on_submitted_work_done());
    }
}

fn create_default_render_pipeline(device: &Device, fragment_shader: &str) -> RenderPipeline {
    let quad_shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(include_str!("../../resources/quad.wgsl").into()),
    });

    let fragment_shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(fragment_shader.into()),
    });

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: VertexState {
            module: &quad_shader,
            entry_point: "main",
            buffers: &[],
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        fragment: Some(FragmentState {
            module: &fragment_shader,
            entry_point: "main",
            targets: &[ColorTargetState {
                format: TARGET_FORMAT,
                blend: None,
                write_mask: ColorWrites::ALL,
            }],
        }),
    })
}

fn create_default_texture(device: &Device, size: (u32, u32)) -> TextureView {
    let texture = device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TARGET_FORMAT,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
    });

    texture.create_view(&Default::default())
}

fn create_default_sampler(device: &Device) -> Sampler {
    device.create_sampler(&SamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        mipmap_filter: FilterMode::Linear,
        ..Default::default()
    })
}

fn create_default_bind_group_with_sampler(
    device: &Device,
    pipeline: &RenderPipeline,
    texture_view: &TextureView,
    sampler: &Sampler,
) -> BindGroup {
    device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(sampler),
            },
        ],
    })
}

fn create_default_bind_group(
    device: &Device,
    pipeline: &RenderPipeline,
    texture_view: &TextureView,
) -> BindGroup {
    device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(texture_view),
        }],
    })
}

fn execute_default_pass(
    encoder: &mut CommandEncoder,
    pipeline: &RenderPipeline,
    bind_group: &BindGroup,
    push_constants: &[u8],
    output: &TextureView,
) {
    let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
        color_attachments: &[RenderPassColorAttachment {
            view: output,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::BLACK),
                store: true,
            },
        }],
        ..Default::default()
    });

    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, bind_group, &[]);
    pass.set_push_constants(ShaderStages::FRAGMENT, 0, push_constants);

    pass.draw(0..4, 0..1);

    // here the pass is dropped and applied to the command encoder
}
