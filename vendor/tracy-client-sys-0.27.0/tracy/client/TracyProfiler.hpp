#ifndef __TRACYPROFILER_HPP__
#define __TRACYPROFILER_HPP__

#include <assert.h>
#include <atomic>
#include <stdint.h>
#include <string.h>
#include <time.h>

#include "tracy_concurrentqueue.h"
#include "tracy_SPSCQueue.h"
#include "TracyCallstack.hpp"
#include "TracyKCore.hpp"
#include "TracySysPower.hpp"
#include "TracySysTime.hpp"
#include "TracyFastVector.hpp"
#include "../common/TracyQueue.hpp"
#include "../common/TracyAlign.hpp"
#include "../common/TracyAlloc.hpp"
#include "../common/TracyMutex.hpp"
#include "../common/TracyProtocol.hpp"

#if defined _WIN32
#  include <intrin.h>
#endif
#ifdef __APPLE__
#  include <TargetConditionals.h>
#  include <mach/mach_time.h>
#endif

#if ( (defined _WIN32 && !(defined _M_ARM64 || defined _M_ARM)) || ( defined __i386 || defined _M_IX86 || defined __x86_64__ || defined _M_X64 ) || ( defined TARGET_OS_IOS && TARGET_OS_IOS == 1 ) )
#  define TRACY_HW_TIMER
#endif

#ifdef __linux__
#  include <signal.h>
#endif

#if defined TRACY_TIMER_FALLBACK || !defined TRACY_HW_TIMER
#  include <chrono>
#endif

#ifndef TracyConcat
#  define TracyConcat(x,y) TracyConcatIndirect(x,y)
#endif
#ifndef TracyConcatIndirect
#  define TracyConcatIndirect(x,y) x##y
#endif

namespace tracy
{
#if defined(TRACY_DELAYED_INIT) && defined(TRACY_MANUAL_LIFETIME)
TRACY_API void StartupProfiler();
TRACY_API void ShutdownProfiler();
TRACY_API bool IsProfilerStarted();
#  define TracyIsStarted tracy::IsProfilerStarted()
#else
#  define TracyIsStarted true
#endif

TRACY_API bool BeginSamplingProfiling();
TRACY_API void EndSamplingProfiling();

class GpuCtx;
class Profiler;
class Socket;
class UdpBroadcast;

struct GpuCtxWrapper
{
    GpuCtx* ptr;
};

TRACY_API moodycamel::ConcurrentQueue<QueueItem>::ExplicitProducer* GetToken();
TRACY_API Profiler& GetProfiler();
TRACY_API std::atomic<uint32_t>& GetLockCounter();
TRACY_API std::atomic<uint8_t>& GetGpuCtxCounter();
TRACY_API GpuCtxWrapper& GetGpuCtx();
TRACY_API uint32_t GetThreadHandle();
TRACY_API bool ProfilerAvailable();
TRACY_API bool ProfilerAllocatorAvailable();
TRACY_API int64_t GetFrequencyQpc();

#if defined TRACY_TIMER_FALLBACK && defined TRACY_HW_TIMER && ( defined __i386 || defined _M_IX86 || defined __x86_64__ || defined _M_X64 )
TRACY_API bool HardwareSupportsInvariantTSC();  // check, if we need fallback scenario
#else
#  if defined TRACY_HW_TIMER
tracy_force_inline bool HardwareSupportsInvariantTSC()
{
    return true;  // this is checked at startup
}
#  else
tracy_force_inline bool HardwareSupportsInvariantTSC()
{
    return false;
}
#  endif
#endif


struct SourceLocationData
{
    const char* name;
    const char* function;
    const char* file;
    uint32_t line;
    uint32_t color;
};

#ifdef TRACY_ON_DEMAND
struct LuaZoneState
{
    uint32_t counter;
    bool active;
};
#endif


#define TracyLfqPrepare( _type ) \
    tracy::moodycamel::ConcurrentQueueDefaultTraits::index_t __magic; \
    auto __token = tracy::GetToken(); \
    auto& __tail = __token->get_tail_index(); \
    auto item = __token->enqueue_begin( __magic ); \
    tracy::MemWrite( &item->hdr.type, _type );

#define TracyLfqCommit \
    __tail.store( __magic + 1, std::memory_order_release );

#define TracyLfqPrepareC( _type ) \
    tracy::moodycamel::ConcurrentQueueDefaultTraits::index_t __magic; \
    auto __token = tracy::GetToken(); \
    auto& __tail = __token->get_tail_index(); \
    auto item = __token->enqueue_begin( __magic ); \
    tracy::MemWrite( &item->hdr.type, _type );

#define TracyLfqCommitC \
    __tail.store( __magic + 1, std::memory_order_release );


#ifdef TRACY_FIBERS
#  define TracyQueuePrepare( _type ) \
    auto item = tracy::Profiler::QueueSerial(); \
    tracy::MemWrite( &item->hdr.type, _type );
#  define TracyQueueCommit( _name ) \
    tracy::MemWrite( &item->_name.thread, tracy::GetThreadHandle() ); \
    tracy::Profiler::QueueSerialFinish();
#  define TracyQueuePrepareC( _type ) \
    auto item = tracy::Profiler::QueueSerial(); \
    tracy::MemWrite( &item->hdr.type, _type );
#  define TracyQueueCommitC( _name ) \
    tracy::MemWrite( &item->_name.thread, tracy::GetThreadHandle() ); \
    tracy::Profiler::QueueSerialFinish();
#else
#  define TracyQueuePrepare( _type ) TracyLfqPrepare( _type )
#  define TracyQueueCommit( _name ) TracyLfqCommit
#  define TracyQueuePrepareC( _type ) TracyLfqPrepareC( _type )
#  define TracyQueueCommitC( _name ) TracyLfqCommitC
#endif


typedef void(*ParameterCallback)( void* data, uint32_t idx, int32_t val );
typedef char*(*SourceContentsCallback)( void* data, const char* filename, size_t& size );

class Profiler
{
    struct FrameImageQueueItem
    {
        void* image;
        uint32_t frame;
        uint16_t w;
        uint16_t h;
        bool flip;
    };

    enum class SymbolQueueItemType
    {
        CallstackFrame,
        SymbolQuery,
        ExternalName,
        KernelCode,
        SourceCode
    };

    struct SymbolQueueItem
    {
        SymbolQueueItemType type;
        uint64_t ptr;
        uint64_t extra;
        uint32_t id;
    };

public:
    Profiler();
    ~Profiler();

    void SpawnWorkerThreads();

    static tracy_force_inline int64_t GetTime()
    {
#ifdef TRACY_HW_TIMER
#  if defined TARGET_OS_IOS && TARGET_OS_IOS == 1
        if( HardwareSupportsInvariantTSC() ) return mach_absolute_time();
#  elif defined _WIN32
#    ifdef TRACY_TIMER_QPC
        return GetTimeQpc();
#    else
        if( HardwareSupportsInvariantTSC() ) return int64_t( __rdtsc() );
#    endif
#  elif defined __i386 || defined _M_IX86
        if( HardwareSupportsInvariantTSC() )
        {
            uint32_t eax, edx;
            asm volatile ( "rdtsc" : "=a" (eax), "=d" (edx) );
            return ( uint64_t( edx ) << 32 ) + uint64_t( eax );
        }
#  elif defined __x86_64__ || defined _M_X64
        if( HardwareSupportsInvariantTSC() )
        {
            uint64_t rax, rdx;
#ifdef TRACY_PATCHABLE_NOPSLEDS
            // Some external tooling (such as rr) wants to patch our rdtsc and replace it by a
            // branch to control the external input seen by a program. This kind of patching is
            // not generally possible depending on the surrounding code and can lead to significant
            // slowdowns if the compiler generated unlucky code and rr and tracy are used together.
            // To avoid this, use the rr-safe `nopl 0(%rax, %rax, 1); rdtsc` instruction sequence,
            // which rr promises will be patchable independent of the surrounding code.
            asm volatile (
                    // This is nopl 0(%rax, %rax, 1), but assemblers are inconsistent about whether
                    // they emit that as a 4 or 5 byte sequence and we need to be guaranteed to use
                    // the 5 byte one.
                    ".byte 0x0f, 0x1f, 0x44, 0x00, 0x00\n\t"
                    "rdtsc" : "=a" (rax), "=d" (rdx) );
#else
            asm volatile ( "rdtsc" : "=a" (rax), "=d" (rdx) );
#endif
            return (int64_t)(( rdx << 32 ) + rax);
        }
#  else
#    error "TRACY_HW_TIMER detection logic needs fixing"
#  endif
#endif

#if !defined TRACY_HW_TIMER || defined TRACY_TIMER_FALLBACK
#  if defined __linux__ && defined CLOCK_MONOTONIC_RAW
        struct timespec ts;
        clock_gettime( CLOCK_MONOTONIC_RAW, &ts );
        return int64_t( ts.tv_sec ) * 1000000000ll + int64_t( ts.tv_nsec );
#  else
        return std::chrono::duration_cast<std::chrono::nanoseconds>( std::chrono::high_resolution_clock::now().time_since_epoch() ).count();
#  endif
#endif

#if !defined TRACY_TIMER_FALLBACK
        return 0;  // unreachable branch
#endif
    }

    bool BeginSamplingProfiling();
    void EndSamplingProfiling();

    tracy_force_inline uint32_t GetNextZoneId()
    {
        return m_zoneId.fetch_add( 1, std::memory_order_relaxed );
    }

    static tracy_force_inline QueueItem* QueueSerial()
    {
        auto& p = GetProfiler();
        p.m_serialLock.lock();
        return p.m_serialQueue.prepare_next();
    }

    static tracy_force_inline QueueItem* QueueSerialCallstack( void* ptr )
    {
        auto& p = GetProfiler();
        p.m_serialLock.lock();
        p.SendCallstackSerial( ptr );
        return p.m_serialQueue.prepare_next();
    }

    static tracy_force_inline void QueueSerialFinish()
    {
        auto& p = GetProfiler();
        p.m_serialQueue.commit_next();
        p.m_serialLock.unlock();
    }

    static tracy_force_inline void SendFrameMark( const char* name )
    {
        if( !name ) GetProfiler().m_frameCount.fetch_add( 1, std::memory_order_relaxed );
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        auto item = QueueSerial();
        MemWrite( &item->hdr.type, QueueType::FrameMarkMsg );
        MemWrite( &item->frameMark.time, GetTime() );
        MemWrite( &item->frameMark.name, uint64_t( name ) );
        QueueSerialFinish();
    }

    static tracy_force_inline void SendFrameMark( const char* name, QueueType type )
    {
        assert( type == QueueType::FrameMarkMsgStart || type == QueueType::FrameMarkMsgEnd );
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        auto item = QueueSerial();
        MemWrite( &item->hdr.type, type );
        MemWrite( &item->frameMark.time, GetTime() );
        MemWrite( &item->frameMark.name, uint64_t( name ) );
        QueueSerialFinish();
    }

    static tracy_force_inline void SendFrameImage( const void* image, uint16_t w, uint16_t h, uint8_t offset, bool flip )
    {
#ifndef TRACY_NO_FRAME_IMAGE
        auto& profiler = GetProfiler();
        assert( profiler.m_frameCount.load( std::memory_order_relaxed ) < (std::numeric_limits<uint32_t>::max)() );
#  ifdef TRACY_ON_DEMAND
        if( !profiler.IsConnected() ) return;
#  endif
        const auto sz = size_t( w ) * size_t( h ) * 4;
        auto ptr = (char*)tracy_malloc( sz );
        memcpy( ptr, image, sz );

        profiler.m_fiLock.lock();
        auto fi = profiler.m_fiQueue.prepare_next();
        fi->image = ptr;
        fi->frame = uint32_t( profiler.m_frameCount.load( std::memory_order_relaxed ) - offset );
        fi->w = w;
        fi->h = h;
        fi->flip = flip;
        profiler.m_fiQueue.commit_next();
        profiler.m_fiLock.unlock();
#else
        static_cast<void>(image); // unused
        static_cast<void>(w); // unused
        static_cast<void>(h); // unused
        static_cast<void>(offset); // unused
        static_cast<void>(flip); // unused
#endif
    }

    static tracy_force_inline void PlotData( const char* name, int64_t val )
    {
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        TracyLfqPrepare( QueueType::PlotDataInt );
        MemWrite( &item->plotDataInt.name, (uint64_t)name );
        MemWrite( &item->plotDataInt.time, GetTime() );
        MemWrite( &item->plotDataInt.val, val );
        TracyLfqCommit;
    }

    static tracy_force_inline void PlotData( const char* name, float val )
    {
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        TracyLfqPrepare( QueueType::PlotDataFloat );
        MemWrite( &item->plotDataFloat.name, (uint64_t)name );
        MemWrite( &item->plotDataFloat.time, GetTime() );
        MemWrite( &item->plotDataFloat.val, val );
        TracyLfqCommit;
    }

    static tracy_force_inline void PlotData( const char* name, double val )
    {
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        TracyLfqPrepare( QueueType::PlotDataDouble );
        MemWrite( &item->plotDataDouble.name, (uint64_t)name );
        MemWrite( &item->plotDataDouble.time, GetTime() );
        MemWrite( &item->plotDataDouble.val, val );
        TracyLfqCommit;
    }

    static tracy_force_inline void ConfigurePlot( const char* name, PlotFormatType type, bool step, bool fill, uint32_t color )
    {
        TracyLfqPrepare( QueueType::PlotConfig );
        MemWrite( &item->plotConfig.name, (uint64_t)name );
        MemWrite( &item->plotConfig.type, (uint8_t)type );
        MemWrite( &item->plotConfig.step, (uint8_t)step );
        MemWrite( &item->plotConfig.fill, (uint8_t)fill );
        MemWrite( &item->plotConfig.color, color );

#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif

        TracyLfqCommit;
    }

    static tracy_force_inline void Message( const char* txt, size_t size, int32_t callstack_depth )
    {
        assert( size < (std::numeric_limits<uint16_t>::max)() );
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        if( callstack_depth != 0 && has_callstack() )
        {
            tracy::GetProfiler().SendCallstack( callstack_depth );
        }

        auto ptr = (char*)tracy_malloc( size );
        memcpy( ptr, txt, size );

        TracyQueuePrepare( callstack_depth == 0 ? QueueType::Message : QueueType::MessageCallstack );
        MemWrite( &item->messageFat.time, GetTime() );
        MemWrite( &item->messageFat.text, (uint64_t)ptr );
        MemWrite( &item->messageFat.size, (uint16_t)size );
        TracyQueueCommit( messageFatThread );
    }

    static tracy_force_inline void Message( const char* txt, int32_t callstack_depth )
    {
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        if( callstack_depth != 0 && has_callstack() )
        {
            tracy::GetProfiler().SendCallstack( callstack_depth );
        }

        TracyQueuePrepare( callstack_depth == 0 ? QueueType::MessageLiteral : QueueType::MessageLiteralCallstack );
        MemWrite( &item->messageLiteral.time, GetTime() );
        MemWrite( &item->messageLiteral.text, (uint64_t)txt );
        TracyQueueCommit( messageLiteralThread );
    }

    static tracy_force_inline void MessageColor( const char* txt, size_t size, uint32_t color, int32_t callstack_depth )
    {
        assert( size < (std::numeric_limits<uint16_t>::max)() );
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        if( callstack_depth != 0 && has_callstack() )
        {
            tracy::GetProfiler().SendCallstack( callstack_depth );
        }

        auto ptr = (char*)tracy_malloc( size );
        memcpy( ptr, txt, size );

        TracyQueuePrepare( callstack_depth == 0 ? QueueType::MessageColor : QueueType::MessageColorCallstack );
        MemWrite( &item->messageColorFat.time, GetTime() );
        MemWrite( &item->messageColorFat.text, (uint64_t)ptr );
        MemWrite( &item->messageColorFat.b, uint8_t( ( color       ) & 0xFF ) );
        MemWrite( &item->messageColorFat.g, uint8_t( ( color >> 8  ) & 0xFF ) );
        MemWrite( &item->messageColorFat.r, uint8_t( ( color >> 16 ) & 0xFF ) );
        MemWrite( &item->messageColorFat.size, (uint16_t)size );
        TracyQueueCommit( messageColorFatThread );
    }

    static tracy_force_inline void MessageColor( const char* txt, uint32_t color, int32_t callstack_depth )
    {
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        if( callstack_depth != 0 && has_callstack() )
        {
            tracy::GetProfiler().SendCallstack( callstack_depth );
        }

        TracyQueuePrepare( callstack_depth == 0 ? QueueType::MessageLiteralColor : QueueType::MessageLiteralColorCallstack );
        MemWrite( &item->messageColorLiteral.time, GetTime() );
        MemWrite( &item->messageColorLiteral.text, (uint64_t)txt );
        MemWrite( &item->messageColorLiteral.b, uint8_t( ( color       ) & 0xFF ) );
        MemWrite( &item->messageColorLiteral.g, uint8_t( ( color >> 8  ) & 0xFF ) );
        MemWrite( &item->messageColorLiteral.r, uint8_t( ( color >> 16 ) & 0xFF ) );
        TracyQueueCommit( messageColorLiteralThread );
    }

    static tracy_force_inline void MessageAppInfo( const char* txt, size_t size )
    {
        assert( size < (std::numeric_limits<uint16_t>::max)() );
        auto ptr = (char*)tracy_malloc( size );
        memcpy( ptr, txt, size );
        TracyLfqPrepare( QueueType::MessageAppInfo );
        MemWrite( &item->messageFat.time, GetTime() );
        MemWrite( &item->messageFat.text, (uint64_t)ptr );
        MemWrite( &item->messageFat.size, (uint16_t)size );

#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif

        TracyLfqCommit;
    }

    static tracy_force_inline void MemAlloc( const void* ptr, size_t size, bool secure )
    {
        if( secure && !ProfilerAvailable() ) return;
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        const auto thread = GetThreadHandle();

        GetProfiler().m_serialLock.lock();
        SendMemAlloc( QueueType::MemAlloc, thread, ptr, size );
        GetProfiler().m_serialLock.unlock();
    }

    static tracy_force_inline void MemFree( const void* ptr, bool secure )
    {
        if( secure && !ProfilerAvailable() ) return;
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        const auto thread = GetThreadHandle();

        GetProfiler().m_serialLock.lock();
        SendMemFree( QueueType::MemFree, thread, ptr );
        GetProfiler().m_serialLock.unlock();
    }

    static tracy_force_inline void MemAllocCallstack( const void* ptr, size_t size, int32_t depth, bool secure )
    {
        if( secure && !ProfilerAvailable() ) return;
        if( depth > 0 && has_callstack() )
        {
            auto& profiler = GetProfiler();
#  ifdef TRACY_ON_DEMAND
            if( !profiler.IsConnected() ) return;
#  endif
            const auto thread = GetThreadHandle();

            auto callstack = Callstack( depth );

            profiler.m_serialLock.lock();
            SendCallstackSerial( callstack );
            SendMemAlloc( QueueType::MemAllocCallstack, thread, ptr, size );
            profiler.m_serialLock.unlock();
        }
        else
        {
            MemAlloc( ptr, size, secure );
        }
    }

    static tracy_force_inline void MemFreeCallstack( const void* ptr, int32_t depth, bool secure )
    {
        if( secure && !ProfilerAvailable() ) return;
        if( !ProfilerAllocatorAvailable() )
        {
            MemFree( ptr, secure );
            return;
        }
        if( depth > 0 && has_callstack() )
        {
            auto& profiler = GetProfiler();
#  ifdef TRACY_ON_DEMAND
            if( !profiler.IsConnected() ) return;
#  endif
            const auto thread = GetThreadHandle();

            auto callstack = Callstack( depth );

            profiler.m_serialLock.lock();
            SendCallstackSerial( callstack );
            SendMemFree( QueueType::MemFreeCallstack, thread, ptr );
            profiler.m_serialLock.unlock();
        }
        else
        {
            MemFree( ptr, secure );
        }
    }

    static tracy_force_inline void MemAllocNamed( const void* ptr, size_t size, bool secure, const char* name )
    {
        if( secure && !ProfilerAvailable() ) return;
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        const auto thread = GetThreadHandle();

        GetProfiler().m_serialLock.lock();
        SendMemName( name );
        SendMemAlloc( QueueType::MemAllocNamed, thread, ptr, size );
        GetProfiler().m_serialLock.unlock();
    }

    static tracy_force_inline void MemFreeNamed( const void* ptr, bool secure, const char* name )
    {
        if( secure && !ProfilerAvailable() ) return;
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        const auto thread = GetThreadHandle();

        GetProfiler().m_serialLock.lock();
        SendMemName( name );
        SendMemFree( QueueType::MemFreeNamed, thread, ptr );
        GetProfiler().m_serialLock.unlock();
    }

    static tracy_force_inline void MemAllocCallstackNamed( const void* ptr, size_t size, int32_t depth, bool secure, const char* name )
    {
        if( secure && !ProfilerAvailable() ) return;
        if( depth > 0 && has_callstack() )
        {
            auto& profiler = GetProfiler();
#  ifdef TRACY_ON_DEMAND
            if( !profiler.IsConnected() ) return;
#  endif
            const auto thread = GetThreadHandle();

            auto callstack = Callstack( depth );

            profiler.m_serialLock.lock();
            SendCallstackSerial( callstack );
            SendMemName( name );
            SendMemAlloc( QueueType::MemAllocCallstackNamed, thread, ptr, size );
            profiler.m_serialLock.unlock();
        }
        else
        {
            MemAllocNamed( ptr, size, secure, name );
        }
    }

    static tracy_force_inline void MemFreeCallstackNamed( const void* ptr, int32_t depth, bool secure, const char* name )
    {
        if( secure && !ProfilerAvailable() ) return;
        if( depth > 0 && has_callstack() )
        {
            auto& profiler = GetProfiler();
#  ifdef TRACY_ON_DEMAND
            if( !profiler.IsConnected() ) return;
#  endif
            const auto thread = GetThreadHandle();

            auto callstack = Callstack( depth );

            profiler.m_serialLock.lock();
            SendCallstackSerial( callstack );
            SendMemName( name );
            SendMemFree( QueueType::MemFreeCallstackNamed, thread, ptr );
            profiler.m_serialLock.unlock();
        }
        else
        {
            MemFreeNamed( ptr, secure, name );
        }
    }

    static tracy_force_inline void MemDiscard( const char* name, bool secure )
    {
        if( secure && !ProfilerAvailable() ) return;
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        const auto thread = GetThreadHandle();

        GetProfiler().m_serialLock.lock();
        SendMemDiscard( QueueType::MemDiscard, thread, name );
        GetProfiler().m_serialLock.unlock();
    }

    static tracy_force_inline void MemDiscardCallstack( const char* name, bool secure, int32_t depth )
    {
        if( secure && !ProfilerAvailable() ) return;
        if( depth > 0 && has_callstack() )
        {
#  ifdef TRACY_ON_DEMAND
            if( !GetProfiler().IsConnected() ) return;
#  endif
            const auto thread = GetThreadHandle();

            auto callstack = Callstack( depth );

            GetProfiler().m_serialLock.lock();
            SendCallstackSerial( callstack );
            SendMemDiscard( QueueType::MemDiscard, thread, name );
            GetProfiler().m_serialLock.unlock();
        }
        else
        {
            MemDiscard( name, secure );
        }
    }

    static tracy_force_inline void SendCallstack( int32_t depth )
    {
        if( depth > 0 && has_callstack() )
        {
            auto ptr = Callstack( depth );
            TracyQueuePrepare( QueueType::Callstack );
            MemWrite( &item->callstackFat.ptr, (uint64_t)ptr );
            TracyQueueCommit( callstackFatThread );
        }
    }

    static tracy_force_inline void ParameterRegister( ParameterCallback cb, void* data )
    {
        auto& profiler = GetProfiler();
        profiler.m_paramCallback = cb;
        profiler.m_paramCallbackData = data;
    }

    static tracy_force_inline void ParameterSetup( uint32_t idx, const char* name, bool isBool, int32_t val )
    {
        TracyLfqPrepare( QueueType::ParamSetup );
        tracy::MemWrite( &item->paramSetup.idx, idx );
        tracy::MemWrite( &item->paramSetup.name, (uint64_t)name );
        tracy::MemWrite( &item->paramSetup.isBool, (uint8_t)isBool );
        tracy::MemWrite( &item->paramSetup.val, val );

#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif

        TracyLfqCommit;
    }

    static tracy_force_inline void SourceCallbackRegister( SourceContentsCallback cb, void* data )
    {
        auto& profiler = GetProfiler();
        profiler.m_sourceCallback = cb;
        profiler.m_sourceCallbackData = data;
    }

#ifdef TRACY_FIBERS
    static tracy_force_inline void EnterFiber( const char* fiber, int32_t groupHint )
    {
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        TracyQueuePrepare( QueueType::FiberEnter );
        MemWrite( &item->fiberEnter.time, GetTime() );
        MemWrite( &item->fiberEnter.fiber, (uint64_t)fiber );
        MemWrite( &item->fiberEnter.groupHint, groupHint );
        TracyQueueCommit( fiberEnter );
    }

    static tracy_force_inline void LeaveFiber()
    {
#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() ) return;
#endif
        TracyQueuePrepare( QueueType::FiberLeave );
        MemWrite( &item->fiberLeave.time, GetTime() );
        TracyQueueCommit( fiberLeave );
    }
#endif

    void SendCallstack( int32_t depth, const char* skipBefore );
    static void CutCallstack( void* callstack, const char* skipBefore );

    static bool ShouldExit();

    tracy_force_inline bool IsConnected() const
    {
        return m_isConnected.load( std::memory_order_acquire );
    }

    tracy_force_inline void SetProgramName( const char* name )
    {
        m_programNameLock.lock();
        m_programName = name;
        m_programNameLock.unlock();
    }

#ifdef TRACY_ON_DEMAND
    tracy_force_inline uint64_t ConnectionId() const
    {
        return m_connectionId.load( std::memory_order_acquire );
    }

    tracy_force_inline void DeferItem( const QueueItem& item )
    {
        m_deferredLock.lock();
        auto dst = m_deferredQueue.push_next();
        memcpy( dst, &item, sizeof( item ) );
        m_deferredLock.unlock();
    }
#endif

    void RequestShutdown() { m_shutdown.store( true, std::memory_order_relaxed ); m_shutdownManual.store( true, std::memory_order_relaxed ); }
    bool HasShutdownFinished() const { return m_shutdownFinished.load( std::memory_order_relaxed ); }

    void SendString( uint64_t str, const char* ptr, QueueType type ) { SendString( str, ptr, strlen( ptr ), type ); }
    void SendString( uint64_t str, const char* ptr, size_t len, QueueType type );
    void SendSingleString( const char* ptr ) { SendSingleString( ptr, strlen( ptr ) ); }
    void SendSingleString( const char* ptr, size_t len );
    void SendSecondString( const char* ptr ) { SendSecondString( ptr, strlen( ptr ) ); }
    void SendSecondString( const char* ptr, size_t len );


    // Allocated source location data layout:
    //  2b  payload size
    //  4b  color
    //  4b  source line
    //  fsz function name
    //  1b  null terminator
    //  ssz source file name
    //  1b  null terminator
    //  nsz zone name (optional)

    static tracy_force_inline uint64_t AllocSourceLocation( uint32_t line, const char* source, const char* function, uint32_t color = 0 )
    {
        return AllocSourceLocation( line, source, function, nullptr, 0, color );
    }

    static tracy_force_inline uint64_t AllocSourceLocation( uint32_t line, const char* source, const char* function, const char* name, size_t nameSz, uint32_t color = 0 )
    {
        return AllocSourceLocation( line, source, strlen(source), function, strlen(function), name, nameSz, color );
    }

    static tracy_force_inline uint64_t AllocSourceLocation( uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, uint32_t color = 0 )
    {
        return AllocSourceLocation( line, source, sourceSz, function, functionSz, nullptr, 0, color );
    }

    static tracy_force_inline uint64_t AllocSourceLocation( uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, uint32_t color = 0 )
    {
        const auto sz32 = uint32_t( 2 + 4 + 4 + functionSz + 1 + sourceSz + 1 + nameSz );
        assert( sz32 <= (std::numeric_limits<uint16_t>::max)() );
        const auto sz = uint16_t( sz32 );
        auto ptr = (char*)tracy_malloc( sz );
        memcpy( ptr, &sz, 2 );
        memcpy( ptr + 2, &color, 4 );
        memcpy( ptr + 6, &line, 4 );
        memcpy( ptr + 10, function, functionSz );
        ptr[10 + functionSz] = '\0';
        memcpy( ptr + 10 + functionSz + 1, source, sourceSz );
        ptr[10 + functionSz + 1 + sourceSz] = '\0';
        if( nameSz != 0 )
        {
            memcpy( ptr + 10 + functionSz + 1 + sourceSz + 1, name, nameSz );
        }
        return uint64_t( ptr );
    }

private:
    enum class DequeueStatus { DataDequeued, ConnectionLost, QueueEmpty };
    enum class ThreadCtxStatus { Same, Changed, ConnectionLost };

    static void LaunchWorker( void* ptr ) { ((Profiler*)ptr)->Worker(); }
    void Worker();

#ifndef TRACY_NO_FRAME_IMAGE
    static void LaunchCompressWorker( void* ptr ) { ((Profiler*)ptr)->CompressWorker(); }
    void CompressWorker();
#endif

#ifdef TRACY_HAS_CALLSTACK
    static void LaunchSymbolWorker( void* ptr ) { ((Profiler*)ptr)->SymbolWorker(); }
    void SymbolWorker();
    void HandleSymbolQueueItem( const SymbolQueueItem& si );
#endif

    void InstallCrashHandler();
    void RemoveCrashHandler();

    void ClearQueues( tracy::moodycamel::ConsumerToken& token );
    void ClearSerial();
    DequeueStatus Dequeue( tracy::moodycamel::ConsumerToken& token );
    DequeueStatus DequeueContextSwitches( tracy::moodycamel::ConsumerToken& token, int64_t& timeStop );
    DequeueStatus DequeueSerial();
    ThreadCtxStatus ThreadCtxCheck( uint32_t threadId );
    bool CommitData();

    tracy_force_inline bool AppendData( const void* data, size_t len )
    {
        const auto ret = NeedDataSize( len );
        AppendDataUnsafe( data, len );
        return ret;
    }

    tracy_force_inline bool NeedDataSize( size_t len )
    {
        assert( len <= TargetFrameSize );
        bool ret = true;
        if( m_bufferOffset - m_bufferStart + (int)len > TargetFrameSize )
        {
            ret = CommitData();
        }
        return ret;
    }

    tracy_force_inline void AppendDataUnsafe( const void* data, size_t len )
    {
        memcpy( m_buffer + m_bufferOffset, data, len );
        m_bufferOffset += int( len );
    }

    char* SafeCopyProlog( const char* p, size_t size );
    void SafeCopyEpilog( char* buf );

    template<class Callable> // must be void( const char* buf, size_t size )
    bool WithSafeCopy( const char* p, size_t size, Callable&& callable )
    {
        if( char* buf = SafeCopyProlog( p, size ) )
        {
            callable( buf, size );
            SafeCopyEpilog( buf );
            return true;
        }
        return false;
    }

    bool SendData( const char* data, size_t len );
    void SendLongString( uint64_t ptr, const char* str, size_t len, QueueType type );
    void SendSourceLocation( uint64_t ptr );
    void SendSourceLocationPayload( uint64_t ptr );
    void SendCallstackPayload( uint64_t ptr );
    void SendCallstackPayload64( uint64_t ptr );
    void SendCallstackAlloc( uint64_t ptr );

    void QueueCallstackFrame( uint64_t ptr );
    void QueueSymbolQuery( uint64_t symbol );
    void QueueExternalName( uint64_t ptr );
    void QueueKernelCode( uint64_t symbol, uint32_t size );
    void QueueSourceCodeQuery( uint32_t id );

    bool HandleServerQuery();
    void HandleDisconnect();
    void HandleParameter( uint64_t payload );
    void HandleSymbolCodeQuery( uint64_t symbol, uint32_t size );
    void HandleSourceCodeQuery( char* data, char* image, uint32_t id );

    void AckServerQuery();
    void AckSymbolCodeNotAvailable();

    void CalibrateTimer();
    void CalibrateDelay();
    void ReportTopology();

    static tracy_force_inline void SendCallstackSerial( void* ptr )
    {
        if( has_callstack() )
        {
            auto item = GetProfiler().m_serialQueue.prepare_next();
            MemWrite( &item->hdr.type, QueueType::CallstackSerial );
            MemWrite( &item->callstackFat.ptr, (uint64_t)ptr );
            GetProfiler().m_serialQueue.commit_next();
        }
    }

    static tracy_force_inline void SendMemAlloc( QueueType type, const uint32_t thread, const void* ptr, size_t size )
    {
        assert( type == QueueType::MemAlloc || type == QueueType::MemAllocCallstack || type == QueueType::MemAllocNamed || type == QueueType::MemAllocCallstackNamed );

        auto item = GetProfiler().m_serialQueue.prepare_next();
        MemWrite( &item->hdr.type, type );
        MemWrite( &item->memAlloc.time, GetTime() );
        MemWrite( &item->memAlloc.thread, thread );
        MemWrite( &item->memAlloc.ptr, (uint64_t)ptr );
        if( compile_time_condition<sizeof( size ) == 4>::value )
        {
            memcpy( &item->memAlloc.size, &size, 4 );
            memset( &item->memAlloc.size + 4, 0, 2 );
        }
        else
        {
            assert( sizeof( size ) == 8 );
            memcpy( &item->memAlloc.size, &size, 4 );
            memcpy( ((char*)&item->memAlloc.size)+4, ((char*)&size)+4, 2 );
        }
        GetProfiler().m_serialQueue.commit_next();
    }

    static tracy_force_inline void SendMemFree( QueueType type, const uint32_t thread, const void* ptr )
    {
        assert( type == QueueType::MemFree || type == QueueType::MemFreeCallstack || type == QueueType::MemFreeNamed || type == QueueType::MemFreeCallstackNamed );

        auto item = GetProfiler().m_serialQueue.prepare_next();
        MemWrite( &item->hdr.type, type );
        MemWrite( &item->memFree.time, GetTime() );
        MemWrite( &item->memFree.thread, thread );
        MemWrite( &item->memFree.ptr, (uint64_t)ptr );
        GetProfiler().m_serialQueue.commit_next();
    }

    static tracy_force_inline void SendMemDiscard( QueueType type, const uint32_t thread, const char* name )
    {
        assert( type == QueueType::MemDiscard || type == QueueType::MemDiscardCallstack );

        auto item = GetProfiler().m_serialQueue.prepare_next();
        MemWrite( &item->hdr.type, type );
        MemWrite( &item->memDiscard.time, GetTime() );
        MemWrite( &item->memDiscard.thread, thread );
        MemWrite( &item->memDiscard.name, (uint64_t)name );
        GetProfiler().m_serialQueue.commit_next();
    }

    static tracy_force_inline void SendMemName( const char* name )
    {
        assert( name );
        auto item = GetProfiler().m_serialQueue.prepare_next();
        MemWrite( &item->hdr.type, QueueType::MemNamePayload );
        MemWrite( &item->memName.name, (uint64_t)name );
        GetProfiler().m_serialQueue.commit_next();
    }

#if defined _WIN32 && defined TRACY_TIMER_QPC
    static int64_t GetTimeQpc();
#endif

    double m_timerMul;
    uint64_t m_resolution;
    std::atomic<int64_t> m_timeBegin;
    uint32_t m_mainThread;
    uint64_t m_epoch, m_exectime;
    std::atomic<bool> m_shutdown;
    std::atomic<bool> m_shutdownManual;
    std::atomic<bool> m_shutdownFinished;
    Socket* m_sock;
    UdpBroadcast* m_broadcast;
    bool m_noExit;
    uint32_t m_userPort;
    std::atomic<uint32_t> m_zoneId;
    int64_t m_samplingPeriod;

    uint32_t m_threadCtx;
    int64_t m_refTimeThread;
    int64_t m_refTimeSerial;
    int64_t m_refTimeCtx;
    int64_t m_refTimeGpu;

    void* m_stream;     // LZ4_stream_t*
    char* m_buffer;
    int m_bufferOffset;
    int m_bufferStart;

    char* m_lz4Buf;

    FastVector<QueueItem> m_serialQueue, m_serialDequeue;
    TracyMutex m_serialLock;

#ifndef TRACY_NO_FRAME_IMAGE
    FastVector<FrameImageQueueItem> m_fiQueue, m_fiDequeue;
    TracyMutex m_fiLock;
#endif

    SPSCQueue<SymbolQueueItem> m_symbolQueue;

    std::atomic<uint64_t> m_frameCount;
    std::atomic<bool> m_isConnected;
#ifdef TRACY_ON_DEMAND
    std::atomic<uint64_t> m_connectionId;
    std::atomic<bool> m_symbolsBusy;

    TracyMutex m_deferredLock;
    FastVector<QueueItem> m_deferredQueue;
#endif

#ifdef TRACY_HAS_SYSTIME
    void ProcessSysTime();

    SysTime m_sysTime;
    uint64_t m_sysTimeLast = 0;
#else
    void ProcessSysTime() {}
#endif

#ifdef TRACY_HAS_SYSPOWER
    SysPower m_sysPower;
#endif

    ParameterCallback m_paramCallback;
    void* m_paramCallbackData;
    SourceContentsCallback m_sourceCallback;
    void* m_sourceCallbackData;

    char* m_queryImage;
    char* m_queryData;
    char* m_queryDataPtr;

#ifndef NDEBUG
    // m_safeSendBuffer and m_pipe should only be used by the Tracy Profiler thread; this ensures that in debug builds.
    std::atomic_bool m_inUse{ false };
#endif
    char* m_safeSendBuffer;

#if defined _WIN32
    void* m_prevHandler;
#else
    int m_pipe[2];
    int m_pipeBufSize;
#endif

#ifdef __linux__
    struct {
        struct sigaction pwr, ill, fpe, segv, pipe, bus, abrt;
    } m_prevSignal;
    KCore* m_kcore;
#endif
    bool m_crashHandlerInstalled;

    const char* m_programName;
    TracyMutex m_programNameLock;
};

}

#endif
