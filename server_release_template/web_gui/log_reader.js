const log_listener = new WebSocket("ws://127.0.0.1:8080/log");
log_listener.addEventListener('message', function (e) { alert(e.data) });

function sendSession() {
    var xhttp = new XMLHttpRequest();
    xhttp.open('POST', 'session', true);
    xhttp.setRequestHeader('Content-Type', 'application/json;charset=UTF-8');
    xhttp.send('{"setupWizard":true,"revertConfirmDialog":true,"lastClients":[],"settingsCache":{"video":{"adapterIndex":0,"refreshRate":72,"renderResolution":{"width":2880,"height":1600},"recommendedTargetResolution":{"width":2880,"height":1600},"eyeFov":[{"left":45.0,"right":45.0,"top":45.0,"bottom":45.0},{"left":45.0,"right":45.0,"top":45.0,"bottom":45.0}],"secondsFromVsyncToPhotons":0.005,"ipd":0.063,"foveatedRendering":{"enabled":false,"content":{"strength":2.0,"shape":1.5,"verticalOffset":0.0}},"colorCorrection":{"enabled":false,"content":{"brightness":0.0,"contrast":0.0,"saturation":0.0,"gamma":1.0,"sharpening":0.0}},"codec":{"variant":"H264"},"encodeBitrateMbs":30,"force60hz":false,"nv12":false},"audio":{"gameAudio":{"enabled":true,"content":{"device":""}},"microphone":{"enabled":false,"content":{"device":""}}},"headset":{"serialNumber":"1WMGH000XX0000","trackingSystemName":"oculus","modelNumber":"Oculus Rift S","driverVersion":"1.42.0","manufacturerName":"Oculus","renderModelName":"generic_hmd","registeredDeviceType":"oculus / 1WMGH000XX0000","trackingFrameOffset":0,"positionOffset":[0.0,0.0,0.0],"useTrackingReference":false,"force3dof":false,"controllers":{"enabled":true,"content":{"ctrlTrackingSystemName":"oculus","ctrlManufacturerName":"Oculus","ctrlModelNumber":"Oculus Rift S","renderModelNameLeft":"oculus_rifts_controller_left","renderModelNameRight":"oculus_rifts_controller_right","ctrlSerialNumber":"1WMGH000XX0000_Controller","ctrlType":"oculus_touch","ctrlRegisteredDeviceType":"oculus / 1WMGH000XX0000_Controller","inputProfilePath":"{ oculus } / input / touch_profile.json","triggerMode":24,"trackpadClickMode":28,"trackpadTouchMode":29,"backMode":0,"recenterButton":0,"poseTimeOffset":0,"positionOffsetLeft":[0.0,0.0,0.0],"rotationOffsetLeft":[36.0,0.0,0.0],"hapticsIntensity":1.0,"ctrlModeIdx":1}}},"connection":{"host":"0.0.0.0","port":9944,"controlHost":"0.0.0.0","controlPort":9944,"autoConnectHost":"","autoConnectPort":0,"throttlingBitrateMbs":0,"sendingTimeslotUs":500,"limitTimeslotPackets":0,"clientRecvBufferSize":60000,"frameQueueSize":1,"aggressiveKeyframeResend":false},"debug":{"log":true,"choiceTest":{"b":42,"c":{"testC":123.456},"variant":"b"}}}}');
}