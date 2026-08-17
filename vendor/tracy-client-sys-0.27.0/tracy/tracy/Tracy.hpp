#ifndef __TRACY_HPP__
#define __TRACY_HPP__

#include "../common/TracyColor.hpp"
#include "../common/TracySystem.hpp"

#ifndef TracyFunction
#  define TracyFunction __FUNCTION__
#endif

#ifndef TracyFile
#  define TracyFile __FILE__
#endif

#ifndef TracyLine
#  define TracyLine TracyConcat(__LINE__,U) // MSVC Edit and continue __LINE__ is non-constant. See https://developercommunity.visualstudio.com/t/-line-cannot-be-used-as-an-argument-for-constexpr/195665
#endif

#ifndef TRACY_ENABLE

#define TracyNoop

#define ZoneNamed(x,y)
#define ZoneNamedN(x,y,z)
#define ZoneNamedC(x,y,z)
#define ZoneNamedNC(x,y,z,w)

#define ZoneTransient(x,y)
#define ZoneTransientN(x,y,z)

#define ZoneScoped
#define ZoneScopedN(x)
#define ZoneScopedC(x)
#define ZoneScopedNC(x,y)

#define ZoneText(x,y)
#define ZoneTextV(x,y,z)
#define ZoneTextF(x,...)
#define ZoneTextVF(x,y,...)
#define ZoneName(x,y)
#define ZoneNameV(x,y,z)
#define ZoneNameF(x,...)
#define ZoneNameVF(x,y,...)
#define ZoneColor(x)
#define ZoneColorV(x,y)
#define ZoneValue(x)
#define ZoneValueV(x,y)
#define ZoneIsActive false
#define ZoneIsActiveV(x) false

#define FrameMark
#define FrameMarkNamed(x)
#define FrameMarkStart(x)
#define FrameMarkEnd(x)

#define FrameImage(x,y,z,w,a)

#define TracyLockable( type, varname ) type varname
#define TracyLockableN( type, varname, desc ) type varname
#define TracySharedLockable( type, varname ) type varname
#define TracySharedLockableN( type, varname, desc ) type varname
#define LockableBase( type ) type
#define SharedLockableBase( type ) type
#define LockMark(x) (void)x
#define LockableName(x,y,z)

#define TracyPlot(x,y)
#define TracyPlotConfig(x,y,z,w,a)

#define TracyMessage(x,y)
#define TracyMessageL(x)
#define TracyMessageC(x,y,z)
#define TracyMessageLC(x,y)
#define TracyAppInfo(x,y)

#define TracyAlloc(x,y)
#define TracyFree(x)
#define TracyMemoryDiscard(x)
#define TracySecureAlloc(x,y)
#define TracySecureFree(x)
#define TracySecureMemoryDiscard(x)

#define TracyAllocN(x,y,z)
#define TracyFreeN(x,y)
#define TracySecureAllocN(x,y,z)
#define TracySecureFreeN(x,y)

#define ZoneNamedS(x,y,z)
#define ZoneNamedNS(x,y,z,w)
#define ZoneNamedCS(x,y,z,w)
#define ZoneNamedNCS(x,y,z,w,a)

#define ZoneTransientS(x,y,z)
#define ZoneTransientNS(x,y,z,w)

#define ZoneScopedS(x)
#define ZoneScopedNS(x,y)
#define ZoneScopedCS(x,y)
#define ZoneScopedNCS(x,y,z)

#define TracyAllocS(x,y,z)
#define TracyFreeS(x,y)
#define TracyMemoryDiscardS(x,y)
#define TracySecureAllocS(x,y,z)
#define TracySecureFreeS(x,y)
#define TracySecureMemoryDiscardS(x,y)

#define TracyAllocNS(x,y,z,w)
#define TracyFreeNS(x,y,z)
#define TracySecureAllocNS(x,y,z,w)
#define TracySecureFreeNS(x,y,z)

#define TracyMessageS(x,y,z)
#define TracyMessageLS(x,y)
#define TracyMessageCS(x,y,z,w)
#define TracyMessageLCS(x,y,z)

#define TracySourceCallbackRegister(x,y)
#define TracyParameterRegister(x,y)
#define TracyParameterSetup(x,y,z,w)
#define TracyIsConnected false
#define TracyIsStarted false
#define TracySetProgramName(x)

#define TracyFiberEnter(x)
#define TracyFiberEnterHint(x,y)
#define TracyFiberLeave

#else

#include <string.h>

#include "../client/TracyLock.hpp"
#include "../client/TracyProfiler.hpp"
#include "../client/TracyScoped.hpp"

#ifndef TRACY_CALLSTACK
#define TRACY_CALLSTACK 0
#endif

#define TracyNoop tracy::ProfilerAvailable()

#define ZoneNamed( varname, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_source_location,TracyLine) { nullptr, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::ScopedZone varname( &TracyConcat(__tracy_source_location,TracyLine), TRACY_CALLSTACK, active )
#define ZoneNamedN( varname, name, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::ScopedZone varname( &TracyConcat(__tracy_source_location,TracyLine), TRACY_CALLSTACK, active )
#define ZoneNamedC( varname, color, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_source_location,TracyLine) { nullptr, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::ScopedZone varname( &TracyConcat(__tracy_source_location,TracyLine), TRACY_CALLSTACK, active )
#define ZoneNamedNC( varname, name, color, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::ScopedZone varname( &TracyConcat(__tracy_source_location,TracyLine), TRACY_CALLSTACK, active )

#define ZoneTransient( varname, active ) tracy::ScopedZone varname( TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), nullptr, 0, TRACY_CALLSTACK, active )
#define ZoneTransientN( varname, name, active ) tracy::ScopedZone varname( TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), TRACY_CALLSTACK, active )
#define ZoneTransientNC( varname, name, color, active ) tracy::ScopedZone varname( TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), color, TRACY_CALLSTACK, active )

#if defined(TRACY_ALLOW_SHADOW_WARNING)
    #define SuppressVarShadowWarning(Expr) Expr
#elif defined(__clang__)
    #define SuppressVarShadowWarning(Expr) \
        _Pragma("clang diagnostic push") \
        _Pragma("clang diagnostic ignored \"-Wshadow\"") \
        Expr \
        _Pragma("clang diagnostic pop")
#elif defined(__GNU__)
    #define SuppressVarShadowWarning(Expr) \
        _Pragma("GCC diagnostic push") \
        _Pragma("GCC diagnostic ignored \"-Wshadow\"") \
        Expr \
        _Pragma("GCC diagnostic pop")
#elif defined(_MSC_VER) 
    #define SuppressVarShadowWarning(Expr) \
        _Pragma("warning(push)") \
        _Pragma("warning(disable : 4456)") \
        Expr \
        _Pragma("warning(pop)")
#else
    #define SuppressVarShadowWarning(Expr) Expr
#endif

#define ZoneScoped SuppressVarShadowWarning( ZoneNamed( ___tracy_scoped_zone, true ) )
#define ZoneScopedN( name ) SuppressVarShadowWarning( ZoneNamedN( ___tracy_scoped_zone, name, true ) )
#define ZoneScopedC( color ) SuppressVarShadowWarning( ZoneNamedC( ___tracy_scoped_zone, color, true ) )
#define ZoneScopedNC( name, color ) SuppressVarShadowWarning( ZoneNamedNC( ___tracy_scoped_zone, name, color, true ) )

#define ZoneText( txt, size ) ___tracy_scoped_zone.Text( txt, size )
#define ZoneTextV( varname, txt, size ) varname.Text( txt, size )
#define ZoneTextF( fmt, ... ) ___tracy_scoped_zone.TextFmt( fmt, ##__VA_ARGS__ )
#define ZoneTextVF( varname, fmt, ... ) varname.TextFmt( fmt, ##__VA_ARGS__ )
#define ZoneName( txt, size ) ___tracy_scoped_zone.Name( txt, size )
#define ZoneNameV( varname, txt, size ) varname.Name( txt, size )
#define ZoneNameF( fmt, ... ) ___tracy_scoped_zone.NameFmt( fmt, ##__VA_ARGS__ )
#define ZoneNameVF( varname, fmt, ... ) varname.NameFmt( fmt, ##__VA_ARGS__ )
#define ZoneColor( color ) ___tracy_scoped_zone.Color( color )
#define ZoneColorV( varname, color ) varname.Color( color )
#define ZoneValue( value ) ___tracy_scoped_zone.Value( value )
#define ZoneValueV( varname, value ) varname.Value( value )
#define ZoneIsActive ___tracy_scoped_zone.IsActive()
#define ZoneIsActiveV( varname ) varname.IsActive()

#define FrameMark tracy::Profiler::SendFrameMark( nullptr )
#define FrameMarkNamed( name ) tracy::Profiler::SendFrameMark( name )
#define FrameMarkStart( name ) tracy::Profiler::SendFrameMark( name, tracy::QueueType::FrameMarkMsgStart )
#define FrameMarkEnd( name ) tracy::Profiler::SendFrameMark( name, tracy::QueueType::FrameMarkMsgEnd )

#define FrameImage( image, width, height, offset, flip ) tracy::Profiler::SendFrameImage( image, width, height, offset, flip )

#define TracyLockable( type, varname ) tracy::Lockable<type> varname { [] () -> const tracy::SourceLocationData* { static constexpr tracy::SourceLocationData srcloc { nullptr, #type " " #varname, TracyFile, TracyLine, 0 }; return &srcloc; }() }
#define TracyLockableN( type, varname, desc ) tracy::Lockable<type> varname { [] () -> const tracy::SourceLocationData* { static constexpr tracy::SourceLocationData srcloc { nullptr, desc, TracyFile, TracyLine, 0 }; return &srcloc; }() }
#define TracySharedLockable( type, varname ) tracy::SharedLockable<type> varname { [] () -> const tracy::SourceLocationData* { static constexpr tracy::SourceLocationData srcloc { nullptr, #type " " #varname, TracyFile, TracyLine, 0 }; return &srcloc; }() }
#define TracySharedLockableN( type, varname, desc ) tracy::SharedLockable<type> varname { [] () -> const tracy::SourceLocationData* { static constexpr tracy::SourceLocationData srcloc { nullptr, desc, TracyFile, TracyLine, 0 }; return &srcloc; }() }
#define LockableBase( type ) tracy::Lockable<type>
#define SharedLockableBase( type ) tracy::SharedLockable<type>
#define LockMark( varname ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_lock_location_,TracyLine) { nullptr, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; varname.Mark( &TracyConcat(__tracy_lock_location_,TracyLine) )
#define LockableName( varname, txt, size ) varname.CustomName( txt, size )

#define TracyPlot( name, val ) tracy::Profiler::PlotData( name, val )
#define TracyPlotConfig( name, type, step, fill, color ) tracy::Profiler::ConfigurePlot( name, type, step, fill, color )

#define TracyAppInfo( txt, size ) tracy::Profiler::MessageAppInfo( txt, size )

#define TracyMessage( txt, size ) tracy::Profiler::Message( txt, size, TRACY_CALLSTACK )
#define TracyMessageL( txt ) tracy::Profiler::Message( txt, TRACY_CALLSTACK )
#define TracyMessageC( txt, size, color ) tracy::Profiler::MessageColor( txt, size, color, TRACY_CALLSTACK )
#define TracyMessageLC( txt, color ) tracy::Profiler::MessageColor( txt, color, TRACY_CALLSTACK )

#define TracyAlloc( ptr, size ) tracy::Profiler::MemAllocCallstack( ptr, size, TRACY_CALLSTACK, false )
#define TracyFree( ptr ) tracy::Profiler::MemFreeCallstack( ptr, TRACY_CALLSTACK, false )
#define TracySecureAlloc( ptr, size ) tracy::Profiler::MemAllocCallstack( ptr, size, TRACY_CALLSTACK, true )
#define TracySecureFree( ptr ) tracy::Profiler::MemFreeCallstack( ptr, TRACY_CALLSTACK, true )

#define TracyAllocN( ptr, size, name ) tracy::Profiler::MemAllocCallstackNamed( ptr, size, TRACY_CALLSTACK, false, name )
#define TracyFreeN( ptr, name ) tracy::Profiler::MemFreeCallstackNamed( ptr, TRACY_CALLSTACK, false, name )
#define TracyMemoryDiscard( name ) tracy::Profiler::MemDiscardCallstack( name, false, TRACY_CALLSTACK )
#define TracySecureAllocN( ptr, size, name ) tracy::Profiler::MemAllocCallstackNamed( ptr, size, TRACY_CALLSTACK, true, name )
#define TracySecureFreeN( ptr, name ) tracy::Profiler::MemFreeCallstackNamed( ptr, TRACY_CALLSTACK, true, name )
#define TracySecureMemoryDiscard( name ) tracy::Profiler::MemDiscardCallstack( name, true, TRACY_CALLSTACK )

#define ZoneNamedS( varname, depth, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_source_location,TracyLine) { nullptr, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::ScopedZone varname( &TracyConcat(__tracy_source_location,TracyLine), depth, active )
#define ZoneNamedNS( varname, name, depth, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::ScopedZone varname( &TracyConcat(__tracy_source_location,TracyLine), depth, active )
#define ZoneNamedCS( varname, color, depth, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_source_location,TracyLine) { nullptr, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::ScopedZone varname( &TracyConcat(__tracy_source_location,TracyLine), depth, active )
#define ZoneNamedNCS( varname, name, color, depth, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::ScopedZone varname( &TracyConcat(__tracy_source_location,TracyLine), depth, active )

#define ZoneTransientS( varname, depth, active ) tracy::ScopedZone varname( TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), nullptr, 0, depth, active )
#define ZoneTransientNS( varname, name, depth, active ) tracy::ScopedZone varname( TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), depth, active )

#define ZoneScopedS( depth ) ZoneNamedS( ___tracy_scoped_zone, depth, true )
#define ZoneScopedNS( name, depth ) ZoneNamedNS( ___tracy_scoped_zone, name, depth, true )
#define ZoneScopedCS( color, depth ) ZoneNamedCS( ___tracy_scoped_zone, color, depth, true )
#define ZoneScopedNCS( name, color, depth ) ZoneNamedNCS( ___tracy_scoped_zone, name, color, depth, true )

#define TracyAllocS( ptr, size, depth ) tracy::Profiler::MemAllocCallstack( ptr, size, depth, false )
#define TracyFreeS( ptr, depth ) tracy::Profiler::MemFreeCallstack( ptr, depth, false )
#define TracySecureAllocS( ptr, size, depth ) tracy::Profiler::MemAllocCallstack( ptr, size, depth, true )
#define TracySecureFreeS( ptr, depth ) tracy::Profiler::MemFreeCallstack( ptr, depth, true )

#define TracyAllocNS( ptr, size, depth, name ) tracy::Profiler::MemAllocCallstackNamed( ptr, size, depth, false, name )
#define TracyFreeNS( ptr, depth, name ) tracy::Profiler::MemFreeCallstackNamed( ptr, depth, false, name )
#define TracyMemoryDiscardS( name, depth ) tracy::Profiler::MemDiscardCallstack( name, false, depth )
#define TracySecureAllocNS( ptr, size, depth, name ) tracy::Profiler::MemAllocCallstackNamed( ptr, size, depth, true, name )
#define TracySecureFreeNS( ptr, depth, name ) tracy::Profiler::MemFreeCallstackNamed( ptr, depth, true, name )
#define TracySecureMemoryDiscardS( name, depth ) tracy::Profiler::MemDiscardCallstack( name, true, depth )

#define TracyMessageS( txt, size, depth ) tracy::Profiler::Message( txt, size, depth )
#define TracyMessageLS( txt, depth ) tracy::Profiler::Message( txt, depth )
#define TracyMessageCS( txt, size, color, depth ) tracy::Profiler::MessageColor( txt, size, color, depth )
#define TracyMessageLCS( txt, color, depth ) tracy::Profiler::MessageColor( txt, color, depth )

#define TracySourceCallbackRegister( cb, data ) tracy::Profiler::SourceCallbackRegister( cb, data )
#define TracyParameterRegister( cb, data ) tracy::Profiler::ParameterRegister( cb, data )
#define TracyParameterSetup( idx, name, isBool, val ) tracy::Profiler::ParameterSetup( idx, name, isBool, val )
#define TracyIsConnected tracy::GetProfiler().IsConnected()
#define TracySetProgramName( name ) tracy::GetProfiler().SetProgramName( name );

#ifdef TRACY_FIBERS
#  define TracyFiberEnter( fiber ) tracy::Profiler::EnterFiber( fiber, 0 )
#  define TracyFiberEnterHint( fiber, groupHint ) tracy::Profiler::EnterFiber( fiber, groupHint )
#  define TracyFiberLeave tracy::Profiler::LeaveFiber()
#endif

#endif

#endif
