#pragma once

#include <stdint.h>
#include "ipctools.h"
#include "Settings.h"

class IDRScheduler
{
public:
	IDRScheduler();
	~IDRScheduler();

	void OnPacketLoss();

	void OnStreamStart();
	void InsertIDR();

	bool CheckIDRInsertion();
private:
	static const int MIN_IDR_FRAME_INTERVAL = 100 * 1000; // 100-milliseconds
	static const int MIN_IDR_FRAME_INTERVAL_AGGRESSIVE = 5 * 1000; // 5-milliseconds (less than screen refresh interval)
	uint64_t m_insertIDRTime = 0;
	bool m_scheduled = false;
	IPCCriticalSection m_IDRCS;
	int m_minIDRFrameInterval = MIN_IDR_FRAME_INTERVAL;
};

