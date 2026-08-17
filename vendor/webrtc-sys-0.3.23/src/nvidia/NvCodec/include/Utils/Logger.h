/*
* Copyright 2017-2022 NVIDIA Corporation.  All rights reserved.
*
* Please refer to the NVIDIA end user license agreement (EULA) associated
* with this source code for terms and conditions that govern your use of
* this software. Any use, reproduction, disclosure, or distribution of
* this software and related documentation outside the terms of the EULA
* is strictly prohibited.
*
*/

#pragma once

#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <mutex>
#include <time.h>

#ifdef _WIN32
#include <winsock.h>
#include <windows.h>

#pragma comment(lib, "ws2_32.lib")
#undef ERROR
#else
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#define SOCKET int
#define INVALID_SOCKET -1
#endif

enum LogLevel {
    TRACE,
    INFO,
    WARNING,
    ERROR,
    FATAL
};

namespace simplelogger{
class Logger {
public:
    Logger(LogLevel level, bool bPrintTimeStamp) : level(level), bPrintTimeStamp(bPrintTimeStamp) {}
    virtual ~Logger() {}
    virtual std::ostream& GetStream() = 0;
    virtual void FlushStream() {}
    bool ShouldLogFor(LogLevel l) {
        return l >= level;
    }
    char* GetLead(LogLevel l, const char *szFile, int nLine, const char *szFunc) {
        if (l < TRACE || l > FATAL) {
            sprintf(szLead, "[?????] ");
            return szLead;
        }
        const char *szLevels[] = {"TRACE", "INFO", "WARN", "ERROR", "FATAL"};
        if (bPrintTimeStamp) {
            time_t t = time(NULL);
            struct tm *ptm = localtime(&t);
            sprintf(szLead, "[%-5s][%02d:%02d:%02d] ", 
                szLevels[l], ptm->tm_hour, ptm->tm_min, ptm->tm_sec);
        } else {
            sprintf(szLead, "[%-5s] ", szLevels[l]);
        }
        return szLead;
    }
    void EnterCriticalSection() {
        mtx.lock();
    }
    void LeaveCriticalSection() {
        mtx.unlock();
    }
private:
    LogLevel level;
    char szLead[80];
    bool bPrintTimeStamp;
    std::mutex mtx;
};

class LoggerFactory {
public:
    static Logger* CreateFileLogger(std::string strFilePath, 
            LogLevel level = INFO, bool bPrintTimeStamp = true) {
        return new FileLogger(strFilePath, level, bPrintTimeStamp);
    }
    static Logger* CreateConsoleLogger(LogLevel level = INFO, 
            bool bPrintTimeStamp = true) {
        return new ConsoleLogger(level, bPrintTimeStamp);
    }
    static Logger* CreateUdpLogger(char *szHost, unsigned uPort, LogLevel level = INFO, 
            bool bPrintTimeStamp = true) {
        return new UdpLogger(szHost, uPort, level, bPrintTimeStamp);
    }
private:
    LoggerFactory() {}

    class FileLogger : public Logger {
    public:
        FileLogger(std::string strFilePath, LogLevel level, bool bPrintTimeStamp) 
        : Logger(level, bPrintTimeStamp) {
            pFileOut = new std::ofstream();
            pFileOut->open(strFilePath.c_str());
        }
        ~FileLogger() {
            pFileOut->close();
        }
        std::ostream& GetStream() {
            return *pFileOut;
        }
    private:
        std::ofstream *pFileOut;
    };

    class ConsoleLogger : public Logger {
    public:
        ConsoleLogger(LogLevel level, bool bPrintTimeStamp) 
        : Logger(level, bPrintTimeStamp) {}
        std::ostream& GetStream() {
            return std::cout;
        }
    };

    class UdpLogger : public Logger {
    private:
        class UdpOstream : public std::ostream {
        public:
            UdpOstream(char *szHost, unsigned short uPort) : std::ostream(&sb), socket(INVALID_SOCKET){
#ifdef _WIN32
                WSADATA w;
                if (WSAStartup(0x0101, &w) != 0) {
                    fprintf(stderr, "WSAStartup() failed.\n");
                    return;
                }
#endif
                socket = ::socket(AF_INET, SOCK_DGRAM, 0);
                if (socket == INVALID_SOCKET) {
#ifdef _WIN32
                    WSACleanup();
#endif
                    fprintf(stderr, "socket() failed.\n");
                    return;
                }
#ifdef _WIN32
                unsigned int b1, b2, b3, b4;
                sscanf(szHost, "%u.%u.%u.%u", &b1, &b2, &b3, &b4);
                struct in_addr addr = {(unsigned char)b1, (unsigned char)b2, (unsigned char)b3, (unsigned char)b4};
#else
                struct in_addr addr = {inet_addr(szHost)};
#endif
                struct sockaddr_in s = {AF_INET, htons(uPort), addr};
                server = s;
            }
            ~UdpOstream() throw() {
                if (socket == INVALID_SOCKET) {
                    return;
                }
#ifdef _WIN32
                closesocket(socket);
                WSACleanup();
#else
                close(socket);
#endif
            }
            void Flush() {
                if (sendto(socket, sb.str().c_str(), (int)sb.str().length() + 1, 
                        0, (struct sockaddr *)&server, (int)sizeof(sockaddr_in)) == -1) {
                    fprintf(stderr, "sendto() failed.\n");
                }
                sb.str("");
            }

        private:
            std::stringbuf sb;
            SOCKET socket;
            struct sockaddr_in server;
        };
    public:
        UdpLogger(char *szHost, unsigned uPort, LogLevel level, bool bPrintTimeStamp) 
        : Logger(level, bPrintTimeStamp), udpOut(szHost, (unsigned short)uPort) {}
        UdpOstream& GetStream() {
            return udpOut;
        }
        virtual void FlushStream() {
            udpOut.Flush();
        }
    private:
        UdpOstream udpOut;
    };
};

class LogTransaction {
public:
    LogTransaction(Logger *pLogger, LogLevel level, const char *szFile, const int nLine, const char *szFunc) : pLogger(pLogger), level(level) {
        if (!pLogger) {
            std::cout << "[-----] ";
            return;
        }
        if (!pLogger->ShouldLogFor(level)) {
            return;
        }
        pLogger->EnterCriticalSection();
        pLogger->GetStream() << pLogger->GetLead(level, szFile, nLine, szFunc);
    }
    ~LogTransaction() {
        if (!pLogger) {
            std::cout << std::endl;
            return;
        }
        if (!pLogger->ShouldLogFor(level)) {
            return;
        }
        pLogger->GetStream() << std::endl;
        pLogger->FlushStream();
        pLogger->LeaveCriticalSection();
        if (level == FATAL) {
            exit(1);
        }
    }
    std::ostream& GetStream() {
        if (!pLogger) {
            return std::cout;
        }
        if (!pLogger->ShouldLogFor(level)) {
            return ossNull;
        }
        return pLogger->GetStream();
    }
private:
    Logger *pLogger;
    LogLevel level;
    std::ostringstream ossNull;
};

}

extern simplelogger::Logger *logger;
#define LOG(level) simplelogger::LogTransaction(logger, level, __FILE__, __LINE__, __FUNCTION__).GetStream()
