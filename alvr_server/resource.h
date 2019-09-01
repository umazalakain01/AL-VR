#define IDR_FONT 1000
#define IDR_VS 1001
#define IDR_PS 1002
#define IDR_RECENTER_TEXTURE 1003
#define IDR_MESSAGE_BG_TEXTURE 1004
#define IDR_QUAD_SHADER 1005
#define IDR_HORIZ_BLUR_SHADER 1006
#define IDR_DISTORTION_SHADER 1007

#define APP_VERSION_MAJOR 2
#define APP_VERSION_MINOR 4
#define APP_VERSION_PATCH 0
#define APP_VERSION_SUFFIX "-experimental v5"
#define APP_VERSION_STRING__(major, minor, patch, suffix) #major "." #minor "." #patch "" suffix
#define APP_VERSION_STRING_(major, minor, patch, suffix) APP_VERSION_STRING__(major, minor, patch, suffix)
#define APP_VERSION_STRING APP_VERSION_STRING_(APP_VERSION_MAJOR, APP_VERSION_MINOR, APP_VERSION_PATCH, APP_VERSION_SUFFIX)

#define APP_NAME "ALVR"
#define APP_MODULE_NAME "ALVR Server Driver"

#define APP_MUTEX_NAME "ALVR_DRIVER_MUTEX_67F10C68-E26D-4ABE-AE25-32BF346D877B"
#define APP_FILEMAPPING_NAME "ALVR_DRIVER_FILEMAPPING_0B124897-7730-4B84-AA32-088E9B92851F"

