// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! QoS definitions for NDIS components.
use shared::minwindef::ULONG;
pub type SERVICETYPE = ULONG;
STRUCT!{struct FLOWSPEC {
    TokenRate: ULONG,
    TokenBucketSize: ULONG,
    PeakBandwidth: ULONG,
    Latency: ULONG,
    DelayVariation: ULONG,
    ServiceType: SERVICETYPE,
    MaxSduSize: ULONG,
    MinimumPolicedSize: ULONG,
}}
pub type PFLOWSPEC = *mut FLOWSPEC;
pub type LPFLOWSPEC = *mut FLOWSPEC;
