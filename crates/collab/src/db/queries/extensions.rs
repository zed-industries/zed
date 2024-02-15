use super::*;

impl Database {
    pub async fn get_extensions(
        &self,
        filter: Option<&str>,
        limit: usize,
    ) -> Result<Vec<ExtensionMetadata>> {
        self.transaction(|tx| async move {
            let mut condition = Condition::all();
            if let Some(filter) = filter {
                let fuzzy_name_filter = Self::fuzzy_like_string(filter);
                condition = condition.add(Expr::cust_with_expr("name ILIKE $1", fuzzy_name_filter));
            }

            let extensions = extension::Entity::find()
                .filter(condition)
                .order_by_desc(extension::Column::TotalDownloadCount)
                .order_by_asc(extension::Column::Name)
                .limit(Some(limit as u64))
                .filter(
                    extension::Column::LatestVersion
                        .into_expr()
                        .eq(extension_version::Column::Version.into_expr()),
                )
                .inner_join(extension_version::Entity)
                .select_also(extension_version::Entity)
                .all(&*tx)
                .await?;

            Ok(extensions
                .into_iter()
                .filter_map(|(extension, latest_version)| {
                    let version = latest_version?;
                    Some(ExtensionMetadata {
                        id: extension.external_id,
                        name: extension.name,
                        version: version.version,
                        authors: version
                            .authors
                            .split(',')
                            .map(|author| author.trim().to_string())
                            .collect::<Vec<_>>(),
                        description: version.description,
                        repository: version.repository,
                        published_at: version.published_at,
                        download_count: extension.total_download_count as u64,
                    })
                })
                .collect())
        })
        .await
    }

    pub async fn get_known_extension_versions<'a>(&self) -> Result<HashMap<String, Vec<String>>> {
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
                    external_id: ActiveValue::Set(external_id.to_string()),
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
                        .ok_or_else(|| anyhow!("failed to insert extension"))?
                };

                extension_version::Entity::insert_many(versions.iter().map(|version| {
                    extension_version::ActiveModel {
                        extension_id: ActiveValue::Set(extension.id),
                        published_at: ActiveValue::Set(version.published_at),
                        version: ActiveValue::Set(version.version.to_string()),
                        authors: ActiveValue::Set(version.authors.join(", ")),
                        repository: ActiveValue::Set(version.repository.clone()),
                        description: ActiveValue::Set(version.description.clone()),
                        download_count: ActiveValue::NotSet,
                    }
                }))
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&*tx)
                .await?;

                if let Ok(db_version) = semver::Version::parse(&extension.latest_version) {
                    if db_version >= latest_version.version {
                        continue;
                    }
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
