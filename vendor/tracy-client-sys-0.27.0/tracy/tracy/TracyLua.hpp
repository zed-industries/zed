#ifndef __TRACYLUA_HPP__
#define __TRACYLUA_HPP__

// Include this file after you include lua headers.

#ifndef TRACY_ENABLE

#include <string.h>

namespace tracy
{

namespace detail
{
static inline int noop( lua_State* L ) { return 0; }
}

static inline void LuaRegister( lua_State* L )
{
    lua_newtable( L );
    lua_pushcfunction( L, detail::noop );
    lua_setfield( L, -2, "ZoneBegin" );
    lua_pushcfunction( L, detail::noop );
    lua_setfield( L, -2, "ZoneBeginN" );
    lua_pushcfunction( L, detail::noop );
    lua_setfield( L, -2, "ZoneBeginS" );
    lua_pushcfunction( L, detail::noop );
    lua_setfield( L, -2, "ZoneBeginNS" );
    lua_pushcfunction( L, detail::noop );
    lua_setfield( L, -2, "ZoneEnd" );
    lua_pushcfunction( L, detail::noop );
    lua_setfield( L, -2, "ZoneText" );
    lua_pushcfunction( L, detail::noop );
    lua_setfield( L, -2, "ZoneName" );
    lua_pushcfunction( L, detail::noop );
    lua_setfield( L, -2, "Message" );
    lua_setglobal( L, "tracy" );
}

static inline char* FindEnd( char* ptr )
{
    unsigned int cnt = 1;
    while( cnt != 0 )
    {
        if( *ptr == '(' ) cnt++;
        else if( *ptr == ')' ) cnt--;
        ptr++;
    }
    return ptr;
}

static inline void LuaRemove( char* script )
{
    while( *script )
    {
        if( strncmp( script, "tracy.", 6 ) == 0 )
        {
            if( strncmp( script + 6, "Zone", 4 ) == 0 )
            {
                if( strncmp( script + 10, "End()", 5 ) == 0 )
                {
                    memset( script, ' ', 15 );
                    script += 15;
                }
                else if( strncmp( script + 10, "Begin()", 7 ) == 0 )
                {
                    memset( script, ' ', 17 );
                    script += 17;
                }
                else if( strncmp( script + 10, "Text(", 5 ) == 0 )
                {
                    auto end = FindEnd( script + 15 );
                    memset( script, ' ', end - script );
                    script = end;
                }
                else if( strncmp( script + 10, "Name(", 5 ) == 0 )
                {
                    auto end = FindEnd( script + 15 );
                    memset( script, ' ', end - script );
                    script = end;
                }
                else if( strncmp( script + 10, "BeginN(", 7 ) == 0 )
                {
                    auto end = FindEnd( script + 17 );
                    memset( script, ' ', end - script );
                    script = end;
                }
                else if( strncmp( script + 10, "BeginS(", 7 ) == 0 )
                {
                    auto end = FindEnd( script + 17 );
                    memset( script, ' ', end - script );
                    script = end;
                }
                else if( strncmp( script + 10, "BeginNS(", 8 ) == 0 )
                {
                    auto end = FindEnd( script + 18 );
                    memset( script, ' ', end - script );
                    script = end;
                }
                else
                {
                    script += 10;
                }
            }
            else if( strncmp( script + 6, "Message(", 8 ) == 0 )
            {
                auto end = FindEnd( script + 14 );
                memset( script, ' ', end - script );
                script = end;
            }
            else
            {
                script += 6;
            }
        }
        else
        {
            script++;
        }
    }
}

static inline void LuaHook( lua_State* L, lua_Debug* ar ) {}

}

#else

#include <assert.h>
#include <limits>

#include "../common/TracyColor.hpp"
#include "../common/TracyAlign.hpp"
#include "../common/TracyForceInline.hpp"
#include "../common/TracySystem.hpp"
#include "../client/TracyProfiler.hpp"

namespace tracy
{

#ifdef TRACY_ON_DEMAND
TRACY_API LuaZoneState& GetLuaZoneState();
#endif

namespace detail
{

static inline void LuaShortenSrc( char* dst, const char* src )
{
    size_t l = std::min( (size_t)255, strlen( src ) );
    memcpy( dst, src, l );
    dst[l] = 0;
}

#ifdef TRACY_HAS_CALLSTACK
static tracy_force_inline void SendLuaCallstack( lua_State* L, uint32_t depth )
{
    assert( depth <= 64 );
    lua_Debug dbg[64];
    const char* func[64];
    uint32_t fsz[64];
    uint32_t ssz[64];

    uint8_t cnt;
    uint16_t spaceNeeded = sizeof( cnt );
    for( cnt=0; cnt<depth; cnt++ )
    {
        if( lua_getstack( L, cnt+1, dbg+cnt ) == 0 ) break;
        lua_getinfo( L, "Snl", dbg+cnt );
        func[cnt] = dbg[cnt].name ? dbg[cnt].name : dbg[cnt].short_src;
        fsz[cnt] = uint32_t( strlen( func[cnt] ) );
        ssz[cnt] = uint32_t( strlen( dbg[cnt].source ) );
        spaceNeeded += fsz[cnt] + ssz[cnt];
    }
    spaceNeeded += cnt * ( 4 + 2 + 2 );     // source line, function string length, source string length

    auto ptr = (char*)tracy_malloc( spaceNeeded + 2 );
    auto dst = ptr;
    memcpy( dst, &spaceNeeded, 2 ); dst += 2;
    memcpy( dst, &cnt, 1 ); dst++;
    for( uint8_t i=0; i<cnt; i++ )
    {
        const uint32_t line = dbg[i].currentline;
        memcpy( dst, &line, 4 ); dst += 4;
        assert( fsz[i] <= (std::numeric_limits<uint16_t>::max)() );
        memcpy( dst, fsz+i, 2 ); dst += 2;
        memcpy( dst, func[i], fsz[i] ); dst += fsz[i];
        assert( ssz[i] <= (std::numeric_limits<uint16_t>::max)() );
        memcpy( dst, ssz+i, 2 ); dst += 2;
        memcpy( dst, dbg[i].source, ssz[i] ), dst += ssz[i];
    }
    assert( dst - ptr == spaceNeeded + 2 );

    TracyQueuePrepare( QueueType::CallstackAlloc );
    MemWrite( &item->callstackAllocFat.ptr, (uint64_t)ptr );
    MemWrite( &item->callstackAllocFat.nativePtr, (uint64_t)Callstack( depth ) );
    TracyQueueCommit( callstackAllocFatThread );
}

static inline int LuaZoneBeginS( lua_State* L )
{
#ifdef TRACY_ON_DEMAND
    const auto zoneCnt = GetLuaZoneState().counter++;
    if( zoneCnt != 0 && !GetLuaZoneState().active ) return 0;
    GetLuaZoneState().active = GetProfiler().IsConnected();
    if( !GetLuaZoneState().active ) return 0;
#endif

#ifdef TRACY_CALLSTACK
    const uint32_t depth = TRACY_CALLSTACK;
#else
    const auto depth = uint32_t( lua_tointeger( L, 1 ) );
#endif
    SendLuaCallstack( L, depth );

    lua_Debug dbg;
    lua_getstack( L, 1, &dbg );
    lua_getinfo( L, "Snl", &dbg );
    char src[256];
    LuaShortenSrc( src, dbg.source );
    const auto srcloc = Profiler::AllocSourceLocation( dbg.currentline, src, dbg.name ? dbg.name : dbg.short_src );

    TracyQueuePrepare( QueueType::ZoneBeginAllocSrcLocCallstack );
    MemWrite( &item->zoneBegin.time, Profiler::GetTime() );
    MemWrite( &item->zoneBegin.srcloc, srcloc );
    TracyQueueCommit( zoneBeginThread );

    return 0;
}

static inline int LuaZoneBeginNS( lua_State* L )
{
#ifdef TRACY_ON_DEMAND
    const auto zoneCnt = GetLuaZoneState().counter++;
    if( zoneCnt != 0 && !GetLuaZoneState().active ) return 0;
    GetLuaZoneState().active = GetProfiler().IsConnected();
    if( !GetLuaZoneState().active ) return 0;
#endif

#ifdef TRACY_CALLSTACK
    const uint32_t depth = TRACY_CALLSTACK;
#else
    const auto depth = uint32_t( lua_tointeger( L, 2 ) );
#endif
    SendLuaCallstack( L, depth );

    lua_Debug dbg;
    lua_getstack( L, 1, &dbg );
    lua_getinfo( L, "Snl", &dbg );
    size_t nsz;
    char src[256];
    LuaShortenSrc( src, dbg.source );
    const auto name = lua_tolstring( L, 1, &nsz );
    const auto srcloc = Profiler::AllocSourceLocation( dbg.currentline, src, dbg.name ? dbg.name : dbg.short_src, name, nsz );

    TracyQueuePrepare( QueueType::ZoneBeginAllocSrcLocCallstack );
    MemWrite( &item->zoneBegin.time, Profiler::GetTime() );
    MemWrite( &item->zoneBegin.srcloc, srcloc );
    TracyQueueCommit( zoneBeginThread );

    return 0;
}
#endif

static inline int LuaZoneBegin( lua_State* L )
{
#if defined TRACY_HAS_CALLSTACK && defined TRACY_CALLSTACK
    return LuaZoneBeginS( L );
#else
#ifdef TRACY_ON_DEMAND
    const auto zoneCnt = GetLuaZoneState().counter++;
    if( zoneCnt != 0 && !GetLuaZoneState().active ) return 0;
    GetLuaZoneState().active = GetProfiler().IsConnected();
    if( !GetLuaZoneState().active ) return 0;
#endif

    lua_Debug dbg;
    lua_getstack( L, 1, &dbg );
    lua_getinfo( L, "Snl", &dbg );
    char src[256];
    LuaShortenSrc( src, dbg.source );
    const auto srcloc = Profiler::AllocSourceLocation( dbg.currentline, src, dbg.name ? dbg.name : dbg.short_src );

    TracyQueuePrepare( QueueType::ZoneBeginAllocSrcLoc );
    MemWrite( &item->zoneBegin.time, Profiler::GetTime() );
    MemWrite( &item->zoneBegin.srcloc, srcloc );
    TracyQueueCommit( zoneBeginThread );
    return 0;
#endif
}

static inline int LuaZoneBeginN( lua_State* L )
{
#if defined TRACY_HAS_CALLSTACK && defined TRACY_CALLSTACK
    return LuaZoneBeginNS( L );
#else
#ifdef TRACY_ON_DEMAND
    const auto zoneCnt = GetLuaZoneState().counter++;
    if( zoneCnt != 0 && !GetLuaZoneState().active ) return 0;
    GetLuaZoneState().active = GetProfiler().IsConnected();
    if( !GetLuaZoneState().active ) return 0;
#endif

    lua_Debug dbg;
    lua_getstack( L, 1, &dbg );
    lua_getinfo( L, "Snl", &dbg );
    size_t nsz;
    char src[256];
    LuaShortenSrc( src, dbg.source );
    const auto name = lua_tolstring( L, 1, &nsz );
    const auto srcloc = Profiler::AllocSourceLocation( dbg.currentline, src, dbg.name ? dbg.name : dbg.short_src, name, nsz );

    TracyQueuePrepare( QueueType::ZoneBeginAllocSrcLoc );
    MemWrite( &item->zoneBegin.time, Profiler::GetTime() );
    MemWrite( &item->zoneBegin.srcloc, srcloc );
    TracyQueueCommit( zoneBeginThread );
    return 0;
#endif
}

static inline int LuaZoneEnd( lua_State* L )
{
#ifdef TRACY_ON_DEMAND
    assert( GetLuaZoneState().counter != 0 );
    GetLuaZoneState().counter--;
    if( !GetLuaZoneState().active ) return 0;
    if( !GetProfiler().IsConnected() )
    {
        GetLuaZoneState().active = false;
        return 0;
    }
#endif

    TracyQueuePrepare( QueueType::ZoneEnd );
    MemWrite( &item->zoneEnd.time, Profiler::GetTime() );
    TracyQueueCommit( zoneEndThread );
    return 0;
}

static inline int LuaZoneText( lua_State* L )
{
#ifdef TRACY_ON_DEMAND
    if( !GetLuaZoneState().active ) return 0;
    if( !GetProfiler().IsConnected() )
    {
        GetLuaZoneState().active = false;
        return 0;
    }
#endif

    auto txt = lua_tostring( L, 1 );
    const auto size = strlen( txt );
    assert( size < (std::numeric_limits<uint16_t>::max)() );

    auto ptr = (char*)tracy_malloc( size );
    memcpy( ptr, txt, size );

    TracyQueuePrepare( QueueType::ZoneText );
    MemWrite( &item->zoneTextFat.text, (uint64_t)ptr );
    MemWrite( &item->zoneTextFat.size, (uint16_t)size );
    TracyQueueCommit( zoneTextFatThread );
    return 0;
}

static inline int LuaZoneName( lua_State* L )
{
#ifdef TRACY_ON_DEMAND
    if( !GetLuaZoneState().active ) return 0;
    if( !GetProfiler().IsConnected() )
    {
        GetLuaZoneState().active = false;
        return 0;
    }
#endif

    auto txt = lua_tostring( L, 1 );
    const auto size = strlen( txt );
    assert( size < (std::numeric_limits<uint16_t>::max)() );

    auto ptr = (char*)tracy_malloc( size );
    memcpy( ptr, txt, size );

    TracyQueuePrepare( QueueType::ZoneName );
    MemWrite( &item->zoneTextFat.text, (uint64_t)ptr );
    MemWrite( &item->zoneTextFat.size, (uint16_t)size );
    TracyQueueCommit( zoneTextFatThread );
    return 0;
}

static inline int LuaMessage( lua_State* L )
{
#ifdef TRACY_ON_DEMAND
    if( !GetProfiler().IsConnected() ) return 0;
#endif

    auto txt = lua_tostring( L, 1 );
    const auto size = strlen( txt );
    assert( size < (std::numeric_limits<uint16_t>::max)() );

    auto ptr = (char*)tracy_malloc( size );
    memcpy( ptr, txt, size );

    TracyQueuePrepare( QueueType::Message );
    MemWrite( &item->messageFat.time, Profiler::GetTime() );
    MemWrite( &item->messageFat.text, (uint64_t)ptr );
    MemWrite( &item->messageFat.size, (uint16_t)size );
    TracyQueueCommit( messageFatThread );
    return 0;
}

}

static inline void LuaRegister( lua_State* L )
{
    lua_newtable( L );
    lua_pushcfunction( L, detail::LuaZoneBegin );
    lua_setfield( L, -2, "ZoneBegin" );
    lua_pushcfunction( L, detail::LuaZoneBeginN );
    lua_setfield( L, -2, "ZoneBeginN" );
#ifdef TRACY_HAS_CALLSTACK
    lua_pushcfunction( L, detail::LuaZoneBeginS );
    lua_setfield( L, -2, "ZoneBeginS" );
    lua_pushcfunction( L, detail::LuaZoneBeginNS );
    lua_setfield( L, -2, "ZoneBeginNS" );
#else
    lua_pushcfunction( L, detail::LuaZoneBegin );
    lua_setfield( L, -2, "ZoneBeginS" );
    lua_pushcfunction( L, detail::LuaZoneBeginN );
    lua_setfield( L, -2, "ZoneBeginNS" );
#endif
    lua_pushcfunction( L, detail::LuaZoneEnd );
    lua_setfield( L, -2, "ZoneEnd" );
    lua_pushcfunction( L, detail::LuaZoneText );
    lua_setfield( L, -2, "ZoneText" );
    lua_pushcfunction( L, detail::LuaZoneName );
    lua_setfield( L, -2, "ZoneName" );
    lua_pushcfunction( L, detail::LuaMessage );
    lua_setfield( L, -2, "Message" );
    lua_setglobal( L, "tracy" );
}

static inline void LuaRemove( char* script ) {}

static inline void LuaHook( lua_State* L, lua_Debug* ar )
{
    if ( ar->event == LUA_HOOKCALL )
    {
#ifdef TRACY_ON_DEMAND
        const auto zoneCnt = GetLuaZoneState().counter++;
        if ( zoneCnt != 0 && !GetLuaZoneState().active ) return;
        GetLuaZoneState().active = GetProfiler().IsConnected();
        if ( !GetLuaZoneState().active ) return;
#endif
        lua_getinfo( L, "Snl", ar );

        char src[256];
        detail::LuaShortenSrc( src, ar->short_src );

        const auto srcloc = Profiler::AllocSourceLocation( ar->currentline, src, ar->name ? ar->name : ar->short_src );
        TracyQueuePrepare( QueueType::ZoneBeginAllocSrcLoc );
        MemWrite( &item->zoneBegin.time, Profiler::GetTime() );
        MemWrite( &item->zoneBegin.srcloc, srcloc );
        TracyQueueCommit( zoneBeginThread );
    }
    else if (ar->event == LUA_HOOKRET) {
#ifdef TRACY_ON_DEMAND
        assert( GetLuaZoneState().counter != 0 );
        GetLuaZoneState().counter--;
        if ( !GetLuaZoneState().active ) return;
        if ( !GetProfiler().IsConnected() )
        {
            GetLuaZoneState().active = false;
            return;
        }
#endif
        TracyQueuePrepare( QueueType::ZoneEnd );
        MemWrite( &item->zoneEnd.time, Profiler::GetTime() );
        TracyQueueCommit( zoneEndThread );
    }
}

}

#endif

#endif
