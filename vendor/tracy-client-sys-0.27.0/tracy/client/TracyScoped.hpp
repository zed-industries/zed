#ifndef __TRACYSCOPED_HPP__
#define __TRACYSCOPED_HPP__

#include <limits>
#include <stdarg.h>
#include <stdint.h>
#include <string.h>

#include "../common/TracySystem.hpp"
#include "../common/TracyAlign.hpp"
#include "../common/TracyAlloc.hpp"
#include "TracyProfiler.hpp"
#include "TracyCallstack.hpp"

#if (defined(__GNUC__) || defined(__clang__))
#  define TRACY_ATTRIBUTE_FORMAT_PRINTF(fmt_idx, arg_idx) \
     __attribute__((format(printf, fmt_idx, arg_idx)))
#else
#  define TRACY_ATTRIBUTE_FORMAT_PRINTF(fmt_idx, arg_idx)
#endif
namespace tracy
{

class ScopedZone
{
public:
    ScopedZone( const ScopedZone& ) = delete;
    ScopedZone( ScopedZone&& ) = delete;
    ScopedZone& operator=( const ScopedZone& ) = delete;
    ScopedZone& operator=( ScopedZone&& ) = delete;

    tracy_force_inline ScopedZone( const SourceLocationData* srcloc, int32_t depth = -1, bool is_active = true )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        m_connectionId = GetProfiler().ConnectionId();
#endif
        auto zoneQueue = QueueType::ZoneBegin;
        if( depth > 0 && has_callstack() )
        {
            GetProfiler().SendCallstack( depth );
            zoneQueue = QueueType::ZoneBeginCallstack;
        }
        TracyQueuePrepare( zoneQueue );
        MemWrite( &item->zoneBegin.time, Profiler::GetTime() );
        MemWrite( &item->zoneBegin.srcloc, (uint64_t)srcloc );
        TracyQueueCommit( zoneBeginThread );
    }

    tracy_force_inline ScopedZone( uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, uint32_t color, int32_t depth = -1, bool is_active = true )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        m_connectionId = GetProfiler().ConnectionId();
#endif
        auto zoneQueue = QueueType::ZoneBeginAllocSrcLoc;
        if( depth > 0 && has_callstack() )
        {
            GetProfiler().SendCallstack( depth );
            zoneQueue = QueueType::ZoneBeginAllocSrcLocCallstack;
        }
        TracyQueuePrepare( zoneQueue );
        const auto srcloc =
            Profiler::AllocSourceLocation( line, source, sourceSz, function, functionSz, name, nameSz, color );
        MemWrite( &item->zoneBegin.time, Profiler::GetTime() );
        MemWrite( &item->zoneBegin.srcloc, srcloc );
        TracyQueueCommit( zoneBeginThread );
    }

    tracy_force_inline ScopedZone( uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, int32_t depth, bool is_active = true ) : ScopedZone( line, source, sourceSz, function, functionSz, name, nameSz, 0, depth, is_active ) {}

    tracy_force_inline ~ScopedZone()
    {
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        if( GetProfiler().ConnectionId() != m_connectionId ) return;
#endif
        TracyQueuePrepare( QueueType::ZoneEnd );
        MemWrite( &item->zoneEnd.time, Profiler::GetTime() );
        TracyQueueCommit( zoneEndThread );
    }

    tracy_force_inline void Text( const char* txt, size_t size )
    {
        assert( size < (std::numeric_limits<uint16_t>::max)() );
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        if( GetProfiler().ConnectionId() != m_connectionId ) return;
#endif
        auto ptr = (char*)tracy_malloc( size );
        memcpy( ptr, txt, size );
        TracyQueuePrepare( QueueType::ZoneText );
        MemWrite( &item->zoneTextFat.text, (uint64_t)ptr );
        MemWrite( &item->zoneTextFat.size, (uint16_t)size );
        TracyQueueCommit( zoneTextFatThread );
    }

    void TextFmt( const char* fmt, ... ) TRACY_ATTRIBUTE_FORMAT_PRINTF(2, 3)
    {
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        if( GetProfiler().ConnectionId() != m_connectionId ) return;
#endif
        va_list args;
        va_start( args, fmt );
        auto size = vsnprintf( nullptr, 0, fmt, args );
        va_end( args );
        if( size < 0 ) return;
        assert( size < (std::numeric_limits<uint16_t>::max)() );

        char* ptr = (char*)tracy_malloc( size_t( size ) + 1 );
        va_start( args, fmt );
        vsnprintf( ptr, size_t( size ) + 1, fmt, args );
        va_end( args );

        TracyQueuePrepare( QueueType::ZoneText );
        MemWrite( &item->zoneTextFat.text, (uint64_t)ptr );
        MemWrite( &item->zoneTextFat.size, (uint16_t)size );
        TracyQueueCommit( zoneTextFatThread );
    }

    tracy_force_inline void Name( const char* txt, size_t size )
    {
        assert( size < (std::numeric_limits<uint16_t>::max)() );
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        if( GetProfiler().ConnectionId() != m_connectionId ) return;
#endif
        auto ptr = (char*)tracy_malloc( size );
        memcpy( ptr, txt, size );
        TracyQueuePrepare( QueueType::ZoneName );
        MemWrite( &item->zoneTextFat.text, (uint64_t)ptr );
        MemWrite( &item->zoneTextFat.size, (uint16_t)size );
        TracyQueueCommit( zoneTextFatThread );
    }

    void NameFmt( const char* fmt, ... ) TRACY_ATTRIBUTE_FORMAT_PRINTF(2, 3)
    {
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        if( GetProfiler().ConnectionId() != m_connectionId ) return;
#endif
        va_list args;
        va_start( args, fmt );
        auto size = vsnprintf( nullptr, 0, fmt, args );
        va_end( args );
        if( size < 0 ) return;
        assert( size < (std::numeric_limits<uint16_t>::max)() );

        char* ptr = (char*)tracy_malloc( size_t( size ) + 1 );
        va_start( args, fmt );
        vsnprintf( ptr, size_t( size ) + 1, fmt, args );
        va_end( args );

        TracyQueuePrepare( QueueType::ZoneName );
        MemWrite( &item->zoneTextFat.text, (uint64_t)ptr );
        MemWrite( &item->zoneTextFat.size, (uint16_t)size );
        TracyQueueCommit( zoneTextFatThread );
    }

    tracy_force_inline void Color( uint32_t color )
    {
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        if( GetProfiler().ConnectionId() != m_connectionId ) return;
#endif
        TracyQueuePrepare( QueueType::ZoneColor );
        MemWrite( &item->zoneColor.b, uint8_t( ( color       ) & 0xFF ) );
        MemWrite( &item->zoneColor.g, uint8_t( ( color >> 8  ) & 0xFF ) );
        MemWrite( &item->zoneColor.r, uint8_t( ( color >> 16 ) & 0xFF ) );
        TracyQueueCommit( zoneColorThread );
    }

    tracy_force_inline void Value( uint64_t value )
    {
        if( !m_active ) return;
#ifdef TRACY_ON_DEMAND
        if( GetProfiler().ConnectionId() != m_connectionId ) return;
#endif
        TracyQueuePrepare( QueueType::ZoneValue );
        MemWrite( &item->zoneValue.value, value );
        TracyQueueCommit( zoneValueThread );
    }

    tracy_force_inline bool IsActive() const { return m_active; }

private:
    const bool m_active;

#ifdef TRACY_ON_DEMAND
    uint64_t m_connectionId = 0;
#endif
};

}

#endif
