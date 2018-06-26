# ALVR - Air Light VR

ALVR is an open source remote VR display for Gear VR and Oculus Go. With it, you can play SteamVR games in your standalone headset.

English | [Japanese](https://github.com/polygraphene/ALVR/blob/master/README-ja.md)

## Description

ALVR streams VR display output from your PC to Gear VR / Oculus Go via Wi-Fi. This is similar to Riftcat or Trinus VR, but our purpose is optimization for Gear VR. ALVR provides smooth head-tracking compared to other apps in a Wi-Fi environment using Asynchronous Timewarp.

Now, we have Gear VR / Oculus Go Controller support!

Note that many PCVR games require 6DoF controller or multiple buttons, so you might not able to play those games.
You can find playable games in [List of tested VR games and experiences](https://github.com/polygraphene/ALVR/wiki/List-of-tested-VR-games-and-experiences).

I have started crowdfunding about support for AMD GPU on [this page](https://www.bountysource.com/issues/59270271-will-we-get-an-amd-compatible-version-in-the-future). Also refer issue [#33](https://github.com/polygraphene/ALVR/issues/33).
    
## Requirements

ALVR requires any of the following devices:

- Gear VR
- Oculus Go

|Device|Working?|
|---|---|
|Oculus Go|OK|
|GalaxyS9/S9+|OK|
|GalaxyS8/S8+|OK|
|GalaxyS7|OK|
|GalaxyS6(Edge)|OK|

- High-end gaming PC with NVIDIA GPU which supports NVENC
    - Only Windows 10 is supported
- 802.11n/ac wireless or ethernet wired connection
    - It is recommended to use 802.11ac for the headset and ethernet for PC
        - You need to connect both to the same router
- SteamVR

## Installation

### Install ALVR server for PC

1. Install SteamVR
2. Download and install vc\_redist.x64.exe from [here](https://go.microsoft.com/fwlink/?LinkId=746572)
3. Download zip from [Releases](https://github.com/polygraphene/ALVR/releases)
4. Extract the zip to any folder
5. (First run only) Make sure the ALVR driver is installed in Steam
  * Use the driver_install.bat helper located in ALVR\driver to install the driver if your SteamVR installation is in the default C:\Program Files (x86)\Steam\steamapps\common\SteamVR location
  * If SteamVR is installed in any other location, use `[SteamVR install dir]\bin\win32\vrpathreg.exe adddriver [drive-and-path-to-ALVR\driver]` or simply edit the driver_install.bat and driver_uninstall.bat to reflect the location of your SteamVR install directory.
6. Launch ALVR.exe

### Install ALVR client for headset

### From Oculus Store

- You can download ALVR Client from Oculus Store with key.
- Open [the key distribution page](https://alvr-dist.appspot.com/) on your smartphone and follow the instruction.

### Install from apk

- Check [Installation](https://github.com/polygraphene/ALVR/wiki/Installation).

## Usage

- Launch ALVR.exe
- Press "Start Server" button or launch VR game
- SteamVR's small window will appear. You should see a headset icon in the SteamVR status window that looks like a green block with a bold S in the middle
- Launch ALVR Client in your headset
- IP Address of headset will appear in the server tab of ALVR.exe
- Press "Connect" button

## Troubleshoot

- If you got some error, please check [Troubleshooting](https://github.com/polygraphene/ALVR/wiki/Troubleshooting)

## Uninstallation

- Execute driver\_uninstall.bat in the driver folder
- Delete the install folder (ALVR does not use the registry)
- If you already deleted the folder without executing driver\_uninstall.bat:
    - Open C:\Users\\%USERNAME%\AppData\Local\openvr\openvrpaths.vrpath and check install directory
    - Execute
    `"C:\Program Files (x86)\Steam\steamapps\common\SteamVR\bin\win32\vrpathreg.exe" removedriver (install folder)`
    in Command Prompt

## Future work

- Support H.265 hevc encoding (currently H.264 only)
- AMD support [#33](https://github.com/polygraphene/ALVR/issues/33)
- Windows 7 support
- Better installer

## Build

### ALVR Server and GUI (Launcher)

- Open ALVR.sln with Visual Studio 2017 and build
    - alvr\_server project is the driver for SteamVR written in C++
    - ALVR project is the launcher GUI written in C#

### ALVR Client

- Clone [ALVR Client](https://github.com/polygraphene/ALVRClient)
- Put your [osig file](https://developer.oculus.com/documentation/mobilesdk/latest/concepts/mobile-submission-sig-file/) on assets folder (only for Gear VR)
- Build with Android Studio
- Install apk via adb

## License

ALVR is licensed under MIT License.

## Donate

If you like this project, please donate!

#### Donate by paypal

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=polygraphene@gmail.com&lc=US&item_name=Donate+for+ALVR+developer&no_note=0&cn=&curency_code=USD&bn=PP-DonationsBF:btn_donateCC_LG.gif:NonHosted)
If you could not use this link, please try the following.
1. Login your paypal account
2. Open "Send and request" tab
3. Click "Pay for goods or services"
4. Put "polygraphene@gmail.com" (it is my paypal account) and click next

#### Donate by bitcoin

bitcoin:1FCbmFVSjsmpnAj6oLx2EhnzQzzhyxTLEv
