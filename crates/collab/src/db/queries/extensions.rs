use std::str::FromStr;

use anyhow::Context;
use chrono::Utc;
use sea_orm::sea_query::IntoCondition;
use util::ResultExt;

use super::*;

impl Database {
    pub async fn get_extensions(
        &self,
        filter: Option<&str>,
        provides_filter: Option<&BTreeSet<ExtensionProvides>>,
        max_schema_version: i32,
        limit: usize,
    ) -> Result<Vec<ExtensionMetadata>> {
        self.transaction(|tx| async move {
            let mut condition = Condition::all()
                .add(
                    extension::Column::LatestVersion
                        .into_expr()
                        .eq(extension_version::Column::Version.into_expr()),
                )
                .add(extension_version::Column::SchemaVersion.lte(max_schema_version));
            if let Some(filter) = filter {
                let fuzzy_name_filter = Self::fuzzy_like_string(filter);
                condition = condition.add(Expr::cust_with_expr("name ILIKE $1", fuzzy_name_filter));
            }

            if let Some(provides_filter) = provides_filter {
                condition = apply_provides_filter(condition, provides_filter);
            }

            self.get_extensions_where(condition, Some(limit as u64), &tx)
                .await
        })
        .await
    }

    pub async fn get_extensions_by_ids(
        &self,
        ids: &[&str],
        constraints: Option<&ExtensionVersionConstraints>,
    ) -> Result<Vec<ExtensionMetadata>> {
        self.transaction(|tx| async move {
            let extensions = extension::Entity::find()
                .filter(extension::Column::ExternalId.is_in(ids.iter().copied()))
                .all(&*tx)
                .await?;

            let mut max_versions = self
                .get_latest_versions_for_extensions(&extensions, constraints, &tx)
                .await?;

            Ok(extensions
                .into_iter()
                .filter_map(|extension| {
                    let (version, _) = max_versions.remove(&extension.id)?;
                    Some(metadata_from_extension_and_version(extension, version))
                })
                .collect())
        })
        .await
    }

    async fn get_latest_versions_for_extensions(
        &self,
        extensions: &[extension::Model],
        constraints: Option<&ExtensionVersionConstraints>,
        tx: &DatabaseTransaction,
    ) -> Result<HashMap<ExtensionId, (extension_version::Model, SemanticVersion)>> {
        let mut versions = extension_version::Entity::find()
            .filter(
                extension_version::Column::ExtensionId
                    .is_in(extensions.iter().map(|extension| extension.id)),
            )
            .stream(tx)
            .await?;

        let mut max_versions =
            HashMap::<ExtensionId, (extension_version::Model, SemanticVersion)>::default();
        while let Some(version) = versions.next().await {
            let version = version?;
            let Some(extension_version) = SemanticVersion::from_str(&version.version).log_err()
            else {
                continue;
            };

            if let Some((_, max_extension_version)) = &max_versions.get(&version.extension_id)
                && max_extension_version > &extension_version
            {
                continue;
            }

            if let Some(constraints) = constraints {
                if !constraints
                    .schema_versions
                    .contains(&version.schema_version)
                {
                    continue;
                }

                if let Some(wasm_api_version) = version.wasm_api_version.as_ref() {
                    if let Some(version) = SemanticVersion::from_str(wasm_api_version).log_err() {
                        if !constraints.wasm_api_versions.contains(&version) {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
            }

            max_versions.insert(version.extension_id, (version, extension_version));
        }

        Ok(max_versions)
    }

    /// Returns all of the versions for the extension with the given ID.
    pub async fn get_extension_versions(
        &self,
        extension_id: &str,
    ) -> Result<Vec<ExtensionMetadata>> {
        self.transaction(|tx| async move {
            let condition = extension::Column::ExternalId
                .eq(extension_id)
                .into_condition();

            self.get_extensions_where(condition, None, &tx).await
        })
        .await
    }

    async fn get_extensions_where(
        &self,
        condition: Condition,
        limit: Option<u64>,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ExtensionMetadata>> {
        let extensions = extension::Entity::find()
            .inner_join(extension_version::Entity)
            .select_also(extension_version::Entity)
            .filter(condition)
            .order_by_desc(extension::Column::TotalDownloadCount)
            .order_by_asc(extension::Column::Name)
            .limit(limit)
            .all(tx)
            .await?;

        Ok(extensions
            .into_iter()
            .filter_map(|(extension, version)| {
                Some(metadata_from_extension_and_version(extension, version?))
            })
            .collect())
    }

    pub async fn get_extension(
        &self,
        extension_id: &str,
        constraints: Option<&ExtensionVersionConstraints>,
    ) -> Result<Option<ExtensionMetadata>> {
        self.transaction(|tx| async move {
            let extension = extension::Entity::find()
                .filter(extension::Column::ExternalId.eq(extension_id))
                .one(&*tx)
                .await?
                .with_context(|| format!("no such extension: {extension_id}"))?;

            let extensions = [extension];
            let mut versions = self
                .get_latest_versions_for_extensions(&extensions, constraints, &tx)
                .await?;
            let [extension] = extensions;

            Ok(versions.remove(&extension.id).map(|(max_version, _)| {
                metadata_from_extension_and_version(extension, max_version)
            }))
        })
        .await
    }

    pub async fn get_extension_version(
        &self,
        extension_id: &str,
        version: &str,
    ) -> Result<Option<ExtensionMetadata>> {
        self.transaction(|tx| async move {
            let extension = extension::Entity::find()
                .filter(extension::Column::ExternalId.eq(extension_id))
                .filter(extension_version::Column::Version.eq(version))
                .inner_join(extension_version::Entity)
                .select_also(extension_version::Entity)
                .one(&*tx)
                .await?;

            Ok(extension.and_then(|(extension, version)| {
                Some(metadata_from_extension_and_version(extension, version?))
            }))
        })
        .await
    }

    pub async fn get_known_extension_versions(&self) -> Result<HashMap<String, Vec<String>>> {
        self.transaction(|tx| async move {
            let mut extension_external_ids_by_id = HashMap::default();

            let mut rows = extension::Entity::find().stream(&*tx).await?;
            while let Some(row) = rows.next().await {
                let row = row?;
                extension_external_ids_by_id.insert(row.id, row.external_id);
            }
            drop(rows);

            let mut known_versions_by_extension_id: HashMap<String, Vec<String>> =
                HashMap::default();
            let mut rows = extension_version::Entity::find().stream(&*tx).await?;
            while let Some(row) = rows.next().await {
                let row = row?;

                let Some(extension_id) = extension_external_ids_by_id.get(&row.extension_id) else {
                    continue;
                };

                let versions = known_versions_by_extension_id
                    .entry(extension_id.clone())
                    .or_default();
                if let Err(ix) = versions.binary_search(&row.version) {
                    versions.insert(ix, row.version);
                }
            }
            drop(rows);

            Ok(known_versions_by_extension_id)
        })
        .await
    }

    pub async fn insert_extension_versions(
        &self,
        versions_by_extension_id: &HashMap<&str, Vec<NewExtensionVersion>>,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            for (external_id, versions) in versions_by_extension_id {
                if versions.is_empty() {
                    continue;
                }

                let latest_version = versions
                    .iter()
                    .max_by_key(|version| &version.version)
                    .unwrap();

                let insert = extension::Entity::insert(extension::ActiveModel {
                    name: ActiveValue::Set(latest_version.name.clone()),
                    external_id: ActiveValue::Set((*external_id).to_owned()),
                    id: ActiveValue::NotSet,
                    latest_version: ActiveValue::Set(latest_version.version.to_string()),
                    total_download_count: ActiveValue::NotSet,
                })
                .on_conflict(
                    OnConflict::columns([extension::Column::ExternalId])
                        .update_column(extension::Column::ExternalId)
                        .to_owned(),
                );

                let extension = if tx.support_returning() {
                    insert.exec_with_returning(&*tx).await?
                } else {
                    // Sqlite
                    insert.exec_without_returning(&*tx).await?;
                    extension::Entity::find()
                        .filter(extension::Column::ExternalId.eq(*external_id))
                        .one(&*tx)
                        .await?
                        .context("failed to insert extension")?
                };

                extension_version::Entity::insert_many(versions.iter().map(|version| {
                    extension_version::ActiveModel {
                        extension_id: ActiveValue::Set(extension.id),
                        published_at: ActiveValue::Set(version.published_at),
                        version: ActiveValue::Set(version.version.to_string()),
                        authors: ActiveValue::Set(version.authors.join(", ")),
                        repository: ActiveValue::Set(version.repository.clone()),
                        description: ActiveValue::Set(version.description.clone()),
                        schema_version: ActiveValue::Set(version.schema_version),
                        wasm_api_version: ActiveValue::Set(version.wasm_api_version.clone()),
                        provides_themes: ActiveValue::Set(
                            version.provides.contains(&ExtensionProvides::Themes),
                        ),
                        provides_icon_themes: ActiveValue::Set(
                            version.provides.contains(&ExtensionProvides::IconThemes),
                        ),
                        provides_languages: ActiveValue::Set(
                            version.provides.contains(&ExtensionProvides::Languages),
                        ),
                        provides_grammars: ActiveValue::Set(
                            version.provides.contains(&ExtensionProvides::Grammars),
                        ),
                        provides_language_servers: ActiveValue::Set(
                            version
                                .provides
                                .contains(&ExtensionProvides::LanguageServers),
                        ),
                        provides_context_servers: ActiveValue::Set(
                            version
                                .provides
                                .contains(&ExtensionProvides::ContextServers),
                        ),
                        provides_slash_commands: ActiveValue::Set(
                            version.provides.contains(&ExtensionProvides::SlashCommands),
                        ),
                        provides_indexed_docs_providers: ActiveValue::Set(
                            version
                                .provides
                                .contains(&ExtensionProvides::IndexedDocsProviders),
                        ),
                        provides_snippets: ActiveValue::Set(
                            version.provides.contains(&ExtensionProvides::Snippets),
                        ),
                        provides_debug_adapters: ActiveValue::Set(
                            version.provides.contains(&ExtensionProvides::DebugAdapters),
                        ),
                        download_count: ActiveValue::NotSet,
                    }
                }))
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&*tx)
                .await?;

                if let Ok(db_version) = semver::Version::parse(&extension.latest_version)
                    && db_version >= latest_version.version
                {
                    continue;
                }

                let mut extension = extension.into_active_model();
                extension.latest_version = ActiveValue::Set(latest_version.version.to_string());
                extension.name = ActiveValue::set(latest_version.name.clone());
                extension::Entity::update(extension).exec(&*tx).await?;
            }

            Ok(())
        })
        .await
    }

    pub async fn record_extension_download(&self, extension: &str, version: &str) -> Result<bool> {
        self.transaction(|tx| async move {
            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryId {
                Id,
            }

            let extension_id: Option<ExtensionId> = extension::Entity::find()
                .filter(extension::Column::ExternalId.eq(extension))
                .select_only()
                .column(extension::Column::Id)
                .into_values::<_, QueryId>()
                .one(&*tx)
                .await?;
            let Some(extension_id) = extension_id else {
                return Ok(false);
            };

            extension_version::Entity::update_many()
                .col_expr(
                    extension_version::Column::DownloadCount,
                    extension_version::Column::DownloadCount.into_expr().add(1),
                )
                .filter(
                    extension_version::Column::ExtensionId
                        .eq(extension_id)
                        .and(extension_version::Column::Version.eq(version)),
                )
                .exec(&*tx)
                .await?;

            extension::Entity::update_many()
                .col_expr(
                    extension::Column::TotalDownloadCount,
                    extension::Column::TotalDownloadCount.into_expr().add(1),
                )
                .filter(extension::Column::Id.eq(extension_id))
                .exec(&*tx)
                .await?;

            Ok(true)
        })
        .await
    }
}

fn apply_provides_filter(
    mut condition: Condition,
    provides_filter: &BTreeSet<ExtensionProvides>,
) -> Condition {
    if provides_filter.contains(&ExtensionProvides::Themes) {
        condition = condition.add(extension_version::Column::ProvidesThemes.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::IconThemes) {
        condition = condition.add(extension_version::Column::ProvidesIconThemes.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::Languages) {
        condition = condition.add(extension_version::Column::ProvidesLanguages.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::Grammars) {
        condition = condition.add(extension_version::Column::ProvidesGrammars.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::LanguageServers) {
        condition = condition.add(extension_version::Column::ProvidesLanguageServers.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::ContextServers) {
        condition = condition.add(extension_version::Column::ProvidesContextServers.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::SlashCommands) {
        condition = condition.add(extension_version::Column::ProvidesSlashCommands.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::IndexedDocsProviders) {
        condition = condition.add(extension_version::Column::ProvidesIndexedDocsProviders.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::Snippets) {
        condition = condition.add(extension_version::Column::ProvidesSnippets.eq(true));
    }

    if provides_filter.contains(&ExtensionProvides::DebugAdapters) {
        condition = condition.add(extension_version::Column::ProvidesDebugAdapters.eq(true));
    }

    condition
}

fn metadata_from_extension_and_version(
    extension: extension::Model,
    version: extension_version::Model,
) -> ExtensionMetadata {
    let provides = version.provides();

    ExtensionMetadata {
        id: extension.external_id.into(),
        manifest: rpc::ExtensionApiManifest {
            name: extension.name,
            version: version.version.into(),
            authors: version
                .authors
                .split(',')
                .map(|author| author.trim().to_string())
                .collect::<Vec<_>>(),
            description: Some(version.description),
            repository: version.repository,
            schema_version: Some(version.schema_version),
            wasm_api_version: version.wasm_api_version,
            provides,
        },

        published_at: convert_time_to_chrono(version.published_at),
        download_count: extension.total_download_count as u64,
    }
}

pub fn convert_time_to_chrono(time: time::PrimitiveDateTime) -> chrono::DateTime<Utc> {
    chrono::DateTime::from_naive_utc_and_offset(
        #[allow(deprecated)]
        chrono::NaiveDateTime::from_timestamp_opt(time.assume_utc().unix_timestamp(), 0).unwrap(),
        Utc,
    )
}
