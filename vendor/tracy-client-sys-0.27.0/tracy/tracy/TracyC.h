#ifndef __TRACYC_HPP__
#define __TRACYC_HPP__

#include <stddef.h>
#include <stdint.h>

#include "../common/TracyApi.h"

#ifdef __cplusplus
extern "C" {
#endif

enum TracyPlotFormatEnum
{
    TracyPlotFormatNumber,
    TracyPlotFormatMemory,
    TracyPlotFormatPercentage,
    TracyPlotFormatWatt
};

TRACY_API void ___tracy_set_thread_name( const char* name );

#define TracyCSetThreadName( name ) ___tracy_set_thread_name( name );

#ifndef TracyFunction
#  define TracyFunction __FUNCTION__
#endif

#ifndef TracyFile
#  define TracyFile __FILE__
#endif

#ifndef TracyLine
#  define TracyLine __LINE__
#endif

#ifndef TRACY_ENABLE

typedef const void* TracyCZoneCtx;

typedef const void* TracyCLockCtx;

#define TracyCZone(c,x)
#define TracyCZoneN(c,x,y)
#define TracyCZoneC(c,x,y)
#define TracyCZoneNC(c,x,y,z)
#define TracyCZoneEnd(c)
#define TracyCZoneText(c,x,y)
#define TracyCZoneName(c,x,y)
#define TracyCZoneColor(c,x)
#define TracyCZoneValue(c,x)

#define TracyCAlloc(x,y)
#define TracyCFree(x)
#define TracyCMemoryDiscard(x)
#define TracyCSecureAlloc(x,y)
#define TracyCSecureFree(x)
#define TracyCSecureMemoryDiscard(x)

#define TracyCAllocN(x,y,z)
#define TracyCFreeN(x,y)
#define TracyCSecureAllocN(x,y,z)
#define TracyCSecureFreeN(x,y)

#define TracyCFrameMark
#define TracyCFrameMarkNamed(x)
#define TracyCFrameMarkStart(x)
#define TracyCFrameMarkEnd(x)
#define TracyCFrameImage(x,y,z,w,a)

#define TracyCPlot(x,y)
#define TracyCPlotF(x,y)
#define TracyCPlotI(x,y)
#define TracyCPlotConfig(x,y,z,w,a)

#define TracyCMessage(x,y)
#define TracyCMessageL(x)
#define TracyCMessageC(x,y,z)
#define TracyCMessageLC(x,y)
#define TracyCAppInfo(x,y)

#define TracyCZoneS(x,y,z)
#define TracyCZoneNS(x,y,z,w)
#define TracyCZoneCS(x,y,z,w)
#define TracyCZoneNCS(x,y,z,w,a)

#define TracyCAllocS(x,y,z)
#define TracyCFreeS(x,y)
#define TracyCMemoryDiscardS(x,y)
#define TracyCSecureAllocS(x,y,z)
#define TracyCSecureFreeS(x,y)
#define TracyCSecureMemoryDiscardS(x,y)

#define TracyCAllocNS(x,y,z,w)
#define TracyCFreeNS(x,y,z)
#define TracyCSecureAllocNS(x,y,z,w)
#define TracyCSecureFreeNS(x,y,z)

#define TracyCMessageS(x,y,z)
#define TracyCMessageLS(x,y)
#define TracyCMessageCS(x,y,z,w)
#define TracyCMessageLCS(x,y,z)

#define TracyCLockCtx(l)
#define TracyCLockAnnounce(l)
#define TracyCLockTerminate(l)
#define TracyCLockBeforeLock(l)
#define TracyCLockAfterLock(l)
#define TracyCLockAfterUnlock(l)
#define TracyCLockAfterTryLock(l,x)
#define TracyCLockMark(l)
#define TracyCLockCustomName(l,x,y)

#define TracyCIsConnected 0
#define TracyCIsStarted 0

#define TracyCBeginSamplingProfiling() 0
#define TracyCEndSamplingProfiling()

#ifdef TRACY_FIBERS
#  define TracyCFiberEnter(fiber)
#  define TracyCFiberLeave
#endif

#else

#ifndef TracyConcat
#  define TracyConcat(x,y) TracyConcatIndirect(x,y)
#endif
#ifndef TracyConcatIndirect
#  define TracyConcatIndirect(x,y) x##y
#endif

struct ___tracy_source_location_data
{
    const char* name;
    const char* function;
    const char* file;
    uint32_t line;
    uint32_t color;
};

struct ___tracy_c_zone_context
{
    uint32_t id;
    int32_t active;
};

struct ___tracy_gpu_time_data
{
    int64_t gpuTime;
    uint16_t queryId;
    uint8_t context;
};

struct ___tracy_gpu_zone_begin_data {
    uint64_t srcloc;
    uint16_t queryId;
    uint8_t context;
};

struct ___tracy_gpu_zone_begin_callstack_data {
    uint64_t srcloc;
    int32_t depth;
    uint16_t queryId;
    uint8_t context;
};

struct ___tracy_gpu_zone_end_data {
    uint16_t queryId;
    uint8_t context;
};

struct ___tracy_gpu_new_context_data {
    int64_t gpuTime;
    float period;
    uint8_t context;
    uint8_t flags;
    uint8_t type;
};

struct ___tracy_gpu_context_name_data {
    uint8_t context;
    const char* name;
    uint16_t len;
};

struct ___tracy_gpu_calibration_data {
    int64_t gpuTime;
    int64_t cpuDelta;
    uint8_t context;
};

struct ___tracy_gpu_time_sync_data {
    int64_t gpuTime;
    uint8_t context;
};

struct __tracy_lockable_context_data;

// Some containers don't support storing const types.
// This struct, as visible to user, is immutable, so treat it as if const was declared here.
typedef /*const*/ struct ___tracy_c_zone_context TracyCZoneCtx;

typedef struct __tracy_lockable_context_data* TracyCLockCtx;

#ifdef TRACY_MANUAL_LIFETIME
TRACY_API void ___tracy_startup_profiler(void);
TRACY_API void ___tracy_shutdown_profiler(void);
TRACY_API int32_t ___tracy_profiler_started(void);

#  define TracyCIsStarted ___tracy_profiler_started()
#else
#  define TracyCIsStarted 1
#endif

TRACY_API uint64_t ___tracy_alloc_srcloc( uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, uint32_t color );
TRACY_API uint64_t ___tracy_alloc_srcloc_name( uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, uint32_t color );

TRACY_API TracyCZoneCtx ___tracy_emit_zone_begin( const struct ___tracy_source_location_data* srcloc, int32_t active );
TRACY_API TracyCZoneCtx ___tracy_emit_zone_begin_callstack( const struct ___tracy_source_location_data* srcloc, int32_t depth, int32_t active );
TRACY_API TracyCZoneCtx ___tracy_emit_zone_begin_alloc( uint64_t srcloc, int32_t active );
TRACY_API TracyCZoneCtx ___tracy_emit_zone_begin_alloc_callstack( uint64_t srcloc, int32_t depth, int32_t active );
TRACY_API void ___tracy_emit_zone_end( TracyCZoneCtx ctx );
TRACY_API void ___tracy_emit_zone_text( TracyCZoneCtx ctx, const char* txt, size_t size );
TRACY_API void ___tracy_emit_zone_name( TracyCZoneCtx ctx, const char* txt, size_t size );
TRACY_API void ___tracy_emit_zone_color( TracyCZoneCtx ctx, uint32_t color );
TRACY_API void ___tracy_emit_zone_value( TracyCZoneCtx ctx, uint64_t value );

TRACY_API void ___tracy_emit_gpu_zone_begin( const struct ___tracy_gpu_zone_begin_data );
TRACY_API void ___tracy_emit_gpu_zone_begin_callstack( const struct ___tracy_gpu_zone_begin_callstack_data );
TRACY_API void ___tracy_emit_gpu_zone_begin_alloc( const struct ___tracy_gpu_zone_begin_data );
TRACY_API void ___tracy_emit_gpu_zone_begin_alloc_callstack( const struct ___tracy_gpu_zone_begin_callstack_data );
TRACY_API void ___tracy_emit_gpu_zone_end( const struct ___tracy_gpu_zone_end_data data );
TRACY_API void ___tracy_emit_gpu_time( const struct ___tracy_gpu_time_data );
TRACY_API void ___tracy_emit_gpu_new_context( const struct ___tracy_gpu_new_context_data );
TRACY_API void ___tracy_emit_gpu_context_name( const struct ___tracy_gpu_context_name_data );
TRACY_API void ___tracy_emit_gpu_calibration( const struct ___tracy_gpu_calibration_data );
TRACY_API void ___tracy_emit_gpu_time_sync( const struct ___tracy_gpu_time_sync_data );

TRACY_API void ___tracy_emit_gpu_zone_begin_serial( const struct ___tracy_gpu_zone_begin_data );
TRACY_API void ___tracy_emit_gpu_zone_begin_callstack_serial( const struct ___tracy_gpu_zone_begin_callstack_data );
TRACY_API void ___tracy_emit_gpu_zone_begin_alloc_serial( const struct ___tracy_gpu_zone_begin_data );
TRACY_API void ___tracy_emit_gpu_zone_begin_alloc_callstack_serial( const struct ___tracy_gpu_zone_begin_callstack_data );
TRACY_API void ___tracy_emit_gpu_zone_end_serial( const struct ___tracy_gpu_zone_end_data data );
TRACY_API void ___tracy_emit_gpu_time_serial( const struct ___tracy_gpu_time_data );
TRACY_API void ___tracy_emit_gpu_new_context_serial( const struct ___tracy_gpu_new_context_data );
TRACY_API void ___tracy_emit_gpu_context_name_serial( const struct ___tracy_gpu_context_name_data );
TRACY_API void ___tracy_emit_gpu_calibration_serial( const struct ___tracy_gpu_calibration_data );
TRACY_API void ___tracy_emit_gpu_time_sync_serial( const struct ___tracy_gpu_time_sync_data );

TRACY_API int32_t ___tracy_connected(void);

#ifndef TRACY_CALLSTACK
#define TRACY_CALLSTACK 0
#endif

#define TracyCZone( ctx, active ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { NULL, __func__,  TracyFile, (uint32_t)TracyLine, 0 }; TracyCZoneCtx ctx = ___tracy_emit_zone_begin_callstack( &TracyConcat(__tracy_source_location,TracyLine), TRACY_CALLSTACK, active );
#define TracyCZoneN( ctx, name, active ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { name, __func__,  TracyFile, (uint32_t)TracyLine, 0 }; TracyCZoneCtx ctx = ___tracy_emit_zone_begin_callstack( &TracyConcat(__tracy_source_location,TracyLine), TRACY_CALLSTACK, active );
#define TracyCZoneC( ctx, color, active ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { NULL, __func__,  TracyFile, (uint32_t)TracyLine, color }; TracyCZoneCtx ctx = ___tracy_emit_zone_begin_callstack( &TracyConcat(__tracy_source_location,TracyLine), TRACY_CALLSTACK, active );
#define TracyCZoneNC( ctx, name, color, active ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { name, __func__,  TracyFile, (uint32_t)TracyLine, color }; TracyCZoneCtx ctx = ___tracy_emit_zone_begin_callstack( &TracyConcat(__tracy_source_location,TracyLine), TRACY_CALLSTACK, active );

#define TracyCZoneEnd( ctx ) ___tracy_emit_zone_end( ctx );

#define TracyCZoneText( ctx, txt, size ) ___tracy_emit_zone_text( ctx, txt, size );
#define TracyCZoneName( ctx, txt, size ) ___tracy_emit_zone_name( ctx, txt, size );
#define TracyCZoneColor( ctx, color ) ___tracy_emit_zone_color( ctx, color );
#define TracyCZoneValue( ctx, value ) ___tracy_emit_zone_value( ctx, value );


TRACY_API void ___tracy_emit_memory_alloc( const void* ptr, size_t size, int32_t secure );
TRACY_API void ___tracy_emit_memory_alloc_callstack( const void* ptr, size_t size, int32_t depth, int32_t secure );
TRACY_API void ___tracy_emit_memory_free( const void* ptr, int32_t secure );
TRACY_API void ___tracy_emit_memory_free_callstack( const void* ptr, int32_t depth, int32_t secure );
TRACY_API void ___tracy_emit_memory_alloc_named( const void* ptr, size_t size, int32_t secure, const char* name );
TRACY_API void ___tracy_emit_memory_alloc_callstack_named( const void* ptr, size_t size, int32_t depth, int32_t secure, const char* name );
TRACY_API void ___tracy_emit_memory_free_named( const void* ptr, int32_t secure, const char* name );
TRACY_API void ___tracy_emit_memory_free_callstack_named( const void* ptr, int32_t depth, int32_t secure, const char* name );
TRACY_API void ___tracy_emit_memory_discard( const char* name, int32_t secure );
TRACY_API void ___tracy_emit_memory_discard_callstack( const char* name, int32_t secure, int32_t depth );

TRACY_API void ___tracy_emit_message( const char* txt, size_t size, int32_t callstack_depth );
TRACY_API void ___tracy_emit_messageL( const char* txt, int32_t callstack_depth );
TRACY_API void ___tracy_emit_messageC( const char* txt, size_t size, uint32_t color, int32_t callstack_depth );
TRACY_API void ___tracy_emit_messageLC( const char* txt, uint32_t color, int32_t callstack_depth );

#define TracyCAlloc( ptr, size ) ___tracy_emit_memory_alloc_callstack( ptr, size, TRACY_CALLSTACK, 0 )
#define TracyCFree( ptr ) ___tracy_emit_memory_free_callstack( ptr, TRACY_CALLSTACK, 0 )
#define TracyCMemoryDiscard( name ) ___tracy_emit_memory_discard_callstack( name, 0, TRACY_CALLSTACK );
#define TracyCSecureAlloc( ptr, size ) ___tracy_emit_memory_alloc_callstack( ptr, size, TRACY_CALLSTACK, 1 )
#define TracyCSecureFree( ptr ) ___tracy_emit_memory_free_callstack( ptr, TRACY_CALLSTACK, 1 )
#define TracyCSecureMemoryDiscard( name ) ___tracy_emit_memory_discard_callstack( name, 1, TRACY_CALLSTACK );

#define TracyCAllocN( ptr, size, name ) ___tracy_emit_memory_alloc_callstack_named( ptr, size, TRACY_CALLSTACK, 0, name )
#define TracyCFreeN( ptr, name ) ___tracy_emit_memory_free_callstack_named( ptr, TRACY_CALLSTACK, 0, name )
#define TracyCSecureAllocN( ptr, size, name ) ___tracy_emit_memory_alloc_callstack_named( ptr, size, TRACY_CALLSTACK, 1, name )
#define TracyCSecureFreeN( ptr, name ) ___tracy_emit_memory_free_callstack_named( ptr, TRACY_CALLSTACK, 1, name )

#define TracyCMessage( txt, size ) ___tracy_emit_message( txt, size, TRACY_CALLSTACK );
#define TracyCMessageL( txt ) ___tracy_emit_messageL( txt, TRACY_CALLSTACK );
#define TracyCMessageC( txt, size, color ) ___tracy_emit_messageC( txt, size, color, TRACY_CALLSTACK );
#define TracyCMessageLC( txt, color ) ___tracy_emit_messageLC( txt, color, TRACY_CALLSTACK );


TRACY_API void ___tracy_emit_frame_mark( const char* name );
TRACY_API void ___tracy_emit_frame_mark_start( const char* name );
TRACY_API void ___tracy_emit_frame_mark_end( const char* name );
TRACY_API void ___tracy_emit_frame_image( const void* image, uint16_t w, uint16_t h, uint8_t offset, int32_t flip );

#define TracyCFrameMark ___tracy_emit_frame_mark( 0 );
#define TracyCFrameMarkNamed( name ) ___tracy_emit_frame_mark( name );
#define TracyCFrameMarkStart( name ) ___tracy_emit_frame_mark_start( name );
#define TracyCFrameMarkEnd( name ) ___tracy_emit_frame_mark_end( name );
#define TracyCFrameImage( image, width, height, offset, flip ) ___tracy_emit_frame_image( image, width, height, offset, flip );


TRACY_API void ___tracy_emit_plot( const char* name, double val );
TRACY_API void ___tracy_emit_plot_float( const char* name, float val );
TRACY_API void ___tracy_emit_plot_int( const char* name, int64_t val );
TRACY_API void ___tracy_emit_plot_config( const char* name, int32_t type, int32_t step, int32_t fill, uint32_t color );
TRACY_API void ___tracy_emit_message_appinfo( const char* txt, size_t size );

#define TracyCPlot( name, val ) ___tracy_emit_plot( name, val );
#define TracyCPlotF( name, val ) ___tracy_emit_plot_float( name, val );
#define TracyCPlotI( name, val ) ___tracy_emit_plot_int( name, val );
#define TracyCPlotConfig( name, type, step, fill, color ) ___tracy_emit_plot_config( name, type, step, fill, color );
#define TracyCAppInfo( txt, size ) ___tracy_emit_message_appinfo( txt, size );


#define TracyCZoneS( ctx, depth, active ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { NULL, __func__,  TracyFile, (uint32_t)TracyLine, 0 }; TracyCZoneCtx ctx = ___tracy_emit_zone_begin_callstack( &TracyConcat(__tracy_source_location,TracyLine), depth, active );
#define TracyCZoneNS( ctx, name, depth, active ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { name, __func__,  TracyFile, (uint32_t)TracyLine, 0 }; TracyCZoneCtx ctx = ___tracy_emit_zone_begin_callstack( &TracyConcat(__tracy_source_location,TracyLine), depth, active );
#define TracyCZoneCS( ctx, color, depth, active ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { NULL, __func__,  TracyFile, (uint32_t)TracyLine, color }; TracyCZoneCtx ctx = ___tracy_emit_zone_begin_callstack( &TracyConcat(__tracy_source_location,TracyLine), depth, active );
#define TracyCZoneNCS( ctx, name, color, depth, active ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { name, __func__,  TracyFile, (uint32_t)TracyLine, color }; TracyCZoneCtx ctx = ___tracy_emit_zone_begin_callstack( &TracyConcat(__tracy_source_location,TracyLine), depth, active );

#define TracyCAllocS( ptr, size, depth ) ___tracy_emit_memory_alloc_callstack( ptr, size, depth, 0 )
#define TracyCFreeS( ptr, depth ) ___tracy_emit_memory_free_callstack( ptr, depth, 0 )
#define TracyCMemoryDiscardS( name, depth ) ___tracy_emit_memory_discard_callstack( name, 0, depth )
#define TracyCSecureAllocS( ptr, size, depth ) ___tracy_emit_memory_alloc_callstack( ptr, size, depth, 1 )
#define TracyCSecureFreeS( ptr, depth ) ___tracy_emit_memory_free_callstack( ptr, depth, 1 )
#define TracyCSecureMemoryDiscardS( name, depth ) ___tracy_emit_memory_discard_callstack( name, 1, depth )

#define TracyCAllocNS( ptr, size, depth, name ) ___tracy_emit_memory_alloc_callstack_named( ptr, size, depth, 0, name )
#define TracyCFreeNS( ptr, depth, name ) ___tracy_emit_memory_free_callstack_named( ptr, depth, 0, name )
#define TracyCSecureAllocNS( ptr, size, depth, name ) ___tracy_emit_memory_alloc_callstack_named( ptr, size, depth, 1, name )
#define TracyCSecureFreeNS( ptr, depth, name ) ___tracy_emit_memory_free_callstack_named( ptr, depth, 1, name )

#define TracyCMessageS( txt, size, depth ) ___tracy_emit_message( txt, size, depth );
#define TracyCMessageLS( txt, depth ) ___tracy_emit_messageL( txt, depth );
#define TracyCMessageCS( txt, size, color, depth ) ___tracy_emit_messageC( txt, size, color, depth );
#define TracyCMessageLCS( txt, color, depth ) ___tracy_emit_messageLC( txt, color, depth );


TRACY_API struct __tracy_lockable_context_data* ___tracy_announce_lockable_ctx( const struct ___tracy_source_location_data* srcloc );
TRACY_API void ___tracy_terminate_lockable_ctx( struct __tracy_lockable_context_data* lockdata );
TRACY_API int32_t ___tracy_before_lock_lockable_ctx( struct __tracy_lockable_context_data* lockdata );
TRACY_API void ___tracy_after_lock_lockable_ctx( struct __tracy_lockable_context_data* lockdata );
TRACY_API void ___tracy_after_unlock_lockable_ctx( struct __tracy_lockable_context_data* lockdata );
TRACY_API void ___tracy_after_try_lock_lockable_ctx( struct __tracy_lockable_context_data* lockdata, int32_t acquired );
TRACY_API void ___tracy_mark_lockable_ctx( struct __tracy_lockable_context_data* lockdata, const struct ___tracy_source_location_data* srcloc );
TRACY_API void ___tracy_custom_name_lockable_ctx( struct __tracy_lockable_context_data* lockdata, const char* name, size_t nameSz );

#define TracyCLockAnnounce( lock ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { NULL, __func__,  TracyFile, (uint32_t)TracyLine, 0 }; lock = ___tracy_announce_lockable_ctx( &TracyConcat(__tracy_source_location,TracyLine) );
#define TracyCLockTerminate( lock ) ___tracy_terminate_lockable_ctx( lock );
#define TracyCLockBeforeLock( lock ) ___tracy_before_lock_lockable_ctx( lock );
#define TracyCLockAfterLock( lock ) ___tracy_after_lock_lockable_ctx( lock );
#define TracyCLockAfterUnlock( lock ) ___tracy_after_unlock_lockable_ctx( lock );
#define TracyCLockAfterTryLock( lock, acquired ) ___tracy_after_try_lock_lockable_ctx( lock, acquired );
#define TracyCLockMark( lock ) static const struct ___tracy_source_location_data TracyConcat(__tracy_source_location,TracyLine) = { NULL, __func__,  TracyFile, (uint32_t)TracyLine, 0 }; ___tracy_mark_lockable_ctx( lock, &TracyConcat(__tracy_source_location,TracyLine) );
#define TracyCLockCustomName( lock, name, nameSz ) ___tracy_custom_name_lockable_ctx( lock, name, nameSz );

#define TracyCIsConnected ___tracy_connected()

TRACY_API int ___tracy_begin_sampling_profiler( void );
TRACY_API void ___tracy_end_sampling_profiler( void );

#define TracyCBeginSamplingProfiling() ___tracy_begin_sampling_profiling()
#define TracyCEndSamplingProfiling() ___tracy_end_sampling_profiling()

#ifdef TRACY_FIBERS
TRACY_API void ___tracy_fiber_enter( const char* fiber );
TRACY_API void ___tracy_fiber_leave( void );

#  define TracyCFiberEnter( fiber ) ___tracy_fiber_enter( fiber );
#  define TracyCFiberLeave ___tracy_fiber_leave();
#endif

#endif

#ifdef __cplusplus
}
#endif

#endif
