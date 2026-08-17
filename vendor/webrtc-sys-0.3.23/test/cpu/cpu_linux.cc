/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#include "cpu_linux.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

namespace webrtc {
CpuLinux::CpuLinux()
    : m_oldBusyTime(0),
      m_oldIdleTime(0),
      m_oldBusyTimeMulti(NULL),
      m_oldIdleTimeMulti(NULL),
      m_idleArray(NULL),
      m_busyArray(NULL),
      m_resultArray(NULL),
      m_numCores(0) {
    const int result = GetNumCores();
    if (result != -1) {
      m_numCores = result;
      m_oldBusyTimeMulti = new long long[m_numCores];
      memset(m_oldBusyTimeMulti, 0, sizeof(long long) * m_numCores);
      m_oldIdleTimeMulti = new long long[m_numCores];
      memset(m_oldIdleTimeMulti, 0, sizeof(long long) * m_numCores);
      m_idleArray = new long long[m_numCores];
      memset(m_idleArray, 0, sizeof(long long) * m_numCores);
      m_busyArray = new long long[m_numCores];
      memset(m_busyArray, 0, sizeof(long long) * m_numCores);
      m_resultArray = new uint32_t[m_numCores];

      GetData(m_oldBusyTime, m_oldIdleTime, m_busyArray, m_idleArray);
    }
}

CpuLinux::~CpuLinux()
{
    delete [] m_oldBusyTimeMulti;
    delete [] m_oldIdleTimeMulti;
    delete [] m_idleArray;
    delete [] m_busyArray;
    delete [] m_resultArray;
}

int32_t CpuLinux::CpuUsage()
{
    uint32_t dummy = 0;
    uint32_t* dummyArray = NULL;
    return CpuUsageMultiCore(dummy, dummyArray);
}

int32_t CpuLinux::CpuUsageMultiCore(uint32_t& numCores,
                                          uint32_t*& coreArray)
{
    coreArray = m_resultArray;
    numCores = m_numCores;
    long long busy = 0;
    long long idle = 0;
    if (GetData(busy, idle, m_busyArray, m_idleArray) != 0)
        return -1;

    long long deltaBusy = busy - m_oldBusyTime;
    long long deltaIdle = idle - m_oldIdleTime;
    m_oldBusyTime = busy;
    m_oldIdleTime = idle;

    int retVal = -1;
    if (deltaBusy + deltaIdle == 0)
    {
        retVal = 0;
    }
    else
    {
        retVal = (int)(100 * (deltaBusy) / (deltaBusy + deltaIdle));
    }

    if (coreArray == NULL)
    {
      return retVal;
    }

    for (int32_t i = 0; i < m_numCores; i++)
    {
        deltaBusy = m_busyArray[i] - m_oldBusyTimeMulti[i];
        deltaIdle = m_idleArray[i] - m_oldIdleTimeMulti[i];
        m_oldBusyTimeMulti[i] = m_busyArray[i];
        m_oldIdleTimeMulti[i] = m_idleArray[i];
        if(deltaBusy + deltaIdle == 0)
        {
            coreArray[i] = 0;
        }
        else
        {
            coreArray[i] = (int)(100 * (deltaBusy) / (deltaBusy+deltaIdle));
        }
    }
    return retVal;
}


int CpuLinux::GetData(long long& busy, long long& idle, long long*& busyArray,
                      long long*& idleArray)
{
    FILE* fp = fopen("/proc/stat", "r");
    if (!fp)
    {
        return -1;
    }

    char line[100];
    if (fgets(line, 100, fp) == NULL) {
        fclose(fp);
        return -1;
    }
    char firstWord[100];
    if (sscanf(line, "%s ", firstWord) != 1) {
        fclose(fp);
        return -1;
    }
    if (strncmp(firstWord, "cpu", 3) != 0) {
        fclose(fp);
        return -1;
    }
    char sUser[100];
    char sNice[100];
    char sSystem[100];
    char sIdle[100];
    if (sscanf(line, "%s %s %s %s %s ",
               firstWord, sUser, sNice, sSystem, sIdle) != 5) {
        fclose(fp);
        return -1;
    }
    long long luser = atoll(sUser);
    long long lnice = atoll(sNice);
    long long lsystem = atoll(sSystem);
    long long lidle = atoll (sIdle);

    busy = luser + lnice + lsystem;
    idle = lidle;
    for (int32_t i = 0; i < m_numCores; i++)
    {
        if (fgets(line, 100, fp) == NULL) {
            fclose(fp);
            return -1;
        }
        if (sscanf(line, "%s %s %s %s %s ", firstWord, sUser, sNice, sSystem,
                   sIdle) != 5) {
            fclose(fp);
            return -1;
        }
        luser = atoll(sUser);
        lnice = atoll(sNice);
        lsystem = atoll(sSystem);
        lidle = atoll (sIdle);
        busyArray[i] = luser + lnice + lsystem;
        idleArray[i] = lidle;
    }
    fclose(fp);
    return 0;
}

int CpuLinux::GetNumCores()
{
    FILE* fp = fopen("/proc/stat", "r");
    if (!fp)
    {
        return -1;
    }
    // Skip first line
    char line[100];
    if (!fgets(line, 100, fp))
    {
        fclose(fp);
        return -1;
    }
    int numCores = -1;
    char firstWord[100];
    do
    {
        numCores++;
        if (fgets(line, 100, fp))
        {
            if (sscanf(line, "%s ", firstWord) != 1) {
                firstWord[0] = '\0';
            }
        } else {
            break;
        }
    } while (strncmp(firstWord, "cpu", 3) == 0);
    fclose(fp);
    return numCores;
}

CpuWrapper* CpuWrapper::CreateCpu()
{
    return new CpuLinux();
}

} // namespace webrtc
