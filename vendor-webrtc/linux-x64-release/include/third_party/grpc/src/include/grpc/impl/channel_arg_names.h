// Copyright 2023 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_IMPL_CHANNEL_ARG_NAMES_H
#define GRPC_IMPL_CHANNEL_ARG_NAMES_H

// IWYU pragma: private, include <grpc/grpc.h>
// IWYU pragma: friend "src/.*"
// IWYU pragma: friend "test/.*"

/** \defgroup grpc_arg_keys
 * Channel argument keys.
 * \{
 */
/** If non-zero, enable census for tracing and stats collection. */
#define GRPC_ARG_ENABLE_CENSUS "grpc.census"
/** If non-zero, enable load reporting. */
#define GRPC_ARG_ENABLE_LOAD_REPORTING "grpc.loadreporting"
/** If non-zero, call metric recording is enabled. */
#define GRPC_ARG_SERVER_CALL_METRIC_RECORDING \
  "grpc.server_call_metric_recording"
/** Request that optional features default to off (regardless of what they
    usually default to) - to enable tight control over what gets enabled
    Boolean valued. Defaults to false. */
#define GRPC_ARG_MINIMAL_STACK "grpc.minimal_stack"
/** Maximum number of concurrent incoming streams to allow on a http2
    connection. Int valued. Deafult to -1(indicating no explicit limit).*/
#define GRPC_ARG_MAX_CONCURRENT_STREAMS "grpc.max_concurrent_streams"
/** Maximum message length that the channel can receive. Int valued, bytes.
    -1 means unlimited. Defaults to 4MB(4*1024*1024 bytes). -1 means unlimited.
 */
#define GRPC_ARG_MAX_RECEIVE_MESSAGE_LENGTH "grpc.max_receive_message_length"
/** \deprecated For backward compatibility.
 * Use GRPC_ARG_MAX_RECEIVE_MESSAGE_LENGTH instead. */
#define GRPC_ARG_MAX_MESSAGE_LENGTH GRPC_ARG_MAX_RECEIVE_MESSAGE_LENGTH
/** Maximum message length that the channel can send. Int valued, bytes.
    -1 means unlimited. */
#define GRPC_ARG_MAX_SEND_MESSAGE_LENGTH "grpc.max_send_message_length"
/** Maximum time that a channel may have no outstanding rpcs, after which the
 * server will close the connection. Int valued, milliseconds. INT_MAX means
 * unlimited. Defaults to INT_MAX. */
#define GRPC_ARG_MAX_CONNECTION_IDLE_MS "grpc.max_connection_idle_ms"
/** Maximum amount of time in milliseconds that a connection may exist before it
 * will be gracefully shut down. Refer
 * https://github.com/grpc/proposal/blob/master/A9-server-side-conn-mgt.md for
 * more details. Int valued, defaults to INT_MAX (disabled). */
#define GRPC_ARG_MAX_CONNECTION_AGE_MS "grpc.max_connection_age_ms"
/** Grace period in milliseconds after connection reaches its max age for
 * outstanding RPCs to complete. Int valued, defaults to INT_MAX (disabled). */
#define GRPC_ARG_MAX_CONNECTION_AGE_GRACE_MS "grpc.max_connection_age_grace_ms"
/** Timeout after the last RPC finishes on the client channel at which the
 * channel goes back into IDLE state. Int valued, milliseconds. INT_MAX means
 * unlimited. The default value is 30 minutes and the min value is 1 second. */
#define GRPC_ARG_CLIENT_IDLE_TIMEOUT_MS "grpc.client_idle_timeout_ms"
/** Enable/disable support for per-message compression. Boolean valued. Defaults
   to true, unless GRPC_ARG_MINIMAL_STACK is enabled, in which case it defaults
   to false. */
#define GRPC_ARG_ENABLE_PER_MESSAGE_COMPRESSION "grpc.per_message_compression"
/** Experimental Arg. Enable/disable support for per-message decompression.
   Defaults to 1. If disabled, decompression will not be performed and the
   application will see the compressed message in the byte buffer. */
#define GRPC_ARG_ENABLE_PER_MESSAGE_DECOMPRESSION \
  "grpc.per_message_decompression"
/** Initial stream ID for http2 transports. Int valued. Defaults to -1
    indicating use of default http2 setting initial stream ID (1). */
#define GRPC_ARG_HTTP2_INITIAL_SEQUENCE_NUMBER \
  "grpc.http2.initial_sequence_number"
/** Amount to read ahead on individual streams. Defaults to 64kb, larger
    values can help throughput on high-latency connections.
    NOTE: at some point we'd like to auto-tune this, and this parameter
    will become a no-op. Int valued, bytes. */
#define GRPC_ARG_HTTP2_STREAM_LOOKAHEAD_BYTES "grpc.http2.lookahead_bytes"
/** How much memory to use for hpack decoding. Int valued, bytes. Defaults to -1
    indicating use of default http2 setting(4096 bytes). */
#define GRPC_ARG_HTTP2_HPACK_TABLE_SIZE_DECODER \
  "grpc.http2.hpack_table_size.decoder"
/** How much memory to use for hpack encoding. Int valued, bytes. Defaults to -1
    indicating use of default http2 setting(4096 bytes). */
#define GRPC_ARG_HTTP2_HPACK_TABLE_SIZE_ENCODER \
  "grpc.http2.hpack_table_size.encoder"
/** How big a frame are we willing to receive via HTTP2.
    Min 16384, max 16777215. Larger values give lower CPU usage for large
    messages, but more head of line blocking for small messages. Defaults to
   -1(indicating no explicit max frame size), so it will take standard http/2
   specified max frame size to 16 KB. */
#define GRPC_ARG_HTTP2_MAX_FRAME_SIZE "grpc.http2.max_frame_size"
/** Should BDP probing be performed?
    Boolean valued. Defaults to true(enabled). */
#define GRPC_ARG_HTTP2_BDP_PROBE "grpc.http2.bdp_probe"
/** (DEPRECATED) Does not have any effect.
    Earlier, this arg configured the minimum time between successive ping frames
    without receiving any data/header frame, Int valued, milliseconds. This put
    unnecessary constraints on the configuration of keepalive pings,
    requiring users to set this channel arg along with
    GRPC_ARG_KEEPALIVE_TIME_MS. This arg also limited the activity of the other
    source of pings in gRPC Core - BDP pings, but BDP pings are only sent when
    there is receive-side data activity, making this arg unuseful for BDP pings
    too.  */
#define GRPC_ARG_HTTP2_MIN_SENT_PING_INTERVAL_WITHOUT_DATA_MS \
  "grpc.http2.min_time_between_pings_ms"
/** Minimum allowed time between a server receiving successive ping frames
   without sending any data/header frame. Int valued, milliseconds
 */
#define GRPC_ARG_HTTP2_MIN_RECV_PING_INTERVAL_WITHOUT_DATA_MS \
  "grpc.http2.min_ping_interval_without_data_ms"
/** Maximum time to allow a request to be:
    (1) received by the server, but
    (2) not requested by a RequestCall (in the completion queue based API)
    before the request is cancelled */
#define GRPC_ARG_SERVER_MAX_UNREQUESTED_TIME_IN_SERVER_SECONDS \
  "grpc.server_max_unrequested_time_in_server"
/** Channel arg to override the http2 :scheme header. String valued. */
#define GRPC_ARG_HTTP2_SCHEME "grpc.http2_scheme"
/** How many pings can the client send before needing to send a data/header
   frame? (0 indicates that an infinite number of pings can be sent without
   sending a data frame or header frame).
   If experiment "max_pings_wo_data_throttle" is enabled, instead of pings being
   completely blocked, they are throttled.
  * Integer valued. Defaults to 2. */
#define GRPC_ARG_HTTP2_MAX_PINGS_WITHOUT_DATA \
  "grpc.http2.max_pings_without_data"
/** How many misbehaving pings the server can bear before sending goaway and
    closing the transport? (0 indicates that the server can bear an infinite
    number of misbehaving pings)
  * Integer valued. Defaults to 2. */
#define GRPC_ARG_HTTP2_MAX_PING_STRIKES "grpc.http2.max_ping_strikes"
/** How much data are we willing to queue up per stream if
    GRPC_WRITE_BUFFER_HINT is set? This is an upper bound
  * Integer valued, bytes. Defaults to 65535 bytes. */
#define GRPC_ARG_HTTP2_WRITE_BUFFER_SIZE "grpc.http2.write_buffer_size"
/** Should we allow receipt of true-binary data on http2 connections?
    Defaults to on (1) */
#define GRPC_ARG_HTTP2_ENABLE_TRUE_BINARY "grpc.http2.true_binary"
/** An experimental channel arg which determines whether the preferred crypto
 * frame size http2 setting sent to the peer at startup. If set to 0 (false
 * - default), the preferred frame size is not sent to the peer. Otherwise it
 * sends a default preferred crypto frame size value of 4GB to the peer at
 * the startup of each connection. */
#define GRPC_ARG_EXPERIMENTAL_HTTP2_PREFERRED_CRYPTO_FRAME_SIZE \
  "grpc.experimental.http2.enable_preferred_frame_size"
/** After a duration of this time the client/server pings its peer to see if the
    transport is still alive. Int valued, milliseconds. Defaults to 7200000 (2
   hours). */
#define GRPC_ARG_KEEPALIVE_TIME_MS "grpc.keepalive_time_ms"
/** After waiting for a duration of this time, if the keepalive ping sender does
    not receive the ping ack, it will close the transport. Int valued,
    milliseconds. Defaults to 20000 (20 seonds). */
#define GRPC_ARG_KEEPALIVE_TIMEOUT_MS "grpc.keepalive_timeout_ms"
/** Is it permissible to send keepalive pings from the client without any
   outstanding streams. Int valued, 0(false)/1(true). Defaults to 0. */
#define GRPC_ARG_KEEPALIVE_PERMIT_WITHOUT_CALLS \
  "grpc.keepalive_permit_without_calls"
/** Default authority to pass if none specified on call construction. A string.
 * */
#define GRPC_ARG_DEFAULT_AUTHORITY "grpc.default_authority"
/** Primary user agent: goes at the start of the user-agent metadata
    sent on each request. A string. Defaults to empty string(""). */
#define GRPC_ARG_PRIMARY_USER_AGENT_STRING "grpc.primary_user_agent"
/** Secondary user agent: goes at the end of the user-agent metadata
    sent on each request. A string. */
#define GRPC_ARG_SECONDARY_USER_AGENT_STRING "grpc.secondary_user_agent"
/** The minimum time between subsequent connection attempts, in ms. Refer to
 * MIN_CONNECT_TIMEOUT from
 * https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md. Defaults
 * to 20 seconds. */
#define GRPC_ARG_MIN_RECONNECT_BACKOFF_MS "grpc.min_reconnect_backoff_ms"
/** The maximum time between subsequent connection attempts, in ms. Refer to
 * MAX_BACKOFF from
 * https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md. Defaults
 * to 120 seconds.  */
#define GRPC_ARG_MAX_RECONNECT_BACKOFF_MS "grpc.max_reconnect_backoff_ms"
/** The time between the first and second connection attempts, in ms. Refer to
 * INITIAL_BACKOFF from
 * https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md. Defaults
 * to 1 second. */
#define GRPC_ARG_INITIAL_RECONNECT_BACKOFF_MS \
  "grpc.initial_reconnect_backoff_ms"
/** Minimum amount of time between DNS resolutions, in ms. Defaults to 30
 * seconds. */
#define GRPC_ARG_DNS_MIN_TIME_BETWEEN_RESOLUTIONS_MS \
  "grpc.dns_min_time_between_resolutions_ms"
/** The timeout used on servers for finishing handshaking on an incoming
    connection.  Defaults to 120 seconds. */
#define GRPC_ARG_SERVER_HANDSHAKE_TIMEOUT_MS "grpc.server_handshake_timeout_ms"
/** This *should* be used for testing only.
    The caller of the secure_channel_create functions may override the target
    name used for SSL host name checking using this channel argument which is of
    type \a GRPC_ARG_STRING. If this argument is not specified, the name used
    for SSL host name checking will be the target parameter (assuming that the
    secure channel is an SSL channel). If this parameter is specified and the
    underlying is not an SSL channel, it will just be ignored. */
#define GRPC_SSL_TARGET_NAME_OVERRIDE_ARG "grpc.ssl_target_name_override"
/** If non-zero, a pointer to a session cache (a pointer of type
    grpc_ssl_session_cache*). (use grpc_ssl_session_cache_arg_vtable() to fetch
    an appropriate pointer arg vtable). */
#define GRPC_SSL_SESSION_CACHE_ARG "grpc.ssl_session_cache"
/** If non-zero, it will determine the maximum frame size used by TSI's frame
 *  protector. Defaults to zero.
 */
#define GRPC_ARG_TSI_MAX_FRAME_SIZE "grpc.tsi.max_frame_size"
/** Maximum metadata size (soft limit), in bytes. Note this limit applies to the
   max sum of all metadata key-value entries in a batch of headers. Some random
   sample of requests between this limit and
   `GRPC_ARG_ABSOLUTE_MAX_METADATA_SIZE` will be rejected. Defaults to maximum
   of 8 KB and `GRPC_ARG_ABSOLUTE_MAX_METADATA_SIZE` * 0.8 (if set).
 */
#define GRPC_ARG_MAX_METADATA_SIZE "grpc.max_metadata_size"
/** Maximum metadata size (hard limit), in bytes. Note this limit applies to the
   max sum of all metadata key-value entries in a batch of headers. All requests
   exceeding this limit will be rejected. Defaults to maximum of 16 KB and
   `GRPC_ARG_MAX_METADATA_SIZE` * 1.25 (if set). */
#define GRPC_ARG_ABSOLUTE_MAX_METADATA_SIZE "grpc.absolute_max_metadata_size"
/** If non-zero, allow the use of SO_REUSEPORT if it's available (default 1) */
#define GRPC_ARG_ALLOW_REUSEPORT "grpc.so_reuseport"
/** If non-zero, a pointer to a buffer pool (a pointer of type
 * grpc_resource_quota*). (use grpc_resource_quota_arg_vtable() to fetch an
 * appropriate pointer arg vtable). */
#define GRPC_ARG_RESOURCE_QUOTA "grpc.resource_quota"
/** If non-zero, expand wildcard addresses to a list of local addresses. Boolean
 * valued. */
#define GRPC_ARG_EXPAND_WILDCARD_ADDRS "grpc.expand_wildcard_addrs"
/** Service config data in JSON form.
    This value will be ignored if the name resolver returns a service config. A
   string value. */
#define GRPC_ARG_SERVICE_CONFIG "grpc.service_config"
/** Disable looking up the service config via the name resolver. Boolean valued.
 * Defaults to true. */
#define GRPC_ARG_SERVICE_CONFIG_DISABLE_RESOLUTION \
  "grpc.service_config_disable_resolution"
/** LB policy name. A string value.*/
#define GRPC_ARG_LB_POLICY_NAME "grpc.lb_policy_name"
/** Cap for ring size in the ring_hash LB policy.  The min and max ring size
    values set in the LB policy config will be capped to this value.
    Default is 4096. */
#define GRPC_ARG_RING_HASH_LB_RING_SIZE_CAP "grpc.lb.ring_hash.ring_size_cap"
/** The grpc_socket_mutator instance that set the socket options. A pointer. */
#define GRPC_ARG_SOCKET_MUTATOR "grpc.socket_mutator"
/** The grpc_socket_factory instance to create and bind sockets. A pointer. */
#define GRPC_ARG_SOCKET_FACTORY "grpc.socket_factory"
/** The maximum amount of memory used by trace events per channel trace node.
 * Once the maximum is reached, subsequent events will evict the oldest events
 * from the buffer. The unit for this knob is bytes. Setting it to zero causes
 * channel tracing to be disabled. Defaults to (1024 * 4) bytes. */
#define GRPC_ARG_MAX_CHANNEL_TRACE_EVENT_MEMORY_PER_NODE \
  "grpc.max_channel_trace_event_memory_per_node"
/** If non-zero, gRPC library will track stats and information at at per channel
 * level. Disabling channelz naturally disables channel tracing. The default
 * is for channelz to be enabled. */
#define GRPC_ARG_ENABLE_CHANNELZ "grpc.enable_channelz"
/** Channel arg (integer) setting how large a slice to try and read from the
   wire each time recvmsg (or equivalent). Range varied from 1
   to (32 * 1024 * 1024) bytes. Defaults to 8192 bytes. **/
#define GRPC_ARG_TCP_READ_CHUNK_SIZE "grpc.experimental.tcp_read_chunk_size"
/** Note this is not a "channel arg" key. This is the default slice size to use
 * when trying to read from the wire if the GRPC_ARG_TCP_READ_CHUNK_SIZE
 * channel arg is unspecified. */
#define GRPC_TCP_DEFAULT_READ_SLICE_SIZE 8192
/** Minimum read chunk size defaults to 256 bytes. Range varies from 1 to (32 *
 * 1024 * 1024) bytes. */
#define GRPC_ARG_TCP_MIN_READ_CHUNK_SIZE \
  "grpc.experimental.tcp_min_read_chunk_size"
#define GRPC_ARG_TCP_MAX_READ_CHUNK_SIZE \
  "grpc.experimental.tcp_max_read_chunk_size"
/* TCP TX Zerocopy enable state: zero is disabled, non-zero is enabled. By
   default, it is disabled. */
#define GRPC_ARG_TCP_TX_ZEROCOPY_ENABLED \
  "grpc.experimental.tcp_tx_zerocopy_enabled"
/* TCP TX Zerocopy send threshold: only zerocopy if >= this many bytes sent. By
   default, this is set to 16KB. */
#define GRPC_ARG_TCP_TX_ZEROCOPY_SEND_BYTES_THRESHOLD \
  "grpc.experimental.tcp_tx_zerocopy_send_bytes_threshold"
/* TCP TX Zerocopy max simultaneous sends: limit for maximum number of pending
   calls to tcp_write() using zerocopy. A tcp_write() is considered pending
   until the kernel performs the zerocopy-done callback for all sendmsg() calls
   issued by the tcp_write(). By default, this is set to 4. */
#define GRPC_ARG_TCP_TX_ZEROCOPY_MAX_SIMULT_SENDS \
  "grpc.experimental.tcp_tx_zerocopy_max_simultaneous_sends"
/* Overrides the TCP socket receive buffer size, SO_RCVBUF.
    Default value is -1(kReadBufferSizeUnset) indicating that the system will
    decide the buffer size. Range varies from 0 to INT_MAX. */
#define GRPC_ARG_TCP_RECEIVE_BUFFER_SIZE "grpc.tcp_receive_buffer_size"
/* Timeout in milliseconds to use for calls to the grpclb load balancer.
   If 0 or unset, the balancer calls will have no deadline. Defaults to 0 ms. */
#define GRPC_ARG_GRPCLB_CALL_TIMEOUT_MS "grpc.grpclb_call_timeout_ms"
/* Specifies the xDS bootstrap config as a JSON string.
   FOR TESTING PURPOSES ONLY -- DO NOT USE IN PRODUCTION.
   This option allows controlling the bootstrap configuration on a
   per-channel basis, which is useful in tests.  However, this results
   in having a separate xDS client instance per channel rather than
   using the global instance, which is not the intended way to use xDS.
   Currently, this will (a) add unnecessary load on the xDS server and
   (b) break use of CSDS, and there may be additional side effects in
   the future. */
#define GRPC_ARG_TEST_ONLY_DO_NOT_USE_IN_PROD_XDS_BOOTSTRAP_CONFIG \
  "grpc.TEST_ONLY_DO_NOT_USE_IN_PROD.xds_bootstrap_config"
/* Timeout in milliseconds to wait for the serverlist from the grpclb load
   balancer before using fallback backend addresses from the resolver.
   If 0, enter fallback mode immediately. Default value is 10000ms. */
#define GRPC_ARG_GRPCLB_FALLBACK_TIMEOUT_MS "grpc.grpclb_fallback_timeout_ms"
/* Experimental Arg. Channel args to be used for the control-plane channel
 * created to the grpclb load balancers. This is a pointer arg whose value is a
 * grpc_channel_args object. If unset, most channel args from the parent channel
 * will be propagated to the grpclb channel. */
#define GRPC_ARG_EXPERIMENTAL_GRPCLB_CHANNEL_ARGS \
  "grpc.experimental.grpclb_channel_args"
/* Timeout in milliseconds to wait for the child of a specific priority to
   complete its initial connection attempt before the priority LB policy fails
   over to the next priority. Default value is 10 seconds. */
#define GRPC_ARG_PRIORITY_FAILOVER_TIMEOUT_MS \
  "grpc.priority_failover_timeout_ms"
/** String defining the optimization target for a channel.
    Can be: "latency"    - attempt to minimize latency at the cost of throughput
            "blend"      - try to balance latency and throughput
            "throughput" - attempt to maximize throughput at the expense of
                           latency
    Defaults to "blend". In the current implementation "blend" is equivalent to
    "latency". */
#define GRPC_ARG_OPTIMIZATION_TARGET "grpc.optimization_target"
/** Enables retry functionality.  Defaults to true.  When enabled,
    transparent retries will be performed as appropriate, and configurable
    retries are enabled when they are configured via the service config.
    For details, see:
      https://github.com/grpc/proposal/blob/master/A6-client-retries.md
    NOTE: Hedging functionality is not yet implemented, so those
          fields in the service config will currently be ignored.  See
          also the GRPC_ARG_EXPERIMENTAL_ENABLE_HEDGING arg below.
 */
#define GRPC_ARG_ENABLE_RETRIES "grpc.enable_retries"
/** Enables hedging functionality, as described in:
      https://github.com/grpc/proposal/blob/master/A6-client-retries.md
    Default is currently false, since this functionality is not yet
    fully implemented.
    NOTE: This channel arg is experimental and will eventually be removed.
          Once hedging functionality has been implemented and proves stable,
          this arg will be removed, and the hedging functionality will
          be enabled via the GRPC_ARG_ENABLE_RETRIES arg above. */
#define GRPC_ARG_EXPERIMENTAL_ENABLE_HEDGING "grpc.experimental.enable_hedging"
/** Per-RPC retry buffer size, in bytes. Default is 256 KiB. */
#define GRPC_ARG_PER_RPC_RETRY_BUFFER_SIZE "grpc.per_rpc_retry_buffer_size"
/** Channel arg that carries the bridged objective c object for custom metrics
 * logging filter. */
#define GRPC_ARG_MOBILE_LOG_CONTEXT "grpc.mobile_log_context"
/** If non-zero, client authority filter is disabled for the channel.
 *  Boolean valued. Defaults to false. */
#define GRPC_ARG_DISABLE_CLIENT_AUTHORITY_FILTER \
  "grpc.disable_client_authority_filter"
/** If set to zero, disables use of http proxies. Enabled by default. */
#define GRPC_ARG_ENABLE_HTTP_PROXY "grpc.enable_http_proxy"
/** Channel arg to set http proxy per channel. If set, the channel arg
 *  value will be preferred over the environment variable settings. String
 * value. */
#define GRPC_ARG_HTTP_PROXY "grpc.http_proxy"
/** Specifies an HTTP proxy to use for individual addresses.
 *  The proxy must be specified as an IP address, not a DNS name.
 *  If set, the channel arg value will be preferred over the environment
 *  variable settings. */
#define GRPC_ARG_ADDRESS_HTTP_PROXY "grpc.address_http_proxy"
/** Comma separated list of addresses or address ranges that are behind the
 *  address HTTP proxy.
 */
#define GRPC_ARG_ADDRESS_HTTP_PROXY_ENABLED_ADDRESSES \
  "grpc.address_http_proxy_enabled_addresses"
/** If set to non zero, surfaces the user agent string to the server. User
    agent is surfaced by default. */
#define GRPC_ARG_SURFACE_USER_AGENT "grpc.surface_user_agent"
/** If set, inhibits health checking (which may be enabled via the
 *  service config.). Boolean valued. Defaults to false. */
#define GRPC_ARG_INHIBIT_HEALTH_CHECKING "grpc.inhibit_health_checking"
/** If enabled, the channel's DNS resolver queries for SRV records.
 *  This is useful only when using the "grpclb" load balancing policy,
 *  as described in the following documents:
 *   https://github.com/grpc/proposal/blob/master/A5-grpclb-in-dns.md
 *   https://github.com/grpc/proposal/blob/master/A24-lb-policy-config.md
 *   https://github.com/grpc/proposal/blob/master/A26-grpclb-selection.md
 *  Note that this works only with the "ares" DNS resolver; it isn't supported
 *  by the "native" DNS resolver. Boolean valued. Defaults to false. */
#define GRPC_ARG_DNS_ENABLE_SRV_QUERIES "grpc.dns_enable_srv_queries"
/** If set, determines an upper bound on the number of milliseconds that the
 * c-ares based DNS resolver will wait on queries before cancelling them.
 * The default value is 120,000ms. Setting this to "0" will disable the
 * overall timeout entirely. Note that this doesn't include internal c-ares
 * timeouts/backoff/retry logic, and so the actual DNS resolution may time out
 * sooner than the value specified here. */
#define GRPC_ARG_DNS_ARES_QUERY_TIMEOUT_MS "grpc.dns_ares_query_timeout"
/** If set, uses a local subchannel pool within the channel. Otherwise, uses the
 * global subchannel pool. Boolean valued. Defaults to false. */
#define GRPC_ARG_USE_LOCAL_SUBCHANNEL_POOL "grpc.use_local_subchannel_pool"
/** gRPC Objective-C channel pooling domain string. */
#define GRPC_ARG_CHANNEL_POOL_DOMAIN "grpc.channel_pooling_domain"
/** gRPC Objective-C channel pooling id. */
#define GRPC_ARG_CHANNEL_ID "grpc.channel_id"
/** Channel argument for grpc_authorization_policy_provider. If present, enables
    gRPC authorization check. */
#define GRPC_ARG_AUTHORIZATION_POLICY_PROVIDER \
  "grpc.authorization_policy_provider"
/** EXPERIMENTAL. Updates to a server's configuration from a config fetcher (for
 * example, listener updates from xDS) cause all older connections to be
 * gracefully shut down (i.e., "drained") with a grace period configured by this
 * channel arg. Int valued, milliseconds. Defaults to 10 minutes.*/
#define GRPC_ARG_SERVER_CONFIG_CHANGE_DRAIN_GRACE_TIME_MS \
  "grpc.experimental.server_config_change_drain_grace_time_ms"
/** Configure the Differentiated Services Code Point used on outgoing packets.
 *  Integer value ranging from 0 to 63. */
#define GRPC_ARG_DSCP "grpc.dscp"
/** Connection Attempt Delay for use in Happy Eyeballs, in milliseconds.
 *  Defaults to 250ms. */
#define GRPC_ARG_HAPPY_EYEBALLS_CONNECTION_ATTEMPT_DELAY_MS \
  "grpc.happy_eyeballs_connection_attempt_delay_ms"
/** It accepts a MemoryAllocatorFactory as input and If specified, it forces
 * the default event engine to use memory allocators created using the provided
 * factory. */
#define GRPC_ARG_EVENT_ENGINE_USE_MEMORY_ALLOCATOR_FACTORY \
  "grpc.event_engine_use_memory_allocator_factory"
/** Configure the max number of allowed incoming connections to the server.
 * If unspecified, it is unlimited */
#define GRPC_ARG_MAX_ALLOWED_INCOMING_CONNECTIONS \
  "grpc.max_allowed_incoming_connections"
/** Configure per-channel or per-server stats plugins. */
#define GRPC_ARG_EXPERIMENTAL_STATS_PLUGINS "grpc.experimental.stats_plugins"
/** If non-zero, allow security frames to be sent and received. */
#define GRPC_ARG_SECURITY_FRAME_ALLOWED "grpc.security_frame_allowed"
/** \} */

#endif /* GRPC_IMPL_CHANNEL_ARG_NAMES_H */
