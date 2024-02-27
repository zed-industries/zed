__declspec(dllimport) extern "C" unsigned int __stdcall GetCurrentThreadId();

extern "C" unsigned int MAIN_THREAD_ID;
unsigned int MAIN_THREAD_ID = GetCurrentThreadId();
