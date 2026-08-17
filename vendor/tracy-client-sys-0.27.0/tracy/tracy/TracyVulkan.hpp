#ifndef __TRACYVULKAN_HPP__
#define __TRACYVULKAN_HPP__

#if !defined TRACY_ENABLE

#define TracyVkContext(x,y,z,w) nullptr
#define TracyVkContextCalibrated(x,y,z,w,a,b) nullptr
#if defined VK_EXT_host_query_reset
#define TracyVkContextHostCalibrated(x,y,z,w,a) nullptr
#endif
#define TracyVkDestroy(x)
#define TracyVkContextName(c,x,y)
#define TracyVkNamedZone(c,x,y,z,w)
#define TracyVkNamedZoneC(c,x,y,z,w,a)
#define TracyVkZone(c,x,y)
#define TracyVkZoneC(c,x,y,z)
#define TracyVkZoneTransient(c,x,y,z,w)
#define TracyVkCollect(c,x)
#define TracyVkCollectHost(c)

#define TracyVkNamedZoneS(c,x,y,z,w,a)
#define TracyVkNamedZoneCS(c,x,y,z,w,v,a)
#define TracyVkZoneS(c,x,y,z)
#define TracyVkZoneCS(c,x,y,z,w)
#define TracyVkZoneTransientS(c,x,y,z,w,a)

namespace tracy
{
class VkCtxScope {};
}

using TracyVkCtx = void*;

#else

#if !defined VK_NULL_HANDLE
#  error "You must include Vulkan headers before including TracyVulkan.hpp"
#endif

#include <assert.h>
#include <stdlib.h>
#include "Tracy.hpp"
#include "../client/TracyProfiler.hpp"
#include "../client/TracyCallstack.hpp"

#include <atomic>

namespace tracy
{

#if defined TRACY_VK_USE_SYMBOL_TABLE
#define LoadVkDeviceCoreSymbols(Operation) \
    Operation(vkBeginCommandBuffer) \
    Operation(vkCmdResetQueryPool) \
    Operation(vkCmdWriteTimestamp) \
    Operation(vkCreateQueryPool) \
    Operation(vkDestroyQueryPool) \
    Operation(vkEndCommandBuffer) \
    Operation(vkGetQueryPoolResults) \
    Operation(vkQueueSubmit) \
    Operation(vkQueueWaitIdle) \
    Operation(vkResetQueryPool)

#define LoadVkDeviceExtensionSymbols(Operation) \
    Operation(vkGetCalibratedTimestampsEXT)

#define LoadVkInstanceExtensionSymbols(Operation) \
    Operation(vkGetPhysicalDeviceCalibrateableTimeDomainsEXT)

#define LoadVkInstanceCoreSymbols(Operation) \
    Operation(vkGetPhysicalDeviceProperties)

struct VkSymbolTable
{
#define MAKE_PFN(name) PFN_##name name;
    LoadVkDeviceCoreSymbols(MAKE_PFN)
    LoadVkDeviceExtensionSymbols(MAKE_PFN)
    LoadVkInstanceExtensionSymbols(MAKE_PFN)
    LoadVkInstanceCoreSymbols(MAKE_PFN)
#undef MAKE_PFN
};

#define VK_FUNCTION_WRAPPER(callSignature) m_symbols.callSignature
#define CONTEXT_VK_FUNCTION_WRAPPER(callSignature) m_ctx->m_symbols.callSignature
#else
#define VK_FUNCTION_WRAPPER(callSignature) callSignature
#define CONTEXT_VK_FUNCTION_WRAPPER(callSignature) callSignature
#endif

class VkCtx
{
    friend class VkCtxScope;

    enum { QueryCount = 64 * 1024 };

public:
#if defined TRACY_VK_USE_SYMBOL_TABLE
    VkCtx( VkInstance instance, VkPhysicalDevice physdev, VkDevice device, VkQueue queue, VkCommandBuffer cmdbuf, PFN_vkGetInstanceProcAddr instanceProcAddr, PFN_vkGetDeviceProcAddr deviceProcAddr, bool calibrated )
#else
    VkCtx( VkPhysicalDevice physdev, VkDevice device, VkQueue queue, VkCommandBuffer cmdbuf, PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT vkGetPhysicalDeviceCalibrateableTimeDomainsEXT, PFN_vkGetCalibratedTimestampsEXT vkGetCalibratedTimestampsEXT)
#endif
        : m_device( device )
        , m_timeDomain( VK_TIME_DOMAIN_DEVICE_EXT )
        , m_context( GetGpuCtxCounter().fetch_add( 1, std::memory_order_relaxed ) )
        , m_head( 0 )
        , m_tail( 0 )
        , m_oldCnt( 0 )
        , m_queryCount( QueryCount )
#if !defined TRACY_VK_USE_SYMBOL_TABLE
        , m_vkGetCalibratedTimestampsEXT( vkGetCalibratedTimestampsEXT )
#endif
    {
        assert( m_context != 255 );

#if defined TRACY_VK_USE_SYMBOL_TABLE
        PopulateSymbolTable(instance, instanceProcAddr, deviceProcAddr);
        if ( calibrated )
        {
            m_vkGetCalibratedTimestampsEXT = m_symbols.vkGetCalibratedTimestampsEXT;
        }

#endif

        if( VK_FUNCTION_WRAPPER( vkGetPhysicalDeviceCalibrateableTimeDomainsEXT ) && m_vkGetCalibratedTimestampsEXT )
        {
            FindAvailableTimeDomains( physdev, VK_FUNCTION_WRAPPER( vkGetPhysicalDeviceCalibrateableTimeDomainsEXT ) );
        }

        CreateQueryPool();

        VkCommandBufferBeginInfo beginInfo = {};
        beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        beginInfo.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;

        VkSubmitInfo submitInfo = {};
        submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submitInfo.commandBufferCount = 1;
        submitInfo.pCommandBuffers = &cmdbuf;

        VK_FUNCTION_WRAPPER( vkBeginCommandBuffer( cmdbuf, &beginInfo ) );
        VK_FUNCTION_WRAPPER( vkCmdResetQueryPool( cmdbuf, m_query, 0, m_queryCount ) );
        VK_FUNCTION_WRAPPER( vkEndCommandBuffer( cmdbuf ) );
        VK_FUNCTION_WRAPPER( vkQueueSubmit( queue, 1, &submitInfo, VK_NULL_HANDLE ) );
        VK_FUNCTION_WRAPPER( vkQueueWaitIdle( queue ) );

        int64_t tcpu, tgpu;
        if( m_timeDomain == VK_TIME_DOMAIN_DEVICE_EXT )
        {
            VK_FUNCTION_WRAPPER( vkBeginCommandBuffer( cmdbuf, &beginInfo ) );
            VK_FUNCTION_WRAPPER( vkCmdWriteTimestamp( cmdbuf, VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, m_query, 0 ) );
            VK_FUNCTION_WRAPPER( vkEndCommandBuffer( cmdbuf ) );
            VK_FUNCTION_WRAPPER( vkQueueSubmit( queue, 1, &submitInfo, VK_NULL_HANDLE ) );
            VK_FUNCTION_WRAPPER( vkQueueWaitIdle( queue ) );

            tcpu = Profiler::GetTime();
            VK_FUNCTION_WRAPPER( vkGetQueryPoolResults( device, m_query, 0, 1, sizeof( tgpu ), &tgpu, sizeof( tgpu ), VK_QUERY_RESULT_64_BIT | VK_QUERY_RESULT_WAIT_BIT ) );

            VK_FUNCTION_WRAPPER( vkBeginCommandBuffer( cmdbuf, &beginInfo ) );
            VK_FUNCTION_WRAPPER( vkCmdResetQueryPool( cmdbuf, m_query, 0, 1 ) );
            VK_FUNCTION_WRAPPER( vkEndCommandBuffer( cmdbuf ) );
            VK_FUNCTION_WRAPPER( vkQueueSubmit( queue, 1, &submitInfo, VK_NULL_HANDLE ) );
            VK_FUNCTION_WRAPPER( vkQueueWaitIdle( queue ) );
        }
        else
        {
            FindCalibratedTimestampDeviation();
            Calibrate( device, m_prevCalibration, tgpu );
            tcpu = Profiler::GetTime();
        }

        WriteInitialItem( physdev, tcpu, tgpu );

        m_res = (int64_t*)tracy_malloc( sizeof( int64_t ) * m_queryCount );
    }

#if defined VK_EXT_host_query_reset
    /**
     * This alternative constructor does not use command buffers and instead uses functionality from
     * VK_EXT_host_query_reset (core with 1.2 and non-optional) and VK_EXT_calibrated_timestamps. This requires
     * the physical device to have another time domain apart from DEVICE to be calibrateable.
     */
#if defined TRACY_VK_USE_SYMBOL_TABLE
    VkCtx( VkInstance instance, VkPhysicalDevice physdev, VkDevice device, PFN_vkGetInstanceProcAddr instanceProcAddr, PFN_vkGetDeviceProcAddr deviceProcAddr )
#else
    VkCtx( VkPhysicalDevice physdev, VkDevice device, PFN_vkResetQueryPoolEXT vkResetQueryPool, PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT vkGetPhysicalDeviceCalibrateableTimeDomainsEXT, PFN_vkGetCalibratedTimestampsEXT vkGetCalibratedTimestampsEXT )
#endif
        : m_device( device )
        , m_timeDomain( VK_TIME_DOMAIN_DEVICE_EXT )
        , m_context( GetGpuCtxCounter().fetch_add(1, std::memory_order_relaxed) )
        , m_head( 0 )
        , m_tail( 0 )
        , m_oldCnt( 0 )
        , m_queryCount( QueryCount )
#if !defined TRACY_VK_USE_SYMBOL_TABLE
        , m_vkGetCalibratedTimestampsEXT( vkGetCalibratedTimestampsEXT )
#endif
    {
        assert( m_context != 255);

#if defined TRACY_VK_USE_SYMBOL_TABLE
        PopulateSymbolTable(instance, instanceProcAddr, deviceProcAddr);
        m_vkGetCalibratedTimestampsEXT = m_symbols.vkGetCalibratedTimestampsEXT;
#endif

        assert( VK_FUNCTION_WRAPPER( vkResetQueryPool ) != nullptr );
        assert( VK_FUNCTION_WRAPPER( vkGetPhysicalDeviceCalibrateableTimeDomainsEXT ) != nullptr );
        assert( VK_FUNCTION_WRAPPER( vkGetCalibratedTimestampsEXT ) != nullptr );

        FindAvailableTimeDomains( physdev, VK_FUNCTION_WRAPPER( vkGetPhysicalDeviceCalibrateableTimeDomainsEXT ) );

        // We require a host time domain to be available to properly calibrate.
        FindCalibratedTimestampDeviation();
        int64_t tgpu;
        Calibrate( device, m_prevCalibration, tgpu );
        int64_t tcpu = Profiler::GetTime();

        CreateQueryPool();
        VK_FUNCTION_WRAPPER( vkResetQueryPool( device, m_query, 0, m_queryCount ) );

        WriteInitialItem( physdev, tcpu, tgpu );

        // We need the buffer to be twice as large for availability values
        size_t resSize = sizeof( int64_t ) * m_queryCount * 2;
        m_res = (int64_t*)tracy_malloc( resSize );
    }
#endif

    ~VkCtx()
    {
        tracy_free( m_res );
        VK_FUNCTION_WRAPPER( vkDestroyQueryPool( m_device, m_query, nullptr ) );
    }

    void Name( const char* name, uint16_t len )
    {
        auto ptr = (char*)tracy_malloc( len );
        memcpy( ptr, name, len );

        auto item = Profiler::QueueSerial();
        MemWrite( &item->hdr.type, QueueType::GpuContextName );
        MemWrite( &item->gpuContextNameFat.context, m_context );
        MemWrite( &item->gpuContextNameFat.ptr, (uint64_t)ptr );
        MemWrite( &item->gpuContextNameFat.size, len );
#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif
        Profiler::QueueSerialFinish();
    }

    void Collect( VkCommandBuffer cmdbuf )
    {
        ZoneScopedC( Color::Red4 );

        const uint64_t head = m_head.load(std::memory_order_relaxed);
        if( m_tail == head ) return;

#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() )
        {
            cmdbuf ?
                VK_FUNCTION_WRAPPER( vkCmdResetQueryPool( cmdbuf, m_query, 0, m_queryCount ) ) :
                VK_FUNCTION_WRAPPER( vkResetQueryPool( m_device, m_query, 0, m_queryCount ) );
            m_tail = head;
            m_oldCnt = 0;
            int64_t tgpu;
            if( m_timeDomain != VK_TIME_DOMAIN_DEVICE_EXT ) Calibrate( m_device, m_prevCalibration, tgpu );
            return;
        }
#endif
        assert( head > m_tail );

        const unsigned int wrappedTail = (unsigned int)( m_tail % m_queryCount );

        unsigned int cnt;
        if( m_oldCnt != 0 )
        {
            cnt = m_oldCnt;
            m_oldCnt = 0;
        }
        else
        {
            cnt = (unsigned int)( head - m_tail );
            assert( cnt <= m_queryCount );
            if( wrappedTail + cnt > m_queryCount )
            {
                cnt = m_queryCount - wrappedTail;
            }
        }


        VK_FUNCTION_WRAPPER( vkGetQueryPoolResults( m_device, m_query, wrappedTail, cnt, sizeof( int64_t ) * m_queryCount * 2, m_res, sizeof( int64_t ) * 2, VK_QUERY_RESULT_64_BIT | VK_QUERY_RESULT_WITH_AVAILABILITY_BIT ) );

        for( unsigned int idx=0; idx<cnt; idx++ )
        {
            int64_t avail = m_res[idx * 2 + 1];
            if( avail == 0 )
            {
                m_oldCnt = cnt - idx;
                cnt = idx;

                break;
            }

            auto item = Profiler::QueueSerial();
            MemWrite( &item->hdr.type, QueueType::GpuTime );
            MemWrite( &item->gpuTime.gpuTime, m_res[idx * 2] );
            MemWrite( &item->gpuTime.queryId, uint16_t( wrappedTail + idx ) );
            MemWrite( &item->gpuTime.context, m_context );
            Profiler::QueueSerialFinish();
        }

        if( m_timeDomain != VK_TIME_DOMAIN_DEVICE_EXT )
        {
            int64_t tgpu, tcpu;
            Calibrate( m_device, tcpu, tgpu );
            const auto refCpu = Profiler::GetTime();
            const auto delta = tcpu - m_prevCalibration;
            if( delta > 0 )
            {
                m_prevCalibration = tcpu;
                auto item = Profiler::QueueSerial();
                MemWrite( &item->hdr.type, QueueType::GpuCalibration );
                MemWrite( &item->gpuCalibration.gpuTime, tgpu );
                MemWrite( &item->gpuCalibration.cpuTime, refCpu );
                MemWrite( &item->gpuCalibration.cpuDelta, delta );
                MemWrite( &item->gpuCalibration.context, m_context );
                Profiler::QueueSerialFinish();
            }
        }

        cmdbuf ?
            VK_FUNCTION_WRAPPER( vkCmdResetQueryPool( cmdbuf, m_query, wrappedTail, cnt ) ) :
            VK_FUNCTION_WRAPPER( vkResetQueryPool( m_device, m_query, wrappedTail, cnt ) );

        m_tail += cnt;
    }

    tracy_force_inline unsigned int NextQueryId()
    {
        const uint64_t id = m_head.fetch_add(1, std::memory_order_relaxed);
        return id % m_queryCount;
    }

    tracy_force_inline uint8_t GetId() const
    {
        return m_context;
    }

    tracy_force_inline VkQueryPool GetQueryPool() const
    {
         return m_query;
    }

private:
    tracy_force_inline void Calibrate( VkDevice device, int64_t& tCpu, int64_t& tGpu )
    {
        assert( m_timeDomain != VK_TIME_DOMAIN_DEVICE_EXT );
        VkCalibratedTimestampInfoEXT spec[2] = {
            { VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT, nullptr, VK_TIME_DOMAIN_DEVICE_EXT },
            { VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT, nullptr, m_timeDomain },
        };
        uint64_t ts[2];
        uint64_t deviation;
        do
        {
            m_vkGetCalibratedTimestampsEXT( device, 2, spec, ts, &deviation );
        }
        while( deviation > m_deviation );

#if defined _WIN32
        tGpu = ts[0];
        tCpu = ts[1] * m_qpcToNs;
#elif defined __linux__ && defined CLOCK_MONOTONIC_RAW
        tGpu = ts[0];
        tCpu = ts[1];
#else
        assert( false );
#endif
    }

    tracy_force_inline void CreateQueryPool()
    {
        VkQueryPoolCreateInfo poolInfo = {};
        poolInfo.sType = VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO;
        poolInfo.queryCount = m_queryCount;
        poolInfo.queryType = VK_QUERY_TYPE_TIMESTAMP;
        while ( VK_FUNCTION_WRAPPER( vkCreateQueryPool( m_device, &poolInfo, nullptr, &m_query ) != VK_SUCCESS ) )
        {
            m_queryCount /= 2;
            poolInfo.queryCount = m_queryCount;
        }
    }

    tracy_force_inline void FindAvailableTimeDomains( VkPhysicalDevice physicalDevice, PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT _vkGetPhysicalDeviceCalibrateableTimeDomainsEXT )
    {
        uint32_t num;
        _vkGetPhysicalDeviceCalibrateableTimeDomainsEXT( physicalDevice, &num, nullptr );
        if(num > 4) num = 4;
        VkTimeDomainEXT data[4];
        _vkGetPhysicalDeviceCalibrateableTimeDomainsEXT( physicalDevice, &num, data );
        VkTimeDomainEXT supportedDomain = (VkTimeDomainEXT)-1;
#if defined _WIN32
        supportedDomain = VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT;
#elif defined __linux__ && defined CLOCK_MONOTONIC_RAW
        supportedDomain = VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT;
#endif
        for( uint32_t i=0; i<num; i++ ) {
            if(data[i] == supportedDomain) {
                m_timeDomain = data[i];
                break;
            }
        }
    }

    tracy_force_inline void FindCalibratedTimestampDeviation()
    {
        assert( m_timeDomain != VK_TIME_DOMAIN_DEVICE_EXT );
        constexpr size_t NumProbes = 32;
        VkCalibratedTimestampInfoEXT spec[2] = {
            { VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT, nullptr, VK_TIME_DOMAIN_DEVICE_EXT },
            { VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT, nullptr, m_timeDomain },
        };
        uint64_t ts[2];
        uint64_t deviation[NumProbes];
        for( size_t i=0; i<NumProbes; i++ ) {
            m_vkGetCalibratedTimestampsEXT( m_device, 2, spec, ts, deviation + i );
        }
        uint64_t minDeviation = deviation[0];
        for( size_t i=1; i<NumProbes; i++ ) {
            if ( minDeviation > deviation[i] ) {
                minDeviation = deviation[i];
            }
        }
        m_deviation = minDeviation * 3 / 2;

#if defined _WIN32
        m_qpcToNs = int64_t( 1000000000. / GetFrequencyQpc() );
#endif
    }

    tracy_force_inline void WriteInitialItem( VkPhysicalDevice physdev, int64_t tcpu, int64_t tgpu )
    {
        uint8_t flags = 0;
        if( m_timeDomain != VK_TIME_DOMAIN_DEVICE_EXT ) flags |= GpuContextCalibration;

        VkPhysicalDeviceProperties prop;
        VK_FUNCTION_WRAPPER( vkGetPhysicalDeviceProperties( physdev, &prop ) );
        const float period = prop.limits.timestampPeriod;

        auto item = Profiler::QueueSerial();
        MemWrite( &item->hdr.type, QueueType::GpuNewContext );
        MemWrite( &item->gpuNewContext.cpuTime, tcpu );
        MemWrite( &item->gpuNewContext.gpuTime, tgpu );
        memset( &item->gpuNewContext.thread, 0, sizeof( item->gpuNewContext.thread ) );
        MemWrite( &item->gpuNewContext.period, period );
        MemWrite( &item->gpuNewContext.context, m_context );
        MemWrite( &item->gpuNewContext.flags, flags );
        MemWrite( &item->gpuNewContext.type, GpuContextType::Vulkan );

#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif
        Profiler::QueueSerialFinish();
    }

#if defined TRACY_VK_USE_SYMBOL_TABLE
    void PopulateSymbolTable( VkInstance instance, PFN_vkGetInstanceProcAddr instanceProcAddr, PFN_vkGetDeviceProcAddr deviceProcAddr )
    {
#define VK_GET_DEVICE_SYMBOL( name ) \
        (PFN_##name)deviceProcAddr( m_device, #name );
#define VK_LOAD_DEVICE_SYMBOL( name ) \
        m_symbols.name = VK_GET_DEVICE_SYMBOL( name );
#define VK_GET_INSTANCE_SYMBOL( name ) \
        (PFN_##name)instanceProcAddr( instance, #name );
#define VK_LOAD_INSTANCE_SYMBOL( name ) \
        m_symbols.name = VK_GET_INSTANCE_SYMBOL( name );

        LoadVkDeviceCoreSymbols( VK_LOAD_DEVICE_SYMBOL )
        LoadVkDeviceExtensionSymbols( VK_LOAD_DEVICE_SYMBOL )
        LoadVkInstanceExtensionSymbols( VK_LOAD_INSTANCE_SYMBOL )
        LoadVkInstanceCoreSymbols( VK_LOAD_INSTANCE_SYMBOL )
#undef VK_GET_DEVICE_SYMBOL
#undef VK_LOAD_DEVICE_SYMBOL
#undef VK_GET_INSTANCE_SYMBOL
#undef VK_LOAD_INSTANCE_SYMBOL
    }
#endif

    VkDevice m_device;
    VkQueryPool m_query;
    VkTimeDomainEXT m_timeDomain;
#if defined TRACY_VK_USE_SYMBOL_TABLE
    VkSymbolTable m_symbols;
#endif
    uint64_t m_deviation;
#ifdef _WIN32
    int64_t m_qpcToNs;
#endif
    int64_t m_prevCalibration;
    uint8_t m_context;

    std::atomic<uint64_t> m_head;
    uint64_t m_tail;
    unsigned int m_oldCnt;
    unsigned int m_queryCount;

    int64_t* m_res;

    PFN_vkGetCalibratedTimestampsEXT m_vkGetCalibratedTimestampsEXT;
};

class VkCtxScope
{
public:
    tracy_force_inline VkCtxScope( VkCtx* ctx, const SourceLocationData* srcloc, VkCommandBuffer cmdbuf, bool is_active )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;
        m_cmdbuf = cmdbuf;
        m_ctx = ctx;

        const auto queryId = ctx->NextQueryId();
        CONTEXT_VK_FUNCTION_WRAPPER( vkCmdWriteTimestamp( cmdbuf, VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, ctx->m_query, queryId ) );

        auto item = Profiler::QueueSerial();
        MemWrite( &item->hdr.type, QueueType::GpuZoneBeginSerial );
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneBegin.srcloc, (uint64_t)srcloc );
        MemWrite( &item->gpuZoneBegin.thread, GetThreadHandle() );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, ctx->GetId() );
        Profiler::QueueSerialFinish();
    }

    tracy_force_inline VkCtxScope( VkCtx* ctx, const SourceLocationData* srcloc, VkCommandBuffer cmdbuf, int32_t depth, bool is_active )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;
        m_cmdbuf = cmdbuf;
        m_ctx = ctx;

        const auto queryId = ctx->NextQueryId();
        CONTEXT_VK_FUNCTION_WRAPPER( vkCmdWriteTimestamp( cmdbuf, VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, ctx->m_query, queryId ) );

        QueueItem *item;
        if( depth > 0 && has_callstack() )
        {
            item = Profiler::QueueSerialCallstack( Callstack( depth ) );
            MemWrite( &item->hdr.type, QueueType::GpuZoneBeginCallstackSerial );
        }
        else
        {
            item = Profiler::QueueSerial();
            MemWrite( &item->hdr.type, QueueType::GpuZoneBeginSerial );
        }
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneBegin.srcloc, (uint64_t)srcloc );
        MemWrite( &item->gpuZoneBegin.thread, GetThreadHandle() );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, ctx->GetId() );
        Profiler::QueueSerialFinish();
    }

    tracy_force_inline VkCtxScope( VkCtx* ctx, uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, VkCommandBuffer cmdbuf, bool is_active )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;
        m_cmdbuf = cmdbuf;
        m_ctx = ctx;

        const auto queryId = ctx->NextQueryId();
        CONTEXT_VK_FUNCTION_WRAPPER( vkCmdWriteTimestamp( cmdbuf, VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, ctx->m_query, queryId ) );

        const auto srcloc = Profiler::AllocSourceLocation( line, source, sourceSz, function, functionSz, name, nameSz );
        auto item = Profiler::QueueSerial();
        MemWrite( &item->hdr.type, QueueType::GpuZoneBeginAllocSrcLocSerial );
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneBegin.srcloc, srcloc );
        MemWrite( &item->gpuZoneBegin.thread, GetThreadHandle() );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, ctx->GetId() );
        Profiler::QueueSerialFinish();
    }

    tracy_force_inline VkCtxScope( VkCtx* ctx, uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, VkCommandBuffer cmdbuf, int32_t depth, bool is_active )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;
        m_cmdbuf = cmdbuf;
        m_ctx = ctx;

        const auto queryId = ctx->NextQueryId();
        CONTEXT_VK_FUNCTION_WRAPPER( vkCmdWriteTimestamp( cmdbuf, VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, ctx->m_query, queryId ) );

        const auto srcloc = Profiler::AllocSourceLocation( line, source, sourceSz, function, functionSz, name, nameSz );
        QueueItem *item;
        if( depth > 0 && has_callstack() )
        {
            item = Profiler::QueueSerialCallstack( Callstack( depth ) );
            MemWrite( &item->hdr.type, QueueType::GpuZoneBeginAllocSrcLocCallstackSerial );
        }
        else
        {
            item = Profiler::QueueSerial();
            MemWrite( &item->hdr.type, QueueType::GpuZoneBeginAllocSrcLocSerial );
        }
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneBegin.srcloc, srcloc );
        MemWrite( &item->gpuZoneBegin.thread, GetThreadHandle() );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, ctx->GetId() );
        Profiler::QueueSerialFinish();
    }

    tracy_force_inline ~VkCtxScope()
    {
        if( !m_active ) return;

        const auto queryId = m_ctx->NextQueryId();
        CONTEXT_VK_FUNCTION_WRAPPER( vkCmdWriteTimestamp( m_cmdbuf, VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, m_ctx->m_query, queryId ) );

        auto item = Profiler::QueueSerial();
        MemWrite( &item->hdr.type, QueueType::GpuZoneEndSerial );
        MemWrite( &item->gpuZoneEnd.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneEnd.thread, GetThreadHandle() );
        MemWrite( &item->gpuZoneEnd.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneEnd.context, m_ctx->GetId() );
        Profiler::QueueSerialFinish();
    }

private:
    const bool m_active;

    VkCommandBuffer m_cmdbuf;
    VkCtx* m_ctx;
};

#if defined TRACY_VK_USE_SYMBOL_TABLE
static inline VkCtx* CreateVkContext( VkInstance instance, VkPhysicalDevice physdev, VkDevice device, VkQueue queue, VkCommandBuffer cmdbuf, PFN_vkGetInstanceProcAddr instanceProcAddr, PFN_vkGetDeviceProcAddr getDeviceProcAddr, bool calibrated = false )
#else
static inline VkCtx* CreateVkContext( VkPhysicalDevice physdev, VkDevice device, VkQueue queue, VkCommandBuffer cmdbuf, PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT gpdctd, PFN_vkGetCalibratedTimestampsEXT gct )
#endif
{
    auto ctx = (VkCtx*)tracy_malloc( sizeof( VkCtx ) );
#if defined TRACY_VK_USE_SYMBOL_TABLE
    new(ctx) VkCtx( instance, physdev, device, queue, cmdbuf, instanceProcAddr, getDeviceProcAddr, calibrated );
#else
    new(ctx) VkCtx( physdev, device, queue, cmdbuf, gpdctd, gct );
#endif
    return ctx;
}

#if defined VK_EXT_host_query_reset
#if defined TRACY_VK_USE_SYMBOL_TABLE
static inline VkCtx* CreateVkContext( VkInstance instance, VkPhysicalDevice physdev, VkDevice device, PFN_vkGetInstanceProcAddr instanceProcAddr, PFN_vkGetDeviceProcAddr getDeviceProcAddr )
#else
static inline VkCtx* CreateVkContext( VkPhysicalDevice physdev, VkDevice device, PFN_vkResetQueryPoolEXT qpreset, PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT gpdctd, PFN_vkGetCalibratedTimestampsEXT gct )
#endif
{
    auto ctx = (VkCtx*)tracy_malloc( sizeof( VkCtx ) );
#if defined TRACY_VK_USE_SYMBOL_TABLE
    new(ctx) VkCtx( instance, physdev, device, instanceProcAddr, getDeviceProcAddr );
#else
    new(ctx) VkCtx( physdev, device, qpreset, gpdctd, gct );
#endif
    return ctx;
}
#endif

static inline void DestroyVkContext( VkCtx* ctx )
{
    ctx->~VkCtx();
    tracy_free( ctx );
}

}

using TracyVkCtx = tracy::VkCtx*;

#if defined TRACY_VK_USE_SYMBOL_TABLE
#define TracyVkContext( instance, physdev, device, queue, cmdbuf, instanceProcAddr, deviceProcAddr ) tracy::CreateVkContext( instance, physdev, device, queue, cmdbuf, instanceProcAddr, deviceProcAddr );
#else
#define TracyVkContext( physdev, device, queue, cmdbuf ) tracy::CreateVkContext( physdev, device, queue, cmdbuf, nullptr, nullptr );
#endif
#if defined TRACY_VK_USE_SYMBOL_TABLE
#define TracyVkContextCalibrated( instance, physdev, device, queue, cmdbuf, instanceProcAddr, deviceProcAddr ) tracy::CreateVkContext( instance, physdev, device, queue, cmdbuf, instanceProcAddr, deviceProcAddr, true );
#else
#define TracyVkContextCalibrated( physdev, device, queue, cmdbuf, gpdctd, gct ) tracy::CreateVkContext( physdev, device, queue, cmdbuf, gpdctd, gct );
#endif
#if defined VK_EXT_host_query_reset
#if defined TRACY_VK_USE_SYMBOL_TABLE
#define TracyVkContextHostCalibrated( instance, physdev, device, instanceProcAddr, deviceProcAddr ) tracy::CreateVkContext( instance, physdev, device, instanceProcAddr, deviceProcAddr );
#else
#define TracyVkContextHostCalibrated( physdev, device, qpreset, gpdctd, gct ) tracy::CreateVkContext( physdev, device, qpreset, gpdctd, gct );
#endif
#endif
#define TracyVkDestroy( ctx ) tracy::DestroyVkContext( ctx );
#define TracyVkContextName( ctx, name, size ) ctx->Name( name, size );
#if defined TRACY_HAS_CALLSTACK && defined TRACY_CALLSTACK
#  define TracyVkNamedZone( ctx, varname, cmdbuf, name, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::VkCtxScope varname( ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), cmdbuf, TRACY_CALLSTACK, active );
#  define TracyVkNamedZoneC( ctx, varname, cmdbuf, name, color, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::VkCtxScope varname( ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), cmdbuf, TRACY_CALLSTACK, active );
#  define TracyVkZone( ctx, cmdbuf, name ) TracyVkNamedZoneS( ctx, ___tracy_gpu_zone, cmdbuf, name, TRACY_CALLSTACK, true )
#  define TracyVkZoneC( ctx, cmdbuf, name, color ) TracyVkNamedZoneCS( ctx, ___tracy_gpu_zone, cmdbuf, name, color, TRACY_CALLSTACK, true )
#  define TracyVkZoneTransient( ctx, varname, cmdbuf, name, active ) TracyVkZoneTransientS( ctx, varname, cmdbuf, name, TRACY_CALLSTACK, active )
#else
#  define TracyVkNamedZone( ctx, varname, cmdbuf, name, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::VkCtxScope varname( ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), cmdbuf, active );
#  define TracyVkNamedZoneC( ctx, varname, cmdbuf, name, color, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::VkCtxScope varname( ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), cmdbuf, active );
#  define TracyVkZone( ctx, cmdbuf, name ) TracyVkNamedZone( ctx, ___tracy_gpu_zone, cmdbuf, name, true )
#  define TracyVkZoneC( ctx, cmdbuf, name, color ) TracyVkNamedZoneC( ctx, ___tracy_gpu_zone, cmdbuf, name, color, true )
#  define TracyVkZoneTransient( ctx, varname, cmdbuf, name, active ) tracy::VkCtxScope varname( ctx, TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), cmdbuf, active );
#endif
#define TracyVkCollect( ctx, cmdbuf ) ctx->Collect( cmdbuf );
#define TracyVkCollectHost( ctx ) ctx->Collect( VK_NULL_HANDLE );

#ifdef TRACY_HAS_CALLSTACK
#  define TracyVkNamedZoneS( ctx, varname, cmdbuf, name, depth, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::VkCtxScope varname( ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), cmdbuf, depth, active );
#  define TracyVkNamedZoneCS( ctx, varname, cmdbuf, name, color, depth, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::VkCtxScope varname( ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), cmdbuf, depth, active );
#  define TracyVkZoneS( ctx, cmdbuf, name, depth ) TracyVkNamedZoneS( ctx, ___tracy_gpu_zone, cmdbuf, name, depth, true )
#  define TracyVkZoneCS( ctx, cmdbuf, name, color, depth ) TracyVkNamedZoneCS( ctx, ___tracy_gpu_zone, cmdbuf, name, color, depth, true )
#  define TracyVkZoneTransientS( ctx, varname, cmdbuf, name, depth, active ) tracy::VkCtxScope varname( ctx, TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), cmdbuf, depth, active );
#else
#  define TracyVkNamedZoneS( ctx, varname, cmdbuf, name, depth, active ) TracyVkNamedZone( ctx, varname, cmdbuf, name, active )
#  define TracyVkNamedZoneCS( ctx, varname, cmdbuf, name, color, depth, active ) TracyVkNamedZoneC( ctx, varname, cmdbuf, name, color, active )
#  define TracyVkZoneS( ctx, cmdbuf, name, depth ) TracyVkZone( ctx, cmdbuf, name )
#  define TracyVkZoneCS( ctx, cmdbuf, name, color, depth ) TracyVkZoneC( ctx, cmdbuf, name, color )
#  define TracyVkZoneTransientS( ctx, varname, cmdbuf, name, depth, active ) TracyVkZoneTransient( ctx, varname, cmdbuf, name, active )
#endif

#endif

#endif
