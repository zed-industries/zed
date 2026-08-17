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

//---------------------------------------------------------------------------
//! \file NvCodecUtils.h
//! \brief Miscellaneous classes and error checking functions.
//!
//! Used by Transcode/Encode samples apps for reading input files, mutithreading, performance measurement or colorspace conversion while decoding.
//---------------------------------------------------------------------------

#pragma once
#include <iomanip>
#include <chrono>
#include <sys/stat.h>
#include <assert.h>
#include <stdint.h>
#include <string.h>
#include <ios>
#include <sstream>
#include <thread>
#include <list>
#include <vector>
#include <condition_variable>

#include "Logger.h"

extern simplelogger::Logger *logger;

#ifdef __cuda_cuda_h__
inline bool check(CUresult e, int iLine, const char *szFile) {
    if (e != CUDA_SUCCESS) {
        const char *szErrName = NULL;
        cuGetErrorName(e, &szErrName);
        LOG(FATAL) << "CUDA driver API error " << szErrName << " at line " << iLine << " in file " << szFile;
        return false;
    }
    return true;
}
#endif

#ifdef __CUDA_RUNTIME_H__
inline bool check(cudaError_t e, int iLine, const char *szFile) {
    if (e != cudaSuccess) {
        LOG(FATAL) << "CUDA runtime API error " << cudaGetErrorName(e) << " at line " << iLine << " in file " << szFile;
        return false;
    }
    return true;
}
#endif

#ifdef _NV_ENCODEAPI_H_
inline bool check(NVENCSTATUS e, int iLine, const char *szFile) {
    const char *aszErrName[] = {
        "NV_ENC_SUCCESS",
        "NV_ENC_ERR_NO_ENCODE_DEVICE",
        "NV_ENC_ERR_UNSUPPORTED_DEVICE",
        "NV_ENC_ERR_INVALID_ENCODERDEVICE",
        "NV_ENC_ERR_INVALID_DEVICE",
        "NV_ENC_ERR_DEVICE_NOT_EXIST",
        "NV_ENC_ERR_INVALID_PTR",
        "NV_ENC_ERR_INVALID_EVENT",
        "NV_ENC_ERR_INVALID_PARAM",
        "NV_ENC_ERR_INVALID_CALL",
        "NV_ENC_ERR_OUT_OF_MEMORY",
        "NV_ENC_ERR_ENCODER_NOT_INITIALIZED",
        "NV_ENC_ERR_UNSUPPORTED_PARAM",
        "NV_ENC_ERR_LOCK_BUSY",
        "NV_ENC_ERR_NOT_ENOUGH_BUFFER",
        "NV_ENC_ERR_INVALID_VERSION",
        "NV_ENC_ERR_MAP_FAILED",
        "NV_ENC_ERR_NEED_MORE_INPUT",
        "NV_ENC_ERR_ENCODER_BUSY",
        "NV_ENC_ERR_EVENT_NOT_REGISTERD",
        "NV_ENC_ERR_GENERIC",
        "NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY",
        "NV_ENC_ERR_UNIMPLEMENTED",
        "NV_ENC_ERR_RESOURCE_REGISTER_FAILED",
        "NV_ENC_ERR_RESOURCE_NOT_REGISTERED",
        "NV_ENC_ERR_RESOURCE_NOT_MAPPED",
    };
    if (e != NV_ENC_SUCCESS) {
        LOG(FATAL) << "NVENC error " << aszErrName[e] << " at line " << iLine << " in file " << szFile;
        return false;
    }
    return true;
}
#endif

#ifdef _WINERROR_
inline bool check(HRESULT e, int iLine, const char *szFile) {
    if (e != S_OK) {
        std::stringstream stream;
        stream << std::hex << std::uppercase << e;
        LOG(FATAL) << "HRESULT error 0x" << stream.str() << " at line " << iLine << " in file " << szFile;
        return false;
    }
    return true;
}
#endif

#if defined(__gl_h_) || defined(__GL_H__)
inline bool check(GLenum e, int iLine, const char *szFile) {
    if (e != 0) {
        LOG(ERROR) << "GLenum error " << e << " at line " << iLine << " in file " << szFile;
        return false;
    }
    return true;
}
#endif

inline bool check(int e, int iLine, const char *szFile) {
    if (e < 0) {
        LOG(ERROR) << "General error " << e << " at line " << iLine << " in file " << szFile;
        return false;
    }
    return true;
}

#define ck(call) check(call, __LINE__, __FILE__)
#define MAKE_FOURCC( ch0, ch1, ch2, ch3 )                               \
                ( (uint32_t)(uint8_t)(ch0) | ( (uint32_t)(uint8_t)(ch1) << 8 ) |    \
                ( (uint32_t)(uint8_t)(ch2) << 16 ) | ( (uint32_t)(uint8_t)(ch3) << 24 ) )

/**
* @brief Wrapper class around std::thread
*/
class NvThread
{
public:
    NvThread() = default;
    NvThread(const NvThread&) = delete;
    NvThread& operator=(const NvThread& other) = delete;

    NvThread(std::thread&& thread) : t(std::move(thread))
    {

    }

    NvThread(NvThread&& thread) : t(std::move(thread.t))
    {

    }

    NvThread& operator=(NvThread&& other)
    {
        t = std::move(other.t);
        return *this;
    }

    ~NvThread()
    {
        join();
    }

    void join()
    {
        if (t.joinable())
        {
            t.join();
        }
    }
private:
    std::thread t;
};

#ifndef _WIN32
#define _stricmp strcasecmp
#define _stat64 stat64
#endif

/**
* @brief Utility class to allocate buffer memory. Helps avoid I/O during the encode/decode loop in case of performance tests.
*/
class BufferedFileReader {
public:
    /**
    * @brief Constructor function to allocate appropriate memory and copy file contents into it
    */
    BufferedFileReader(const char *szFileName, bool bPartial = false) {
        struct _stat64 st;

        if (_stat64(szFileName, &st) != 0) {
            return;
        }
        
        nSize = st.st_size;
        while (nSize) {
            try {
                pBuf = new uint8_t[(size_t)nSize];
                if (nSize != st.st_size) {
                    LOG(WARNING) << "File is too large - only " << std::setprecision(4) << 100.0 * nSize / st.st_size << "% is loaded"; 
                }
                break;
            } catch(std::bad_alloc) {
                if (!bPartial) {
                    LOG(ERROR) << "Failed to allocate memory in BufferedReader";
                    return;
                }
                nSize = (uint32_t)(nSize * 0.9);
            }
        }

        std::ifstream fpIn(szFileName, std::ifstream::in | std::ifstream::binary);
        if (!fpIn)
        {
            LOG(ERROR) << "Unable to open input file: " << szFileName;
            return;
        }

        std::streamsize nRead = fpIn.read(reinterpret_cast<char*>(pBuf), nSize).gcount();
        fpIn.close();

        assert(nRead == nSize);
    }
    ~BufferedFileReader() {
        if (pBuf) {
            delete[] pBuf;
        }
    }
    bool GetBuffer(uint8_t **ppBuf, uint64_t *pnSize) {
        if (!pBuf) {
            return false;
        }

        *ppBuf = pBuf;
        *pnSize = nSize;
        return true;
    }

private:
    uint8_t *pBuf = NULL;
    uint64_t nSize = 0;
};

/**
* @brief Template class to facilitate color space conversion
*/
template<typename T>
class YuvConverter {
public:
    YuvConverter(int nWidth, int nHeight) : nWidth(nWidth), nHeight(nHeight) {
        pQuad = new T[((nWidth + 1) / 2) * ((nHeight + 1) / 2)];
    }
    ~YuvConverter() {
        delete[] pQuad;
    }
    void PlanarToUVInterleaved(T *pFrame, int nPitch = 0) {
        if (nPitch == 0) {
            nPitch = nWidth;
        }

        // sizes of source surface plane
        int nSizePlaneY = nPitch * nHeight;
        int nSizePlaneU = ((nPitch + 1) / 2) * ((nHeight + 1) / 2);
        int nSizePlaneV = nSizePlaneU;

        T *puv = pFrame + nSizePlaneY;
        if (nPitch == nWidth) {
            memcpy(pQuad, puv, nSizePlaneU * sizeof(T));
        } else {
            for (int i = 0; i < (nHeight + 1) / 2; i++) {
                memcpy(pQuad + ((nWidth + 1) / 2) * i, puv + ((nPitch + 1) / 2) * i, ((nWidth + 1) / 2) * sizeof(T));
            }
        }
        T *pv = puv + nSizePlaneU;
        for (int y = 0; y < (nHeight + 1) / 2; y++) {
            for (int x = 0; x < (nWidth + 1) / 2; x++) {
                puv[y * nPitch + x * 2] = pQuad[y * ((nWidth + 1) / 2) + x];
                puv[y * nPitch + x * 2 + 1] = pv[y * ((nPitch + 1) / 2) + x];
            }
        }
    }
    void UVInterleavedToPlanar(T *pFrame, int nPitch = 0) {
        if (nPitch == 0) {
            nPitch = nWidth;
        }

        // sizes of source surface plane
        int nSizePlaneY = nPitch * nHeight;
        int nSizePlaneU = ((nPitch + 1) / 2) * ((nHeight + 1) / 2);
        int nSizePlaneV = nSizePlaneU;

        T *puv = pFrame + nSizePlaneY,
            *pu = puv, 
            *pv = puv + nSizePlaneU;

        // split chroma from interleave to planar
        for (int y = 0; y < (nHeight + 1) / 2; y++) {
            for (int x = 0; x < (nWidth + 1) / 2; x++) {
                pu[y * ((nPitch + 1) / 2) + x] = puv[y * nPitch + x * 2];
                pQuad[y * ((nWidth + 1) / 2) + x] = puv[y * nPitch + x * 2 + 1];
            }
        }
        if (nPitch == nWidth) {
            memcpy(pv, pQuad, nSizePlaneV * sizeof(T));
        } else {
            for (int i = 0; i < (nHeight + 1) / 2; i++) {
                memcpy(pv + ((nPitch + 1) / 2) * i, pQuad + ((nWidth + 1) / 2) * i, ((nWidth + 1) / 2) * sizeof(T));
            }
        }
    }

private:
    T *pQuad;
    int nWidth, nHeight;
};

/**
* @brief Class for writing IVF format header for AV1 codec
*/
class IVFUtils {
public:
    void WriteFileHeader(std::vector<uint8_t> &vPacket, uint32_t nFourCC, uint32_t nWidth, uint32_t nHeight, uint32_t nFrameRateNum, uint32_t nFrameRateDen, uint32_t nFrameCnt)
    {
        char header[32];

        header[0] = 'D';
        header[1] = 'K';
        header[2] = 'I';
        header[3] = 'F';
        mem_put_le16(header + 4, 0);                    // version
        mem_put_le16(header + 6, 32);                   // header size
        mem_put_le32(header + 8, nFourCC);              // fourcc
        mem_put_le16(header + 12, nWidth);              // width
        mem_put_le16(header + 14, nHeight);             // height
        mem_put_le32(header + 16, nFrameRateNum);       // rate
        mem_put_le32(header + 20, nFrameRateDen);       // scale
        mem_put_le32(header + 24, nFrameCnt);           // length
        mem_put_le32(header + 28, 0);                   // unused

        vPacket.insert(vPacket.end(), &header[0], &header[32]);
    }
    
    void WriteFrameHeader(std::vector<uint8_t> &vPacket,  size_t nFrameSize, int64_t pts)
    {
        char header[12];
        mem_put_le32(header, (int)nFrameSize);
        mem_put_le32(header + 4, (int)(pts & 0xFFFFFFFF));
        mem_put_le32(header + 8, (int)(pts >> 32));
        
        vPacket.insert(vPacket.end(), &header[0], &header[12]);
    }
    
private:
    static inline void mem_put_le32(void *vmem, int val)
    {
        unsigned char *mem = (unsigned char *)vmem;
        mem[0] = (unsigned char)((val >>  0) & 0xff);
        mem[1] = (unsigned char)((val >>  8) & 0xff);
        mem[2] = (unsigned char)((val >> 16) & 0xff);
        mem[3] = (unsigned char)((val >> 24) & 0xff);
    }

    static inline void mem_put_le16(void *vmem, int val)
    {
        unsigned char *mem = (unsigned char *)vmem;
        mem[0] = (unsigned char)((val >>  0) & 0xff);
        mem[1] = (unsigned char)((val >>  8) & 0xff);
    }

};
    
/**
* @brief Utility class to measure elapsed time in seconds between the block of executed code
*/
class StopWatch {
public:
    void Start() {
        t0 = std::chrono::high_resolution_clock::now();
    }
    double Stop() {
        return std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::high_resolution_clock::now().time_since_epoch() - t0.time_since_epoch()).count() / 1.0e9;
    }

private:
    std::chrono::high_resolution_clock::time_point t0;
};

template<typename T>
class ConcurrentQueue
{
    public:

    ConcurrentQueue() {}
    ConcurrentQueue(size_t size) : maxSize(size) {}
    ConcurrentQueue(const ConcurrentQueue&) = delete;
    ConcurrentQueue& operator=(const ConcurrentQueue&) = delete;

    void setSize(size_t s) {
        maxSize = s;
    }

    void push_back(const T& value) {
        // Do not use a std::lock_guard here. We will need to explicitly
        // unlock before notify_one as the other waiting thread will
        // automatically try to acquire mutex once it wakes up
        // (which will happen on notify_one)
        std::unique_lock<std::mutex> lock(m_mutex);
        auto wasEmpty = m_List.empty();

        while (full()) {
            m_cond.wait(lock);
        }

        m_List.push_back(value);
        if (wasEmpty && !m_List.empty()) {
            lock.unlock();
            m_cond.notify_one();
        }
    }

    T pop_front() {
        std::unique_lock<std::mutex> lock(m_mutex);

        while (m_List.empty()) {
            m_cond.wait(lock);
        }
        auto wasFull = full();
        T data = std::move(m_List.front());
        m_List.pop_front();

        if (wasFull && !full()) {
            lock.unlock();
            m_cond.notify_one();
        }

        return data;
    }

    T front() {
        std::unique_lock<std::mutex> lock(m_mutex);

        while (m_List.empty()) {
            m_cond.wait(lock);
        }

        return m_List.front();
    }

    size_t size() {
        std::unique_lock<std::mutex> lock(m_mutex);
        return m_List.size();
    }

    bool empty() {
        std::unique_lock<std::mutex> lock(m_mutex);
        return m_List.empty();
    }
    void clear() {
        std::unique_lock<std::mutex> lock(m_mutex);
        m_List.clear();
    }

private:
    bool full() {
        if (m_List.size() == maxSize)
            return true;
        return false;
    }

private:
    std::list<T> m_List;
    std::mutex m_mutex;
    std::condition_variable m_cond;
    size_t maxSize;
};

inline void CheckInputFile(const char *szInFilePath) {
    std::ifstream fpIn(szInFilePath, std::ios::in | std::ios::binary);
    if (fpIn.fail()) {
        std::ostringstream err;
        err << "Unable to open input file: " << szInFilePath << std::endl;
        throw std::invalid_argument(err.str());
    }
}

inline void ValidateResolution(int nWidth, int nHeight) {
    
    if (nWidth <= 0 || nHeight <= 0) {
        std::ostringstream err;
        err << "Please specify positive non zero resolution as -s WxH. Current resolution is " << nWidth << "x" << nHeight << std::endl;
        throw std::invalid_argument(err.str());
    }
}

template <class COLOR32>
void Nv12ToColor32(uint8_t *dpNv12, int nNv12Pitch, uint8_t *dpBgra, int nBgraPitch, int nWidth, int nHeight, int iMatrix = 0);
template <class COLOR64>
void Nv12ToColor64(uint8_t *dpNv12, int nNv12Pitch, uint8_t *dpBgra, int nBgraPitch, int nWidth, int nHeight, int iMatrix = 0);

template <class COLOR32>
void P016ToColor32(uint8_t *dpP016, int nP016Pitch, uint8_t *dpBgra, int nBgraPitch, int nWidth, int nHeight, int iMatrix = 4);
template <class COLOR64>
void P016ToColor64(uint8_t *dpP016, int nP016Pitch, uint8_t *dpBgra, int nBgraPitch, int nWidth, int nHeight, int iMatrix = 4);

template <class COLOR32>
void YUV444ToColor32(uint8_t *dpYUV444, int nPitch, uint8_t *dpBgra, int nBgraPitch, int nWidth, int nHeight, int iMatrix = 0);
template <class COLOR64>
void YUV444ToColor64(uint8_t *dpYUV444, int nPitch, uint8_t *dpBgra, int nBgraPitch, int nWidth, int nHeight, int iMatrix = 0);

template <class COLOR32>
void YUV444P16ToColor32(uint8_t *dpYUV444, int nPitch, uint8_t *dpBgra, int nBgraPitch, int nWidth, int nHeight, int iMatrix = 4);
template <class COLOR64>
void YUV444P16ToColor64(uint8_t *dpYUV444, int nPitch, uint8_t *dpBgra, int nBgraPitch, int nWidth, int nHeight, int iMatrix = 4);

template <class COLOR32>
void Nv12ToColorPlanar(uint8_t *dpNv12, int nNv12Pitch, uint8_t *dpBgrp, int nBgrpPitch, int nWidth, int nHeight, int iMatrix = 0);
template <class COLOR32>
void P016ToColorPlanar(uint8_t *dpP016, int nP016Pitch, uint8_t *dpBgrp, int nBgrpPitch, int nWidth, int nHeight, int iMatrix = 4);

template <class COLOR32>
void YUV444ToColorPlanar(uint8_t *dpYUV444, int nPitch, uint8_t *dpBgrp, int nBgrpPitch, int nWidth, int nHeight, int iMatrix = 0);
template <class COLOR32>
void YUV444P16ToColorPlanar(uint8_t *dpYUV444, int nPitch, uint8_t *dpBgrp, int nBgrpPitch, int nWidth, int nHeight, int iMatrix = 4);

void Bgra64ToP016(uint8_t *dpBgra, int nBgraPitch, uint8_t *dpP016, int nP016Pitch, int nWidth, int nHeight, int iMatrix = 4);

void ConvertUInt8ToUInt16(uint8_t *dpUInt8, uint16_t *dpUInt16, int nSrcPitch, int nDestPitch, int nWidth, int nHeight);
void ConvertUInt16ToUInt8(uint16_t *dpUInt16, uint8_t *dpUInt8, int nSrcPitch, int nDestPitch, int nWidth, int nHeight);

void ResizeNv12(unsigned char *dpDstNv12, int nDstPitch, int nDstWidth, int nDstHeight, unsigned char *dpSrcNv12, int nSrcPitch, int nSrcWidth, int nSrcHeight, unsigned char *dpDstNv12UV = nullptr);
void ResizeP016(unsigned char *dpDstP016, int nDstPitch, int nDstWidth, int nDstHeight, unsigned char *dpSrcP016, int nSrcPitch, int nSrcWidth, int nSrcHeight, unsigned char *dpDstP016UV = nullptr);

void ScaleYUV420(unsigned char *dpDstY, unsigned char* dpDstU, unsigned char* dpDstV, int nDstPitch, int nDstChromaPitch, int nDstWidth, int nDstHeight,
    unsigned char *dpSrcY, unsigned char* dpSrcU, unsigned char* dpSrcV, int nSrcPitch, int nSrcChromaPitch, int nSrcWidth, int nSrcHeight, bool bSemiplanar);

#ifdef __cuda_cuda_h__
void ComputeCRC(uint8_t *pBuffer, uint32_t *crcValue, CUstream_st *outputCUStream);
#endif
