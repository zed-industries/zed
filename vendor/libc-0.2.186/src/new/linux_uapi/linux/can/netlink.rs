//! Header: `linux/can/netlink.h`

use crate::prelude::*;

s! {
    pub struct can_bittiming {
        pub bitrate: u32,
        pub sample_point: u32,
        pub tq: u32,
        pub prop_seg: u32,
        pub phase_seg1: u32,
        pub phase_seg2: u32,
        pub sjw: u32,
        pub brp: u32,
    }

    pub struct can_bittiming_const {
        pub name: [c_char; 16],
        pub tseg1_min: u32,
        pub tseg1_max: u32,
        pub tseg2_min: u32,
        pub tseg2_max: u32,
        pub sjw_max: u32,
        pub brp_min: u32,
        pub brp_max: u32,
        pub brp_inc: u32,
    }

    pub struct can_clock {
        pub freq: u32,
    }

    pub struct can_berr_counter {
        pub txerr: u16,
        pub rxerr: u16,
    }

    pub struct can_ctrlmode {
        pub mask: u32,
        pub flags: u32,
    }

    pub struct can_device_stats {
        pub bus_error: u32,
        pub error_warning: u32,
        pub error_passive: u32,
        pub bus_off: u32,
        pub arbitration_lost: u32,
        pub restarts: u32,
    }
}

c_enum! {
    #[repr(c_uint)]
    pub enum can_state {
        pub CAN_STATE_ERROR_ACTIVE = 0,
        pub CAN_STATE_ERROR_WARNING,
        pub CAN_STATE_ERROR_PASSIVE,
        pub CAN_STATE_BUS_OFF,
        pub CAN_STATE_STOPPED,
        pub CAN_STATE_SLEEPING,
    }
}

pub const CAN_CTRLMODE_LOOPBACK: u32 = 0x01;
pub const CAN_CTRLMODE_LISTENONLY: u32 = 0x02;
pub const CAN_CTRLMODE_3_SAMPLES: u32 = 0x04;
pub const CAN_CTRLMODE_ONE_SHOT: u32 = 0x08;
pub const CAN_CTRLMODE_BERR_REPORTING: u32 = 0x10;
pub const CAN_CTRLMODE_FD: u32 = 0x20;
pub const CAN_CTRLMODE_PRESUME_ACK: u32 = 0x40;
pub const CAN_CTRLMODE_FD_NON_ISO: u32 = 0x80;
pub const CAN_CTRLMODE_CC_LEN8_DLC: u32 = 0x100;
pub const CAN_CTRLMODE_TDC_AUTO: u32 = 0x200;
pub const CAN_CTRLMODE_TDC_MANUAL: u32 = 0x400;

c_enum! {
    #[repr(c_int)]
    pub enum #anon {
        pub IFLA_CAN_UNSPEC = 0,
        pub IFLA_CAN_BITTIMING,
        pub IFLA_CAN_BITTIMING_CONST,
        pub IFLA_CAN_CLOCK,
        pub IFLA_CAN_STATE,
        pub IFLA_CAN_CTRLMODE,
        pub IFLA_CAN_RESTART_MS,
        pub IFLA_CAN_RESTART,
        pub IFLA_CAN_BERR_COUNTER,
        pub IFLA_CAN_DATA_BITTIMING,
        pub IFLA_CAN_DATA_BITTIMING_CONST,
        pub IFLA_CAN_TERMINATION,
        pub IFLA_CAN_TERMINATION_CONST,
        pub IFLA_CAN_BITRATE_CONST,
        pub IFLA_CAN_DATA_BITRATE_CONST,
        pub IFLA_CAN_BITRATE_MAX,
        pub IFLA_CAN_TDC,
        pub IFLA_CAN_CTRLMODE_EXT,
    }
}

c_enum! {
    #[repr(c_int)]
    pub enum #anon {
        pub IFLA_CAN_TDC_UNSPEC = 0,
        pub IFLA_CAN_TDC_TDCV_MIN,
        pub IFLA_CAN_TDC_TDCV_MAX,
        pub IFLA_CAN_TDC_TDCO_MIN,
        pub IFLA_CAN_TDC_TDCO_MAX,
        pub IFLA_CAN_TDC_TDCF_MIN,
        pub IFLA_CAN_TDC_TDCF_MAX,
        pub IFLA_CAN_TDC_TDCV,
        pub IFLA_CAN_TDC_TDCO,
        pub IFLA_CAN_TDC_TDCF,
    }
}

c_enum! {
    #[repr(c_int)]
    pub enum #anon {
        pub IFLA_CAN_CTRLMODE_UNSPEC = 0,
        pub IFLA_CAN_CTRLMODE_SUPPORTED,
    }
}

pub const CAN_TERMINATION_DISABLED: u16 = 0;
