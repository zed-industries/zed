//! Debugger Adapter Protocol types for Rust.
//!
//! Based on: <https://microsoft.github.io/debug-adapter-protocol/specification>
//! (generated from machine-readable schema).

/// Types representing events, with associated payload types.
pub mod events;
/// Types representing protocol messages.
pub mod messages;
/// Types representing requests, with associated argument and response types.
pub mod requests;
mod types;

use std::cmp::Ordering;

pub use crate::types::*;

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            additional_module_columns: None,
            breakpoint_modes: None,
            completion_trigger_characters: None,
            exception_breakpoint_filters: None,
            support_suspend_debuggee: None,
            support_terminate_debuggee: None,
            supported_checksum_algorithms: None,
            supports_breakpoint_locations_request: None,
            supports_cancel_request: None,
            supports_clipboard_context: None,
            supports_completions_request: None,
            supports_conditional_breakpoints: None,
            supports_configuration_done_request: None,
            supports_data_breakpoint_bytes: None,
            supports_data_breakpoints: None,
            supports_delayed_stack_trace_loading: None,
            supports_disassemble_request: None,
            supports_evaluate_for_hovers: None,
            supports_exception_filter_options: None,
            supports_exception_info_request: None,
            supports_exception_options: None,
            supports_function_breakpoints: None,
            supports_goto_targets_request: None,
            supports_hit_conditional_breakpoints: None,
            supports_instruction_breakpoints: None,
            supports_loaded_sources_request: None,
            supports_log_points: None,
            supports_modules_request: None,
            supports_read_memory_request: None,
            supports_restart_frame: None,
            supports_restart_request: None,
            supports_set_expression: None,
            supports_set_variable: None,
            supports_single_thread_execution_requests: None,
            supports_step_back: None,
            supports_step_in_targets_request: None,
            supports_stepping_granularity: None,
            supports_terminate_request: None,
            supports_terminate_threads_request: None,
            supports_value_formatting_options: None,
            supports_write_memory_request: None,
            supports_ansistyling: None,
        }
    }
}

impl Capabilities {
    pub fn merge(&self, other: Capabilities) -> Capabilities {
        Capabilities {
            supports_configuration_done_request: other
                .supports_configuration_done_request
                .or(self.supports_configuration_done_request),
            supports_function_breakpoints: other
                .supports_function_breakpoints
                .or(self.supports_function_breakpoints),
            supports_conditional_breakpoints: other
                .supports_conditional_breakpoints
                .or(self.supports_conditional_breakpoints),
            supports_hit_conditional_breakpoints: other
                .supports_hit_conditional_breakpoints
                .or(self.supports_hit_conditional_breakpoints),
            supports_evaluate_for_hovers: other
                .supports_evaluate_for_hovers
                .or(self.supports_evaluate_for_hovers),
            exception_breakpoint_filters: other
                .exception_breakpoint_filters
                .or_else(|| self.exception_breakpoint_filters.clone()),
            supports_step_back: other.supports_step_back.or(self.supports_step_back),
            supports_set_variable: other.supports_set_variable.or(self.supports_set_variable),
            supports_restart_frame: other.supports_restart_frame.or(self.supports_restart_frame),
            supports_goto_targets_request: other
                .supports_goto_targets_request
                .or(self.supports_goto_targets_request),
            supports_step_in_targets_request: other
                .supports_step_in_targets_request
                .or(self.supports_step_in_targets_request),
            supports_completions_request: other
                .supports_completions_request
                .or(self.supports_completions_request),
            completion_trigger_characters: other
                .completion_trigger_characters
                .or_else(|| self.completion_trigger_characters.clone()),
            supports_modules_request: other
                .supports_modules_request
                .or(self.supports_modules_request),
            additional_module_columns: other
                .additional_module_columns
                .or_else(|| self.additional_module_columns.clone()),
            supported_checksum_algorithms: other
                .supported_checksum_algorithms
                .or_else(|| self.supported_checksum_algorithms.clone()),
            supports_restart_request: other
                .supports_restart_request
                .or(self.supports_restart_request),
            supports_exception_options: other
                .supports_exception_options
                .or(self.supports_exception_options),
            supports_value_formatting_options: other
                .supports_value_formatting_options
                .or(self.supports_value_formatting_options),
            supports_exception_info_request: other
                .supports_exception_info_request
                .or(self.supports_exception_info_request),
            support_terminate_debuggee: other
                .support_terminate_debuggee
                .or(self.support_terminate_debuggee),
            support_suspend_debuggee: other
                .support_suspend_debuggee
                .or(self.support_suspend_debuggee),
            supports_delayed_stack_trace_loading: other
                .supports_delayed_stack_trace_loading
                .or(self.supports_delayed_stack_trace_loading),
            supports_loaded_sources_request: other
                .supports_loaded_sources_request
                .or(self.supports_loaded_sources_request),
            supports_log_points: other.supports_log_points.or(self.supports_log_points),
            supports_terminate_threads_request: other
                .supports_terminate_threads_request
                .or(self.supports_terminate_threads_request),
            supports_set_expression: other
                .supports_set_expression
                .or(self.supports_set_expression),
            supports_terminate_request: other
                .supports_terminate_request
                .or(self.supports_terminate_request),
            supports_data_breakpoints: other
                .supports_data_breakpoints
                .or(self.supports_data_breakpoints),
            supports_read_memory_request: other
                .supports_read_memory_request
                .or(self.supports_read_memory_request),
            supports_write_memory_request: other
                .supports_write_memory_request
                .or(self.supports_write_memory_request),
            supports_disassemble_request: other
                .supports_disassemble_request
                .or(self.supports_disassemble_request),
            supports_cancel_request: other
                .supports_cancel_request
                .or(self.supports_cancel_request),
            supports_breakpoint_locations_request: other
                .supports_breakpoint_locations_request
                .or(self.supports_breakpoint_locations_request),
            supports_clipboard_context: other
                .supports_clipboard_context
                .or(self.supports_clipboard_context),
            supports_stepping_granularity: other
                .supports_stepping_granularity
                .or(self.supports_stepping_granularity),
            supports_instruction_breakpoints: other
                .supports_instruction_breakpoints
                .or(self.supports_instruction_breakpoints),
            supports_exception_filter_options: other
                .supports_exception_filter_options
                .or(self.supports_exception_filter_options),
            supports_single_thread_execution_requests: other
                .supports_single_thread_execution_requests
                .or(self.supports_single_thread_execution_requests),
            supports_data_breakpoint_bytes: other
                .supports_data_breakpoint_bytes
                .or(self.supports_data_breakpoint_bytes),
            breakpoint_modes: other
                .breakpoint_modes
                .or_else(|| self.breakpoint_modes.clone()),
            supports_ansistyling: other.supports_ansistyling.or(self.supports_ansistyling),
        }
    }
}

impl Ord for StackFrame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for StackFrame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Scope {
    fn cmp(&self, other: &Self) -> Ordering {
        self.variables_reference.cmp(&other.variables_reference)
    }
}

impl PartialOrd for Scope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
