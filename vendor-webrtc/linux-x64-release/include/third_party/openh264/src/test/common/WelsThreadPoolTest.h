#ifndef _WELS_THREAD_POOL_TEST_H_
#define _WELS_THREAD_POOL_TEST_H_

#include "WelsLock.h"
#include "WelsThreadPool.h"

using namespace WelsCommon;

class CThreadPoolTest : public IWelsTaskSink {
 public:
  CThreadPoolTest() {
    m_iTaskCount = 0;
  }

  ~CThreadPoolTest() {}

  virtual int32_t OnTaskExecuted (IWelsTask* pTask) {
    WelsCommon::CWelsAutoLock cAutoLock (m_cTaskCountLock);
    m_iTaskCount ++;
    //fprintf(stdout, "Task execute over count is %d\n", m_iTaskCount);
    return cmResultSuccess;
  }

  virtual int32_t OnTaskCancelled (IWelsTask* pTask) {
    WelsCommon::CWelsAutoLock cAutoLock (m_cTaskCountLock);
    m_iTaskCount ++;
    //fprintf(stdout, "Task execute cancelled count is %d\n", m_iTaskCount);
    return cmResultSuccess;
  }

  virtual int32_t OnTaskExecuted() {
    WelsCommon::CWelsAutoLock cAutoLock (m_cTaskCountLock);
    m_iTaskCount ++;
    //fprintf(stdout, "Task execute over count is %d\n", m_iTaskCount);
    return cmResultSuccess;
  }

  virtual int32_t OnTaskCancelled() {
    WelsCommon::CWelsAutoLock cAutoLock (m_cTaskCountLock);
    m_iTaskCount ++;
    //fprintf(stdout, "Task execute cancelled count is %d\n", m_iTaskCount);
    return cmResultSuccess;
  }

  int32_t  GetTaskCount() {
    return m_iTaskCount;
  }

 private:
  int32_t  m_iTaskCount;
  WelsCommon::CWelsLock  m_cTaskCountLock;
};



class CSimpleTask : public IWelsTask {
 public:
  static uint32_t id;

  CSimpleTask (WelsCommon::IWelsTaskSink* pSink) : IWelsTask (pSink) {
    m_uiID = id ++;
  }

  virtual ~CSimpleTask() {
  }

  virtual int32_t Execute() {
    uint32_t uiSleepTime = (m_uiID > 99) ? 10 : m_uiID;
    WelsSleep (uiSleepTime);
    //printf ("Task %d executing\n", m_uiID);
    return cmResultSuccess;
  }

 private:
  uint32_t m_uiID;
};

#endif

