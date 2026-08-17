use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::{fs, io};

use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use syn::Type;

pub use input::QueryMacroInput;
use quote::{format_ident, quote};
use sqlx_core::database::Database;
use sqlx_core::{column::Column, describe::Describe, type_info::TypeInfo};

use crate::database::DatabaseExt;
use crate::query::data::{hash_string, DynQueryData, QueryData};
use crate::query::input::RecordType;
use either::Either;
use url::Url;

mod args;
mod data;
mod input;
mod output;

#[derive(Copy, Clone)]
pub struct QueryDriver {
    db_name: &'static str,
    url_schemes: &'static [&'static str],
    expand: fn(QueryMacroInput, QueryDataSource) -> crate::Result<TokenStream>,
}

impl QueryDriver {
    pub const fn new<DB: DatabaseExt>() -> Self
    where
        Describe<DB>: serde::Serialize + serde::de::DeserializeOwned,
    {
        QueryDriver {
            db_name: DB::NAME,
            url_schemes: DB::URL_SCHEMES,
            expand: expand_with::<DB>,
        }
    }
}
pub enum QueryDataSource<'a> {
    Live {
        database_url: &'a str,
        database_url_parsed: Url,
    },
    Cached(DynQueryData),
}

impl<'a> QueryDataSource<'a> {
    pub fn live(database_url: &'a str) -> crate::Result<Self> {
        Ok(QueryDataSource::Live {
            database_url,
            database_url_parsed: database_url.parse()?,
        })
    }

    pub fn matches_driver(&self, driver: &QueryDriver) -> bool {
        match self {
            Self::Live {
                database_url_parsed,
                ..
            } => driver.url_schemes.contains(&database_url_parsed.scheme()),
            Self::Cached(dyn_data) => dyn_data.db_name == driver.db_name,
        }
    }
}

struct Metadata {
    #[allow(unused)]
    manifest_dir: PathBuf,
    offline: bool,
    database_url: Option<String>,
    offline_dir: Option<String>,
    workspace_root: Arc<Mutex<Option<PathBuf>>>,
}

impl Metadata {
    pub fn workspace_root(&self) -> PathBuf {
        let mut root = self.workspace_root.lock().unwrap();
        if root.is_none() {
            use serde::Deserialize;
            use std::process::Command;

            let cargo = env("CARGO").expect("`CARGO` must be set");

            let output = Command::new(cargo)
                .args(["metadata", "--format-version=1", "--no-deps"])
                .current_dir(&self.manifest_dir)
                .env_remove("__CARGO_FIX_PLZ")
                .output()
                .expect("Could not fetch metadata");

            #[derive(Deserialize)]
            struct CargoMetadata {
                workspace_root: PathBuf,
            }

            let metadata: CargoMetadata =
                serde_json::from_slice(&output.stdout).expect("Invalid `cargo metadata` output");

            *root = Some(metadata.workspace_root);
        }
        root.clone().unwrap()
    }
}

static METADATA: Lazy<Mutex<HashMap<String, Metadata>>> = Lazy::new(Default::default);

// If we are in a workspace, lookup `workspace_root` since `CARGO_MANIFEST_DIR` won't
// reflect the workspace dir: https://github.com/rust-lang/cargo/issues/3946
fn init_metadata(manifest_dir: &String) -> Metadata {
    let manifest_dir: PathBuf = manifest_dir.into();

    let (database_url, offline, offline_dir) = load_dot_env(&manifest_dir);

    let offline = env("SQLX_OFFLINE")
        .ok()
        .or(offline)
        .map(|s| s.eq_ignore_ascii_case("true") || s == "1")
        .unwrap_or(false);

    let database_url = env("DATABASE_URL").ok().or(database_url);

    Metadata {
        manifest_dir,
        offline,
        database_url,
        offline_dir,
        workspace_root: Arc::new(Mutex::new(None)),
    }
}

pub fn expand_input<'a>(
    input: QueryMacroInput,
    drivers: impl IntoIterator<Item = &'a QueryDriver>,
) -> crate::Result<TokenStream> {
    let manifest_dir = env("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` must be set");

    let mut metadata_lock = METADATA
        .lock()
        // Just reset the metadata on error
        .unwrap_or_else(|poison_err| {
            let mut guard = poison_err.into_inner();
            *guard = Default::default();
            guard
        });

    let metadata = metadata_lock
        .entry(manifest_dir)
        .or_insert_with_key(init_metadata);

    let data_source = match &metadata {
        Metadata {
            offline: false,
            database_url: Some(db_url),
            ..
        } => QueryDataSource::live(db_url)?,
        Metadata { offline, .. } => {
            // Try load the cached query metadata file.
            let filename = format!("query-{}.json", hash_string(&input.sql));

            // Check SQLX_OFFLINE_DIR, then local .sqlx, then workspace .sqlx.
            let dirs = [
                |meta: &Metadata| meta.offline_dir.as_deref().map(PathBuf::from),
                |meta: &Metadata| Some(meta.manifest_dir.join(".sqlx")),
                |meta: &Metadata| Some(meta.workspace_root().join(".sqlx")),
            ];
            let Some(data_file_path) = dirs
                .iter()
                .filter_map(|path| path(metadata))
                .map(|path| path.join(&filename))
                .find(|path| path.exists())
            else {
                return Err(
                    if *offline {
                        "`SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`"
                    } else {
                        "set `DATABASE_URL` to use query macros online, or run `cargo sqlx prepare` to update the query cache"
                    }.into()
                );
            };
            QueryDataSource::Cached(DynQueryData::from_data_file(&data_file_path, &input.sql)?)
        }
    };

    for driver in drivers {
        if data_source.matches_driver(driver) {
            return (driver.expand)(input, data_source);
        }
    }

    match data_source {
        QueryDataSource::Live {
            database_url_parsed,
            ..
        } => Err(format!(
            "no database driver found matching URL scheme {:?}; the corresponding Cargo feature may need to be enabled", 
            database_url_parsed.scheme()
        ).into()),
        QueryDataSource::Cached(data) => {
            Err(format!(
                "found cached data for database {:?} but no matching driver; the corresponding Cargo feature may need to be enabled",
                data.db_name
            ).into())
        }
    }
}

fn expand_with<DB: DatabaseExt>(
    input: QueryMacroInput,
    data_source: QueryDataSource,
) -> crate::Result<TokenStream>
where
    Describe<DB>: DescribeExt,
{
    let (query_data, offline): (QueryData<DB>, bool) = match data_source {
        QueryDataSource::Cached(dyn_data) => (QueryData::from_dyn_data(dyn_data)?, true),
        QueryDataSource::Live { database_url, .. } => {
            let describe = DB::describe_blocking(&input.sql, database_url)?;
            (QueryData::from_describe(&input.sql, describe), false)
        }
    };

    expand_with_data(input, query_data, offline)
}

// marker trait for `Describe` that lets us conditionally require it to be `Serialize + Deserialize`
trait DescribeExt: serde::Serialize + serde::de::DeserializeOwned {}

impl<DB: Database> DescribeExt for Describe<DB> where
    Describe<DB>: serde::Serialize + serde::de::DeserializeOwned
{
}

fn expand_with_data<DB: DatabaseExt>(
    input: QueryMacroInput,
    data: QueryData<DB>,
    offline: bool,
) -> crate::Result<TokenStream>
where
    Describe<DB>: DescribeExt,
{
    // validate at the minimum that our args match the query's input parameters
    let num_parameters = match data.describe.parameters() {
        Some(Either::Left(params)) => Some(params.len()),
        Some(Either::Right(num)) => Some(num),

        None => None,
    };

    if let Some(num) = num_parameters {
        if num != input.arg_exprs.len() {
            return Err(
                format!("expected {} parameters, got {}", num, input.arg_exprs.len()).into(),
            );
        }
    }

    let args_tokens = args::quote_args(&input, &data.describe)?;

    let query_args = format_ident!("query_args");

    let output = if data
        .describe
        .columns()
        .iter()
        .all(|it| it.type_info().is_void())
    {
        let db_path = DB::db_path();
        let sql = &input.sql;

        quote! {
            ::sqlx::__query_with_result::<#db_path, _>(#sql, #query_args)
        }
    } else {
        match input.record_type {
            RecordType::Generated => {
                let columns = output::columns_to_rust::<DB>(&data.describe)?;

                let record_name: Type = syn::parse_str("Record").unwrap();

                for rust_col in &columns {
                    if rust_col.type_.is_wildcard() {
                        return Err(
                            "wildcard overrides are only allowed with an explicit record type, \
                             e.g. `query_as!()` and its variants"
                                .into(),
                        );
                    }
                }

                let record_fields = columns
                    .iter()
                    .map(|output::RustColumn { ident, type_, .. }| quote!(#ident: #type_,));

                let mut record_tokens = quote! {
                    #[derive(Debug)]
                    #[allow(non_snake_case)]
                    struct #record_name {
                        #(#record_fields)*
                    }
                };

                record_tokens.extend(output::quote_query_as::<DB>(
                    &input,
                    &record_name,
                    &query_args,
                    &columns,
                ));

                record_tokens
            }
            RecordType::Given(ref out_ty) => {
                let columns = output::columns_to_rust::<DB>(&data.describe)?;

                output::quote_query_as::<DB>(&input, out_ty, &query_args, &columns)
            }
            RecordType::Scalar => {
                output::quote_query_scalar::<DB>(&input, &query_args, &data.describe)?
            }
        }
    };

    let ret_tokens = quote! {
        {
            #[allow(clippy::all)]
            {
                use ::sqlx::Arguments as _;

                #args_tokens

                #output
            }
        }
    };

    // Store query metadata only if offline support is enabled but the current build is online.
    // If the build is offline, the cache is our input so it's pointless to also write data for it.
    if !offline {
        // Only save query metadata if SQLX_OFFLINE_DIR is set manually or by `cargo sqlx prepare`.
        // Note: in a cargo workspace this path is relative to the root.
        if let Ok(dir) = env("SQLX_OFFLINE_DIR") {
            let path = PathBuf::from(&dir);

            match fs::metadata(&path) {
                Err(e) => {
                    if e.kind() != io::ErrorKind::NotFound {
                        // Can't obtain information about .sqlx
                        return Err(format!("{e}: {dir}").into());
                    }
                    // .sqlx doesn't exist.
                    return Err(format!("sqlx offline path does not exist: {dir}").into());
                }
                Ok(meta) => {
                    if !meta.is_dir() {
                        return Err(format!(
                            "sqlx offline path exists, but is not a directory: {dir}"
                        )
                        .into());
                    }

                    // .sqlx exists and is a directory, store data.
                    data.save_in(path)?;
                }
            }
        }
    }

    Ok(ret_tokens)
}

/// Get the value of an environment variable, telling the compiler about it if applicable.
fn env(name: &str) -> Result<String, std::env::VarError> {
    #[cfg(procmacro2_semver_exempt)]
    {
        proc_macro::tracked_env::var(name)
    }

    #[cfg(not(procmacro2_semver_exempt))]
    {
        std::env::var(name)
    }
}

/// Get `DATABASE_URL`, `SQLX_OFFLINE` and `SQLX_OFFLINE_DIR` from the `.env`.
fn load_dot_env(manifest_dir: &Path) -> (Option<String>, Option<String>, Option<String>) {
    let mut env_path = manifest_dir.join(".env");

    // If a .env file exists at CARGO_MANIFEST_DIR, load environment variables from this,
    // otherwise fallback to default dotenv file.
    #[cfg_attr(not(procmacro2_semver_exempt), allow(unused_variables))]
    let env_file = if env_path.exists() {
        let res = dotenvy::from_path_iter(&env_path);
        match res {
            Ok(iter) => Some(iter),
            Err(e) => panic!("failed to load environment from {env_path:?}, {e}"),
        }
    } else {
        #[allow(unused_assignments)]
        {
            env_path = PathBuf::from(".env");
        }
        dotenvy::dotenv_iter().ok()
    };

    let mut offline = None;
    let mut database_url = None;
    let mut offline_dir = None;

    if let Some(env_file) = env_file {
        // tell the compiler to watch the `.env` for changes.
        #[cfg(procmacro2_semver_exempt)]
        if let Some(env_path) = env_path.to_str() {
            proc_macro::tracked_path::path(env_path);
        }

        for item in env_file {
            let Ok((key, value)) = item else {
                continue;
            };

            match key.as_str() {
                "DATABASE_URL" => database_url = Some(value),
                "SQLX_OFFLINE" => offline = Some(value),
                "SQLX_OFFLINE_DIR" => offline_dir = Some(value),
                _ => {}
            };
        }
    }

    (database_url, offline, offline_dir)
}
