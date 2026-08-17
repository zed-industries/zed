#![allow(non_camel_case_types)]

const_guid! { POWER_SAVINGS;
	/// Power scheme
	/// [identifiers](https://learn.microsoft.com/en-us/windows/win32/power/power-setting-guids)
	/// (`GUID`).
	///
	/// Originally has `GUID` prefix and `POWER_SAVINGS` suffix.
	=>
	MIN "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"
	MAX "a1841308-3541-4fab-bc81-f71556f20b4a"
	TYPICAL "381b4222-f694-41f0-9685-ff5bb260df2e"
}

const_guid! { POWER_SETTING;
	/// Power setting
	/// [identifiers](https://learn.microsoft.com/en-us/windows/win32/power/power-setting-guids)
	/// (`GUID`).
	///
	/// Originally has `GUID` prefix.
	=>
	ACDC_POWER_SOURCE "5d3e9a59-e9d5-4b00-a6bd-ff34ff516548"
	BATTERY_PERCENTAGE_REMAINING "a7ad8041-b45a-4cae-87a3-eecbb468a9e1"
	CONSOLE_DISPLAY_STATE "6fe69556-704a-47a0-8f24-c28d936fda47"
	GLOBAL_USER_PRESENCE "786e8a1d-b427-4344-9207-09e70bdcbea9"
	IDLE_BACKGROUND_TASK "515c31d8-f734-163d-a0fd-11a08c91e8f1"
	MONITOR_POWER_ON "02731015-4510-4526-99e6-e5a17ebd1aea"
	POWER_SAVING_STATUS "e00958c0-c213-4ace-ac77-fecced2eeea5"
	POWERSCHEME_PERSONALITY "245d8541-3943-4422-b025-13a784f679b7"
	MIN_POWER_SAVINGS "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"
	MAX_POWER_SAVINGS "a1841308-3541-4fab-bc81-f71556f20b4a"
	TYPICAL_POWER_SAVINGS "381b4222-f694-41f0-9685-ff5bb260df2e"
	SESSION_DISPLAY_STATUS "2b84c20e-ad23-4ddf-93db-05ffbd7efca5"
	SESSION_USER_PRESENCE "3c0f4548-c03f-4c4d-b9f2-237ede686376"
	LIDSWITCH_STATE_CHANGE "ba3e0f4d-b817-4094-a2d1-d56379e6a0f3"
	SYSTEM_AWAYMODE "98a7f580-01f7-48aa-9c0f-44352c29e5c0"
}
