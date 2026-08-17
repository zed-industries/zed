/* Simple Plugin API
 *
 * Copyright © 2018 Wim Taymans
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef SPA_PARAM_H
#define SPA_PARAM_H

#ifdef __cplusplus
extern "C" {
#endif

/** \defgroup spa_param Parameters
 * Parameter value enumerations and type information
 */

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/utils/defs.h>

/** different parameter types that can be queried */
enum spa_param_type {
	SPA_PARAM_Invalid,		/**< invalid */
	SPA_PARAM_PropInfo,		/**< property information as SPA_TYPE_OBJECT_PropInfo */
	SPA_PARAM_Props,		/**< properties as SPA_TYPE_OBJECT_Props */
	SPA_PARAM_EnumFormat,		/**< available formats as SPA_TYPE_OBJECT_Format */
	SPA_PARAM_Format,		/**< configured format as SPA_TYPE_OBJECT_Format */
	SPA_PARAM_Buffers,		/**< buffer configurations as SPA_TYPE_OBJECT_ParamBuffers*/
	SPA_PARAM_Meta,			/**< allowed metadata for buffers as SPA_TYPE_OBJECT_ParamMeta*/
	SPA_PARAM_IO,			/**< configurable IO areas as SPA_TYPE_OBJECT_ParamIO */
	SPA_PARAM_EnumProfile,		/**< profile enumeration as SPA_TYPE_OBJECT_ParamProfile */
	SPA_PARAM_Profile,		/**< profile configuration as SPA_TYPE_OBJECT_ParamProfile */
	SPA_PARAM_EnumPortConfig,	/**< port configuration enumeration as SPA_TYPE_OBJECT_ParamPortConfig */
	SPA_PARAM_PortConfig,		/**< port configuration as SPA_TYPE_OBJECT_ParamPortConfig */
	SPA_PARAM_EnumRoute,		/**< routing enumeration as SPA_TYPE_OBJECT_ParamRoute */
	SPA_PARAM_Route,		/**< routing configuration as SPA_TYPE_OBJECT_ParamRoute */
	SPA_PARAM_Control,		/**< Control parameter, a SPA_TYPE_Sequence */
	SPA_PARAM_Latency,		/**< latency reporting, a SPA_TYPE_OBJECT_ParamLatency */
	SPA_PARAM_ProcessLatency,	/**< processing latency, a SPA_TYPE_OBJECT_ParamProcessLatency */
};

/** information about a parameter */
struct spa_param_info {
	uint32_t id;			/**< enum spa_param_type */
#define SPA_PARAM_INFO_SERIAL		(1<<0)	/**< bit to signal update even when the
						 *   read/write flags don't change */
#define SPA_PARAM_INFO_READ		(1<<1)
#define SPA_PARAM_INFO_WRITE		(1<<2)
#define SPA_PARAM_INFO_READWRITE	(SPA_PARAM_INFO_WRITE|SPA_PARAM_INFO_READ)
	uint32_t flags;
	uint32_t user;			/**< private user field. You can use this to keep
					  *  state. */
	uint32_t padding[5];
};

#define SPA_PARAM_INFO(id,flags) (struct spa_param_info){ (id), (flags) }

/** properties for SPA_TYPE_OBJECT_ParamBuffers */
enum spa_param_buffers {
	SPA_PARAM_BUFFERS_START,
	SPA_PARAM_BUFFERS_buffers,	/**< number of buffers (Int) */
	SPA_PARAM_BUFFERS_blocks,	/**< number of data blocks per buffer (Int) */
	SPA_PARAM_BUFFERS_size,		/**< size of a data block memory (Int)*/
	SPA_PARAM_BUFFERS_stride,	/**< stride of data block memory (Int) */
	SPA_PARAM_BUFFERS_align,	/**< alignment of data block memory (Int) */
	SPA_PARAM_BUFFERS_dataType,	/**< possible memory types (Int, mask of enum spa_data_type) */
};

/** properties for SPA_TYPE_OBJECT_ParamMeta */
enum spa_param_meta {
	SPA_PARAM_META_START,
	SPA_PARAM_META_type,	/**< the metadata, one of enum spa_meta_type (Id enum spa_meta_type) */
	SPA_PARAM_META_size,	/**< the expected maximum size the meta (Int) */
};

/** properties for SPA_TYPE_OBJECT_ParamIO */
enum spa_param_io {
	SPA_PARAM_IO_START,
	SPA_PARAM_IO_id,	/**< type ID, uniquely identifies the io area (Id enum spa_io_type) */
	SPA_PARAM_IO_size,	/**< size of the io area (Int) */
};

enum spa_param_availability {
	SPA_PARAM_AVAILABILITY_unknown,	/**< unknown availability */
	SPA_PARAM_AVAILABILITY_no,	/**< not available */
	SPA_PARAM_AVAILABILITY_yes,	/**< available */
};

/** properties for SPA_TYPE_OBJECT_ParamProfile */
enum spa_param_profile {
	SPA_PARAM_PROFILE_START,
	SPA_PARAM_PROFILE_index,	/**< profile index (Int) */
	SPA_PARAM_PROFILE_name,		/**< profile name (String) */
	SPA_PARAM_PROFILE_description,	/**< profile description (String) */
	SPA_PARAM_PROFILE_priority,	/**< profile priority (Int) */
	SPA_PARAM_PROFILE_available,	/**< availability of the profile
					  *  (Id enum spa_param_availability) */
	SPA_PARAM_PROFILE_info,		/**< info (Struct(
					  *		  Int : n_items,
					  *		  (String : key,
					  *		   String : value)*)) */
	SPA_PARAM_PROFILE_classes,	/**< node classes provided by this profile
					  *  (Struct(
					  *	   Int : number of items following
					  *        Struct(
					  *           String : class name (eg. "Audio/Source"),
					  *           Int : number of nodes
					  *           String : property (eg. "card.profile.devices"),
					  *           Array of Int: device indexes
					  *         )*)) */
	SPA_PARAM_PROFILE_save,		/**< If profile should be saved (Bool) */
};

enum spa_param_port_config_mode {
	SPA_PARAM_PORT_CONFIG_MODE_none,	/**< no configuration */
	SPA_PARAM_PORT_CONFIG_MODE_passthrough,	/**< passthrough configuration */
	SPA_PARAM_PORT_CONFIG_MODE_convert,	/**< convert configuration */
	SPA_PARAM_PORT_CONFIG_MODE_dsp,		/**< dsp configuration, depending on the external
						  *  format. For audio, ports will be configured for
						  *  the given number of channels with F32 format. */
};

/** properties for SPA_TYPE_OBJECT_ParamPortConfig */
enum spa_param_port_config {
	SPA_PARAM_PORT_CONFIG_START,
	SPA_PARAM_PORT_CONFIG_direction,	/**< direction, input/output (Id enum spa_direction) */
	SPA_PARAM_PORT_CONFIG_mode,		/**< (Id enum spa_param_port_config_mode) mode */
	SPA_PARAM_PORT_CONFIG_monitor,		/**< (Bool) enable monitor output ports on input ports */
	SPA_PARAM_PORT_CONFIG_control,		/**< (Bool) enable control ports */
	SPA_PARAM_PORT_CONFIG_format,		/**< (Object) format filter */
};

/** properties for SPA_TYPE_OBJECT_ParamRoute */
enum spa_param_route {
	SPA_PARAM_ROUTE_START,
	SPA_PARAM_ROUTE_index,			/**< index of the routing destination (Int) */
	SPA_PARAM_ROUTE_direction,		/**< direction, input/output (Id enum spa_direction) */
	SPA_PARAM_ROUTE_device,			/**< device id (Int) */
	SPA_PARAM_ROUTE_name,			/**< name of the routing destination (String) */
	SPA_PARAM_ROUTE_description,		/**< description of the destination (String) */
	SPA_PARAM_ROUTE_priority,		/**< priority of the destination (Int) */
	SPA_PARAM_ROUTE_available,		/**< availability of the destination
						  *  (Id enum spa_param_availability) */
	SPA_PARAM_ROUTE_info,			/**< info (Struct(
						  *		  Int : n_items,
						  *		  (String : key,
						  *		   String : value)*)) */
	SPA_PARAM_ROUTE_profiles,		/**< associated profile indexes (Array of Int) */
	SPA_PARAM_ROUTE_props,			/**< properties SPA_TYPE_OBJECT_Props */
	SPA_PARAM_ROUTE_devices,		/**< associated device indexes (Array of Int) */
	SPA_PARAM_ROUTE_profile,		/**< profile id (Int) */
	SPA_PARAM_ROUTE_save,			/**< If route should be saved (Bool) */
};


/** properties for SPA_TYPE_OBJECT_ParamLatency */
enum spa_param_latency {
	SPA_PARAM_LATENCY_START,
	SPA_PARAM_LATENCY_direction,		/**< direction, input/output (Id enum spa_direction) */
	SPA_PARAM_LATENCY_minQuantum,		/**< min latency relative to quantum (Float) */
	SPA_PARAM_LATENCY_maxQuantum,		/**< max latency relative to quantum (Float) */
	SPA_PARAM_LATENCY_minRate,		/**< min latency (Int) relative to rate */
	SPA_PARAM_LATENCY_maxRate,		/**< max latency (Int) relative to rate */
	SPA_PARAM_LATENCY_minNs,		/**< min latency (Long) in nanoseconds */
	SPA_PARAM_LATENCY_maxNs,		/**< max latency (Long) in nanoseconds */
};

/** properties for SPA_TYPE_OBJECT_ParamProcessLatency */
enum spa_param_process_latency {
	SPA_PARAM_PROCESS_LATENCY_START,
	SPA_PARAM_PROCESS_LATENCY_quantum,	/**< latency relative to quantum (Float) */
	SPA_PARAM_PROCESS_LATENCY_rate,		/**< latency (Int) relative to rate */
	SPA_PARAM_PROCESS_LATENCY_ns,		/**< latency (Long) in nanoseconds */
};

enum spa_param_bitorder {
	SPA_PARAM_BITORDER_unknown,	/**< unknown bitorder */
	SPA_PARAM_BITORDER_msb,		/**< most significant bit */
	SPA_PARAM_BITORDER_lsb,		/**< least significant bit */
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_H */
