use crate::db::ProjectId;
use anyhow::Result;
use rpc::proto::SetDebugClientCapabilities;
use sea_orm::entity::prelude::*;

const SUPPORTS_LOADED_SOURCES_REQUEST_BIT: u32 = 0;
const SUPPORTS_MODULES_REQUEST_BIT: u32 = 1;
const SUPPORTS_RESTART_REQUEST_BIT: u32 = 2;
const SUPPORTS_SET_EXPRESSION_BIT: u32 = 3;
const SUPPORTS_SINGLE_THREAD_EXECUTION_REQUESTS_BIT: u32 = 4;
const SUPPORTS_STEP_BACK_BIT: u32 = 5;
const SUPPORTS_STEPPING_GRANULARITY_BIT: u32 = 6;
const SUPPORTS_TERMINATE_THREADS_REQUEST_BIT: u32 = 7;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "debug_clients")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(primary_key)]
    pub project_id: ProjectId,
    #[sea_orm(primary_key)]
    pub session_id: i64,
    #[sea_orm(column_type = "Integer")]
    pub capabilities: i32,
}

impl Model {
    pub fn capabilities(&self) -> SetDebugClientCapabilities {
        SetDebugClientCapabilities {
            session_id: self.session_id as u64,
            client_id: self.id as u64,
            project_id: ProjectId::to_proto(self.project_id),
            supports_loaded_sources_request: (self.capabilities
                & (1 << SUPPORTS_LOADED_SOURCES_REQUEST_BIT))
                != 0,
            supports_modules_request: (self.capabilities & (1 << SUPPORTS_MODULES_REQUEST_BIT))
                != 0,
            supports_restart_request: (self.capabilities & (1 << SUPPORTS_RESTART_REQUEST_BIT))
                != 0,
            supports_single_thread_execution_requests: (self.capabilities
                & (1 << SUPPORTS_SINGLE_THREAD_EXECUTION_REQUESTS_BIT))
                != 0,
            supports_set_expression: (self.capabilities & (1 << SUPPORTS_SET_EXPRESSION_BIT)) != 0,
            supports_step_back: (self.capabilities & (1 << SUPPORTS_STEP_BACK_BIT)) != 0,
            supports_stepping_granularity: (self.capabilities
                & (1 << SUPPORTS_STEPPING_GRANULARITY_BIT))
                != 0,
            supports_terminate_threads_request: (self.capabilities
                & (1 << SUPPORTS_TERMINATE_THREADS_REQUEST_BIT))
                != 0,
        }
    }

    pub fn set_capabilities(&mut self, capabilities: &SetDebugClientCapabilities) {
        let mut capabilities_bit_mask = 0i32;
        capabilities_bit_mask |= (capabilities.supports_loaded_sources_request as i32)
            << SUPPORTS_LOADED_SOURCES_REQUEST_BIT;
        capabilities_bit_mask |=
            (capabilities.supports_modules_request as i32) << SUPPORTS_MODULES_REQUEST_BIT;
        capabilities_bit_mask |=
            (capabilities.supports_restart_request as i32) << SUPPORTS_RESTART_REQUEST_BIT;
        capabilities_bit_mask |=
            (capabilities.supports_set_expression as i32) << SUPPORTS_SET_EXPRESSION_BIT;
        capabilities_bit_mask |= (capabilities.supports_single_thread_execution_requests as i32)
            << SUPPORTS_SINGLE_THREAD_EXECUTION_REQUESTS_BIT;
        capabilities_bit_mask |= (capabilities.supports_step_back as i32) << SUPPORTS_STEP_BACK_BIT;
        capabilities_bit_mask |= (capabilities.supports_stepping_granularity as i32)
            << SUPPORTS_STEPPING_GRANULARITY_BIT;
        capabilities_bit_mask |= (capabilities.supports_terminate_threads_request as i32)
            << SUPPORTS_TERMINATE_THREADS_REQUEST_BIT;

        self.capabilities = capabilities_bit_mask;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,
    #[sea_orm(has_many = "super::debug_panel_items::Entity")]
    DebugPanelItems,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::debug_panel_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DebugPanelItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
