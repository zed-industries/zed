use std::{path::Path, sync::Arc};

use configuration::ConfigurationContext;
use gpui::{App, Context, Entity, EventEmitter, Task, WeakEntity};
use language::{ContextProvider as _, LanguageToolchainStore, Location};
use rpc::AnyProtoClient;
use settings::{InvalidSettingsError, SettingsLocation};
use task::TaskVariables;
use util::rel_path::RelPath;

use crate::{
    ConfigurationInventory, ProjectEnvironment, buffer_store::BufferStore,
    task_store::TaskStore, worktree_store::WorktreeStore,
};

pub enum ConfigurationStore {
    Functional(ConfigStoreState),
    Noop,
}

pub struct ConfigStoreState {
    mode: ConfigStoreMode,
    configuration_inventory: Entity<ConfigurationInventory>,
    buffer_store: WeakEntity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    toolchain_store: std::sync::Arc<dyn LanguageToolchainStore>,
    task_store: WeakEntity<TaskStore>,
}

enum ConfigStoreMode {
    Local {
        downstream_client: Option<(AnyProtoClient, u64)>,
        environment: Entity<ProjectEnvironment>,
    },
    Remote {
        upstream_client: AnyProtoClient,
        project_id: u64,
    },
}

impl EventEmitter<crate::Event> for ConfigurationStore {}

#[derive(Debug)]
pub enum ConfigurationSettingsLocation<'a> {
    Global(&'a Path),
    Worktree(SettingsLocation<'a>),
}

impl ConfigurationStore {
    pub fn local(
        buffer_store: WeakEntity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        toolchain_store: std::sync::Arc<dyn LanguageToolchainStore>,
        task_store: WeakEntity<TaskStore>,
        environment: Entity<ProjectEnvironment>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::Functional(ConfigStoreState {
            mode: ConfigStoreMode::Local {
                downstream_client: None,
                environment,
            },
            configuration_inventory: ConfigurationInventory::new(cx),
            buffer_store,
            toolchain_store,
            worktree_store,
            task_store,
        })
    }

    pub fn remote(
        buffer_store: WeakEntity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        toolchain_store: std::sync::Arc<dyn LanguageToolchainStore>,
        task_store: WeakEntity<TaskStore>,
        upstream_client: AnyProtoClient,
        project_id: u64,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::Functional(ConfigStoreState {
            mode: ConfigStoreMode::Remote {
                upstream_client,
                project_id,
            },
            configuration_inventory: ConfigurationInventory::new(cx),
            buffer_store,
            toolchain_store,
            worktree_store,
            task_store,
        })
    }

    pub fn configuration_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut App,
    ) -> Task<Option<ConfigurationContext>> {
        match self {
            ConfigurationStore::Functional(state) => {
                let task_store = state.task_store.clone();
                cx.spawn(async move |cx| {
                    let task_store = task_store.upgrade()?;
                    let task_context = task_store
                        .update(cx, |store, cx| {
                            store.task_context_for_location(captured_variables, location, cx)
                        })
                        .ok()?
                        .await?;
                    Some(ConfigurationContext::from(task_context))
                })
            }
            ConfigurationStore::Noop => Task::ready(None),
        }
    }

    pub fn configuration_inventory(&self) -> Option<&Entity<ConfigurationInventory>> {
        match self {
            ConfigurationStore::Functional(state) => Some(&state.configuration_inventory),
            ConfigurationStore::Noop => None,
        }
    }

    pub fn shared(&mut self, remote_id: u64, new_downstream_client: AnyProtoClient, _cx: &mut App) {
        if let Self::Functional(ConfigStoreState {
            mode: ConfigStoreMode::Local {
                downstream_client, ..
            },
            ..
        }) = self
        {
            *downstream_client = Some((new_downstream_client, remote_id));
        }
    }

    pub fn unshared(&mut self, _: &mut Context<Self>) {
        if let Self::Functional(ConfigStoreState {
            mode: ConfigStoreMode::Local {
                downstream_client, ..
            },
            ..
        }) = self
        {
            *downstream_client = None;
        }
    }

    pub(super) fn update_user_configurations(
        &self,
        location: ConfigurationSettingsLocation<'_>,
        raw_configurations_json: Option<&str>,
        cx: &mut Context<Self>,
    ) -> Result<(), InvalidSettingsError> {
        log::info!("update_user_configurations called with location: {location:?}");
        let configuration_inventory = match self {
            ConfigurationStore::Functional(state) => &state.configuration_inventory,
            ConfigurationStore::Noop => {
                log::warn!("ConfigurationStore is Noop, skipping update");
                return Ok(());
            }
        };
        let raw_configurations_json = raw_configurations_json
            .map(|json| json.trim())
            .filter(|json| !json.is_empty());
        
        if let Some(json) = raw_configurations_json {
            log::info!("Received configuration JSON ({} bytes)", json.len());
        } else {
            log::info!("Received empty/null configuration, clearing configurations");
        }

        configuration_inventory.update(cx, |inventory, _| {
            use crate::configuration_inventory::{
                ConfigurationSettingsLocation as InvLocation, parse_configuration_file,
            };
            
            let parsed_configurations = if let Some(json_str) = raw_configurations_json {
                log::info!("Parsing configuration JSON...");
                let configs = parse_configuration_file(json_str.to_string())?;
                log::info!("Successfully parsed {} configurations", configs.len());
                for (i, config) in configs.iter().enumerate() {
                    log::info!("  Config {}: label='{}', type={:?}", i + 1, config.label, config.config_type);
                }
                configs
            } else {
                log::info!("No JSON content, clearing configurations");
                Vec::new()
            };

            let inv_location = match location {
                ConfigurationSettingsLocation::Global(path) => {
                    log::info!("Updating global configurations at {:?}", path);
                    InvLocation::Global(path.to_path_buf())
                }
                ConfigurationSettingsLocation::Worktree(settings_location) => {
                    log::info!("Updating worktree configurations: worktree_id={:?}, path={:?}", 
                              settings_location.worktree_id, settings_location.path);
                    InvLocation::Worktree {
                        worktree_id: settings_location.worktree_id,
                        directory_in_worktree: Arc::from(settings_location.path.as_ref()),
                    }
                }
            };

            inventory.update_configurations(inv_location, parsed_configurations);
            log::info!("Configuration inventory updated successfully");
            Ok(())
        })?;
        
        // Explicitly notify observers that the inventory has changed
        configuration_inventory.update(cx, |_, cx| {
            cx.notify();
        });
        
        Ok(())
    }
}
