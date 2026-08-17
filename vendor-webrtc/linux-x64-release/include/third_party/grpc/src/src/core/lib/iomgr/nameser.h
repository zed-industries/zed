//
//
// Copyright 2017 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_LIB_IOMGR_NAMESER_H
#define GRPC_SRC_CORE_LIB_IOMGR_NAMESER_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_HAVE_ARPA_NAMESER

#include <arpa/nameser.h>

#else  // GRPC_HAVE_ARPA_NAMESER

typedef enum __ns_class {
  ns_c_invalid = 0,  // Cookie.
  ns_c_in = 1,       // Internet.
  ns_c_2 = 2,        // unallocated/unsupported.
  ns_c_chaos = 3,    // MIT Chaos-net.
  ns_c_hs = 4,       // MIT Hesiod.
  // Query class values which do not appear in resource records
  ns_c_none = 254,  // for prereq. sections in update requests
  ns_c_any = 255,   // Wildcard match.
  ns_c_max = 65536
} ns_class;

typedef enum __ns_type {
  ns_t_invalid = 0,    // Cookie.
  ns_t_a = 1,          // Host address.
  ns_t_ns = 2,         // Authoritative server.
  ns_t_md = 3,         // Mail destination.
  ns_t_mf = 4,         // Mail forwarder.
  ns_t_cname = 5,      // Canonical name.
  ns_t_soa = 6,        // Start of authority zone.
  ns_t_mb = 7,         // Mailbox domain name.
  ns_t_mg = 8,         // Mail group member.
  ns_t_mr = 9,         // Mail rename name.
  ns_t_null = 10,      // Null resource record.
  ns_t_wks = 11,       // Well known service.
  ns_t_ptr = 12,       // Domain name pointer.
  ns_t_hinfo = 13,     // Host information.
  ns_t_minfo = 14,     // Mailbox information.
  ns_t_mx = 15,        // Mail routing information.
  ns_t_txt = 16,       // Text strings.
  ns_t_rp = 17,        // Responsible person.
  ns_t_afsdb = 18,     // AFS cell database.
  ns_t_x25 = 19,       // X_25 calling address.
  ns_t_isdn = 20,      // ISDN calling address.
  ns_t_rt = 21,        // Router.
  ns_t_nsap = 22,      // NSAP address.
  ns_t_nsap_ptr = 23,  // Reverse NSAP lookup (deprecated).
  ns_t_sig = 24,       // Security signature.
  ns_t_key = 25,       // Security key.
  ns_t_px = 26,        // X.400 mail mapping.
  ns_t_gpos = 27,      // Geographical position (withdrawn).
  ns_t_aaaa = 28,      // Ip6 Address.
  ns_t_loc = 29,       // Location Information.
  ns_t_nxt = 30,       // Next domain (security).
  ns_t_eid = 31,       // Endpoint identifier.
  ns_t_nimloc = 32,    // Nimrod Locator.
  ns_t_srv = 33,       // Server Selection.
  ns_t_atma = 34,      // ATM Address
  ns_t_naptr = 35,     // Naming Authority PoinTeR
  ns_t_kx = 36,        // Key Exchange
  ns_t_cert = 37,      // Certification record
  ns_t_a6 = 38,        // IPv6 address (deprecates AAAA)
  ns_t_dname = 39,     // Non-terminal DNAME (for IPv6)
  ns_t_sink = 40,      // Kitchen sink (experimental)
  ns_t_opt = 41,       // EDNS0 option (meta-RR)
  ns_t_apl = 42,       // Address prefix list (RFC3123)
  ns_t_ds = 43,        // Delegation Signer (RFC4034)
  ns_t_sshfp = 44,     // SSH Key Fingerprint (RFC4255)
  ns_t_rrsig = 46,     // Resource Record Signature (RFC4034)
  ns_t_nsec = 47,      // Next Secure (RFC4034)
  ns_t_dnskey = 48,    // DNS Public Key (RFC4034)
  ns_t_tkey = 249,     // Transaction key
  ns_t_tsig = 250,     // Transaction signature.
  ns_t_ixfr = 251,     // Incremental zone transfer.
  ns_t_axfr = 252,     // Transfer zone of authority.
  ns_t_mailb = 253,    // Transfer mailbox records.
  ns_t_maila = 254,    // Transfer mail agent records.
  ns_t_any = 255,      // Wildcard match.
  ns_t_zxfr = 256,     // BIND-specific, nonstandard.
  ns_t_max = 65536
} ns_type;

#endif  // GRPC_HAVE_ARPA_NAMESER

#endif  // GRPC_SRC_CORE_LIB_IOMGR_NAMESER_H
