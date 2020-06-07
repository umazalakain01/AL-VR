#ifndef ALVRCLIENT_VR_CONTEXT_H
#define ALVRCLIENT_VR_CONTEXT_H

#include <memory>
#include <map>
#include <android/native_window.h>
#include <android/native_window_jni.h>
#include <android/input.h>
#include "packet_types.h"
#include "render.h"
#include "utils.h"
#include "ServerConnectionNative.h"
#include "OVR_Platform.h"
#include "ffr.h"

uint32_t ovrButton_Unknown1 = 0x01000000;

class OvrContext {
public:
    void initialize(JNIEnv *env, jobject activity, jobject assetManager, jobject vrThread, bool ARMode, int initialRefreshRate);
    void destroy(JNIEnv *env);


    void onChangeSettings(int Suspend);
    void onSurfaceCreated(jobject surface);
    void onSurfaceDestroyed();
    void onSurfaceChanged(jobject surface);
    void onResume();
    void onPause();

    void render(uint64_t renderedFrameIndex);
    void renderLoading();

    void sendTrackingInfo(JNIEnv *env_, jobject udpReceiverThread);
    void sendMicData(JNIEnv *env_, jobject udpReceiverThread);

    void setFrameGeometry(int width, int height);

    bool isVrMode() { return Ovr != NULL; }

    int getLoadingTexture(){
        return loadingTexture;
    }
    int getSurfaceTextureID(){
        return SurfaceTextureID;
    }
    int getCameraTexture(){
        return CameraTexture;
    }

    void setRefreshRate(int refreshRate, bool forceChange = true);

    void getDeviceDescriptor(JNIEnv *env, jobject deviceDescriptor);

    void onHapticsFeedback(uint64_t startTime, float amplitude, float duration, float frequency, int hand);

    bool getButtonDown();

    void setStreamMic(bool streamMic);

   void setFFRParams(int foveationMode, float foveationStrength, float foveationShape, float foveationVerticalOffset);

private:
    ANativeWindow *window = NULL;
    ovrMobile *Ovr;
    ovrJava java;
    JNIEnv *env;


    int16_t* micBuffer;
    bool mStreamMic;
    size_t mMicMaxElements;

    ovrMicrophoneHandle mMicHandle;

    ovrVector3f lastControllerPos[2];
    double lastStateTime = 0;

    jobject mVrThread = nullptr;
    jobject mServerConnection = nullptr;

    bool UseMultiview = true;
    GLuint SurfaceTextureID = 0;
    GLuint loadingTexture = 0;
    GLuint CameraTexture = 0;
    int suspend = 0;
    bool Resumed = false;
    int FrameBufferWidth = 0;
    int FrameBufferHeight = 0;
    FOVEATION_MODE mFoveationMode = FOVEATION_MODE_DISABLED;
    float mFoveationStrength = 0;
    float mFoveationShape = 1.5;
    float mFoveationVerticalOffset = 0;
    FOVEATION_MODE usedFoveationMode = FOVEATION_MODE_DISABLED;
    float usedFoveationStrength = 0;
    float usedFoveationShape = 0;
    float usedFoveationVerticalOffset = 0;

    bool mExtraLatencyMode = false;

    static const int DEFAULT_REFRESH_RATE = 72;
    int m_currentRefreshRate = DEFAULT_REFRESH_RATE;

    uint64_t FrameIndex = 0;




    // For ARCore
    bool m_ARMode = false;
    float position_offset_y = 0.0f;
    bool previousHeadsetTrackpad = false;
    float previousHeadsetY = 0.0f;
    int g_AROverlayMode = 0; // 0: VR only, 1: AR 30% 2: AR 70% 3: AR 100%

    static const int MAXIMUM_TRACKING_FRAMES = 180;

    struct TrackingFrame {
        ovrTracking2 tracking;
        uint64_t frameIndex;
        uint64_t fetchTime;
        double displayTime;
    };
    typedef std::map<uint64_t, std::shared_ptr<TrackingFrame> > TRACKING_FRAME_MAP;

    TRACKING_FRAME_MAP trackingFrameMap;
    std::mutex trackingFrameMutex;

    ovrRenderer Renderer;

    jmethodID mServerConnection_send;

    struct HapticsState {
        uint64_t startUs;
        uint64_t endUs;
        float amplitude;
        float frequency;
        bool buffered;
    };
    // mHapticsState[0]: right hand state
    // mHapticsState[1]: left hand state
    HapticsState mHapticsState[2];

    // Previous trigger button state.
    bool mButtonPressed;
    uint64_t mapButtons(ovrInputTrackedRemoteCapabilities *remoteCapabilities, ovrInputStateTrackedRemote *remoteInputState);

    void setControllerInfo(TrackingInfo *packet, double displayTime);

    void setTrackingInfo(TrackingInfo *packet, double displayTime, ovrTracking2 *tracking  );

    void setInitialRefreshRate(int initialRefreshRate);


    void onVrModeChange();
    void enterVrMode();
    void leaveVrMode();

    void getRefreshRates(JNIEnv *env_, jintArray refreshRates);
    std::pair<EyeFov, EyeFov> getFov();

    void updateHapticsState();
    void finishHapticsBuffer(ovrDeviceID DeviceID);

    void reflectExtraLatencyMode(bool always);
};

#endif //ALVRCLIENT_VR_CONTEXT_H
