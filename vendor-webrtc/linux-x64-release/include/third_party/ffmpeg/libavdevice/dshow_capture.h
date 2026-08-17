/*
 * DirectShow capture interface
 * Copyright (c) 2010 Ramiro Polla
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVDEVICE_DSHOW_CAPTURE_H
#define AVDEVICE_DSHOW_CAPTURE_H

#define DSHOWDEBUG 0

#include "avdevice.h"

#define COBJMACROS
#include <windows.h>
#define NO_DSHOW_STRSAFE
#include <dshow.h>
#include <dvdmedia.h>

#include "libavcodec/internal.h"
#include "libavcodec/packet_internal.h"

/* EC_DEVICE_LOST is not defined in MinGW dshow headers. */
#ifndef EC_DEVICE_LOST
#define EC_DEVICE_LOST 0x1f
#endif

long ff_copy_dshow_media_type(AM_MEDIA_TYPE *dst, const AM_MEDIA_TYPE *src);
void ff_print_VIDEO_STREAM_CONFIG_CAPS(const VIDEO_STREAM_CONFIG_CAPS *caps);
void ff_print_AUDIO_STREAM_CONFIG_CAPS(const AUDIO_STREAM_CONFIG_CAPS *caps);
void ff_print_AM_MEDIA_TYPE(const AM_MEDIA_TYPE *type);
void ff_printGUID(const GUID *g);

extern const AVClass *ff_dshow_context_class_ptr;
#define dshowdebug(...) ff_dlog(&ff_dshow_context_class_ptr, __VA_ARGS__)

static inline void nothing(void *foo)
{
}

struct GUIDoffset {
    const GUID *iid;
    int offset;
};

enum dshowDeviceType {
    VideoDevice = 0,
    AudioDevice = 1,
};

enum dshowSourceFilterType {
    VideoSourceDevice = 0,
    AudioSourceDevice = 1,
};

#define DECLARE_QUERYINTERFACE(prefix, class, ...)                           \
long WINAPI                                                                  \
ff_dshow_##prefix##_QueryInterface(class *this, const GUID *riid, void **ppvObject) \
{                                                                            \
    struct GUIDoffset ifaces[] = __VA_ARGS__;                                \
    int i;                                                                   \
    dshowdebug("ff_dshow_"AV_STRINGIFY(prefix)"_QueryInterface(%p, %p, %p)\n", this, riid, ppvObject); \
    ff_printGUID(riid);                                                      \
    if (!ppvObject)                                                          \
        return E_POINTER;                                                    \
    for (i = 0; i < sizeof(ifaces)/sizeof(ifaces[0]); i++) {                 \
        if (IsEqualGUID(riid, ifaces[i].iid)) {                              \
            void *obj = (void *) ((uint8_t *) this + ifaces[i].offset);      \
            ff_dshow_##prefix##_AddRef(this);                                \
            dshowdebug("\tfound %d with offset %d\n", i, ifaces[i].offset);  \
            *ppvObject = (void *) obj;                                       \
            return S_OK;                                                     \
        }                                                                    \
    }                                                                        \
    dshowdebug("\tE_NOINTERFACE\n");                                         \
    *ppvObject = NULL;                                                       \
    return E_NOINTERFACE;                                                    \
}
#define DECLARE_ADDREF(prefix, class)                                        \
unsigned long WINAPI                                                         \
ff_dshow_##prefix##_AddRef(class *this)                                      \
{                                                                            \
    dshowdebug("ff_dshow_"AV_STRINGIFY(prefix)"_AddRef(%p)\t%ld\n", this, this->ref+1); \
    return InterlockedIncrement(&this->ref);                                 \
}
#define DECLARE_RELEASE(prefix, class)                                       \
unsigned long WINAPI                                                         \
ff_dshow_##prefix##_Release(class *this)                                     \
{                                                                            \
    long ref = InterlockedDecrement(&this->ref);                             \
    dshowdebug("ff_dshow_"AV_STRINGIFY(prefix)"_Release(%p)\t%ld\n", this, ref); \
    if (!ref)                                                                \
        ff_dshow_##prefix##_Destroy(this);                                   \
    return ref;                                                              \
}

#define DECLARE_DESTROY(prefix, class, func)                                 \
void ff_dshow_##prefix##_Destroy(class *this)                                \
{                                                                            \
    dshowdebug("ff_dshow_"AV_STRINGIFY(prefix)"_Destroy(%p)\n", this);       \
    func(this);                                                              \
    if (this) {                                                              \
        if (this->vtbl)                                                      \
            CoTaskMemFree(this->vtbl);                                       \
        CoTaskMemFree(this);                                                 \
    }                                                                        \
}
#define DECLARE_CREATE(prefix, class, setup, ...)                            \
class *ff_dshow_##prefix##_Create(__VA_ARGS__)                               \
{                                                                            \
    class *this = CoTaskMemAlloc(sizeof(class));                             \
    dshowdebug("ff_dshow_"AV_STRINGIFY(prefix)"_Create(%p)\n", this);        \
    if (!this)                                                               \
        goto fail;                                                           \
    ZeroMemory(this, sizeof(class));                                         \
    this->vtbl = CoTaskMemAlloc(sizeof(*this->vtbl));                        \
    if (!this->vtbl)                                                         \
        goto fail;                                                           \
    ZeroMemory(this->vtbl, sizeof(*this->vtbl));                             \
    this->ref  = 1;                                                          \
    if (!setup)                                                              \
        goto fail;                                                           \
    dshowdebug("created ff_dshow_"AV_STRINGIFY(prefix)" %p\n", this);        \
    return this;                                                             \
fail:                                                                        \
    ff_dshow_##prefix##_Destroy(this);                                       \
    dshowdebug("could not create ff_dshow_"AV_STRINGIFY(prefix)"\n");        \
    return NULL;                                                             \
}

#define SETVTBL(vtbl, prefix, fn) \
    do { (vtbl)->fn = (void *) ff_dshow_##prefix##_##fn; } while(0)

/*****************************************************************************
 * Forward Declarations
 ****************************************************************************/
typedef struct DShowPin DShowPin;
typedef struct DShowMemInputPin DShowMemInputPin;
typedef struct DShowEnumPins DShowEnumPins;
typedef struct DShowEnumMediaTypes DShowEnumMediaTypes;
typedef struct DShowFilter DShowFilter;

/*****************************************************************************
 * DShowPin
 ****************************************************************************/
struct DShowPin {
    IPinVtbl *vtbl;
    long ref;
    DShowFilter *filter;
    IPin *connectedto;
    AM_MEDIA_TYPE type;
    IMemInputPinVtbl *imemvtbl;
};

long          WINAPI ff_dshow_pin_QueryInterface          (DShowPin *, const GUID *, void **);
unsigned long WINAPI ff_dshow_pin_AddRef                  (DShowPin *);
unsigned long WINAPI ff_dshow_pin_Release                 (DShowPin *);
long          WINAPI ff_dshow_pin_Connect                 (DShowPin *, IPin *, const AM_MEDIA_TYPE *);
long          WINAPI ff_dshow_pin_ReceiveConnection       (DShowPin *, IPin *, const AM_MEDIA_TYPE *);
long          WINAPI ff_dshow_pin_Disconnect              (DShowPin *);
long          WINAPI ff_dshow_pin_ConnectedTo             (DShowPin *, IPin **);
long          WINAPI ff_dshow_pin_ConnectionMediaType     (DShowPin *, AM_MEDIA_TYPE *);
long          WINAPI ff_dshow_pin_QueryPinInfo            (DShowPin *, PIN_INFO *);
long          WINAPI ff_dshow_pin_QueryDirection          (DShowPin *, PIN_DIRECTION *);
long          WINAPI ff_dshow_pin_QueryId                 (DShowPin *, wchar_t **);
long          WINAPI ff_dshow_pin_QueryAccept             (DShowPin *, const AM_MEDIA_TYPE *);
long          WINAPI ff_dshow_pin_EnumMediaTypes          (DShowPin *, IEnumMediaTypes **);
long          WINAPI ff_dshow_pin_QueryInternalConnections(DShowPin *, IPin **, unsigned long *);
long          WINAPI ff_dshow_pin_EndOfStream             (DShowPin *);
long          WINAPI ff_dshow_pin_BeginFlush              (DShowPin *);
long          WINAPI ff_dshow_pin_EndFlush                (DShowPin *);
long          WINAPI ff_dshow_pin_NewSegment              (DShowPin *, REFERENCE_TIME, REFERENCE_TIME, double);

long          WINAPI ff_dshow_meminputpin_QueryInterface          (DShowMemInputPin *, const GUID *, void **);
unsigned long WINAPI ff_dshow_meminputpin_AddRef                  (DShowMemInputPin *);
unsigned long WINAPI ff_dshow_meminputpin_Release                 (DShowMemInputPin *);
long          WINAPI ff_dshow_meminputpin_GetAllocator            (DShowMemInputPin *, IMemAllocator **);
long          WINAPI ff_dshow_meminputpin_NotifyAllocator         (DShowMemInputPin *, IMemAllocator *, BOOL);
long          WINAPI ff_dshow_meminputpin_GetAllocatorRequirements(DShowMemInputPin *, ALLOCATOR_PROPERTIES *);
long          WINAPI ff_dshow_meminputpin_Receive                 (DShowMemInputPin *, IMediaSample *);
long          WINAPI ff_dshow_meminputpin_ReceiveMultiple         (DShowMemInputPin *, IMediaSample **, long, long *);
long          WINAPI ff_dshow_meminputpin_ReceiveCanBlock         (DShowMemInputPin *);

void                 ff_dshow_pin_Destroy(DShowPin *);
DShowPin            *ff_dshow_pin_Create (DShowFilter *filter);

void                 ff_dshow_meminputpin_Destroy(DShowMemInputPin *);

/*****************************************************************************
 * DShowEnumPins
 ****************************************************************************/
struct DShowEnumPins {
    IEnumPinsVtbl *vtbl;
    long ref;
    int pos;
    DShowPin *pin;
    DShowFilter *filter;
};

long          WINAPI ff_dshow_enumpins_QueryInterface(DShowEnumPins *, const GUID *, void **);
unsigned long WINAPI ff_dshow_enumpins_AddRef        (DShowEnumPins *);
unsigned long WINAPI ff_dshow_enumpins_Release       (DShowEnumPins *);
long          WINAPI ff_dshow_enumpins_Next          (DShowEnumPins *, unsigned long, IPin **, unsigned long *);
long          WINAPI ff_dshow_enumpins_Skip          (DShowEnumPins *, unsigned long);
long          WINAPI ff_dshow_enumpins_Reset         (DShowEnumPins *);
long          WINAPI ff_dshow_enumpins_Clone         (DShowEnumPins *, DShowEnumPins **);

void                 ff_dshow_enumpins_Destroy(DShowEnumPins *);
DShowEnumPins       *ff_dshow_enumpins_Create (DShowPin *pin, DShowFilter *filter);

/*****************************************************************************
 * DShowEnumMediaTypes
 ****************************************************************************/
struct DShowEnumMediaTypes {
    IEnumMediaTypesVtbl *vtbl;
    long ref;
    int pos;
    AM_MEDIA_TYPE type;
};

long          WINAPI ff_dshow_enummediatypes_QueryInterface(DShowEnumMediaTypes *, const GUID *, void **);
unsigned long WINAPI ff_dshow_enummediatypes_AddRef        (DShowEnumMediaTypes *);
unsigned long WINAPI ff_dshow_enummediatypes_Release       (DShowEnumMediaTypes *);
long          WINAPI ff_dshow_enummediatypes_Next          (DShowEnumMediaTypes *, unsigned long, AM_MEDIA_TYPE **, unsigned long *);
long          WINAPI ff_dshow_enummediatypes_Skip          (DShowEnumMediaTypes *, unsigned long);
long          WINAPI ff_dshow_enummediatypes_Reset         (DShowEnumMediaTypes *);
long          WINAPI ff_dshow_enummediatypes_Clone         (DShowEnumMediaTypes *, DShowEnumMediaTypes **);

void                 ff_dshow_enummediatypes_Destroy(DShowEnumMediaTypes *);
DShowEnumMediaTypes *ff_dshow_enummediatypes_Create(const AM_MEDIA_TYPE *type);

/*****************************************************************************
 * DShowFilter
 ****************************************************************************/
struct DShowFilter {
    IBaseFilterVtbl *vtbl;
    long ref;
    const wchar_t *name;
    DShowPin *pin;
    FILTER_INFO info;
    FILTER_STATE state;
    IReferenceClock *clock;
    enum dshowDeviceType type;
    void *priv_data;
    int stream_index;
    int64_t start_time;
    void (*callback)(void *priv_data, int index, uint8_t *buf, int buf_size, int64_t time, enum dshowDeviceType type);
};

long          WINAPI ff_dshow_filter_QueryInterface (DShowFilter *, const GUID *, void **);
unsigned long WINAPI ff_dshow_filter_AddRef         (DShowFilter *);
unsigned long WINAPI ff_dshow_filter_Release        (DShowFilter *);
long          WINAPI ff_dshow_filter_GetClassID     (DShowFilter *, CLSID *);
long          WINAPI ff_dshow_filter_Stop           (DShowFilter *);
long          WINAPI ff_dshow_filter_Pause          (DShowFilter *);
long          WINAPI ff_dshow_filter_Run            (DShowFilter *, REFERENCE_TIME);
long          WINAPI ff_dshow_filter_GetState       (DShowFilter *, DWORD, FILTER_STATE *);
long          WINAPI ff_dshow_filter_SetSyncSource  (DShowFilter *, IReferenceClock *);
long          WINAPI ff_dshow_filter_GetSyncSource  (DShowFilter *, IReferenceClock **);
long          WINAPI ff_dshow_filter_EnumPins       (DShowFilter *, IEnumPins **);
long          WINAPI ff_dshow_filter_FindPin        (DShowFilter *, const wchar_t *, IPin **);
long          WINAPI ff_dshow_filter_QueryFilterInfo(DShowFilter *, FILTER_INFO *);
long          WINAPI ff_dshow_filter_JoinFilterGraph(DShowFilter *, IFilterGraph *, const wchar_t *);
long          WINAPI ff_dshow_filter_QueryVendorInfo(DShowFilter *, wchar_t **);

void                 ff_dshow_filter_Destroy(DShowFilter *);
DShowFilter         *ff_dshow_filter_Create (void *, void *, enum dshowDeviceType);

/*****************************************************************************
 * dshow_ctx
 ****************************************************************************/
struct dshow_ctx {
    const AVClass *class;

    IGraphBuilder *graph;

    char *device_name[2];
    char *device_unique_name[2];

    int video_device_number;
    int audio_device_number;

    int   list_options;
    int   list_devices;
    int   audio_buffer_size;
    int   crossbar_video_input_pin_number;
    int   crossbar_audio_input_pin_number;
    char *video_pin_name;
    char *audio_pin_name;
    int   show_video_device_dialog;
    int   show_audio_device_dialog;
    int   show_video_crossbar_connection_dialog;
    int   show_audio_crossbar_connection_dialog;
    int   show_analog_tv_tuner_dialog;
    int   show_analog_tv_tuner_audio_dialog;
    char *audio_filter_load_file;
    char *audio_filter_save_file;
    char *video_filter_load_file;
    char *video_filter_save_file;
    int   use_video_device_timestamps;

    IBaseFilter *device_filter[2];
    IPin        *device_pin[2];
    DShowFilter *capture_filter[2];
    DShowPin    *capture_pin[2];

    HANDLE mutex;
    HANDLE event[2]; /* event[0] is set by DirectShow
                      * event[1] is set by callback() */
    PacketListEntry *pktl;

    int eof;

    int64_t curbufsize[2];
    unsigned int video_frame_num;

    IMediaControl *control;
    IMediaEvent *media_event;

    enum AVPixelFormat pixel_format;
    enum AVCodecID video_codec_id;
    char *framerate;

    int requested_width;
    int requested_height;
    AVRational requested_framerate;

    int sample_rate;
    int sample_size;
    int channels;
};

/*****************************************************************************
 * CrossBar
 ****************************************************************************/
HRESULT ff_dshow_try_setup_crossbar_options(ICaptureGraphBuilder2 *graph_builder2,
    IBaseFilter *device_filter, enum dshowDeviceType devtype, AVFormatContext *avctx);

void ff_dshow_show_filter_properties(IBaseFilter *pFilter, AVFormatContext *avctx);

#endif /* AVDEVICE_DSHOW_CAPTURE_H */
