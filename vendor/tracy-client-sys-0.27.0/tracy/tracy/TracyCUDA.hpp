#ifndef __TRACYCUDA_HPP__
#define __TRACYCUDA_HPP__

#ifndef TRACY_ENABLE

#define TracyCUDAContext() nullptr
#define TracyCUDAContextDestroy(ctx)
#define TracyCUDAContextName(ctx, name, size)

#define TracyCUDAStartProfiling(ctx)
#define TracyCUDAStopProfiling(ctx)

#define TracyCUDACollect(ctx)

#else
#include <cupti.h>

#include <cassert>
#include <cmath>
#include <string>
#include <string_view>
#include <atomic>
#include <mutex>
#include <shared_mutex>
#include <condition_variable>
#include <vector>
#include <unordered_set>
#include <unordered_map>

#ifndef _MSC_VER
#include <cxxabi.h>
#endif

#include <tracy/Tracy.hpp>

#ifndef UNREFERENCED
#define UNREFERENCED(x) (void)x
#endif//UNREFERENCED

#ifndef TRACY_CUDA_CALIBRATED_CONTEXT
#define TRACY_CUDA_CALIBRATED_CONTEXT (1)
#endif//TRACY_CUDA_CALIBRATED_CONTEXT

#ifndef TRACY_CUDA_ENABLE_COLLECTOR_THREAD
#define TRACY_CUDA_ENABLE_COLLECTOR_THREAD (1)
#endif//TRACY_CUDA_ENABLE_COLLECTOR_THREAD

#ifndef TRACY_CUDA_ENABLE_CUDA_CALL_STATS
#define TRACY_CUDA_ENABLE_CUDA_CALL_STATS (0)
#endif//TRACY_CUDA_ENABLE_CUDA_CALL_STATS

namespace {

// TODO(marcos): wrap these in structs for better type safety
using CUptiTimestamp = uint64_t;
using TracyTimestamp =  int64_t;

struct IncrementalRegression {
    using float_t = double;
    struct Parameters {
        float_t slope, intercept;
    };

    int n = 0;
    float_t x_mean = 0;
    float_t y_mean = 0;
    float_t x_svar = 0;
    float_t y_svar = 0;
    float_t xy_scov = 0;

    auto parameters() const {
        float_t slope = xy_scov / x_svar;
        float_t intercept = y_mean - slope * x_mean;
        return Parameters{ slope, intercept };
    }

    auto orthogonal() const {
        // NOTE(marcos): orthogonal regression is Deming regression with delta = 1
        float_t delta = float_t(1);   // delta = 1 -> orthogonal regression
        float_t k = y_svar - delta * x_svar;
        float_t slope = (k + sqrt(k * k + 4 * delta * xy_scov * xy_scov)) / (2 * xy_scov);
        float_t intercept = y_mean - slope * x_mean;
        return Parameters{ slope, intercept };
    }

    void addSample(float_t x, float_t y) {
        ++n;
        float_t x_mean_prev = x_mean;
        float_t y_mean_prev = y_mean;
        x_mean += (x - x_mean) / n;
        y_mean += (y - y_mean) / n;
        x_svar += (x - x_mean_prev) * (x - x_mean);
        y_svar += (y - y_mean_prev) * (y - y_mean);
        xy_scov += (x - x_mean_prev) * (y - y_mean);
    }
};

tracy_force_inline TracyTimestamp tracyGetTimestamp() {
    return tracy::Profiler::GetTime();
}

auto& getCachedRegressionParameters() {
    // WARN(marcos): in theory, these linear regression parameters would be loaded/stored atomically;
    // in practice, however, it should not matter so long as the loads/stores are not "sliced"
    static IncrementalRegression::Parameters cached;
    return cached;
}

TracyTimestamp tracyFromCUpti(CUptiTimestamp cuptiTime) {
    // NOTE(marcos): linear regression estimate
    // y_hat = slope * x + intercept | X: CUptiTimestamp, Y: TracyTimestamp
    auto [slope, intercept] = getCachedRegressionParameters();
    double y_hat = slope * cuptiTime + intercept;
    TracyTimestamp tracyTime = TracyTimestamp(y_hat);
    assert(tracyTime >= 0);
    return tracyTime;
}

template<typename T, typename U>
tracy_force_inline void tracyMemWrite(T& where,U what) {
    static_assert(std::is_same_v<T, U>, "tracy::MemWrite: type mismatch.");
    tracy::MemWrite(&where, what);
}

void* tracyMalloc(size_t bytes) {
    return tracy::tracy_malloc(bytes);
}

void tracyFree(void* ptr) {
    tracy::tracy_free(ptr);
}

void tracyZoneBegin(TracyTimestamp time, tracy::SourceLocationData* srcLoc) {
    using namespace tracy;
    TracyQueuePrepare(QueueType::ZoneBegin);
    tracyMemWrite(item->zoneBegin.time, time);
    tracyMemWrite(item->zoneBegin.srcloc, (uint64_t)srcLoc);
    TracyQueueCommit(zoneBeginThread);
}

void tracyZoneEnd(TracyTimestamp time) {
    using namespace tracy;
    TracyQueuePrepare(QueueType::ZoneEnd);
    tracyMemWrite(item->zoneEnd.time, time);
    TracyQueueCommit(zoneEndThread);
}

void tracyPlot(const char* name, float value, TracyTimestamp time) {
    using namespace tracy;
    TracyLfqPrepare(QueueType::PlotDataFloat);
    tracyMemWrite(item->plotDataFloat.name, (uint64_t)name);
    tracyMemWrite(item->plotDataFloat.time, time);
    tracyMemWrite(item->plotDataFloat.val, value);
    TracyLfqCommit;
}

void tracyPlot(const char* name, float value, CUptiTimestamp time) {
    tracyPlot(name, value, tracyFromCUpti(time));
}

void tracyPlotActivity(const char* name, TracyTimestamp start, TracyTimestamp end, float value = 1.0f, float baseline = 0.0f) {
    tracyPlot(name, baseline, start);
    tracyPlot(name, value, start + 3);
    tracyPlot(name, value, end - 3);
    tracyPlot(name, baseline, end);
}

void tracyPlotActivity(const char* name, CUptiTimestamp start, CUptiTimestamp end, float value = 1.0f, float baseline = 0.0f) {
    tracyPlotActivity(name, tracyFromCUpti(start), tracyFromCUpti(end), value, baseline);
}

void tracyPlotBlip(const char* name, TracyTimestamp time, float value = 1.0f, float baseline = 0.0f) {
    tracyPlot(name, baseline, time - 3);
    tracyPlot(name, value, time);
    tracyPlot(name, baseline, time + 3);
}

void tracyPlotBlip(const char* name, CUptiTimestamp time, float value = 1.0f, float baseline = 0.0f) {
    tracyPlotBlip(name, tracyFromCUpti(time), value, baseline);
}

void tracyEmitMemAlloc(const char* name, const void* ptr, size_t size, TracyTimestamp time) {
    using namespace tracy;
    const auto thread = GetThreadHandle();

    auto item = Profiler::QueueSerial();
    tracyMemWrite(item->hdr.type, QueueType::MemNamePayload);
    tracyMemWrite(item->memName.name, (uint64_t)name);
    Profiler::QueueSerialFinish();

    item = Profiler::QueueSerial();
    tracyMemWrite(item->hdr.type, QueueType::MemAllocNamed);
    tracyMemWrite(item->memAlloc.time, time);
    tracyMemWrite(item->memAlloc.thread, thread);
    tracyMemWrite(item->memAlloc.ptr, (uint64_t)ptr);

    if (compile_time_condition<sizeof(size) == 4>::value)
    {
        memcpy(&item->memAlloc.size, &size, 4);
        memset(&item->memAlloc.size + 4, 0, 2);
    }
    else
    {
        assert(sizeof(size) == 8);
        memcpy(&item->memAlloc.size, &size, 4);
        memcpy(((char *)&item->memAlloc.size) + 4, ((char *)&size) + 4, 2);
    }
    Profiler::QueueSerialFinish();
}

void tracyEmitMemFree(const char* name, const void* ptr, TracyTimestamp time) {
    using namespace tracy;
    const auto thread = GetThreadHandle();

    auto item = Profiler::QueueSerial();
    tracyMemWrite(item->hdr.type, QueueType::MemNamePayload);
    tracyMemWrite(item->memName.name, (uint64_t)name);
    Profiler::QueueSerialFinish();

    item = Profiler::QueueSerial();
    tracyMemWrite(item->hdr.type, QueueType::MemFreeNamed);
    tracyMemWrite(item->memFree.time, time);
    tracyMemWrite(item->memFree.thread, thread);
    tracyMemWrite(item->memFree.ptr, (uint64_t)ptr);
    Profiler::QueueSerialFinish();
}

void tracyEmitMemAlloc(const char* name, const void* ptr, size_t size, CUptiTimestamp cuptiTime) {
    tracyEmitMemAlloc(name, ptr, size, tracyFromCUpti(cuptiTime));
}

void tracyEmitMemFree(const char* name, const void* ptr, CUptiTimestamp cuptiTime) {
    tracyEmitMemFree(name, ptr, tracyFromCUpti(cuptiTime));
}

void tracyAnnounceGpuTimestamp(TracyTimestamp apiStart, TracyTimestamp apiEnd,
    uint16_t queryId, uint8_t gpuContextId, 
    const tracy::SourceLocationData* sourceLocation, uint32_t threadId) {
    using namespace tracy;

    auto item = Profiler::QueueSerial();
    tracyMemWrite(item->hdr.type, QueueType::GpuZoneBeginSerial);
    tracyMemWrite(item->gpuZoneBegin.cpuTime, apiStart);
    tracyMemWrite(item->gpuZoneBegin.srcloc, (uint64_t)sourceLocation);
    tracyMemWrite(item->gpuZoneBegin.thread, threadId);
    tracyMemWrite(item->gpuZoneBegin.queryId, uint16_t(queryId+0));
    tracyMemWrite(item->gpuZoneBegin.context, gpuContextId);
    Profiler::QueueSerialFinish();

    item = Profiler::QueueSerial();
    tracyMemWrite(item->hdr.type, QueueType::GpuZoneEndSerial);
    tracyMemWrite(item->gpuZoneEnd.cpuTime, apiEnd);
    tracyMemWrite(item->gpuZoneEnd.thread, threadId);
    tracyMemWrite(item->gpuZoneEnd.queryId, uint16_t(queryId+1));
    tracyMemWrite(item->gpuZoneEnd.context, gpuContextId);
    Profiler::QueueSerialFinish();
}

void tracySubmitGpuTimestamp(CUptiTimestamp gpuStart, CUptiTimestamp gpuEnd,
    uint16_t queryId, uint8_t gpuContextId) {
    using namespace tracy;

    auto item = Profiler::QueueSerial();
    tracyMemWrite(item->hdr.type, QueueType::GpuTime);
    tracyMemWrite(item->gpuTime.gpuTime, (int64_t)gpuStart);
    tracyMemWrite(item->gpuTime.queryId, uint16_t(queryId+0));
    tracyMemWrite(item->gpuTime.context, gpuContextId);
    Profiler::QueueSerialFinish();

    item = Profiler::QueueSerial();
    tracyMemWrite(item->hdr.type, QueueType::GpuTime);
    tracyMemWrite(item->gpuTime.gpuTime, (int64_t)gpuEnd);
    tracyMemWrite(item->gpuTime.queryId, uint16_t(queryId+1));
    tracyMemWrite(item->gpuTime.context, gpuContextId);
    Profiler::QueueSerialFinish();
}

#define CUPTI_API_CALL(call) CUptiCallChecked(call, #call, __FILE__, __LINE__)

#define DRIVER_API_CALL(call) cudaDriverCallChecked(call, #call, __FILE__, __LINE__)

CUptiResult CUptiCallChecked(CUptiResult result, const char* call, const char* file, int line) noexcept {
    if (result == CUPTI_SUCCESS)
        return result;
    const char* resultMsg = "";
    CUPTI_API_CALL(cuptiGetResultString(result, &resultMsg));   // maybe not a good idea to recurse here...
    fprintf(stderr, "ERROR:\t%s:%d:\n\tfunction '%s' failed with error '%s'.\n", file, line, call, resultMsg);
    //assert(result == CUPTI_SUCCESS);
    return result;
}

CUresult cudaDriverCallChecked(CUresult result, const char* call, const char* file, int line) noexcept {
    if (result == CUDA_SUCCESS)
        return result;
    const char* resultMsg = "";
    DRIVER_API_CALL(cuGetErrorString(result, &resultMsg));   // maybe not a good idea to recurse here...
    fprintf(stderr, "ERROR:\t%s:%d:\n\tfunction '%s' failed with error '%s'.\n", file, line, call, resultMsg);
    //assert(result == CUDA_SUCCESS);
    return result;
}

template<typename TKey, typename TValue>
struct ConcurrentHashMap {
    static constexpr bool instrument = false;
    auto acquire_read_lock() {
        if (m.try_lock_shared())
            return std::shared_lock<std::shared_mutex>(m, std::adopt_lock);
        ZoneNamedC(rwlock, tracy::Color::Tomato, instrument);
        return std::shared_lock<std::shared_mutex>(m);
    }
    auto acquire_write_lock() {
        if (m.try_lock())
            return std::unique_lock<std::shared_mutex>(m, std::adopt_lock);
        ZoneNamedC(wxlock, tracy::Color::Tomato, instrument);
        return std::unique_lock<std::shared_mutex>(m);
    }
    std::unordered_map<TKey, TValue> mapping;
    std::shared_mutex m;
    auto& operator[](TKey key) {
        {
            auto lock = acquire_read_lock();
            auto it = mapping.find(key);
            if (it != mapping.end()) {
                return it->second;
            }
        }
        return emplace(key, TValue{}).first->second;
    }
    auto find(TKey key) {
        ZoneNamed(find, instrument);
        auto lock = acquire_read_lock();
        return mapping.find(key);
    }
    auto fetch(TKey key, TValue& value) {
        ZoneNamed(fetch, instrument);
        auto it = mapping.find(key);
        if (it != mapping.end()) {
            value = it->second;
            return true;
        }
        return false;
    }
    auto end() {
        ZoneNamed(end, instrument);
        auto lock = acquire_read_lock();
        return mapping.end();
    }
    template<typename... Args>
    auto emplace(TKey key, Args&&... args) {
        ZoneNamed(emplace, instrument);
        auto lock = acquire_write_lock();
        return mapping.emplace(std::forward<TKey>(key), std::forward<Args>(args)...);
    }
    auto erase(TKey key) {
        ZoneNamed(erase, instrument);
        auto lock = acquire_write_lock();
        return mapping.erase(key);
    }
};

#if TRACY_CUDA_ENABLE_CUDA_CALL_STATS
struct ProfilerStats {
    static constexpr bool instrument = false;

    ConcurrentHashMap<uint32_t, std::atomic<int>> apiCallCount;

    void update(CUpti_CallbackDomain domain, CUpti_CallbackId cbid) {
        ZoneNamed(update, instrument);
        uint32_t key = (domain << 24) | (cbid & 0x00'FFFFFF);
        auto it = apiCallCount.find(key);
        if (it == apiCallCount.end()) {
            it = apiCallCount.emplace(key, 0).first;
        }
        it->second.fetch_add(1, std::memory_order::memory_order_relaxed);
    }
};
#endif

// StringTable: string memoization/interning
struct StringTable {
    static constexpr bool instrument = false;

    // TODO(marcos): this could be just a "ConcurrentHashSet"
    ConcurrentHashMap<std::string_view, std::string_view> table;

    ~StringTable() { /* TODO(marcos): free string copy */ }

    std::string_view operator[](std::string_view str) {
        ZoneNamedN(lookup, "StringTable::lookup", instrument);
        std::string_view memoized;
        if (!table.fetch(str, memoized)) {
            ZoneNamedN(lookup, "StringTable::insert", instrument);
            char* copy = (char*)tracyMalloc(str.size() + 1);
            strncpy(copy, str.data(), str.size());
            copy[str.size()] = '\0';
            std::string_view value (copy, str.size());
            auto [it, inserted] = table.emplace(value, value);
            if (!inserted) {
                // another thread inserted it while we were trying to: cleanup
                tracyFree(copy);
            }
            memoized = it->second;
        }
        assert(str == memoized);
        return memoized;
    }
};

struct SourceLocationMap {
    static constexpr bool instrument = false;

    // NOTE(marcos): the address of an unordered_map value may become invalid
    // later on (e.g., during a rehash), so mapping to a pointer is necessary
    ConcurrentHashMap<std::string_view, tracy::SourceLocationData*> locations;

    ~SourceLocationMap() { /* TODO(marcos): free SourceLocationData* entries */ }

    tracy::SourceLocationData* retrieve(std::string_view function) {
        ZoneNamed(retrieve, instrument);
        tracy::SourceLocationData* pSrcLoc = nullptr;
        locations.fetch(function, pSrcLoc);
        return pSrcLoc;
    }

    tracy::SourceLocationData* add(std::string_view function, std::string_view file, int line, uint32_t color=0) {
        ZoneNamed(emplace, instrument);
        assert(*function.end() == '\0');
        assert(*file.end() == '\0');
        void* bytes = tracyMalloc(sizeof(tracy::SourceLocationData));
        auto pSrcLoc = new(bytes)tracy::SourceLocationData{ function.data(), TracyFunction, file.data(), (uint32_t)line, color };
        auto [it, inserted] = locations.emplace(function, pSrcLoc);
        if (!inserted) {
            // another thread inserted it while we were trying to: cleanup
            tracyFree(pSrcLoc); // POD: no destructor to call
        }
        assert(it->second != nullptr);
        return it->second;
    }
};

struct SourceLocationLUT {
    static constexpr bool instrument = false;

    ~SourceLocationLUT() { /* no action needed: no dynamic allocation */ }

    tracy::SourceLocationData runtime [CUpti_runtime_api_trace_cbid::CUPTI_RUNTIME_TRACE_CBID_SIZE] = {};
    tracy::SourceLocationData driver [CUpti_driver_api_trace_cbid::CUPTI_DRIVER_TRACE_CBID_SIZE] = {};

    tracy::SourceLocationData* retrieve(CUpti_CallbackDomain domain, CUpti_CallbackId cbid, CUpti_CallbackData* apiInfo) {
        ZoneNamed(retrieve, instrument);
        tracy::SourceLocationData* pSrcLoc = nullptr;
        switch (domain) {
        case CUPTI_CB_DOMAIN_RUNTIME_API :
            if ((cbid > 0) && (cbid < CUPTI_RUNTIME_TRACE_CBID_SIZE)) {
                pSrcLoc = &runtime[cbid];
            }
            break;
        case CUPTI_CB_DOMAIN_DRIVER_API :
            if ((cbid > 0) && (cbid < CUPTI_DRIVER_TRACE_CBID_SIZE)) {
                pSrcLoc = &driver[cbid];
            }
            break;
        default:
            break;
        }
        if (pSrcLoc->name == nullptr) {
            const char* function = apiInfo->functionName ? apiInfo->functionName : "cuda???";
            // cuptiGetCallbackName includes the "version suffix" of the function/cbid
            //CUPTI_API_CALL(cuptiGetCallbackName(domain, cbid, &function));
            *pSrcLoc = tracy::SourceLocationData{ function, TracyFunction, TracyFile, TracyLine, 0 };
        }
        return pSrcLoc;
    }
};

uint32_t tracyTimelineId(uint32_t contextId, uint32_t streamId) {
    // 0xA7C5 = 42,949 => 42,949 * 100,000 = 4,294,900,000
    // 4,294,900,000 + 65,535  = 4,294,965,535 < 4,294,967,295 (max uint32)
    assert(contextId <= 0xA7C5);
    assert((streamId == CUPTI_INVALID_STREAM_ID) || (streamId < 0xFFFF));
    uint32_t packed = (contextId * 100'000) + (streamId & 0x0000'FFFF);
    return packed;
}

} // unnamed/anonymous namespace

namespace tracy
{
    class CUDACtx
    {
    public:
        static CUDACtx* Create() {
            auto& s = Singleton::Get();
            std::unique_lock<std::mutex> lock (s.m);
            if (s.ref_count == 0) {
                assert(s.ctx == nullptr);
                s.ctx = new CUDACtx(s.ctx_id);
                s.ref_count += 1;
                s.ctx_id = s.ctx->m_tracyGpuContext;
            }
            return s.ctx;
        }

        static void Destroy(CUDACtx* ctx) {
            auto& s = Singleton::Get();
            std::unique_lock<std::mutex> lock(s.m);
            assert(ctx == s.ctx);
            s.ref_count -= 1;
            if (s.ref_count == 0) {
                delete s.ctx;
                s.ctx = nullptr;
            }
        }

        void Collect()
        {
            ZoneScoped;
            CUPTI::FlushActivity();
        }

        void printStats()
        {
            #if TRACY_CUDA_ENABLE_CUDA_CALL_STATS
            fprintf(stdout, "\nCUDA API stats:\n");
            {
                struct Stats { CUpti_CallbackDomain domain; CUpti_CallbackId cbid; int count; };
                std::vector<Stats> sorted;
                for (auto&& api : stats.apiCallCount.mapping) {
                    auto domain = CUpti_CallbackDomain(api.first >> 24);
                    auto cbid = CUpti_CallbackId(api.first & 0x00'FFFFFF);
                    int count = api.second;
                    sorted.emplace_back(Stats{ domain, cbid, count });
                }
                std::sort(sorted.begin(), sorted.end(), [](const Stats& x, const Stats& y) { return x.count > y.count; });
                for (auto&& api : sorted) {
                    const char* function = "";
                    CUPTI_API_CALL(cuptiGetCallbackName(api.domain, api.cbid, &function));
                    printf("- %s : %d\n", function, api.count);
                }
            }
            #endif
        }

        void StartProfiling()
        {
            ZoneScoped;
            CUPTI::BeginInstrumentation(this);
        }

        void StopProfiling()
        {
            ZoneScoped;
            CUPTI::EndInstrumentation();
            printStats();
        }

        void Name(const char *name, uint16_t len)
        {
            auto ptr = (char*)tracyMalloc(len);
            memcpy(ptr, name, len);

            auto item = Profiler::QueueSerial();
            tracyMemWrite(item->hdr.type, QueueType::GpuContextName);
            tracyMemWrite(item->gpuContextNameFat.context, m_tracyGpuContext);
            tracyMemWrite(item->gpuContextNameFat.ptr, (uint64_t)ptr);
            tracyMemWrite(item->gpuContextNameFat.size, len);
            SubmitQueueItem(item);
        }

        tracy_force_inline void SubmitQueueItem(tracy::QueueItem *item)
        {
#ifdef TRACY_ON_DEMAND
            GetProfiler().DeferItem(*item);
#endif
            Profiler::QueueSerialFinish();
        }

        static void QueryTimestamps(TracyTimestamp& tTracy, CUptiTimestamp& tCUpti) {
            TracyTimestamp tTracy1 = tracyGetTimestamp();
            CUPTI_API_CALL(cuptiGetTimestamp(&tCUpti));
            TracyTimestamp tTracy2 = tracyGetTimestamp();
            // NOTE(marcos): giving more weight to 'tTracy2'
            tTracy = (3*tTracy1 + 5*tTracy2) / 8;
        }

        // NOTE(marcos): recalibration is 'static' since Tracy and CUPTI timestamps
        // are "global" across all contexts; that said, each Tracy GPU context needs
        // its own GpuCalibration message, but for now there's just a singleton context.
        void Recalibrate() {
            ZoneScoped;
            // NOTE(marcos): only one thread should do the calibration, but there's
            // no good reason to block threads that also trying to do the same
            static std::mutex m;
            if (!m.try_lock())
                return;
            std::unique_lock<std::mutex> lock (m, std::adopt_lock);
            ZoneNamedNC(zone, "tracy::CUDACtx::Recalibrate[effective]", tracy::Color::Goldenrod, true);
            TracyTimestamp tTracy;
            CUptiTimestamp tCUpti;
            QueryTimestamps(tTracy, tCUpti);
            #if TRACY_CUDA_CALIBRATED_CONTEXT
            static CUptiTimestamp prevCUptiTime = tCUpti;
            int64_t deltaTicksCUpti = tCUpti - prevCUptiTime;
            if (deltaTicksCUpti > 0) {
                prevCUptiTime = tCUpti;
                auto* item = Profiler::QueueSerial();
                tracyMemWrite(item->hdr.type, QueueType::GpuCalibration);
                tracyMemWrite(item->gpuCalibration.gpuTime, (int64_t)tCUpti);
                tracyMemWrite(item->gpuCalibration.cpuTime, tTracy);
                tracyMemWrite(item->gpuCalibration.cpuDelta, deltaTicksCUpti);
                tracyMemWrite(item->gpuCalibration.context, m_tracyGpuContext);
                Profiler::QueueSerialFinish();
            }
            #endif
            // NOTE(marcos): update linear regression incrementally, which will refine
            // the estimation of Tracy timestamps (Y) from CUpti timestamps (X)
            static IncrementalRegression model;
            model.addSample(double(tCUpti), double(tTracy));
            // NOTE(marcos): using orthogonal regression because the independet variable
            // (X: CUpti timestamps) measurements are also imprecise
            getCachedRegressionParameters() = model.orthogonal();
        }

    protected:
        void EmitGpuZone(TracyTimestamp apiStart, TracyTimestamp apiEnd,
            CUptiTimestamp gpuStart, CUptiTimestamp gpuEnd,
            const tracy::SourceLocationData* pSrcLoc,
            uint32_t cudaContextId, uint32_t cudaStreamId) {
            //uint32_t timelineId = tracy::GetThreadHandle();
            uint32_t timelineId = tracyTimelineId(cudaContextId, cudaStreamId);
            uint16_t queryId = m_queryIdGen.fetch_add(2);
            tracyAnnounceGpuTimestamp(apiStart, apiEnd, queryId, m_tracyGpuContext, pSrcLoc, timelineId);
            tracySubmitGpuTimestamp(gpuStart, gpuEnd, queryId, m_tracyGpuContext);
        }

        void OnEventsProcessed() {
            Recalibrate();
        }

        struct CUPTI {
        static void CUPTIAPI OnBufferRequested(uint8_t **buffer, size_t *size, size_t *maxNumRecords)
        {
            ZoneScoped;
            // TODO(marcos): avoid malloc and instead suballocate from a large circular buffer;
            // according to the CUPTI documentation: "To minimize profiling overhead the client
            // should return as quickly as possible from these callbacks."
            *size = 1 * 1024*1024; // 1MB
            *buffer = (uint8_t*)tracyMalloc(*size);
            assert(*buffer != nullptr);
            FlushActivityAsync();
        }

        static void CUPTIAPI OnBufferCompleted(CUcontext ctx, uint32_t streamId, uint8_t* buffer, size_t size, size_t validSize)
        {
            // CUDA 6.0 onwards: all buffers from this callback are "global" buffers
            // (i.e. there is no context/stream specific buffer; ctx is always NULL)
            ZoneScoped;
            tracy::SetThreadName("NVIDIA CUPTI Worker");
            CUptiResult status;
            CUpti_Activity* record = nullptr;
            while ((status = cuptiActivityGetNextRecord(buffer, validSize, &record)) == CUPTI_SUCCESS) {
                DoProcessDeviceEvent(record);
            }
            if (status != CUPTI_ERROR_MAX_LIMIT_REACHED) {
                CUptiCallChecked(status, "cuptiActivityGetNextRecord", TracyFile, TracyLine);
            }
            size_t dropped = 0;
            CUPTI_API_CALL(cuptiActivityGetNumDroppedRecords(ctx, streamId, &dropped));
            assert(dropped == 0);
            tracyFree(buffer);
            PersistentState::Get().profilerHost->OnEventsProcessed();
        }

        // correlationID -> [CPU start time, CPU end time, CUPTI start time]
        using CorrelationID = uint32_t;
        struct APICallInfo { TracyTimestamp start = 0, end = 0; CUptiTimestamp cupti = CUPTI_TIMESTAMP_UNKNOWN; CUDACtx* host = nullptr; };

        static void CUPTIAPI OnCallbackAPI(
            void* userdata,
            CUpti_CallbackDomain domain,
            CUpti_CallbackId cbid,
            const void* cbdata)
        {
            static constexpr bool instrument = false;

            TracyTimestamp apiCallStartTime = tracyGetTimestamp();
            CUDACtx* profilerHost = (CUDACtx*)userdata;

            switch (domain) {
            case CUPTI_CB_DOMAIN_RUNTIME_API:
            case CUPTI_CB_DOMAIN_DRIVER_API:
                break;
            case CUPTI_CB_DOMAIN_RESOURCE: {
                // match 'callbackId' with CUpti_CallbackIdResource
                // interpret 'cbdata' as CUpti_ResourceData,
                //                 or as CUpti_ModuleResourceData,
                //                 or as CUpti_GraphData,
                //                 or as CUpti_StreamAttrData,
                //                 or as ... (what else?)
                return;
            }
            case CUPTI_CB_DOMAIN_SYNCHRONIZE: {
                // match 'callbackId' with CUpti_CallbackIdSync
                // interpret 'cbdata' as CUpti_SynchronizeData
                return;
            }
            case CUPTI_CB_DOMAIN_STATE: {
                // match 'callbackId' with CUpti_CallbackIdState
                // interpret 'cbdata' as CUpti_StateData
                return;
            }
            case CUPTI_CB_DOMAIN_NVTX: {
                // match 'callbackId' with CUpti_nvtx_api_trace_cbid
                // interpret 'cbdata' as CUpti_NvtxData
                return;
            }
            case CUPTI_CB_DOMAIN_FORCE_INT:
                // NOTE(marcos): the "FORCE_INT" values in CUPTI enums exist only to
                // force the enum to have a specific representation (signed 32bits)
            case CUPTI_CB_DOMAIN_INVALID:
            default:
                // TODO(marcos): unexpected error!
                return;
            }
    
            // if we reached this point, then we are in the (runtime or driver) API domain
            CUpti_CallbackData* apiInfo = (CUpti_CallbackData*)cbdata;

            // Emit the Tracy 'ZoneBegin' message upon entering the API call
            // TODO(marcos): a RAII object could be useful here...
            if (apiInfo->callbackSite == CUPTI_API_ENTER) {
                #if TRACY_CUDA_ENABLE_CUDA_CALL_STATS
                ctx->stats.update(domain, cbid);
                #endif

                auto& cudaCallSourceLocation = PersistentState::Get().cudaCallSourceLocation;
                auto pSrcLoc = cudaCallSourceLocation.retrieve(domain, cbid, apiInfo);

                // HACK(marcos): the SourceLocationLUT::retrieve zone (above) should
                // not be emitted before its enclosing zone (below) actually begins,
                // so we delay the beginning of the enclosing zone to "unstack" them
                if (SourceLocationLUT::instrument)
                    apiCallStartTime = tracyGetTimestamp();
                tracyZoneBegin(apiCallStartTime, pSrcLoc);
            }

            if (apiInfo->callbackSite == CUPTI_API_ENTER) {
                ZoneNamedN(enter, "tracy::CUDACtx::OnCUptiCallback[enter]", instrument);
                // Track API calls that generate device activity:
                bool trackDeviceActivity = false;
                CUstream hStream = nullptr;
                if (domain == CUPTI_CB_DOMAIN_RUNTIME_API) {
                    #define GET_STREAM_FUNC(Params, field) [](CUpti_CallbackData* api) { return ((Params*)api->functionParams)->field; }
                    #define NON_STREAM_FUNC() [](CUpti_CallbackData*) { return cudaStream_t(nullptr); }
                    static std::unordered_map<CUpti_runtime_api_trace_cbid, cudaStream_t(*)(CUpti_CallbackData*)> cbidRuntimeTrackers = {
                        // Runtime: Kernel
                        { CUPTI_RUNTIME_TRACE_CBID_cudaLaunchKernel_v7000,          GET_STREAM_FUNC(cudaLaunchKernel_v7000_params, stream) },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaLaunchKernel_ptsz_v7000,     GET_STREAM_FUNC(cudaLaunchKernel_ptsz_v7000_params, stream) },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaLaunchKernelExC_v11060,      GET_STREAM_FUNC(cudaLaunchKernelExC_v11060_params, config->stream) },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaLaunchKernelExC_ptsz_v11060, GET_STREAM_FUNC(cudaLaunchKernelExC_ptsz_v11060_params, config->stream) },
                        // Runtime: Memory
                        { CUPTI_RUNTIME_TRACE_CBID_cudaMalloc_v3020, NON_STREAM_FUNC() },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaFree_v3020,   NON_STREAM_FUNC() },
                        // Runtime: Memcpy
                        { CUPTI_RUNTIME_TRACE_CBID_cudaMemcpy_v3020,      NON_STREAM_FUNC() },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaMemcpyAsync_v3020, GET_STREAM_FUNC(cudaMemcpyAsync_v3020_params, stream) },
                        // Runtime: Memset
                        { CUPTI_RUNTIME_TRACE_CBID_cudaMemset_v3020,      NON_STREAM_FUNC() },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaMemsetAsync_v3020, GET_STREAM_FUNC(cudaMemsetAsync_v3020_params, stream) },
                        // Runtime: Synchronization
                        { CUPTI_RUNTIME_TRACE_CBID_cudaStreamSynchronize_v3020, NON_STREAM_FUNC() },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaEventSynchronize_v3020,  NON_STREAM_FUNC() },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaEventQuery_v3020,        NON_STREAM_FUNC() },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaStreamWaitEvent_v3020,   NON_STREAM_FUNC() },
                        { CUPTI_RUNTIME_TRACE_CBID_cudaDeviceSynchronize_v3020, NON_STREAM_FUNC() },
                    };
                    #undef NON_STREAM_FUNC
                    #undef GET_STREAM_FUNC
                    auto it = cbidRuntimeTrackers.find(CUpti_runtime_api_trace_cbid(cbid));
                    if (it != cbidRuntimeTrackers.end()) {
                        trackDeviceActivity = true;
                        hStream = (CUstream)it->second(apiInfo);
                    }
                }
                if (domain == CUPTI_CB_DOMAIN_DRIVER_API) {
                    #define GET_STREAM_FUNC(Params, field) [](CUpti_CallbackData* api) { return ((Params*)api->functionParams)->field; }
                    #define NON_STREAM_FUNC() [](CUpti_CallbackData*) { return CUstream(nullptr); }
                    static std::unordered_map<CUpti_driver_api_trace_cbid, CUstream(*)(CUpti_CallbackData*)> cbidDriverTrackers = {
                        // Driver: Kernel
                        { CUPTI_DRIVER_TRACE_CBID_cuLaunchKernel,        GET_STREAM_FUNC(cuLaunchKernel_params, hStream) },
                        { CUPTI_DRIVER_TRACE_CBID_cuLaunchKernel_ptsz,   GET_STREAM_FUNC(cuLaunchKernel_ptsz_params, hStream)} ,
                        { CUPTI_DRIVER_TRACE_CBID_cuLaunchKernelEx,      GET_STREAM_FUNC(cuLaunchKernelEx_params, config->hStream) },
                        { CUPTI_DRIVER_TRACE_CBID_cuLaunchKernelEx_ptsz, GET_STREAM_FUNC(cuLaunchKernelEx_params, config->hStream) },
                    };
                    #undef NON_STREAM_FUNC
                    #undef GET_STREAM_FUNC
                    auto it = cbidDriverTrackers.find(CUpti_driver_api_trace_cbid(cbid));
                    if (it != cbidDriverTrackers.end()) {
                        trackDeviceActivity = true;
                        hStream = it->second(apiInfo);
                    }
                }
                if (trackDeviceActivity) {
                    // NOTE(marcos): we should NOT track if the stream is being captured
                    CUstreamCaptureStatus status = {};
                    DRIVER_API_CALL(cuStreamIsCapturing(hStream, &status));
                    trackDeviceActivity = !(status == CU_STREAM_CAPTURE_STATUS_ACTIVE);
                }
                if (trackDeviceActivity) {
                    CUptiTimestamp tgpu;
                    // TODO(marcos): do a "reverse-estimate" to obtain CUpti time from Tracy time instead?
                    CUPTI_API_CALL(cuptiGetTimestamp(&tgpu));
                    auto& cudaCallSiteInfo = PersistentState::Get().cudaCallSiteInfo;
                    cudaCallSiteInfo.emplace(apiInfo->correlationId, APICallInfo{ apiCallStartTime, apiCallStartTime, tgpu, profilerHost });
                }
                auto& entryFlags = *apiInfo->correlationData;
                assert(entryFlags == 0);
                entryFlags |= trackDeviceActivity ? 0x8000 : 0;
            }

            if (apiInfo->callbackSite == CUPTI_API_EXIT) {
                APICallInfo* pApiInterval = [](CUpti_CallbackData* apiInfo) {
                    ZoneNamedN(exit, "tracy::CUDACtx::OnCUptiCallback[exit]", instrument);
                    auto entryFlags = *apiInfo->correlationData;
                    bool trackDeviceActivity = (entryFlags & 0x8000) != 0;
                    if (trackDeviceActivity) {
                        auto& cudaCallSiteInfo = PersistentState::Get().cudaCallSiteInfo;
                        auto it = cudaCallSiteInfo.find(apiInfo->correlationId);
                        if (it != cudaCallSiteInfo.end()) {
                            // WARN(marcos): leaking the address of a hash-map value could spell trouble
                            return &it->second;
                        }
                    }
                    // NOTE(marcos): this can happen if the GPU activity completes
                    // before the CUDA function that enqueued it returns (e.g., sync)
                    static APICallInfo sentinel;
                    return &sentinel;
                }(apiInfo);
                pApiInterval->end = tracyGetTimestamp();
                tracyZoneEnd(pApiInterval->end);
            }
        }

        static bool matchActivityToAPICall(uint32_t correlationId, APICallInfo& apiCallInfo) {
            static constexpr bool instrument = false;
            ZoneNamed(match, instrument);
            auto& cudaCallSiteInfo = PersistentState::Get().cudaCallSiteInfo;
            if (!cudaCallSiteInfo.fetch(correlationId, apiCallInfo)) {
                return false;
            }
            cudaCallSiteInfo.erase(correlationId);
            assert(apiCallInfo.host != nullptr);
            return true;
        }

        static void matchError(uint32_t correlationId, const char* kind) {
            char msg [128];
            snprintf(msg, sizeof(msg), "ERROR: device activity '%s' has no matching CUDA API call (id=%u).", kind, correlationId);
            TracyMessageC(msg, strlen(msg), tracy::Color::Tomato);
        }

        static std::string extractActualName(char** name){
            //If name does not start with number, return empty string
            if (!isdigit(**name))
            {
                return std::string();
            }
            // Assuming name starts with number followed by actual name
            std::string actualName;
            char* currStr = *name;
            int num = 0;
            while (*currStr >= '0' && *currStr <= '9')
            {
                num = num * 10 + (*currStr - '0');
                currStr++;
            }

            // Return the string start at currStr ends at num
            actualName = std::string(currStr, num);
            // check if actualName starts with _GLOBAL__N__
            if (actualName.rfind("_GLOBAL__N__", 0) == 0)
            {
                // _GLOBAL__N__ with an id stands for anonymous namespace
                actualName = std::string("(anonymous_namespace)");
            }

            *name = currStr + num;
            return actualName;
        }

        static std::string extractActualNameNested(const char* demangledName)
        {
            ZoneNamedN(demangle, "demangle_kernel", false);
            //If name does not start with _Z, return a new std::string with original name
            if (demangledName[0] != '_' || demangledName[1] != 'Z')
            {
                return std::string(demangledName);
            }
            std::string actualName;
            char* currStr = (char*)demangledName + 2;

            if (*currStr == 'N')
            {
                currStr++;
                // extract actual name from nested name
                std::string nestedName = extractActualName(&currStr);
                actualName += nestedName;
                while (1)
                {
                    //Loop until nested name is empty
                    nestedName = extractActualName(&currStr);
                    if (nestedName.empty())
                    {
                        break;
                    }
                    actualName += "::" + nestedName;
                }
            } else
            {
                actualName = extractActualName(&currStr);
            }
            return actualName;
        }

        static tracy::SourceLocationData* getKernelSourceLocation(const char* kernelName)
        {
            auto& kernelSrcLoc = PersistentState::Get().kernelSrcLoc;
            std::string_view demangledName;
        #ifndef _MSC_VER
            // TODO(marcos): extractActualNameNested is the main bottleneck right now;
            // we need a specialized StringTable mapping from "peristent" kernel names
            // (const char*/uintptr_t) to memoized, lazily initialized demangled names
            auto& demangledNameTable = PersistentState::Get().demangledNameTable;
            std::string demangled = extractActualNameNested(kernelName);
            demangledName = demangledNameTable[demangled];
        #else
            demangledName = kernelName;
        #endif
            auto pSrcLoc = kernelSrcLoc.retrieve(demangledName);
            if (pSrcLoc == nullptr) {
                pSrcLoc = kernelSrcLoc.add(demangledName, TracyFile, TracyLine);
            }
            return pSrcLoc;
        }

        static void DoProcessDeviceEvent(CUpti_Activity *record)
        {
            static constexpr bool instrument = false;
            ZoneNamed(activity, instrument);

            switch (record->kind)
            {
            case CUPTI_ACTIVITY_KIND_CONCURRENT_KERNEL:
            {   
                ZoneNamedN(kernel, "tracy::CUDACtx::DoProcessDeviceEvent[kernel]", instrument);
                CUpti_ActivityKernel9* kernel9 = (CUpti_ActivityKernel9*) record;
                APICallInfo apiCall;
                if (!matchActivityToAPICall(kernel9->correlationId, apiCall)) {
                    return matchError(kernel9->correlationId, "KERNEL");
                }
                apiCall.host->EmitGpuZone(apiCall.start, apiCall.end, kernel9->start, kernel9->end, getKernelSourceLocation(kernel9->name), kernel9->contextId, kernel9->streamId);
                auto latency_ms = (kernel9->start - apiCall.cupti) / 1'000'000.0;
                tracyPlotBlip("Kernel Latency (ms)", kernel9->start, latency_ms);
                break;
            }

            case CUPTI_ACTIVITY_KIND_MEMCPY:
            {
                ZoneNamedN(kernel, "tracy::CUDACtx::DoProcessDeviceEvent[memcpy]", instrument);
                CUpti_ActivityMemcpy5* memcpy5 = (CUpti_ActivityMemcpy5*) record;
                APICallInfo apiCall;
                if (!matchActivityToAPICall(memcpy5->correlationId, apiCall)) {
                    return matchError(memcpy5->correlationId, "MEMCPY");
                }
                static constexpr tracy::SourceLocationData TracyCUPTISrcLocDeviceMemcpy { "CUDA::memcpy", TracyFunction, TracyFile, (uint32_t)TracyLine, tracy::Color::Blue };
                apiCall.host->EmitGpuZone(apiCall.start, apiCall.end, memcpy5->start, memcpy5->end, &TracyCUPTISrcLocDeviceMemcpy, memcpy5->contextId, memcpy5->streamId);
                static constexpr const char* graph_name = "CUDA Memory Copy";
                tracyEmitMemAlloc(graph_name, (void*)(uintptr_t)memcpy5->correlationId, memcpy5->bytes, memcpy5->start);
                tracyEmitMemFree (graph_name, (void*)(uintptr_t)memcpy5->correlationId,                 memcpy5->end);
                break;
            }

            case CUPTI_ACTIVITY_KIND_MEMSET:
            {
                ZoneNamedN(kernel, "tracy::CUDACtx::DoProcessDeviceEvent[memset]", instrument);
                CUpti_ActivityMemset4* memset4 = (CUpti_ActivityMemset4*) record;
                APICallInfo apiCall;
                if (!matchActivityToAPICall(memset4->correlationId, apiCall)) {
                    return matchError(memset4->correlationId, "MEMSET");
                }
                static constexpr tracy::SourceLocationData TracyCUPTISrcLocDeviceMemset { "CUDA::memset", TracyFunction, TracyFile, (uint32_t)TracyLine, tracy::Color::Blue };
                apiCall.host->EmitGpuZone(apiCall.start, apiCall.end, memset4->start, memset4->end, &TracyCUPTISrcLocDeviceMemset, memset4->contextId, memset4->streamId);
                static constexpr const char* graph_name = "CUDA Memory Set";
                tracyEmitMemAlloc(graph_name, (void*)(uintptr_t)memset4->correlationId, memset4->bytes, memset4->start);
                tracyEmitMemFree (graph_name, (void*)(uintptr_t)memset4->correlationId,                 memset4->end);
                break;
            }

            case CUPTI_ACTIVITY_KIND_SYNCHRONIZATION:
            {
                ZoneNamedN(kernel, "tracy::CUDACtx::DoProcessDeviceEvent[sync]", instrument);
                CUpti_ActivitySynchronization* synchronization = (CUpti_ActivitySynchronization*) record;
                APICallInfo apiCall;
                if (!matchActivityToAPICall(synchronization->correlationId, apiCall)) {
                    return matchError(synchronization->correlationId, "SYNCHRONIZATION");
                }
                // NOTE(marcos): synchronization can happen at different levels/objects:
                // a. on the entire context : cuCtxSynchronize()    -> timeline(ctx,0)
                // b. on a specific stream  : cuStreamSynchronize() -> timeline(ctx,stream)
                // c. on a specific event   : cuEventSynchronize()  -> timeline(ctx,0xffff)
                static constexpr tracy::SourceLocationData TracyCUPTISrcLocContextSynchronization { "CUDA::Context::sync", TracyFunction, TracyFile, (uint32_t)TracyLine, tracy::Color::Magenta };
                auto* pSrcLoc = &TracyCUPTISrcLocContextSynchronization;
                uint32_t cudaContextId = synchronization->contextId;
                uint32_t cudaStreamId = 0;
                if (synchronization->streamId != CUPTI_SYNCHRONIZATION_INVALID_VALUE) {
                    static constexpr tracy::SourceLocationData TracyCUPTISrcLocStreamSynchronization{ "CUDA::Stream::sync", TracyFunction, TracyFile, (uint32_t)TracyLine, tracy::Color::Magenta3 };
                    pSrcLoc = &TracyCUPTISrcLocStreamSynchronization;
                    cudaStreamId = synchronization->streamId;
                }
                if (synchronization->cudaEventId != CUPTI_SYNCHRONIZATION_INVALID_VALUE) {
                    static constexpr tracy::SourceLocationData TracyCUPTISrcLocEventSynchronization{ "CUDA::Event::sync", TracyFunction, TracyFile, (uint32_t)TracyLine, tracy::Color::Magenta4 };
                    pSrcLoc = &TracyCUPTISrcLocEventSynchronization;
                    cudaStreamId = 0xFFFFFFFF;
                    // TODO(marcos): CUpti_ActivitySynchronization2 introduces a new
                    // field 'cudaEventSyncId' which complements 'cudaEventId'
                }
                apiCall.host->EmitGpuZone(apiCall.start, apiCall.end, synchronization->start, synchronization->end, pSrcLoc, cudaContextId, cudaStreamId);
                static constexpr const char* graph_name = "CUDA Synchronization";
                tracyEmitMemAlloc(graph_name, (void*)(uintptr_t)synchronization->correlationId, 1, synchronization->start);
                tracyEmitMemFree (graph_name, (void*)(uintptr_t)synchronization->correlationId,    synchronization->end);
                break;
            }
            case CUPTI_ACTIVITY_KIND_MEMORY2:
            {
                ZoneNamedN(kernel, "tracy::CUDACtx::DoProcessDeviceEvent[malloc/free]", instrument);
                CUpti_ActivityMemory3* memory3 = (CUpti_ActivityMemory3*)record;
                APICallInfo apiCall;
                if (!matchActivityToAPICall(memory3->correlationId, apiCall)) {
                    return matchError(memory3->correlationId, "MEMORY");
                }
                static constexpr const char* graph_name = "CUDA Memory Allocation";
                if (memory3->memoryOperationType == CUPTI_ACTIVITY_MEMORY_OPERATION_TYPE_ALLOCATION){
                    auto& memAllocAddress = PersistentState::Get().memAllocAddress;
                    memAllocAddress[memory3->address] = 1;
                    tracyEmitMemAlloc(graph_name, (void*)memory3->address, memory3->bytes, memory3->timestamp);
                }
                else if (memory3->memoryOperationType == CUPTI_ACTIVITY_MEMORY_OPERATION_TYPE_RELEASE){
                    auto& memAllocAddress = PersistentState::Get().memAllocAddress;
                    int dontCare;
                    if (!memAllocAddress.fetch(memory3->address, dontCare)){
                        // Note(Frank): This is a hack to handle the case where the memory allocation
                        // corresponds to the memory release is not found.
                        // This can happen when the memory is allocated when profiling is not enabled.
                        matchError(memory3->correlationId, "MEMORY/RELEASE");
                        tracyEmitMemAlloc(graph_name, (void*)memory3->address, memory3->bytes, memory3->timestamp);
                    } else {
                        memAllocAddress.erase(memory3->address);
                    }
                    tracyEmitMemFree(graph_name, (void*)memory3->address, memory3->timestamp);
                }
                break;
            }
            case CUPTI_ACTIVITY_KIND_CUDA_EVENT :
            {
                // NOTE(marcos): a byproduct of CUPTI_ACTIVITY_KIND_SYNCHRONIZATION
                // (I think this is related to cudaEvent*() API calls)
                CUpti_ActivityCudaEvent2* event = (CUpti_ActivityCudaEvent2*)record;
                UNREFERENCED(event);
                break;
            }
            default:
            {
                char buffer[64];
                snprintf(buffer, sizeof(buffer), "Unknown activity record (kind is %d)", record->kind);
                TracyMessageC(buffer, strlen(buffer), tracy::Color::Crimson);
                break;
            }
            }
        }

        static constexpr CUpti_CallbackDomain domains[] = {
            CUPTI_CB_DOMAIN_RUNTIME_API,
            CUPTI_CB_DOMAIN_DRIVER_API,
            //CUPTI_CB_DOMAIN_RESOURCE,
            //CUPTI_CB_DOMAIN_SYNCHRONIZE,
            //CUPTI_CB_DOMAIN_NVTX,
            //CUPTI_CB_DOMAIN_STATE
        };

        static constexpr CUpti_ActivityKind activities[] = {
            //CUPTI_ACTIVITY_KIND_KERNEL, // mutually exclusive with CONCURRENT_KERNEL
            CUPTI_ACTIVITY_KIND_CONCURRENT_KERNEL,
            CUPTI_ACTIVITY_KIND_MEMCPY,
            CUPTI_ACTIVITY_KIND_MEMSET,
            CUPTI_ACTIVITY_KIND_SYNCHRONIZATION,
            CUPTI_ACTIVITY_KIND_MEMORY2,
            //CUPTI_ACTIVITY_KIND_MEMCPY2,
            //CUPTI_ACTIVITY_KIND_OVERHEAD,
            //CUPTI_ACTIVITY_KIND_INTERNAL_LAUNCH_API,
            //CUPTI_ACTIVITY_KIND_RUNTIME,
            //CUPTI_ACTIVITY_KIND_DRIVER,
        };

        static void BeginInstrumentation(CUDACtx* profilerHost) {
            auto& currentProfilerHost = PersistentState::Get().profilerHost;
            if (currentProfilerHost != nullptr) {
                return;
            }
            currentProfilerHost = profilerHost;

            // NOTE(frank): full-stop synchronization to ensure we only handle
            // CUDA API calls and device activities that happens past this point
            cudaDeviceSynchronize();

            auto& subscriber = PersistentState::Get().subscriber;
            CUPTI_API_CALL(cuptiSubscribe(&subscriber, CUPTI::OnCallbackAPI, profilerHost));
            CUPTI_API_CALL(cuptiActivityRegisterCallbacks(CUPTI::OnBufferRequested, CUPTI::OnBufferCompleted));
            for (auto domain : domains) {
                CUPTI_API_CALL(cuptiEnableDomain(uint32_t(true), subscriber, domain));
            }
            for (auto activity : activities) {
                CUPTI_API_CALL(cuptiActivityEnable(activity));
            }

            #if TRACY_CUDA_ENABLE_COLLECTOR_THREAD
            auto& collector = PersistentState::Get().collector;
            collector.period = 160;
            collector.signal.notify_one();
            #endif
        }

        static void EndInstrumentation() {
            auto& currentProfilerHost = PersistentState::Get().profilerHost;
            if (currentProfilerHost == nullptr) {
                return;
            }

            // NOTE(frank): full-stop synchronization to ensure we catch
            // and drain all the activities that has been tracked up to now.
            cudaDeviceSynchronize();

            FlushActivity();

            auto& subscriber = PersistentState::Get().subscriber;
            for (auto activity : activities) {
                CUPTI_API_CALL(cuptiActivityDisable(activity));
            }
            for (auto domain : domains) {
                CUPTI_API_CALL(cuptiEnableDomain(uint32_t(false), subscriber, domain));
            }
            // TODO(marcos): is here a counterpart for 'cuptiActivityRegisterCallbacks()'?
            CUPTI_API_CALL(cuptiUnsubscribe(subscriber));

            #if TRACY_CUDA_ENABLE_COLLECTOR_THREAD
            auto& collector = PersistentState::Get().collector;
            collector.period = ~uint32_t(0);
            collector.signal.notify_one();
            #endif

            currentProfilerHost = nullptr;
        }

        static void FlushActivity()
        {
            // NOTE(marcos): only one thread should do the collection at any given time,
            // but there's no reason to block threads that are also trying to do the same
            static std::mutex m;
            if (!m.try_lock())
                return;
            std::unique_lock<std::mutex> lock (m, std::adopt_lock);
            ZoneNamedNC(zone, "cuptiActivityFlushAll", tracy::Color::Red4, true);
            CUPTI_API_CALL(cuptiActivityFlushAll(CUPTI_ACTIVITY_FLAG_NONE));
        }

        #if TRACY_CUDA_ENABLE_COLLECTOR_THREAD
        // WARN(marcos): technically, CUPTI already offers async flushing of
        // activity records through cuptiActivityFlushPeriod(), but I haven't
        // had much luck getting reliable, consistent delivery with it...
        struct Collector {
            std::atomic<bool> running = true;
            volatile uint32_t period = ~uint32_t(0);
            std::mutex mtx;
            std::condition_variable signal;
            std::thread thread = std::thread(
                [this]() {
                    tracy::SetThreadName("Tracy CUDA Collector");
                    atexit([]() {
                        auto& collector = CUPTI::PersistentState::Get().collector;
                        collector.running = false;
                        collector.signal.notify_one();
                        collector.thread.join();
                    });
                    while (running) {
                        {
                            std::unique_lock<std::mutex> lock(mtx);
                            signal.wait_for(lock, std::chrono::milliseconds(period));
                        }
                        FlushActivity();
                    }
                }
            );
        };
        #endif

        static void FlushActivityAsync()
        {
            #if TRACY_CUDA_ENABLE_COLLECTOR_THREAD
            ZoneScoped;
            auto& collector = PersistentState::Get().collector;
            collector.signal.notify_one();
            #endif
        }

        struct PersistentState {
            // NOTE(marcos): these objects must remain in memory past the application
            // returning from main() because the Tracy client worker thread may still
            // be responding to string/source-location requests from the server
            SourceLocationMap kernelSrcLoc;
            StringTable demangledNameTable;
            SourceLocationLUT cudaCallSourceLocation;

            // NOTE(marcos): these objects do not need to persist, but their relative
            // footprint is trivial enough that we don't care if we let them leak
            ConcurrentHashMap<CorrelationID, APICallInfo> cudaCallSiteInfo;
            ConcurrentHashMap<uintptr_t, int> memAllocAddress;
            CUpti_SubscriberHandle subscriber = {};
            CUDACtx* profilerHost = nullptr;

            Collector collector;

            static PersistentState& Get() {
                static PersistentState& persistent = *(new PersistentState());
                return persistent;
            }
        };

        };

        CUDACtx(uint8_t gpuContextID = 255)
        {
            ZoneScoped;

            if (gpuContextID != 255) {
                m_tracyGpuContext = gpuContextID;
                return;
            }

            m_tracyGpuContext = GetGpuCtxCounter().fetch_add(1, std::memory_order_relaxed);
            assert(m_tracyGpuContext != 255);

            TracyTimestamp tTracy;
            CUptiTimestamp tCUpti;
            QueryTimestamps(tTracy, tCUpti);

            // Announce to Tracy about a new GPU context/timeline:
            auto item = Profiler::QueueSerial();
            tracyMemWrite(item->hdr.type, QueueType::GpuNewContext);
            tracyMemWrite(item->gpuNewContext.cpuTime, tTracy);
            tracyMemWrite(item->gpuNewContext.gpuTime, (int64_t)tCUpti); // TODO: Be more careful about this cast
            tracyMemWrite(item->gpuNewContext.thread, (uint32_t)0);
            tracyMemWrite(item->gpuNewContext.period, 1.0f);
            tracyMemWrite(item->gpuNewContext.type, GpuContextType::CUDA);
            tracyMemWrite(item->gpuNewContext.context, m_tracyGpuContext);
            #if TRACY_CUDA_CALIBRATED_CONTEXT
            tracyMemWrite(item->gpuNewContext.flags, GpuContextCalibration);
            #else
            tracyMemWrite(item->gpuNewContext.flags, tracy::GpuContextFlags(0));
            #endif
            Profiler::QueueSerialFinish();

            constexpr const char* tracyCtxName = "CUDA GPU/Device Activity";
            this->Name(tracyCtxName, uint16_t(strlen(tracyCtxName)));

            // NOTE(marcos): a few rounds of calibation amorthized over 1 second
            // in order to get a meaningful linear regression estimator
            Recalibrate();
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
            Recalibrate();
            std::this_thread::sleep_for(std::chrono::milliseconds(200));
            Recalibrate();
            std::this_thread::sleep_for(std::chrono::milliseconds(300));
            Recalibrate();
            std::this_thread::sleep_for(std::chrono::milliseconds(400));
            Recalibrate();
        }

        ~CUDACtx()
        {
            ZoneScoped;
        }

        struct Singleton {
            CUDACtx* ctx = nullptr;
            std::mutex m;
            int ref_count = 0;
            uint8_t ctx_id = 255;
            static Singleton& Get() {
                static Singleton singleton;
                return singleton;
            }
        };

        #if TRACY_CUDA_ENABLE_CUDA_CALL_STATS
        ProfilerStats stats = {};
        #endif

        uint8_t m_tracyGpuContext = 255;
        static constexpr size_t cacheline = 64;
        alignas(cacheline) std::atomic<uint16_t> m_queryIdGen = 0;
    };

}

#define TracyCUDAContext() tracy::CUDACtx::Create()
#define TracyCUDAContextDestroy(ctx) tracy::CUDACtx::Destroy(ctx)
#define TracyCUDAContextName(ctx, name, size) ctx->Name(name, size)

#define TracyCUDAStartProfiling(ctx) ctx->StartProfiling()
#define TracyCUDAStopProfiling(ctx) ctx->StopProfiling()

#define TracyCUDACollect(ctx) ctx->Collect()

#endif

#endif