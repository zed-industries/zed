//! This module corresponds to `mach/mach_types.h`

use port::mach_port_t;

pub type task_t = mach_port_t;
pub type task_name_t = mach_port_t;
pub type task_suspension_token_t = mach_port_t;
pub type thread_t = mach_port_t;
pub type thread_act_t = mach_port_t;
pub type ipc_space_t = mach_port_t;
pub type coalition_t = mach_port_t;
pub type host_t = mach_port_t;
pub type host_priv_t = mach_port_t;
pub type host_security_t = mach_port_t;
pub type processor_t = mach_port_t;
pub type processor_set_t = mach_port_t;
pub type processor_set_control_t = mach_port_t;
pub type semaphore_t = mach_port_t;
pub type lock_set_t = mach_port_t;
pub type ledger_t = mach_port_t;
pub type alarm_t = mach_port_t;
pub type clock_serv_t = mach_port_t;
pub type clock_ctrl_t = mach_port_t;

pub type processor_set_name_t = processor_set_t;

pub type clock_reply_t = mach_port_t;
pub type bootstrap_t = mach_port_t;
pub type mem_entry_name_port_t = mach_port_t;
pub type exception_handler_t = mach_port_t;
pub type exception_handler_array_t = *mut exception_handler_t;
pub type vm_task_entry_t = mach_port_t;
pub type io_master_t = mach_port_t;
pub type UNDServerRef = mach_port_t;

pub type task_array_t = *mut task_t;
pub type thread_array_t = *mut thread_t;
pub type processor_set_array_t = *mut processor_set_t;
pub type processor_set_name_array_t = *mut processor_set_t;
pub type processor_array_t = *mut processor_t;
pub type thread_act_array_t = *mut thread_act_t;
pub type ledger_array_t = *mut ledger_t;

pub type task_port_t = task_t;
pub type task_port_array_t = task_array_t;
pub type thread_port_t = thread_t;
pub type thread_port_array_t = thread_array_t;
pub type ipc_space_port_t = ipc_space_t;
pub type host_name_t = host_t;
pub type host_name_port_t = host_t;
pub type processor_set_port_t = processor_set_t;
pub type processor_set_name_port_t = processor_set_t;
pub type processor_set_name_port_array_t = processor_set_array_t;
pub type processor_set_control_port_t = processor_set_t;
pub type processor_port_t = processor_t;
pub type processor_port_array_t = processor_array_t;
pub type thread_act_port_t = thread_act_t;
pub type thread_act_port_array_t = thread_act_array_t;
pub type semaphore_port_t = semaphore_t;
pub type lock_set_port_t = lock_set_t;
pub type ledger_port_t = ledger_t;
pub type ledger_port_array_t = ledger_array_t;
pub type alarm_port_t = alarm_t;
pub type clock_serv_port_t = clock_serv_t;
pub type clock_ctrl_port_t = clock_ctrl_t;
pub type exception_port_t = exception_handler_t;
pub type exception_port_arrary_t = exception_handler_array_t;

pub const TASK_NULL: task_t = 0;
pub const TASK_NAME_NULL: task_name_t = 0;
pub const THREAD_NULL: thread_t = 0;
pub const TID_NULL: u64 = 0;
pub const THR_ACT_NULL: thread_act_t = 0;
pub const IPC_SPACE_NULL: ipc_space_t = 0;
pub const COALITION_NULL: coalition_t = 0;
pub const HOST_NULL: host_t = 0;
pub const HOST_PRIV_NULL: host_priv_t = 0;
pub const HOST_SECURITY_NULL: host_security_t = 0;
pub const PROCESSOR_SET_NULL: processor_set_t = 0;
pub const PROCESSOR_NULL: processor_t = 0;
pub const SEMAPHORE_NULL: semaphore_t = 0;
pub const LOCK_SET_NULL: lock_set_t = 0;
pub const LEDGER_NULL: ledger_t = 0;
pub const ALARM_NULL: alarm_t = 0;
pub const CLOCK_NULL: ::libc::clock_t = 0;
pub const UND_SERVER_NULL: UNDServerRef = 0;

// <sys/_types.h>: typedef	unsigned char	__darwin_uuid_t[16];
pub type uuid_t = [::libc::c_uchar; 16];

// <sys/_types/_fsid_t.h>
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct fsid {
    pub val: [i32; 2],
}
pub type fsid_t = fsid;

// <sys/_types/_fsobj_id_t.h>
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct fsobj_id {
    pub fid_objno: u32,
    pub fid_generation: u32,
}
pub type fsobj_id_t = fsobj_id;
