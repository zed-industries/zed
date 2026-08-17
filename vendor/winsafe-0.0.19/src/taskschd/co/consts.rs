#![allow(non_camel_case_types)]

const_ordinary! { TASK_ACTION_TYPE: u32;
	/// [`TASK_ACTION_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/ne-taskschd-task_action_type)
	/// enumeration (`u32`);
	=>
	=>
	EXEC 0
	COM_HANDLER 5
	SEND_EMAIL 6
	SHOW_MESSAGE 7
}

const_bitflag! { TASK_CREATION: u32;
	/// [`TASK_CREATION`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/ne-taskschd-task_creation)
	/// enumeration (`u32`).
	///
	/// Originally has `TASK` prefix.
	=>
	=>
	VALIDATE_ONLY 0x1
	CREATE 0x2
	UPDATE 0x4
	CREATE_OR_UPDATE Self::CREATE.0 | Self::UPDATE.0
	DISABL 0x8
	DONT_ADD_PRINCIPAL_ACE 0x10
	IGNORE_REGISTRATION_TRIGGERS 0x20
}

const_bitflag! { TASK_LOGON: u32;
	/// [`TASK_LOGON_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/ne-taskschd-task_logon_type)
	/// enumeration (`u32`).
	=>
	=>
	NONE 0
	PASSWORD 1
	S4U 2
	INTERACTIVE_TOKEN 3
	GROUP 4
	SERVICE_ACCOUNT 5
	INTERACTIVE_TOKEN_OR_PASSWORD 6
}

const_ordinary! { TASK_STATE: u32;
	/// [`TASK_STATE`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/ne-taskschd-task_state)
	/// enumeration (`u32`).
	=>
	=>
	UNKNOWN 0
	DISABLED 1
	QUEUED 2
	READY 3
	RUNNING 4
}

const_ordinary! { TASK_TRIGGER_TYPE2: u32;
	/// [`TASK_TRIGGER_TYPE2`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/ne-taskschd-task_trigger_type2)
	/// enumeration (`u32`).
	=>
	=>
	EVENT 0
	TIME 1
	DAILY 2
	WEEKLY 3
	MONTHLY 4
	MONTHLYDOW 5
	IDLE 6
	REGISTRATION 7
	BOOT 8
	LOGON 9
	SESSION_STATE_CHANGE 11
	CUSTOM_TRIGGER_01 12
}
