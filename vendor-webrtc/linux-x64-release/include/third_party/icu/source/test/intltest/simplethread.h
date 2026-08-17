// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2015, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef SIMPLETHREAD_H
#define SIMPLETHREAD_H

#include <thread>
#include "unicode/utypes.h"

/*
 * Simple class for creating threads in ICU tests.
 * Originally created to provide a portable abstraction over platform
 * (POSIX or Win32) threading interfaces.
 *
 * New threaded tests should consider skipping this class and directly using C++ std library
 * threading functions. SimpleThread is retained primarily to support existing use.
 */
class SimpleThread
{
  public:
    SimpleThread();
    virtual  ~SimpleThread();
    int32_t   start();            // start the thread. Return 0 if successful.
    void      join();             // A thread must be joined before deleting its SimpleThread.

    virtual void run() = 0;       // Override this to provide the code to run
                                  //   in the thread.
  private:
    std::thread fThread = {};
};


class IntlTest;

// ThreadPool - utililty class to simplify the spawning a group of threads by
//              a multi-threaded test.
//
//   Usage: from within an intltest test function,
//       ThreadPool<TestClass> pool(
//               this,              // The current intltest test object,
//                                  //     of type "TestClass *"
//               numberOfThreads,   // How many threads to spawn.
//               &TestClass::func); // The function to be run by each thread.
//                                  //     It takes one int32_t parameter which
//                                  //     is set to the thread number, 0 to numberOfThreads-1.
//
//       pool.start();              // Start all threads running.
//       pool.join();               // Wait until all threads have terminated.

class ThreadPoolBase {
  public:
    ThreadPoolBase(IntlTest *test, int32_t numThreads);
    virtual ~ThreadPoolBase();
    
    void start();
    void join();

  protected:
    virtual void callFn(int32_t param) = 0;
    friend class ThreadPoolThread;

    IntlTest  *fIntlTest;
    int32_t  fNumThreads;
    SimpleThread **fThreads;
};


template<class TestClass>
class ThreadPool : public ThreadPoolBase {
  private:
    void (TestClass::*fRunFnPtr)(int32_t);
  public:
    ThreadPool(TestClass *test, int howMany, void (TestClass::*runFnPtr)(int32_t threadNumber)) :
        ThreadPoolBase(test, howMany), fRunFnPtr(runFnPtr) {}
    virtual ~ThreadPool() {}
  private:
    virtual void callFn(int32_t param) override {
        TestClass *test = dynamic_cast<TestClass *>(fIntlTest);
        (test->*fRunFnPtr)(param);
    }
};
#endif
