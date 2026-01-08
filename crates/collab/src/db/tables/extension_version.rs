use crate::db::ExtensionId;
use collections::BTreeSet;
use rpc::ExtensionProvides;
use sea_orm::entity::prelude::*;
use time::PrimitiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "extension_versions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub extension_id: ExtensionId,
    #[sea_orm(primary_key)]
    pub version: String,
    pub published_at: PrimitiveDateTime,
    pub authors: String,
    pub repository: String,
    pub description: String,
    pub schema_version: i32,
    pub wasm_api_version: Option<String>,
    pub download_count: i64,
    pub provides_themes: bool,
    pub provides_icon_themes: bool,
    pub provides_languages: bool,
    pub provides_grammars: bool,
    pub provides_language_servers: bool,
    pub provides_context_servers: bool,
    pub provides_agent_servers: bool,
    pub provides_slash_commands: bool,
    pub provides_indexed_docs_providers: bool,
    pub provides_snippets: bool,
    pub provides_debug_adapters: bool,
}

impl Model {
    pub fn provides(&self) -> BTreeSet<ExtensionProvides> {
        let mut provides = BTreeSet::default();
        if self.provides_themes {
            provides.insert(ExtensionProvides::Themes);
        }

        if self.provides_icon_themes {
            provides.insert(ExtensionProvides::IconThemes);
        }

        if self.provides_languages {
            provides.insert(ExtensionProvides::Languages);
        }

        if self.provides_grammars {
            provides.insert(ExtensionProvides::Grammars);
        }

        if self.provides_language_servers {
            provides.insert(ExtensionProvides::LanguageServers);
        }

        if self.provides_context_servers {
            provides.insert(ExtensionProvides::ContextServers);
        }

        if self.provides_agent_servers {
            provides.insert(ExtensionProvides::AgentServers);
        }

        if self.provides_slash_commands {
            provides.insert(ExtensionProvides::SlashCommands);
        }

        if self.provides_indexed_docs_providers {
            provides.insert(ExtensionProvides::IndexedDocsProviders);
        }

        if self.provides_snippets {
            provides.insert(ExtensionProvides::Snippets);
        }

        if self.provides_debug_adapters {
            provides.insert(ExtensionProvides::DebugAdapters);
        }

        provides
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::extension::Entity",
        from = "Column::ExtensionId",
        to = "super::extension::Column::Id"
        on_condition = r#"super::extension::Column::LatestVersion.into_expr().eq(Column::Version.into_expr())"#
    )]
    Extension,
}

impl Related<super::extension::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Extension.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
