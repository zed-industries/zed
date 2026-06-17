mod audio_input_output_setup;
mod audio_test_window;
mod edit_prediction_provider_setup;
mod external_agents_page;
mod feature_flags;
mod llm_providers_page;
mod mcp_servers_page;
mod skill_creator;
mod skills_setup;
mod tool_permissions_setup;

pub(crate) use audio_input_output_setup::{
    render_input_audio_device_dropdown, render_output_audio_device_dropdown,
};
pub(crate) use audio_test_window::open_audio_test_window;
pub(crate) use edit_prediction_provider_setup::render_edit_prediction_setup_page;
pub(crate) use external_agents_page::{CustomAgentForm, render_external_agents_page};
pub(crate) use feature_flags::render_feature_flags_page;
pub(crate) use llm_providers_page::render_llm_providers_page;
pub(crate) use mcp_servers_page::{McpServerForm, render_mcp_servers_page};
pub use skill_creator::SkillCreatorOpenMode;
pub(crate) use skill_creator::{
    SkillCreatorEvent, SkillCreatorPage, render_skill_creator_page, skill_url_from_clipboard,
};
#[cfg(test)]
pub(crate) use skills_setup::displayed_skills;
pub(crate) use skills_setup::render_skills_setup_page;
pub(crate) use tool_permissions_setup::render_tool_permissions_setup_page;

pub use tool_permissions_setup::{
    render_copy_path_tool_config, render_create_directory_tool_config,
    render_delete_path_tool_config, render_edit_file_tool_config, render_fetch_tool_config,
    render_move_path_tool_config, render_terminal_tool_config, render_web_search_tool_config,
    render_write_file_tool_config,
};
