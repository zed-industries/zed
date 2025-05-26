use crate::{AllLanguageModelSettings, settings::LmStudioSettingsContent};
use super::{
    types::{LmStudioServer, AvailableModel},
};

use collections::{BTreeMap, HashSet};
use gpui::{
    div, Context, Entity, IntoElement, Render, Task, Window
};
use lmstudio;
use settings::{Settings, update_settings_file};
use ui::{
    prelude::*, Button, ButtonStyle, IconButton,
    IconName, Label, LabelCommon,
    LabelSize, List, Switch, ToggleState, SwitchColor, Indicator
};
use ui_input::SingleLineInput;
use util::ResultExt;
use uuid;
use workspace::AppState;

pub struct ConfigurationView {
    state: Entity<super::provider::State>,
    loading_models_task: Option<Task<()>>,
    editing_server_index: Option<usize>,
    server_edit_name: String,
    server_edit_url: String,
    is_adding_model: bool,
    new_model_name: String,
    new_model_display_name: String,
    new_model_max_tokens: String,
    is_editing_max_tokens: bool,
    editing_model_server_id: Option<String>,
    editing_model_name: Option<String>,
    edit_max_tokens_value: String,
    is_adding_server: bool,
    new_server_name: String,
    new_server_url: String,
    server_edit_name_input: Option<Entity<SingleLineInput>>,
    server_edit_url_input: Option<Entity<SingleLineInput>>,
    new_model_name_input: Option<Entity<SingleLineInput>>,
    new_model_display_name_input: Option<Entity<SingleLineInput>>,
    new_model_max_tokens_input: Option<Entity<SingleLineInput>>,
    new_server_name_input: Option<Entity<SingleLineInput>>,
    new_server_url_input: Option<Entity<SingleLineInput>>,
    edit_max_tokens_input: Option<Entity<SingleLineInput>>,
    server_connection_status: BTreeMap<String, bool>,
    connection_check_tasks: BTreeMap<String, Task<anyhow::Result<bool>>>,
    expanded_server_models: HashSet<String>,
}

impl ConfigurationView {
    pub fn new(state: Entity<super::provider::State>, cx: &mut Context<Self>) -> Self {
        let loading_models_task = Some(cx.spawn({
            let state = state.clone();
            async move |this: gpui::WeakEntity<Self>, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    task.await.log_err();
                }
                this.update(cx, |this, cx| {
                    this.loading_models_task = None;
                    // Start connection checks for all servers
                    this.check_all_server_connections(cx);
                    cx.notify();
                })
                .log_err();
            }
        }));

        // Initialize server connection status map based on available models
        let mut server_connection_status = BTreeMap::new();
        let settings = AllLanguageModelSettings::get_global(cx);
        
        // Get available models from state to check which servers are already connected
        let available_models = state.read(cx).available_models.clone();
        
        // Track which servers have models in the state (indicating they were connected)
        let servers_with_models: HashSet<String> = available_models.iter()
            .filter_map(|model| model.server_id.clone())
            .collect();
        
        // Pre-populate all server IDs in the status map
        for server in &settings.lmstudio.servers {
            // Consider a server connected if it has models in the state
            let is_connected = servers_with_models.contains(&server.id);
            server_connection_status.insert(server.id.clone(), is_connected);
        }

        Self {
            state,
            loading_models_task,
            editing_server_index: None,
            server_edit_name: String::new(),
            server_edit_url: String::new(),
            is_adding_model: false,
            new_model_name: String::new(),
            new_model_display_name: String::new(),
            new_model_max_tokens: String::new(),
            is_editing_max_tokens: false,
            editing_model_server_id: None,
            editing_model_name: None,
            edit_max_tokens_value: String::new(),
            is_adding_server: false,
            new_server_name: String::new(),
            new_server_url: String::new(),
            server_edit_name_input: None,
            server_edit_url_input: None,
            new_model_name_input: None,
            new_model_display_name_input: None,
            new_model_max_tokens_input: None,
            new_server_name_input: None,
            new_server_url_input: None,
            edit_max_tokens_input: None,
            server_connection_status,
            connection_check_tasks: BTreeMap::new(),
            expanded_server_models: HashSet::default(),
        }
    }

    // Helper methods for text input creation and updates
    fn create_server_edit_inputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Create server edit name input if it doesn't exist
        if self.server_edit_name_input.is_none() {
            let name_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Server name");
                if !self.server_edit_name.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.server_edit_name.clone(), window, cx);
                    });
                }
                input
            });
            self.server_edit_name_input = Some(name_input);
        }

        // Create server edit URL input if it doesn't exist
        if self.server_edit_url_input.is_none() {
            let url_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Server URL");
                if !self.server_edit_url.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.server_edit_url.clone(), window, cx);
                    });
                }
                input
            });
            self.server_edit_url_input = Some(url_input);
        }
    }

    fn create_new_server_inputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Create new server name input if it doesn't exist
        if self.new_server_name_input.is_none() {
            let name_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Server name");
                if !self.new_server_name.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_server_name.clone(), window, cx);
                    });
                }
                input
            });
            self.new_server_name_input = Some(name_input);
        }

        // Create new server URL input if it doesn't exist
        if self.new_server_url_input.is_none() {
            let url_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Server URL");
                if !self.new_server_url.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_server_url.clone(), window, cx);
                    });
                }
                input
            });
            self.new_server_url_input = Some(url_input);
        }
    }

    fn create_model_inputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Create model name input if it doesn't exist
        if self.new_model_name_input.is_none() {
            let name_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Model name");
                if !self.new_model_name.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_model_name.clone(), window, cx);
                    });
                }
                input
            });
            self.new_model_name_input = Some(name_input);
        }

        // Create model display name input if it doesn't exist
        if self.new_model_display_name_input.is_none() {
            let display_name_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Display name (optional)");
                if !self.new_model_display_name.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_model_display_name.clone(), window, cx);
                    });
                }
                input
            });
            self.new_model_display_name_input = Some(display_name_input);
        }

        // Create model max tokens input if it doesn't exist
        if self.new_model_max_tokens_input.is_none() {
            let max_tokens_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Max tokens");
                if !self.new_model_max_tokens.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.new_model_max_tokens.clone(), window, cx);
                    });
                } else {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text("8192".to_string(), window, cx);
                    });
                }
                input
            });
            self.new_model_max_tokens_input = Some(max_tokens_input);
        }
    }

    fn create_max_tokens_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.edit_max_tokens_input.is_none() {
            let tokens_input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "Max tokens");
                if !self.edit_max_tokens_value.is_empty() {
                    input.editor.update(cx, |editor, cx| {
                        editor.set_text(self.edit_max_tokens_value.clone(), window, cx);
                    });
                }
                input
            });
            self.edit_max_tokens_input = Some(tokens_input);
        }
    }

    fn update_field_from_input(&mut self, cx: &mut Context<Self>) {
        // Update server edit fields
        if let Some(name_input) = &self.server_edit_name_input {
            self.server_edit_name = name_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        if let Some(url_input) = &self.server_edit_url_input {
            self.server_edit_url = url_input.read(cx).editor.read(cx).text(cx).to_string();
        }

        // Update new server fields
        if let Some(name_input) = &self.new_server_name_input {
            self.new_server_name = name_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        if let Some(url_input) = &self.new_server_url_input {
            self.new_server_url = url_input.read(cx).editor.read(cx).text(cx).to_string();
        }

        // Update model fields
        if let Some(name_input) = &self.new_model_name_input {
            self.new_model_name = name_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        if let Some(display_name_input) = &self.new_model_display_name_input {
            self.new_model_display_name = display_name_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        if let Some(max_tokens_input) = &self.new_model_max_tokens_input {
            self.new_model_max_tokens = max_tokens_input.read(cx).editor.read(cx).text(cx).to_string();
        }
        
        // Update max tokens edit field
        if let Some(max_tokens_input) = &self.edit_max_tokens_input {
            self.edit_max_tokens_value = max_tokens_input.read(cx).editor.read(cx).text(cx).to_string();
        }
    }

    fn check_server_connection(&mut self, server_id: String, server_url: String, cx: &mut Context<Self>) {
        // Skip if there's already a task running for this server
        if self.connection_check_tasks.contains_key(&server_id) {
            return;
        }

        let http_client = self.state.read(cx).http_client.clone();
        let server_id_clone = server_id.clone();
        let task = cx.spawn(async move |this: gpui::WeakEntity<Self>, cx| {
            let result = lmstudio::healthcheck(&*http_client, &server_url).await;
            let is_healthy = result.unwrap_or(false);
            this.update(cx, |this, cx| {
                this.server_connection_status.insert(server_id_clone.clone(), is_healthy);
                this.connection_check_tasks.remove(&server_id_clone);
                cx.notify();
            })?;
            Ok(is_healthy)
        });

        self.connection_check_tasks.insert(server_id, task);
    }

    fn check_all_server_connections(&mut self, cx: &mut Context<Self>) {
        let settings = AllLanguageModelSettings::get_global(cx);
        let servers_to_check: Vec<(String, String)> = settings.lmstudio.servers.iter()
            .filter(|server| server.enabled)
            .map(|server| (server.id.clone(), server.api_url.clone()))
            .collect();
        
        for (server_id, server_url) in servers_to_check {
            self.check_server_connection(server_id, server_url, cx);
        }
    }

    fn add_server(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.update_field_from_input(cx);

        // Validate inputs
        if self.new_server_name.is_empty() {
            anyhow::bail!("Server name cannot be empty");
        }
        if self.new_server_url.is_empty() {
            anyhow::bail!("Server URL cannot be empty");
        }

        // Create new server
        let new_server = LmStudioServer {
            id: format!("server_{}", uuid::Uuid::new_v4()),
            name: self.new_server_name.clone(),
            api_url: self.new_server_url.clone(),
            enabled: true,
            available_models: None,
        };

        // Clone values before moving into closure
        let new_server_clone = new_server.clone();
        let server_id = new_server.id.clone();
        let server_url = new_server.api_url.clone();

        // Update settings
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if settings.lmstudio.is_none() {
                settings.lmstudio = Some(LmStudioSettingsContent::default());
            }
            if let Some(lmstudio) = &mut settings.lmstudio {
                if lmstudio.servers.is_none() {
                    lmstudio.servers = Some(Vec::new());
                }
                if let Some(servers) = &mut lmstudio.servers {
                    servers.push(new_server_clone);
                }
            }
        });

        // Clear inputs
        self.new_server_name.clear();
        self.new_server_url.clear();
        self.new_server_name_input = None;
        self.new_server_url_input = None;
        self.is_adding_server = false;

        // Start connection check for new server
        self.check_server_connection(server_id, server_url, cx);

        // Refresh models
        self.state.update(cx, |state, cx| state.public_restart_fetch_models_task(cx));

        Ok(())
    }

    fn edit_server(&mut self, server_index: usize, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.update_field_from_input(cx);

        // Validate inputs
        if self.server_edit_name.is_empty() {
            anyhow::bail!("Server name cannot be empty");
        }
        if self.server_edit_url.is_empty() {
            anyhow::bail!("Server URL cannot be empty");
        }

        // Clone values before moving into closure
        let edit_name = self.server_edit_name.clone();
        let edit_url = self.server_edit_url.clone();

        // Update settings
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if let Some(server) = servers.get_mut(server_index) {
                        server.name = edit_name;
                        server.api_url = edit_url;
                    }
                }
            }
        });

        // Start connection check for updated server
        let settings = AllLanguageModelSettings::get_global(cx);
        if let Some(server) = settings.lmstudio.servers.get(server_index) {
            let server_id = server.id.clone();
            let server_url = server.api_url.clone();
            cx.spawn(async move |this: gpui::WeakEntity<Self>, cx| {
                this.update(cx, |this, cx| {
                    this.check_server_connection(server_id, server_url, cx);
                })?;
                Ok::<(), anyhow::Error>(())
            })
            .detach();
        }

        // Clear edit state
        self.server_edit_name.clear();
        self.server_edit_url.clear();
        self.server_edit_name_input = None;
        self.server_edit_url_input = None;
        self.editing_server_index = None;

        // Refresh models
        self.state.update(cx, |state, cx| state.public_restart_fetch_models_task(cx));

        Ok(())
    }

    fn remove_server(&mut self, server_index: usize, cx: &mut Context<Self>) {
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if server_index < servers.len() {
                        servers.remove(server_index);
                    }
                }
            }
        });

        // Refresh models
        self.state.update(cx, |state, cx| state.public_restart_fetch_models_task(cx));
    }

    fn toggle_server(&mut self, server_index: usize, cx: &mut Context<Self>) {
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if let Some(server) = servers.get_mut(server_index) {
                        server.enabled = !server.enabled;
                    }
                }
            }
        });

        // Get the updated server info for connection check
        let settings = AllLanguageModelSettings::get_global(cx);
        if let Some(server) = settings.lmstudio.servers.get(server_index) {
            if server.enabled {
                let server_id = server.id.clone();
                let server_url = server.api_url.clone();
                cx.spawn(async move |this: gpui::WeakEntity<Self>, cx| {
                    this.update(cx, |this, cx| {
                        this.check_server_connection(server_id, server_url, cx);
                    })?;
                    Ok::<(), anyhow::Error>(())
                })
                .detach();
            } else {
                // If disabled, remove connection status and cancel any pending tasks
                self.server_connection_status.remove(&server.id);
                if let Some(task) = self.connection_check_tasks.remove(&server.id) {
                    drop(task);
                }
            }
        }

        // Refresh models
        self.state.update(cx, |state, cx| state.public_restart_fetch_models_task(cx));
    }

    fn toggle_server_models(&mut self, server_id: &str, cx: &mut Context<Self>) {
        if self.expanded_server_models.contains(server_id) {
            self.expanded_server_models.remove(server_id);
        } else {
            self.expanded_server_models.insert(server_id.to_string());
        }
        cx.notify();
    }

    fn add_model(&mut self, server_id: &str, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.update_field_from_input(cx);

        // Validate inputs
        if self.new_model_name.is_empty() {
            anyhow::bail!("Model name cannot be empty");
        }

        // Parse max tokens
        let max_tokens = if !self.new_model_max_tokens.is_empty() {
            match self.new_model_max_tokens.parse::<usize>() {
                Ok(tokens) => Some(tokens),
                Err(_) => anyhow::bail!("Invalid max tokens value"),
            }
        } else {
            None
        };

        // Create new model
        let new_model = AvailableModel::new(
            self.new_model_name.clone(),
            if self.new_model_display_name.is_empty() {
                None
            } else {
                Some(self.new_model_display_name.clone())
            },
            max_tokens.unwrap_or(8192),
            None,
            Some(server_id.to_string()),
            true,
        );

        // Clone server_id for the closure
        let server_id_owned = server_id.to_string();

        // Update settings
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if let Some(server) = servers.iter_mut().find(|s| s.id == server_id_owned) {
                        if server.available_models.is_none() {
                            server.available_models = Some(Vec::new());
                        }
                        
                        if let Some(models) = &mut server.available_models {
                            models.push(new_model);
                        }
                    }
                }
            }
        });

        // Clear inputs
        self.new_model_name.clear();
        self.new_model_display_name.clear();
        self.new_model_max_tokens.clear();
        self.new_model_name_input = None;
        self.new_model_display_name_input = None;
        self.new_model_max_tokens_input = None;
        self.is_adding_model = false;

        // Refresh models
        self.state.update(cx, |state, cx| state.public_restart_fetch_models_task(cx));

        Ok(())
    }

    fn remove_model(&mut self, server_id: &str, model_name: &str, cx: &mut Context<Self>) {
        let server_id_owned = server_id.to_string();
        let model_name_owned = model_name.to_string();
        
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if let Some(server) = servers.iter_mut().find(|s| s.id == server_id_owned) {
                        if let Some(models) = &mut server.available_models {
                            models.retain(|m| m.name != model_name_owned);
                        }
                    }
                }
            }
        });

        // Refresh models
        self.state.update(cx, |state, cx| state.public_restart_fetch_models_task(cx));
    }

    fn toggle_model(&mut self, server_id: &str, model_name: &str, cx: &mut Context<Self>) {
        let server_id_owned = server_id.to_string();
        let model_name_owned = model_name.to_string();
        
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if let Some(server) = servers.iter_mut().find(|s| s.id == server_id_owned) {
                        if let Some(models) = &mut server.available_models {
                            if let Some(model) = models.iter_mut().find(|m| m.name == model_name_owned) {
                                model.enabled = !model.enabled;
                            }
                        }
                    }
                }
            }
        });

        // Refresh models
        self.state.update(cx, |state, cx| state.public_restart_fetch_models_task(cx));
    }

    fn edit_model_max_tokens(&mut self, server_id: &str, model_name: &str, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.update_field_from_input(cx);

        // Parse max tokens
        let max_tokens = if !self.edit_max_tokens_value.is_empty() {
            match self.edit_max_tokens_value.parse::<usize>() {
                Ok(tokens) => Some(tokens),
                Err(_) => anyhow::bail!("Invalid max tokens value"),
            }
        } else {
            None
        };

        let server_id_owned = server_id.to_string();
        let model_name_owned = model_name.to_string();

        // Update settings
        let fs = AppState::global(cx).upgrade().unwrap().fs.clone();
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
            if let Some(lmstudio) = &mut settings.lmstudio {
                if let Some(servers) = &mut lmstudio.servers {
                    if let Some(server) = servers.iter_mut().find(|s| s.id == server_id_owned) {
                        if let Some(models) = &mut server.available_models {
                            if let Some(model) = models.iter_mut().find(|m| m.name == model_name_owned) {
                                model.custom_max_tokens = max_tokens;
                            }
                        }
                    }
                }
            }
        });

        // Clear edit state
        self.edit_max_tokens_value.clear();
        self.edit_max_tokens_input = None;
        self.is_editing_max_tokens = false;
        self.editing_model_server_id = None;
        self.editing_model_name = None;

        // Refresh models
        self.state.update(cx, |state, cx| state.public_restart_fetch_models_task(cx));

        Ok(())
    }

    fn start_edit_model_max_tokens(&mut self, server_id: &str, model_name: &str, current_value: Option<usize>, _cx: &mut Context<Self>) {
        self.is_editing_max_tokens = true;
        self.editing_model_server_id = Some(server_id.to_string());
        self.editing_model_name = Some(model_name.to_string());
        self.edit_max_tokens_value = current_value.map_or_else(String::new, |v| v.to_string());
    }

    #[allow(dead_code)]
    fn start_add_model(&mut self, _cx: &mut Context<Self>) {
        self.is_adding_model = true;
        self.new_model_name.clear();
        self.new_model_display_name.clear();
        self.new_model_max_tokens.clear();
        self.new_model_name_input = None;
        self.new_model_display_name_input = None;
        self.new_model_max_tokens_input = None;
    }

    fn start_edit_server(&mut self, server_index: usize, cx: &mut Context<Self>) {
        let settings = AllLanguageModelSettings::get_global(cx);
        if let Some(server) = settings.lmstudio.servers.get(server_index) {
            self.editing_server_index = Some(server_index);
            self.server_edit_name = server.name.clone();
            self.server_edit_url = server.api_url.clone();
            self.server_edit_name_input = None;
            self.server_edit_url_input = None;
        }
    }

    #[allow(dead_code)]
    fn start_add_server(&mut self, _cx: &mut Context<Self>) {
        self.is_adding_server = true;
        self.new_server_name.clear();
        self.new_server_url.clear();
        self.new_server_name_input = None;
        self.new_server_url_input = None;
    }

    fn cancel_edit_server(&mut self, _cx: &mut Context<Self>) {
        self.editing_server_index = None;
        self.server_edit_name.clear();
        self.server_edit_url.clear();
        self.server_edit_name_input = None;
        self.server_edit_url_input = None;
    }

    fn cancel_add_server(&mut self, _cx: &mut Context<Self>) {
        self.is_adding_server = false;
        self.new_server_name.clear();
        self.new_server_url.clear();
        self.new_server_name_input = None;
        self.new_server_url_input = None;
    }

    fn cancel_add_model(&mut self, _cx: &mut Context<Self>) {
        self.is_adding_model = false;
        self.new_model_name.clear();
        self.new_model_display_name.clear();
        self.new_model_max_tokens.clear();
        self.new_model_name_input = None;
        self.new_model_display_name_input = None;
        self.new_model_max_tokens_input = None;
    }

    fn cancel_edit_max_tokens(&mut self, _cx: &mut Context<Self>) {
        self.is_editing_max_tokens = false;
        self.editing_model_server_id = None;
        self.editing_model_name = None;
        self.edit_max_tokens_value.clear();
        self.edit_max_tokens_input = None;
    }

    fn render_server_list(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AllLanguageModelSettings::get_global(cx);
        let servers = &settings.lmstudio.servers;

        // Clone server data to avoid borrowing issues
        let server_data: Vec<(usize, LmStudioServer, bool, bool, String)> = servers.iter().enumerate()
            .map(|(index, server)| {
                let is_editing = self.editing_server_index == Some(index);
                let is_connected = self.server_connection_status.get(&server.id).copied().unwrap_or(false);
                let server_id = server.id.clone();
                (index, server.clone(), is_editing, is_connected, server_id)
            })
            .collect();

        List::new()
            .children(
                server_data.into_iter().map(|(index, server, is_editing, is_connected, server_id)| {
                    if is_editing {
                        self.create_server_edit_inputs(window, cx);
                        
                        div()
                            .flex()
                            .flex_row()
                            .gap_1()
                            .child(Label::new("Edit Server").size(LabelSize::Default))
                            .child(div().flex_1())
                            .child(
                                IconButton::new("close", IconName::Close)
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.cancel_edit_server(cx);
                                    }))
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap_1()
                                    .child(self.server_edit_name_input.as_ref().unwrap().clone())
                                    .child(self.server_edit_url_input.as_ref().unwrap().clone())
                                    .child(
                                        Button::new("save", "Save")
                                            .style(ButtonStyle::Filled)
                                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                                if let Err(e) = this.edit_server(index, cx) {
                                                    log::error!("Failed to edit server: {}", e);
                                                }
                                            }))
                                    )
                            )
                    } else {
                        div()
                            .flex()
                            .flex_row()
                            .gap_1()
                            .child(
                                Switch::new(gpui::SharedString::from(server_id.clone()), ToggleState::Selected)
                                    .color(SwitchColor::Default)
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.toggle_server(index, cx);
                                    }))
                            )
                            .child(Label::new(&server.name).size(LabelSize::Default))
                            .child(Label::new(&server.api_url).size(LabelSize::Small))
                            .child(div().flex_1())
                            .child(
                                if server.enabled {
                                    if is_connected {
                                        Indicator::dot()
                                    } else {
                                        Indicator::bar()
                                    }
                                } else {
                                    Indicator::dot()
                                }
                            )
                            .child(
                                IconButton::new("expand", IconName::ChevronDown)
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.toggle_server_models(&server_id, cx);
                                    }))
                            )
                            .child(
                                IconButton::new("edit", IconName::Pencil)
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.start_edit_server(index, cx);
                                    }))
                            )
                            .child(
                                IconButton::new("remove", IconName::Trash)
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.remove_server(index, cx);
                                    }))
                            )
                    }
                })
            )
    }

    fn render_model_list(&mut self, server: &LmStudioServer, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let models = server.available_models.as_ref().unwrap_or(&Vec::new()).clone();
        let server_id = server.id.clone();

        // Collect model data to avoid borrowing issues
        let model_data: Vec<(AvailableModel, String, bool)> = models.into_iter()
            .map(|model| {
                let is_editing_max_tokens = self.is_editing_max_tokens &&
                    self.editing_model_server_id.as_deref() == Some(&server_id) &&
                    self.editing_model_name.as_deref() == Some(&model.name);
                let model_switch_id = format!("{}_{}", server_id, model.name);
                (model, model_switch_id, is_editing_max_tokens)
            })
            .collect();

        List::new()
            .children(
                model_data.into_iter().map(|(model, model_switch_id, is_editing_max_tokens)| {
                    if is_editing_max_tokens {
                        self.create_max_tokens_input(window, cx);
                        
                        h_flex()
                            .gap_1()
                            .child(Label::new(&model.name).size(LabelSize::Default))
                            .child(self.edit_max_tokens_input.as_ref().unwrap().clone())
                            .child(
                                Button::new("save", "Save")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener({
                                        let server_id = server_id.clone();
                                        let model_name = model.name.clone();
                                        move |this, _event, _window, cx| {
                                            if let Err(e) = this.edit_model_max_tokens(&server_id, &model_name, cx) {
                                                log::error!("Failed to edit max tokens: {}", e);
                                            }
                                        }
                                    }))
                            )
                            .child(
                                IconButton::new("cancel", IconName::Close)
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.cancel_edit_max_tokens(cx);
                                    }))
                            )
                    } else {
                        let model_name_clone = model.name.clone();
                        let server_id_clone = server_id.clone();
                        
                        h_flex()
                            .gap_1()
                            .child(
                                Switch::new(gpui::SharedString::from(model_switch_id.clone()), ToggleState::Selected)
                                    .color(SwitchColor::Default)
                                    .on_click(cx.listener({
                                        let server_id_clone = server_id_clone.clone();
                                        let model_name_clone = model_name_clone.clone();
                                        move |this, _event, _window, cx| {
                                            this.toggle_model(&server_id_clone, &model_name_clone, cx);
                                        }
                                    }))
                            )
                            .child(Label::new(&model.name).size(LabelSize::Default))
                            .child(
                                if let Some(display_name) = &model.display_name {
                                    Label::new(display_name).size(LabelSize::Small)
                                } else {
                                    Label::new("").size(LabelSize::Small)
                                }
                            )
                            .child(div().flex_1())
                            .child(
                                Label::new(format!("{} tokens", model.server_max_tokens))
                                    .size(LabelSize::Small)
                            )
                            .child(
                                IconButton::new("edit-tokens", IconName::Pencil)
                                    .on_click(cx.listener({
                                        let server_id_clone = server_id_clone.clone();
                                        let model_name_clone = model_name_clone.clone();
                                        let custom_max_tokens = model.custom_max_tokens;
                                        move |this, _event, _window, cx| {
                                            this.start_edit_model_max_tokens(
                                                &server_id_clone,
                                                &model_name_clone,
                                                custom_max_tokens,
                                                cx,
                                            );
                                        }
                                    }))
                            )
                            .child(
                                IconButton::new("remove", IconName::Trash)
                                    .on_click(cx.listener({
                                        let server_id_clone = server_id_clone.clone();
                                        let model_name_clone = model_name_clone.clone();
                                        move |this, _event, _window, cx| {
                                            this.remove_model(&server_id_clone, &model_name_clone, cx);
                                        }
                                    }))
                            )
                    }
                })
            )
    }

    fn render_add_server(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.create_new_server_inputs(window, cx);

        v_flex()
            .gap_1()
            .child(
                h_flex()
                    .gap_1()
                    .child(Label::new("Add Server").size(LabelSize::Default))
                    .child(div().flex_1())
                    .child(
                        IconButton::new("close", IconName::Close)
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                this.cancel_add_server(cx);
                            }))
                    )
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(self.new_server_name_input.as_ref().unwrap().clone())
                    .child(self.new_server_url_input.as_ref().unwrap().clone())
                    .child(
                        Button::new("add", "Add")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                if let Err(e) = this.add_server(cx) {
                                    log::error!("Failed to add server: {}", e);
                                }
                            }))
                    )
            )
    }

    fn render_add_model(&mut self, server_id: &str, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.create_model_inputs(window, cx);
        let server_id = server_id.to_string();

        v_flex()
            .gap_1()
            .child(
                h_flex()
                    .gap_1()
                    .child(Label::new("Add Model").size(LabelSize::Default))
                    .child(div().flex_1())
                    .child(
                        IconButton::new("close", IconName::Close)
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                this.cancel_add_model(cx);
                            }))
                    )
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(self.new_model_name_input.as_ref().unwrap().clone())
                    .child(self.new_model_display_name_input.as_ref().unwrap().clone())
                    .child(self.new_model_max_tokens_input.as_ref().unwrap().clone())
                    .child(
                        Button::new("add", "Add")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                if let Err(e) = this.add_model(&server_id, cx) {
                                    log::error!("Failed to add model: {}", e);
                                }
                            }))
                    )
            )
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AllLanguageModelSettings::get_global(cx);
        
        // Collect all needed data to avoid borrowing issues
        let expanded_server_ids: Vec<String> = settings.lmstudio.servers.iter()
            .filter(|server| self.expanded_server_models.contains(&server.id))
            .map(|server| server.id.clone())
            .collect();
        
        let servers_data: Vec<(String, String)> = settings.lmstudio.servers.iter()
            .filter(|server| expanded_server_ids.contains(&server.id))
            .map(|server| (server.id.clone(), server.name.clone()))
            .collect();

        div()
            .p_4()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                Label::new("LM Studio Configuration")
                                    .size(LabelSize::Large)
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("Configure LM Studio servers and models")
                            )
                    )
                    .child(self.render_server_list(window, cx))
                    .child(
                        if self.is_adding_server {
                            self.render_add_server(window, cx).into_any_element()
                        } else {
                            div().into_any_element()
                        }
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(Label::new("Expanded Models").size(LabelSize::Default))
                            .children(
                                servers_data.into_iter().map(|(server_id, server_name)| {
                                    // Get server data without borrowing cx
                                    let settings = AllLanguageModelSettings::get_global(cx);
                                    let server = settings.lmstudio.servers.iter()
                                        .find(|s| s.id == server_id)
                                        .cloned()
                                        .unwrap();
                                    
                                    let server_id_clone = server.id.clone();
                                    
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .child(Label::new(&server_name).size(LabelSize::Small))
                                        .child(self.render_model_list(&server, window, cx))
                                        .child(
                                            if self.is_adding_model {
                                                self.render_add_model(&server_id_clone, window, cx).into_any_element()
                                            } else {
                                                div().into_any_element()
                                            }
                                        )
                                })
                            )
                    )
            )
    }
} 