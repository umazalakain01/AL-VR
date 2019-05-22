#pragma once
#include <stdio.h>
#include <windows.h>
#include <mmsystem.h>
#include <mmdeviceapi.h>
#include <audioclient.h>
#include <avrt.h>
#include <functiondiscoverykeys_devpkey.h>
#include <wrl.h>
#include <ipctools.h>

#include "Logger.h"
#include "Settings.h"
#include "Utils.h"
#include "Listener.h"
#include "ResampleUtils.h"

using Microsoft::WRL::ComPtr;

class Handle {
public:
	Handle(HANDLE handle = NULL) : m_handle(handle) {
	}
	~Handle() {
		if (m_handle != NULL) {
			CloseHandle(m_handle);
		}
	}
	void Set(HANDLE handle) {
		m_handle = handle;
	}
	bool IsValid() {
		return m_handle != NULL;
	}
	HANDLE Get() {
		return m_handle;
	}

private:
	HANDLE m_handle;
};

class AudioEndPointDescriptor {
public:
	AudioEndPointDescriptor(const ComPtr<IMMDevice> &device, bool isDefault);
	std::wstring GetName() const;
	std::wstring GetId() const;
	bool IsDefault() const;
	bool operator==(const AudioEndPointDescriptor& a);
	bool operator!=(const AudioEndPointDescriptor& a);

	static std::wstring GetDeviceName(const ComPtr<IMMDevice> &pMMDevice);
private:
	std::wstring m_name;
	std::wstring m_id;
	bool m_isDefault;
};

class AudioCapture
{
public:
	AudioCapture(std::shared_ptr<Listener> listener);

	virtual ~AudioCapture();

	static void list_devices(std::vector<AudioEndPointDescriptor> &deviceList);

	void OpenDevice(const std::wstring &id);
	void Start(const std::wstring &id);

	void Shutdown();

	static DWORD WINAPI LoopbackCaptureThreadFunction(LPVOID pContext);
	void CaptureRetry();

	void LoopbackCapture();

	void WriteWaveHeader(HMMIO hFile, LPCWAVEFORMATEX pwfx, MMCKINFO *pckRIFF, MMCKINFO *pckData);
	void FinishWaveFile(HMMIO hFile, MMCKINFO *pckRIFF, MMCKINFO *pckData);
private:
	Handle m_hThread;
	std::shared_ptr<Listener> m_listener;

	ComPtr<IMMDevice> m_pMMDevice;
	PWAVEFORMATEX m_pwfx;
	UINT32 m_frames;

	IPCEvent m_startedEvent;
	IPCEvent m_stopEvent;

	bool m_canRetry;
	std::wstring m_errorMessage;

	static const int DEFAULT_SAMPLE_RATE = 48000;
};

