define({
    "root": {
        "customVideoScale": "custom resolution",
        "steamVRRestartSuccess": "SteamVR successfully restarted",
        "_root_video_tab.name": "Video",
        "_root_video_tab.description": "Video settings",
        "_root_audio_tab.name": "Audio",
        "_root_audio_tab.description": "Audio settings",
        "_root_headset_tab.name": "Headset",
        "_root_connection_tab.name": "Connection",
        "_root_video_adapterIndex.name": "Adapter Index",
        "_root_video_adapterIndex.description": "System index of GPU-Adapter",
        "_root_video_refreshRate.name": "Display Refreshrate",
        "_root_video_refreshRate.description": "Refreshrate used on the HMD",
        "_root_video_encodeBitrateMbs.name": "Video Bitrate",
        "_root_video_encodeBitrateMbs.description": "Bitrate of video streaming. 30Mbps is recommended. \nHigher bitrates result in better image but also higher latency and network traffic ",
        "_root_video_force60hz.name": "Force 60hz Mode",
        "_root_video_force60hz.description": "Forces the refreshrate to 60Hz",
        "_root_video_resolutionDropdown.name": "Video resolution",
        "_root_video_resolutionDropdown.description": `100% results in the native 2880x1600 resolution of the Oculus Quest.
        Setting the resolution can bring some improvement in visual quality, but is not recommended. 
        A resolution lower than 100% can reduce latency and increase network performance`,
        "_root_video_renderResolution-choice-.name": "Video resolution",
        "_root_video_renderResolution_scale-choice-.name": "use video scale",
        "_root_video_renderResolution_scale-choice-.description": "Scale factor for the video resolution",
        "_root_video_renderResolution_scale.name": "Scale",
        "_root_video_renderResolution_scale.description": "The video scale",
        "_root_video_renderResolution_absolute-choice-.name": "use absolute video resolution",
        "_root_video_renderResolution_absolute-choice-.description": "Use absolute video resolution",
        "_root_video_renderResolution_absolute_absolute.name": "Absolute resolution",
        "_root_video_renderResolution_absolute_absolute.description": "The absolute resolution",
        "_root_video_renderResolution_absolute_width.name": "Video width",
        "_root_video_renderResolution_absolute_width.description": "The width of the encoded video",
        "_root_video_renderResolution_absolute_height.name": "Video height",
        "_root_video_renderResolution_absolute_height.description": "The height of the encoded video",
        "_root_video_recommendedTargetResolution-choice-.name": "Target frame resolution",
        "_root_video_recommendedTargetResolution-choice-.description": "Frame resolution requested to SteamVR for rendering",
        "_root_video_recommendedTargetResolution_scale-choice-.name": "use target resolution scale",
        "_root_video_recommendedTargetResolution_scale.name": "Scale",
        "_root_video_recommendedTargetResolution_scale.description": "scale",
        "_root_video_recommendedTargetResolution_absolute-choice-.name": "use absolute target resolution",
        "_root_video_recommendedTargetResolution_absolute_absolute.name": "Absolute target resolution",
        "_root_video_recommendedTargetResolution_absolute_width.name": "Frame width",
        "_root_video_recommendedTargetResolution_absolute_width.description": "Preferred width of the frame",
        "_root_video_recommendedTargetResolution_absolute_height.name": "Video height",
        "_root_video_recommendedTargetResolution_absolute_height.description": "Preferred height of the frame",
        "_root_video_eyeFov.name": "Eye Fov",
        "_root_video_eyeFov.description": "The eye field of view",
        "_root_video_eyeFov_0_eyeFov_0.name": "Left eye",
        "_root_video_eyeFov_0_eyeFov_0.description": "The settings for the left eye",
        "_root_video_eyeFov_0_left.name": "left",
        "_root_video_eyeFov_0_left.description": "left value",
        "_root_video_eyeFov_0_right.name": "right",
        "_root_video_eyeFov_0_right.description": "right value",
        "_root_video_eyeFov_0_top.name": "top",
        "_root_video_eyeFov_0_top.description": "top value",
        "_root_video_eyeFov_0_bottom.name": "bottom",
        "_root_video_eyeFov_0_bottom.description": "bottom value",
        "_root_video_eyeFov_1_eyeFov_1.name": "Right eye",
        "_root_video_eyeFov_1_eyeFov_1.description": "the settings for the right eye",
        "_root_video_eyeFov_1_left.name": "left",
        "_root_video_eyeFov_1_left.description": "left value",
        "_root_video_eyeFov_1_right.name": "right",
        "_root_video_eyeFov_1_right.description": "right value",
        "_root_video_eyeFov_1_top.name": "top",
        "_root_video_eyeFov_1_top.description": "top value",
        "_root_video_eyeFov_1_bottom.name": "bottom",
        "_root_video_eyeFov_1_bottom.description": "bottom value",
        "_root_video_secondsFromVsyncToPhotons.name": "S from VSync to photons",
        "_root_video_secondsFromVsyncToPhotons.description": "Time from vsync event to light generated by the display",
        "_root_video_ipd.name": "Interpupillary distance",
        "_root_video_ipd.description": "Space between the eyes",
        "_root_video_foveatedRendering.name": "Foveated rendering",
        "_root_video_foveatedRendering.description": "Settings for foveated rendering. Renders the edges of the visible area with less resolution",
        "_root_video_foveatedRendering_enabled.name": "enable",
        "_root_video_foveatedRendering_enabled.description": `Technique where the center of the image is rendered in high resolution while the outskirts are rendered in lower resolution.
        Results in a much lower video resolution that needs to be transmitted over the network.
        The smaller video at the same bitrate can preserve more details and lowers the latency at the same time.
        "FFR causes some visual artifacts at the edges of the view that are more or less visible depending on the settings and the game`,
        "_root_video_foveatedRendering_content_strength.name": "strength",
        "_root_video_foveatedRendering_content_strength.description": "Higher value means less detail toward the edges of the frame and more artifacts",
        "_root_video_foveatedRendering_content_shape.name": "Shape",
        "_root_video_foveatedRendering_content_shape.description": "The shape of the foveated rendering",
        "_root_video_foveatedRendering_content_verticalOffset.name": "Vertical offset",
        "_root_video_foveatedRendering_content_verticalOffset.description": "Higher value means the high quality frame region is moved further down",
        "_root_video_colorCorrection.name": "Color correction",
        "_root_video_colorCorrection.description": "The color correction",
        "_root_video_colorCorrection_enabled.name": "Enable",
        "_root_video_colorCorrection_enabled.description": "The color transformations are applied in the order: sharpening, gamma, brightness, contrast, saturation",
        "_root_video_colorCorrection_content_brightness.name": "Brightness",
        "_root_video_colorCorrection_content_brightness.description": "Brightness: range [-1;1], default 0. -1 is completely black and 1 is completely white",
        "_root_video_colorCorrection_content_contrast.name": "Contrast",
        "_root_video_colorCorrection_content_contrast.description": "Contrast: range [-1;1], default 0. -1 is completely gray",
        "_root_video_colorCorrection_content_saturation.name": "Saturation",
        "_root_video_colorCorrection_content_saturation.description": "Saturation: range [-1;1], default 0. -1 is black and white",
        "_root_video_colorCorrection_content_gamma.name": "Gamma",
        "_root_video_colorCorrection_content_gamma.description": "Gamma: range [0;5], default 1. Use a value of 2.2 to correct color from sRGB to RGB space",
        "_root_video_colorCorrection_content_sharpening.name": "Sharpening",
        "_root_video_colorCorrection_content_sharpening.description": "Sharpening: range [-1;5], default 0. -1 is the most blurry and 5 is the most sharp",
        "_root_video_codec-choice-.name": "Video Codec",
        "_root_video_codec-choice-.description": "Used Video codec \nChoose h265 if possible for better visual quality on lower bitrates",
        "_root_video_codec_H264-choice-.name": "H264",
        "_root_video_codec_H264-choice-.description": "Use the h264 codec",
        "_root_video_codec_HEVC-choice-.name": "HEVC (h265)",
        "_root_video_codec_HEVC-choice-.description": "Use the HEVC (h265) codec",
        "_root_audio_gameAudio.name": "Stream game audio",
        "_root_audio_gameAudio.description": "Stream game audio to the HDM",
        "_root_audio_gameAudio_enabled.name": "unused",
        "_root_audio_gameAudio_enabled.description": "Enables the streaming of game audio to the HMD",
        "_root_audio_gameAudio_content_deviceDropdown.name": "Select audio device",
        "_root_audio_gameAudio_content_deviceDropdown.description": "Audio device used to capture audio",
        "_root_audio_gameAudio_content_device.name": "Used audio device",
        "_root_audio_gameAudio_content_device.description": "Used audio device for streaming",
        "_root_audio_microphone.name": "Stream microphone",
        "_root_audio_microphone.description": "Streams the HMD microphone",
        "_root_headset_serialNumber.name": "Serial number",
        "_root_headset_serialNumber.description": "Serial number used for the simulated headset",
        "_root_headset_trackingSystemName.name": "Tracking system name",
        "_root_headset_trackingSystemName.description": "Name of the simulated tracking system",
        "_root_headset_modelNumber.name": "Model number",
        "_root_headset_modelNumber.description": "Model number of the simulated headset",
        "_root_headset_driverVersion.name": "Driver version",
        "_root_headset_driverVersion.description": "Driver version of the simulated headset",
        "_root_headset_manufacturerName.name": "Manufacturer name",
        "_root_headset_manufacturerName.description": "Name of the manufacturer of the simulated headset",
        "_root_headset_renderModelName.name": "Render model name",
        "_root_headset_renderModelName.description": "Name of the render module used",
        "_root_headset_registeredDeviceType.name": "Registered device type",
        "_root_headset_registeredDeviceType.description": "Type of the registered device",
        "_root_headset_trackingFrameOffset.name": "Tracking frame offset",
        "_root_headset_trackingFrameOffset.description": "Offset for the pose prediction algorithm",
        "_root_headset_positionOffset.name": "HMD position offset",
        "_root_headset_positionOffset.description": "Offset for the reported position",
        "_root_headset_positionOffset_0.name": "x",
        "_root_headset_positionOffset_0.description": "X offset",
        "_root_headset_positionOffset_1.name": "y",
        "_root_headset_positionOffset_1.description": "Y offset",
        "_root_headset_positionOffset_2.name": "z",
        "_root_headset_positionOffset_2.description": "Z offset",
        "_root_headset_useTrackingReference.name": "Use tracking reference",
        "_root_headset_useTrackingReference.description": "unused",
        "_root_headset_force3dof.name": "Force 3Dof",
        "_root_headset_force3dof.description": "Forces the 3 degrees of freedom mode (like Oculus Go)",
        "_root_headset_controllers.name": "Controllers",
        "_root_headset_controllers.description": "unused",
        "_root_headset_controllers_enabled.name": "enabled",
        "_root_headset_controllers_enabled.description": "Enables the usage of controllers",
        "_root_headset_controllers_content_trackingSystemName.name": "Tracking system name",
        "_root_headset_controllers_content_trackingSystemName.description": "Name of the simulated tracking system",
        "_root_headset_controllers_content_manufacturerName.name": "Manufacturer Name",
        "_root_headset_controllers_content_manufacturerName.description": "Name of the controller manufacturer",
        "_root_headset_controllers_content_modelNumber.name": "Model number",
        "_root_headset_controllers_content_modelNumber.description": "The controller model number",
        "_root_headset_controllers_content_renderModelNameLeft.name": "Render model name (left)",
        "_root_headset_controllers_content_renderModelNameLeft.description": "Name of the render model for the left controller",
        "_root_headset_controllers_content_renderModelNameRight.name": "Render model name (right)",
        "_root_headset_controllers_content_renderModelNameRight.description": "Name of the render model for the right controller",
        "_root_headset_controllers_content_serialNumber.name": "Serial number",
        "_root_headset_controllers_content_serialNumber.description": "The serial number fo the controller",
        "_root_headset_controllers_content_ctrlType.name": "Controller type",
        "_root_headset_controllers_content_ctrlType.description": "The type of the simulated controller",
        "_root_headset_controllers_content_registeredDeviceType.name": "Device type name",
        "_root_headset_controllers_content_registeredDeviceType.description": "The name of the simulated device type",
        "_root_headset_controllers_content_inputProfilePath.name": "Profile path",
        "_root_headset_controllers_content_inputProfilePath.description": "Path to the input profile",
        "_root_headset_controllers_content_triggerMode.name": "Trigger Mode",
        "_root_headset_controllers_content_triggerMode.description": "unused",
        "_root_headset_controllers_content_trackpadClickMode.name": "Trackpad click mode",
        "_root_headset_controllers_content_trackpadClickMode.description": "unused",
        "_root_headset_controllers_content_trackpadTouchMode.name": "Trackpad touch mode",
        "_root_headset_controllers_content_trackpadTouchMode.description": "unused",
        "_root_headset_controllers_content_backMode.name": "Back mode",
        "_root_headset_controllers_content_backMode.description": "unused",
        "_root_headset_controllers_content_recenterButton.name": "Recenter button",
        "_root_headset_controllers_content_recenterButton.description": "unused",
        "_root_headset_controllers_content_poseTimeOffset.name": "Pose time offset",
        "_root_headset_controllers_content_poseTimeOffset.description": "Offset for the pose prediction algorithm",
        "_root_headset_controllers_content_positionOffsetLeft.name": "Position offset",
        "_root_headset_controllers_content_positionOffsetLeft.description": "Position offset in meters for the left controller. \n For the right controller, x value is mirrored",
        "_root_headset_controllers_content_positionOffsetLeft_0.name": "x",
        "_root_headset_controllers_content_positionOffsetLeft_0.description": "X offset",
        "_root_headset_controllers_content_positionOffsetLeft_1.name": "y",
        "_root_headset_controllers_content_positionOffsetLeft_1.description": "Y offset",
        "_root_headset_controllers_content_positionOffsetLeft_2.name": "z",
        "_root_headset_controllers_content_positionOffsetLeft_2.description": "Z offset",
        "_root_headset_controllers_content_rotationOffsetLeft.name": "Rotation offset",
        "_root_headset_controllers_content_rotationOffsetLeft.description": "Rotation offset in degrees for the left controller.\nFor the right controller, rotations along the Y and Z axes are mirrored",
        "_root_headset_controllers_content_rotationOffsetLeft_0.name": "x",
        "_root_headset_controllers_content_rotationOffsetLeft_0.description": "Y rotation",
        "_root_headset_controllers_content_rotationOffsetLeft_1.name": "y",
        "_root_headset_controllers_content_rotationOffsetLeft_1.description": "Y rotation",
        "_root_headset_controllers_content_rotationOffsetLeft_2.name": "z",
        "_root_headset_controllers_content_rotationOffsetLeft_2.description": "Z rotation",
        "_root_headset_controllers_content_hapticsIntensity.name": "Haptic Intensity",
        "_root_headset_controllers_content_hapticsIntensity.description": "Factor to increase the haptic feedback",
        "_root_headset_controllers_content_modeIdx.name": "Mode",
        "_root_headset_controllers_content_modeIdx.description": "Controller mode index",
        "_root_connection_listenHost.name": "Listening host",
        "_root_connection_listenHost.description": "Server listening IP",
        "_root_connection_listenPort.name": "Listening port",
        "_root_connection_listenPort.description": "Server listening port",
        "_root_connection_throttlingBitrateBits.name": "Throttling bitrate",
        "_root_connection_throttlingBitrateBits.description": "Maximum streaming bitrate ",
        "_root_connection_sendingTimeslotUs.name": "Sending timeslot",
        "_root_connection_sendingTimeslotUs.description": "unused",
        "_root_connection_limitTimeslotPackets.name": "Limit timeslot packets",
        "_root_connection_limitTimeslotPackets.description": "unused",
        "_root_connection_clientRecvBufferSize.name": "Client buffer size",
        "_root_connection_clientRecvBufferSize.description": "Buffer size on client side.\n Depends on the bitrate.\n Calculated size is recommended. If you experience packet loss, enlarge buffer.",
        "_root_connection_frameQueueSize.name": "Frame queue size",
        "_root_connection_frameQueueSize.description": "Maximum queued frames on the client",
        "_root_connection_aggressiveKeyframeResend.name": "Aggressive keyframe resend",
        "_root_connection_aggressiveKeyframeResend.description": `Decrease minimum interval between keyframes from 100 ms to 5 ms.
        "Used only when packet loss is detected. Improves experience on networks with packet loss.`,
        "_root_extra_tab.name": "Extra",
        "_root_extra_revertConfirmDialog.name": "Confirm revert",
        "_root_extra_revertConfirmDialog.description": "Ask for confirmation before reverting settings to default value",
        "_root_extra_restartConfirmDialog.name": "Confirm restart",
        "_root_extra_restartConfirmDialog.description": "Ask for confirmation before restarting SteamVR",
        "_root_extra_notificationLevel-choice-.name": "Notification level",
        "_root_extra_notificationLevel-choice-.description": "Level of logging which will trigger a gui notification",
        "_root_extra_notificationLevel_warning-choice-.name": "Warning",
        "_root_extra_notificationLevel_error-choice-.name": "Error",
        "_root_extra_notificationLevel_info-choice-.name": "Info",
        "_root_extra_notificationLevel_debug-choice-.name": "Debug",
        "_root_extra_excludeNotificationsWithoutId.name": "Exclude Notifications without Id",
        "_root_extra_excludeNotificationsWithoutId.description": "Don't show notifications that don't have an Id",
        "_root_connection_disableThrottling.name": "Disable throttling",
        "_root_connection_disableThrottling.description": "Disables the throttling. Send data as fast as possible",
        "_root_connection_suppressFrameDrop.name": "Suppress frame drops",
        "_root_connection_suppressFrameDrop.description": "Try to suppress frame drops",
        "_root_headset_headsetEmulationMode.name": "Headset emulation mode",
        "_root_headset_headsetEmulationMode.description": "Emulates different headsets for better compatibility",
        "_root_headset_controllers_content_controllerMode.name": "Controller emulation mode",
        "_root_headset_controllers_content_controllerMode.description": "Emulates different controller for better compatibility or enables hand tracking",
        "_root_connection_bufferOffset.name": "Buffer offset",
        "_root_connection_bufferOffset.description": "Offset to increase or decrease the calculated client buffer size. The client buffer can not be negative"
    },
    "it": true,
    "es": true,
    "fr": true,
    "de-de": true
});