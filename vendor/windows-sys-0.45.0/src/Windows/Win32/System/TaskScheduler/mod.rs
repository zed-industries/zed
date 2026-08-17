pub type IAction = *mut ::core::ffi::c_void;
pub type IActionCollection = *mut ::core::ffi::c_void;
pub type IBootTrigger = *mut ::core::ffi::c_void;
pub type IComHandlerAction = *mut ::core::ffi::c_void;
pub type IDailyTrigger = *mut ::core::ffi::c_void;
pub type IEmailAction = *mut ::core::ffi::c_void;
pub type IEnumWorkItems = *mut ::core::ffi::c_void;
pub type IEventTrigger = *mut ::core::ffi::c_void;
pub type IExecAction = *mut ::core::ffi::c_void;
pub type IExecAction2 = *mut ::core::ffi::c_void;
pub type IIdleSettings = *mut ::core::ffi::c_void;
pub type IIdleTrigger = *mut ::core::ffi::c_void;
pub type ILogonTrigger = *mut ::core::ffi::c_void;
pub type IMaintenanceSettings = *mut ::core::ffi::c_void;
pub type IMonthlyDOWTrigger = *mut ::core::ffi::c_void;
pub type IMonthlyTrigger = *mut ::core::ffi::c_void;
pub type INetworkSettings = *mut ::core::ffi::c_void;
pub type IPrincipal = *mut ::core::ffi::c_void;
pub type IPrincipal2 = *mut ::core::ffi::c_void;
pub type IProvideTaskPage = *mut ::core::ffi::c_void;
pub type IRegisteredTask = *mut ::core::ffi::c_void;
pub type IRegisteredTaskCollection = *mut ::core::ffi::c_void;
pub type IRegistrationInfo = *mut ::core::ffi::c_void;
pub type IRegistrationTrigger = *mut ::core::ffi::c_void;
pub type IRepetitionPattern = *mut ::core::ffi::c_void;
pub type IRunningTask = *mut ::core::ffi::c_void;
pub type IRunningTaskCollection = *mut ::core::ffi::c_void;
pub type IScheduledWorkItem = *mut ::core::ffi::c_void;
pub type ISessionStateChangeTrigger = *mut ::core::ffi::c_void;
pub type IShowMessageAction = *mut ::core::ffi::c_void;
pub type ITask = *mut ::core::ffi::c_void;
pub type ITaskDefinition = *mut ::core::ffi::c_void;
pub type ITaskFolder = *mut ::core::ffi::c_void;
pub type ITaskFolderCollection = *mut ::core::ffi::c_void;
pub type ITaskHandler = *mut ::core::ffi::c_void;
pub type ITaskHandlerStatus = *mut ::core::ffi::c_void;
pub type ITaskNamedValueCollection = *mut ::core::ffi::c_void;
pub type ITaskNamedValuePair = *mut ::core::ffi::c_void;
pub type ITaskScheduler = *mut ::core::ffi::c_void;
pub type ITaskService = *mut ::core::ffi::c_void;
pub type ITaskSettings = *mut ::core::ffi::c_void;
pub type ITaskSettings2 = *mut ::core::ffi::c_void;
pub type ITaskSettings3 = *mut ::core::ffi::c_void;
pub type ITaskTrigger = *mut ::core::ffi::c_void;
pub type ITaskVariables = *mut ::core::ffi::c_void;
pub type ITimeTrigger = *mut ::core::ffi::c_void;
pub type ITrigger = *mut ::core::ffi::c_void;
pub type ITriggerCollection = *mut ::core::ffi::c_void;
pub type IWeeklyTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const CLSID_CTask: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x148bd520_a2ab_11ce_b11f_00aa00530503);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const CLSID_CTaskScheduler: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x148bd52a_a2ab_11ce_b11f_00aa00530503);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_APRIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_AUGUST: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DECEMBER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FEBRUARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FIRST_WEEK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DELETE_WHEN_DONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DONT_START_IF_ON_BATTERIES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_HIDDEN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_INTERACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_KILL_IF_GOING_ON_BATTERIES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_KILL_ON_IDLE_END: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RESTART_ON_IDLE_RESUME: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_IF_CONNECTED_TO_INTERNET: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_ONLY_IF_DOCKED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_ONLY_IF_LOGGED_ON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_START_ONLY_IF_IDLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_SYSTEM_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FOURTH_WEEK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FRIDAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JANUARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JULY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JUNE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LAST_WEEK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MARCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MAY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MONDAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_NOVEMBER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_OCTOBER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SATURDAY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SECOND_WEEK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SEPTEMBER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SUNDAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_THIRD_WEEK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_THURSDAY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_HAS_END_DATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_KILL_AT_DURATION_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TUESDAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_WEDNESDAY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TaskHandlerPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2a69db7_da2c_4352_9066_86fee6dacac9);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TaskHandlerStatusPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9f15266d_d7ba_48f0_93c1_e6895f6fe5ac);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TaskScheduler: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0f87369f_a4e5_4cfc_bd3e_73e6154572dd);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASKPAGE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_TASK: TASKPAGE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_SCHEDULE: TASKPAGE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_SETTINGS: TASKPAGE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_ACTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_EXEC: TASK_ACTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_COM_HANDLER: TASK_ACTION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_SEND_EMAIL: TASK_ACTION_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_SHOW_MESSAGE: TASK_ACTION_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_COMPATIBILITY = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_AT: TASK_COMPATIBILITY = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V1: TASK_COMPATIBILITY = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2: TASK_COMPATIBILITY = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_1: TASK_COMPATIBILITY = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_2: TASK_COMPATIBILITY = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_3: TASK_COMPATIBILITY = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_4: TASK_COMPATIBILITY = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_CREATION = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_VALIDATE_ONLY: TASK_CREATION = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CREATE: TASK_CREATION = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_UPDATE: TASK_CREATION = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CREATE_OR_UPDATE: TASK_CREATION = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DISABLE: TASK_CREATION = 8i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DONT_ADD_PRINCIPAL_ACE: TASK_CREATION = 16i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_IGNORE_REGISTRATION_TRIGGERS: TASK_CREATION = 32i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_ENUM_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ENUM_HIDDEN: TASK_ENUM_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_INSTANCES_POLICY = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_PARALLEL: TASK_INSTANCES_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_QUEUE: TASK_INSTANCES_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_IGNORE_NEW: TASK_INSTANCES_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_STOP_EXISTING: TASK_INSTANCES_POLICY = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_LOGON_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_NONE: TASK_LOGON_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_PASSWORD: TASK_LOGON_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_S4U: TASK_LOGON_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_INTERACTIVE_TOKEN: TASK_LOGON_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_GROUP: TASK_LOGON_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_SERVICE_ACCOUNT: TASK_LOGON_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD: TASK_LOGON_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_PROCESSTOKENSID_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_NONE: TASK_PROCESSTOKENSID_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: TASK_PROCESSTOKENSID_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_DEFAULT: TASK_PROCESSTOKENSID_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_RUNLEVEL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUNLEVEL_LUA: TASK_RUNLEVEL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUNLEVEL_HIGHEST: TASK_RUNLEVEL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_RUN_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_NO_FLAGS: TASK_RUN_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_AS_SELF: TASK_RUN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_IGNORE_CONSTRAINTS: TASK_RUN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_USE_SESSION_ID: TASK_RUN_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_USER_SID: TASK_RUN_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_SESSION_STATE_CHANGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CONSOLE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CONSOLE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_REMOTE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_REMOTE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SESSION_LOCK: TASK_SESSION_STATE_CHANGE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SESSION_UNLOCK: TASK_SESSION_STATE_CHANGE_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_STATE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_UNKNOWN: TASK_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_DISABLED: TASK_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_QUEUED: TASK_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_READY: TASK_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_RUNNING: TASK_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_TRIGGER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_ONCE: TASK_TRIGGER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_DAILY: TASK_TRIGGER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_MONTHLYDATE: TASK_TRIGGER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_ON_IDLE: TASK_TRIGGER_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_AT_SYSTEMSTART: TASK_TRIGGER_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_AT_LOGON: TASK_TRIGGER_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_TRIGGER_TYPE2 = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_EVENT: TASK_TRIGGER_TYPE2 = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_TIME: TASK_TRIGGER_TYPE2 = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_DAILY: TASK_TRIGGER_TYPE2 = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE2 = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_MONTHLY: TASK_TRIGGER_TYPE2 = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE2 = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_IDLE: TASK_TRIGGER_TYPE2 = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_REGISTRATION: TASK_TRIGGER_TYPE2 = 7i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_BOOT: TASK_TRIGGER_TYPE2 = 8i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_LOGON: TASK_TRIGGER_TYPE2 = 9i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_SESSION_STATE_CHANGE: TASK_TRIGGER_TYPE2 = 11i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_CUSTOM_TRIGGER_01: TASK_TRIGGER_TYPE2 = 12i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct DAILY {
    pub DaysInterval: u16,
}
impl ::core::marker::Copy for DAILY {}
impl ::core::clone::Clone for DAILY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct MONTHLYDATE {
    pub rgfDays: u32,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDATE {}
impl ::core::clone::Clone for MONTHLYDATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct MONTHLYDOW {
    pub wWhichWeek: u16,
    pub rgfDaysOfTheWeek: u16,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDOW {}
impl ::core::clone::Clone for MONTHLYDOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct TASK_TRIGGER {
    pub cbTriggerSize: u16,
    pub Reserved1: u16,
    pub wBeginYear: u16,
    pub wBeginMonth: u16,
    pub wBeginDay: u16,
    pub wEndYear: u16,
    pub wEndMonth: u16,
    pub wEndDay: u16,
    pub wStartHour: u16,
    pub wStartMinute: u16,
    pub MinutesDuration: u32,
    pub MinutesInterval: u32,
    pub rgFlags: u32,
    pub TriggerType: TASK_TRIGGER_TYPE,
    pub Type: TRIGGER_TYPE_UNION,
    pub Reserved2: u16,
    pub wRandomMinutesInterval: u16,
}
impl ::core::marker::Copy for TASK_TRIGGER {}
impl ::core::clone::Clone for TASK_TRIGGER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub union TRIGGER_TYPE_UNION {
    pub Daily: DAILY,
    pub Weekly: WEEKLY,
    pub MonthlyDate: MONTHLYDATE,
    pub MonthlyDOW: MONTHLYDOW,
}
impl ::core::marker::Copy for TRIGGER_TYPE_UNION {}
impl ::core::clone::Clone for TRIGGER_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct WEEKLY {
    pub WeeksInterval: u16,
    pub rgfDaysOfTheWeek: u16,
}
impl ::core::marker::Copy for WEEKLY {}
impl ::core::clone::Clone for WEEKLY {
    fn clone(&self) -> Self {
        *self
    }
}
