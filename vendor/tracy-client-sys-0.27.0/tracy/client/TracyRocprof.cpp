#include "../server/tracy_robin_hood.h"
#include "TracyProfiler.hpp"
#include "TracyThread.hpp"
#include "tracy/TracyC.h"
#include <rocprofiler-sdk/registration.h>
#include <rocprofiler-sdk/rocprofiler.h>

#include <iostream>
#include <mutex>
#include <set>
#include <shared_mutex>
#include <sstream>
#include <time.h>
#include <unordered_map>
#include <vector>

#define ROCPROFILER_CALL( result, msg )                                                                                \
    {                                                                                                                  \
        rocprofiler_status_t CHECKSTATUS = result;                                                                     \
        if( CHECKSTATUS != ROCPROFILER_STATUS_SUCCESS )                                                                \
        {                                                                                                              \
            std::string status_msg = rocprofiler_get_status_string( CHECKSTATUS );                                     \
            std::cerr << "[" #result "][" << __FILE__ << ":" << __LINE__ << "] " << msg << " failed with error code "  \
                      << CHECKSTATUS << ": " << status_msg << std::endl;                                               \
            std::stringstream errmsg{};                                                                                \
            errmsg << "[" #result "][" << __FILE__ << ":" << __LINE__ << "] " << msg " failure (" << status_msg        \
                   << ")";                                                                                             \
            throw std::runtime_error( errmsg.str() );                                                                  \
        }                                                                                                              \
    }

namespace
{

using kernel_symbol_data_t = rocprofiler_callback_tracing_code_object_kernel_symbol_register_data_t;

struct DispatchData
{
    int64_t launch_start;
    int64_t launch_end;
    uint32_t thread_id;
    uint16_t query_id;
};

struct ToolData
{
    uint32_t version;
    const char* runtime_version;
    uint32_t priority;
    rocprofiler_client_id_t client_id;
    uint8_t context_id;
    bool init;
    uint64_t query_id;
    int64_t previous_cpu_time;
    tracy::unordered_map<rocprofiler_kernel_id_t, kernel_symbol_data_t> client_kernels;
    tracy::unordered_map<rocprofiler_dispatch_id_t, DispatchData> dispatch_data;
    tracy::unordered_set<std::string> counter_names = { "SQ_WAVES", "GL2C_MISS", "GL2C_HIT" };
    std::unique_ptr<tracy::Thread> cal_thread;
    std::mutex mut{};
};

using namespace tracy;

rocprofiler_context_id_t& get_client_ctx()
{
    static rocprofiler_context_id_t ctx{ 0 };
    return ctx;
}

const char* CTX_NAME = "rocprofv3";

uint8_t gpu_context_allocate( ToolData* data )
{

    timespec ts;
    clock_gettime( CLOCK_BOOTTIME, &ts );
    uint64_t cpu_timestamp = Profiler::GetTime();
    uint64_t gpu_timestamp = ( (uint64_t)ts.tv_sec * 1000000000 ) + ts.tv_nsec;
    float timestamp_period = 1.0f;
    data->previous_cpu_time = cpu_timestamp;

    // Allocate the process-unique GPU context ID. There's a max of 255 available;
    // if we are recreating devices a lot we may exceed that. Don't do that, or
    // wrap around and get weird (but probably still usable) numbers.
    uint8_t context_id = tracy::GetGpuCtxCounter().fetch_add( 1, std::memory_order_relaxed );
    if( context_id >= 255 )
    {
        context_id %= 255;
    }

    uint8_t context_flags = 0;
#ifdef TRACY_ROCPROF_CALIBRATION
    // Tell tracy we'll be passing calibrated timestamps and not to mess with
    // the times. We'll periodically send GpuCalibration events in case the
    // times drift.
    context_flags |= tracy::GpuContextCalibration;
#endif
    {
        auto* item = tracy::Profiler::QueueSerial();
        tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuNewContext );
        tracy::MemWrite( &item->gpuNewContext.cpuTime, cpu_timestamp );
        tracy::MemWrite( &item->gpuNewContext.gpuTime, gpu_timestamp );
        memset( &item->gpuNewContext.thread, 0, sizeof( item->gpuNewContext.thread ) );
        tracy::MemWrite( &item->gpuNewContext.period, timestamp_period );
        tracy::MemWrite( &item->gpuNewContext.context, context_id );
        tracy::MemWrite( &item->gpuNewContext.flags, context_flags );
        tracy::MemWrite( &item->gpuNewContext.type, tracy::GpuContextType::Rocprof );
        tracy::Profiler::QueueSerialFinish();
    }

    // Send the name of the context along.
    // NOTE: Tracy will unconditionally free the name so we must clone it here.
    // Since internally Tracy will use its own rpmalloc implementation we must
    // make sure we allocate from the same source.
    size_t name_length = strlen( CTX_NAME );
    char* cloned_name = (char*)tracy::tracy_malloc( name_length );
    memcpy( cloned_name, CTX_NAME, name_length );
    {
        auto* item = tracy::Profiler::QueueSerial();
        tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuContextName );
        tracy::MemWrite( &item->gpuContextNameFat.context, context_id );
        tracy::MemWrite( &item->gpuContextNameFat.ptr, (uint64_t)cloned_name );
        tracy::MemWrite( &item->gpuContextNameFat.size, name_length );
        tracy::Profiler::QueueSerialFinish();
    }

    return context_id;
}

uint64_t kernel_src_loc( ToolData* data, uint64_t kernel_id )
{
    uint64_t src_loc = 0;
    auto _lk = std::unique_lock{ data->mut };
    rocprofiler_kernel_id_t kid = kernel_id;
    if( data->client_kernels.count( kid ) )
    {
        auto& sym_data = data->client_kernels[kid];
        const char* name = sym_data.kernel_name;
        size_t name_len = strlen( name );
        uint32_t line = 0;
        src_loc = tracy::Profiler::AllocSourceLocation( line, NULL, 0, name, name_len, NULL, 0 );
    }
    return src_loc;
}

void record_interval( ToolData* data, rocprofiler_timestamp_t start_timestamp, rocprofiler_timestamp_t end_timestamp,
                      uint64_t src_loc, rocprofiler_dispatch_id_t dispatch_id )
{

    uint16_t query_id = 0;
    uint8_t context_id = data->context_id;

    {
        auto _lk = std::unique_lock{ data->mut };
        query_id = data->query_id;
        data->query_id++;
        if( dispatch_id != UINT64_MAX )
        {
            DispatchData& dispatch_data = data->dispatch_data[dispatch_id];
            dispatch_data.query_id = query_id;
            dispatch_data.thread_id = tracy::GetThreadHandle();
        }
    }

    uint64_t cpu_start_time = 0, cpu_end_time = 0;
    if( dispatch_id == UINT64_MAX )
    {
        cpu_start_time = tracy::Profiler::GetTime();
        cpu_end_time = tracy::Profiler::GetTime();
    }
    else
    {
        auto _lk = std::unique_lock{ data->mut };
        DispatchData& dispatch_data = data->dispatch_data[dispatch_id];
        cpu_start_time = dispatch_data.launch_start;
        cpu_end_time = dispatch_data.launch_end;
    }

    if( src_loc != 0 )
    {
        {
            auto* item = tracy::Profiler::QueueSerial();
            tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuZoneBeginAllocSrcLocSerial );
            tracy::MemWrite( &item->gpuZoneBegin.cpuTime, cpu_start_time );
            tracy::MemWrite( &item->gpuZoneBegin.srcloc, (uint64_t)src_loc );
            tracy::MemWrite( &item->gpuZoneBegin.thread, tracy::GetThreadHandle() );
            tracy::MemWrite( &item->gpuZoneBegin.queryId, query_id );
            tracy::MemWrite( &item->gpuZoneBegin.context, context_id );
            tracy::Profiler::QueueSerialFinish();
        }
    }
    else
    {
        static const ___tracy_source_location_data src_loc = { NULL, NULL, NULL, 0, 0 };
        {
            auto* item = tracy::Profiler::QueueSerial();
            tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuZoneBeginSerial );
            tracy::MemWrite( &item->gpuZoneBegin.cpuTime, cpu_start_time );
            tracy::MemWrite( &item->gpuZoneBegin.srcloc, (uint64_t)&src_loc );
            tracy::MemWrite( &item->gpuZoneBegin.thread, tracy::GetThreadHandle() );
            tracy::MemWrite( &item->gpuZoneBegin.queryId, query_id );
            tracy::MemWrite( &item->gpuZoneBegin.context, context_id );
            tracy::Profiler::QueueSerialFinish();
        }
    }

    {
        auto* item = tracy::Profiler::QueueSerial();
        tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuTime );
        tracy::MemWrite( &item->gpuTime.gpuTime, start_timestamp );
        tracy::MemWrite( &item->gpuTime.queryId, query_id );
        tracy::MemWrite( &item->gpuTime.context, context_id );
        tracy::Profiler::QueueSerialFinish();
    }

    {
        auto* item = tracy::Profiler::QueueSerial();
        tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuZoneEndSerial );
        tracy::MemWrite( &item->gpuZoneEnd.cpuTime, cpu_end_time );
        tracy::MemWrite( &item->gpuZoneEnd.thread, tracy::GetThreadHandle() );
        tracy::MemWrite( &item->gpuZoneEnd.queryId, query_id );
        tracy::MemWrite( &item->gpuZoneEnd.context, context_id );
        tracy::Profiler::QueueSerialFinish();
    }

    {
        auto* item = tracy::Profiler::QueueSerial();
        tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuTime );
        tracy::MemWrite( &item->gpuTime.gpuTime, end_timestamp );
        tracy::MemWrite( &item->gpuTime.queryId, query_id );
        tracy::MemWrite( &item->gpuTime.context, context_id );
        tracy::Profiler::QueueSerialFinish();
    }
}

void record_callback( rocprofiler_dispatch_counting_service_data_t dispatch_data,
                      rocprofiler_record_counter_t* record_data, size_t record_count,
                      rocprofiler_user_data_t /*user_data*/, void* callback_data )
{
    assert( callback_data != nullptr );
    ToolData* data = static_cast<ToolData*>( callback_data );
    if( !data->init ) return;

    std::unordered_map<rocprofiler_counter_instance_id_t, double> sums;
    for( size_t i = 0; i < record_count; ++i )
    {
        auto _counter_id = rocprofiler_counter_id_t{};
        ROCPROFILER_CALL( rocprofiler_query_record_counter_id( record_data[i].id, &_counter_id ),
                          "query record counter id" );
        sums[_counter_id.handle] += record_data[i].counter_value;
    }

    uint16_t query_id = 0;
    uint32_t thread_id = 0;
    {
        auto _lk = std::unique_lock{ data->mut };
        // An assumption is made here that the counter values are supplied after the dispatch
        // complete callback.
        assert( data->dispatch_data.count( dispatch_data.dispatch_info.dispatch_id ) );
        DispatchData& ddata = data->dispatch_data[dispatch_data.dispatch_info.dispatch_id];
        query_id = ddata.query_id;
        thread_id = ddata.thread_id;
    }

    for( auto& p : sums )
    {
        auto* item = tracy::Profiler::QueueSerial();
        tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuZoneAnnotation );
        tracy::MemWrite( &item->zoneAnnotation.noteId, p.first );
        tracy::MemWrite( &item->zoneAnnotation.queryId, query_id );
        tracy::MemWrite( &item->zoneAnnotation.thread, thread_id );
        tracy::MemWrite( &item->zoneAnnotation.value, p.second );
        tracy::MemWrite( &item->zoneAnnotation.context, data->context_id );
        tracy::Profiler::QueueSerialFinish();
    }
}

/**
 * Callback from rocprofiler when an kernel dispatch is enqueued into the HSA queue.
 * rocprofiler_counter_config_id_t* is a return to specify what counters to collect
 * for this dispatch (dispatch_packet).
 */
void dispatch_callback( rocprofiler_dispatch_counting_service_data_t dispatch_data,
                        rocprofiler_profile_config_id_t* config, rocprofiler_user_data_t* /*user_data*/,
                        void* callback_data )
{
    assert( callback_data != nullptr );
    ToolData* data = static_cast<ToolData*>( callback_data );
    if( !data->init ) return;

    /**
     * This simple example uses the same profile counter set for all agents.
     * We store this in a cache to prevent constructing many identical profile counter
     * sets. We first check the cache to see if we have already constructed a counter"
     * set for the agent. If we have, return it. Otherwise, construct a new profile counter
     * set.
     */
    static std::shared_mutex m_mutex = {};
    static std::unordered_map<uint64_t, rocprofiler_profile_config_id_t> profile_cache = {};

    auto search_cache = [&]()
    {
        if( auto pos = profile_cache.find( dispatch_data.dispatch_info.agent_id.handle ); pos != profile_cache.end() )
        {
            *config = pos->second;
            return true;
        }
        return false;
    };

    {
        auto rlock = std::shared_lock{ m_mutex };
        if( search_cache() ) return;
    }

    auto wlock = std::unique_lock{ m_mutex };
    if( search_cache() ) return;

    // GPU Counter IDs
    std::vector<rocprofiler_counter_id_t> gpu_counters;

    // Iterate through the agents and get the counters available on that agent
    ROCPROFILER_CALL(
        rocprofiler_iterate_agent_supported_counters(
            dispatch_data.dispatch_info.agent_id,
            []( rocprofiler_agent_id_t, rocprofiler_counter_id_t* counters, size_t num_counters, void* user_data )
            {
                std::vector<rocprofiler_counter_id_t>* vec =
                    static_cast<std::vector<rocprofiler_counter_id_t>*>( user_data );
                for( size_t i = 0; i < num_counters; i++ )
                {
                    vec->push_back( counters[i] );
                }
                return ROCPROFILER_STATUS_SUCCESS;
            },
            static_cast<void*>( &gpu_counters ) ),
        "Could not fetch supported counters" );

    std::vector<rocprofiler_counter_id_t> collect_counters;
    collect_counters.reserve( data->counter_names.size() );
    // Look for the counters contained in counters_to_collect in gpu_counters
    for( auto& counter : gpu_counters )
    {
        rocprofiler_counter_info_v0_t info;
        ROCPROFILER_CALL(
            rocprofiler_query_counter_info( counter, ROCPROFILER_COUNTER_INFO_VERSION_0, static_cast<void*>( &info ) ),
            "Could not query info" );
        if( data->counter_names.count( std::string( info.name ) ) > 0 )
        {
            collect_counters.push_back( counter );

            size_t name_length = strlen( info.name );
            char* cloned_name = (char*)tracy::tracy_malloc( name_length );
            memcpy( cloned_name, info.name, name_length );
            {
                auto* item = tracy::Profiler::QueueSerial();
                tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuAnnotationName );
                tracy::MemWrite( &item->gpuAnnotationNameFat.context, data->context_id );
                tracy::MemWrite( &item->gpuAnnotationNameFat.noteId, counter.handle );
                tracy::MemWrite( &item->gpuAnnotationNameFat.ptr, (uint64_t)cloned_name );
                tracy::MemWrite( &item->gpuAnnotationNameFat.size, name_length );
                tracy::Profiler::QueueSerialFinish();
            }
        }
    }

    // Create a colleciton profile for the counters
    rocprofiler_profile_config_id_t profile = { .handle = 0 };
    ROCPROFILER_CALL( rocprofiler_create_profile_config( dispatch_data.dispatch_info.agent_id, collect_counters.data(),
                                                         collect_counters.size(), &profile ),
                      "Could not construct profile cfg" );

    profile_cache.emplace( dispatch_data.dispatch_info.agent_id.handle, profile );
    // Return the profile to collect those counters for this dispatch
    *config = profile;
}

void tool_callback_tracing_callback( rocprofiler_callback_tracing_record_t record, rocprofiler_user_data_t* user_data,
                                     void* callback_data )
{
    assert( callback_data != nullptr );
    ToolData* data = static_cast<ToolData*>( callback_data );
    if( !data->init ) return;

    if( record.kind == ROCPROFILER_CALLBACK_TRACING_CODE_OBJECT &&
        record.operation == ROCPROFILER_CODE_OBJECT_DEVICE_KERNEL_SYMBOL_REGISTER )
    {
        auto* sym_data = static_cast<kernel_symbol_data_t*>( record.payload );

        if( record.phase == ROCPROFILER_CALLBACK_PHASE_LOAD )
        {
            auto _lk = std::unique_lock{ data->mut };
            data->client_kernels.emplace( sym_data->kernel_id, *sym_data );
        }
        else if( record.phase == ROCPROFILER_CALLBACK_PHASE_UNLOAD )
        {
            auto _lk = std::unique_lock{ data->mut };
            data->client_kernels.erase( sym_data->kernel_id );
        }
    }
    else if( record.kind == ROCPROFILER_CALLBACK_TRACING_KERNEL_DISPATCH )
    {
        auto* rdata = static_cast<rocprofiler_callback_tracing_kernel_dispatch_data_t*>( record.payload );
        if( record.operation == ROCPROFILER_KERNEL_DISPATCH_ENQUEUE )
        {
            if( record.phase == ROCPROFILER_CALLBACK_PHASE_ENTER )
            {
                auto _lk = std::unique_lock{ data->mut };
                data->dispatch_data[rdata->dispatch_info.dispatch_id].launch_start = tracy::Profiler::GetTime();
            }
            else if( record.phase == ROCPROFILER_CALLBACK_PHASE_EXIT )
            {
                auto _lk = std::unique_lock{ data->mut };
                data->dispatch_data[rdata->dispatch_info.dispatch_id].launch_end = tracy::Profiler::GetTime();
            }
        }
        else if( record.operation == ROCPROFILER_KERNEL_DISPATCH_COMPLETE )
        {
            uint64_t src_loc = kernel_src_loc( data, rdata->dispatch_info.kernel_id );
            record_interval( data, rdata->start_timestamp, rdata->end_timestamp, src_loc,
                             rdata->dispatch_info.dispatch_id );
        }
    }
    else if( record.kind == ROCPROFILER_CALLBACK_TRACING_MEMORY_COPY &&
             record.operation != ROCPROFILER_MEMORY_COPY_NONE && record.phase == ROCPROFILER_CALLBACK_PHASE_EXIT )
    {
        auto* rdata = static_cast<rocprofiler_callback_tracing_memory_copy_data_t*>( record.payload );
        const char* name = nullptr;
        switch( record.operation )
        {
        case ROCPROFILER_MEMORY_COPY_DEVICE_TO_DEVICE:
            name = "DeviceToDeviceCopy";
            break;
        case ROCPROFILER_MEMORY_COPY_DEVICE_TO_HOST:
            name = "DeviceToHostCopy";
            break;
        case ROCPROFILER_MEMORY_COPY_HOST_TO_DEVICE:
            name = "HostToDeviceCopy";
            break;
        case ROCPROFILER_MEMORY_COPY_HOST_TO_HOST:
            name = "HostToHostCopy";
            break;
        }
        size_t name_len = strlen( name );
        uint64_t src_loc = tracy::Profiler::AllocSourceLocation( 0, NULL, 0, name, name_len, NULL, 0 );
        record_interval( data, rdata->start_timestamp, rdata->end_timestamp, src_loc, UINT64_MAX );
    }
}

void calibration_thread( void* ptr )
{
    while( !TracyIsStarted )
        ;
    ToolData* data = static_cast<ToolData*>( ptr );
    data->context_id = gpu_context_allocate( data );
    const char* user_counters = GetEnvVar( "TRACY_ROCPROF_COUNTERS" );
    if( user_counters )
    {
        data->counter_names.clear();
        std::stringstream ss( user_counters );
        std::string counter;
        while( std::getline( ss, counter, ',' ) ) data->counter_names.insert( counter );
    }
    data->init = true;

#ifdef TRACY_ROCPROF_CALIBRATION
    while( data->init )
    {
        sleep( 1 );

        timespec ts;
        // HSA performs a linear interpolation of GPU time to CLOCK_BOOTTIME. However, this is
        // subject to network time updates and can drift relative to tracy's clock.
        clock_gettime( CLOCK_BOOTTIME, &ts );
        int64_t cpu_timestamp = Profiler::GetTime();
        int64_t gpu_timestamp = ts.tv_nsec + ts.tv_sec * 1e9L;

        if( cpu_timestamp > data->previous_cpu_time )
        {
            auto* item = tracy::Profiler::QueueSerial();
            tracy::MemWrite( &item->hdr.type, tracy::QueueType::GpuCalibration );
            tracy::MemWrite( &item->gpuCalibration.gpuTime, gpu_timestamp );
            tracy::MemWrite( &item->gpuCalibration.cpuTime, cpu_timestamp );
            tracy::MemWrite( &item->gpuCalibration.cpuDelta, cpu_timestamp - data->previous_cpu_time );
            tracy::MemWrite( &item->gpuCalibration.context, data->context_id );
            tracy::Profiler::QueueSerialFinish();
            data->previous_cpu_time = cpu_timestamp;
        }
    }
#endif
}

int tool_init( rocprofiler_client_finalize_t fini_func, void* user_data )
{
    ToolData* data = static_cast<ToolData*>( user_data );
    data->cal_thread = std::make_unique<tracy::Thread>( calibration_thread, data );

    ROCPROFILER_CALL( rocprofiler_create_context( &get_client_ctx() ), "context creation failed" );

    ROCPROFILER_CALL( rocprofiler_configure_callback_dispatch_counting_service( get_client_ctx(), dispatch_callback,
                                                                                user_data, record_callback, user_data ),
                      "Could not setup counting service" );

    rocprofiler_tracing_operation_t ops[] = { ROCPROFILER_CODE_OBJECT_DEVICE_KERNEL_SYMBOL_REGISTER };
    ROCPROFILER_CALL( rocprofiler_configure_callback_tracing_service( get_client_ctx(),
                                                                      ROCPROFILER_CALLBACK_TRACING_CODE_OBJECT, ops, 1,
                                                                      tool_callback_tracing_callback, user_data ),
                      "callback tracing service failed to configure" );

    rocprofiler_tracing_operation_t ops2[] = { ROCPROFILER_KERNEL_DISPATCH_COMPLETE,
                                               ROCPROFILER_KERNEL_DISPATCH_ENQUEUE };
    ROCPROFILER_CALL(
        rocprofiler_configure_callback_tracing_service( get_client_ctx(), ROCPROFILER_CALLBACK_TRACING_KERNEL_DISPATCH,
                                                        ops2, 2, tool_callback_tracing_callback, user_data ),
        "callback tracing service failed to configure" );

    ROCPROFILER_CALL( rocprofiler_configure_callback_tracing_service( get_client_ctx(),
                                                                      ROCPROFILER_CALLBACK_TRACING_MEMORY_COPY, nullptr,
                                                                      0, tool_callback_tracing_callback, user_data ),
                      "callback tracing service failed to configure" );

    ROCPROFILER_CALL( rocprofiler_start_context( get_client_ctx() ), "start context" );
    return 0;
}

void tool_fini( void* tool_data_v )
{
    rocprofiler_stop_context( get_client_ctx() );

    ToolData* data = static_cast<ToolData*>( tool_data_v );
    data->init = false;
    data->cal_thread.reset();
}
}

extern "C"
{
    rocprofiler_tool_configure_result_t* rocprofiler_configure( uint32_t version, const char* runtime_version,
                                                                uint32_t priority, rocprofiler_client_id_t* client_id )
    {
        // If not the first tool to register, indicate that the tool doesn't want to do anything
        if( priority > 0 ) return nullptr;

        // (optional) Provide a name for this tool to rocprofiler
        client_id->name = "Tracy";

        // (optional) create configure data
        static ToolData data = ToolData{ version, runtime_version, priority, *client_id, 0, false, 0, 0 };

        // construct configure result
        static auto cfg = rocprofiler_tool_configure_result_t{ sizeof( rocprofiler_tool_configure_result_t ),
                                                               &tool_init, &tool_fini, static_cast<void*>( &data ) };

        return &cfg;
    }
}
