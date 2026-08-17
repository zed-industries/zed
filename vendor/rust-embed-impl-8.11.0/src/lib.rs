#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use rust_embed_utils::PathMatcher;
use std::{
  collections::BTreeMap,
  env,
  io::ErrorKind,
  iter::FromIterator,
  path::{Path, PathBuf},
};
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, MetaNameValue};

fn embedded(
  ident: &syn::Ident, relative_folder_path: Option<&str>, absolute_folder_path: String, prefix: Option<&str>, includes: &[String], excludes: &[String],
  metadata_only: bool, crate_path: &syn::Path,
) -> syn::Result<TokenStream2> {
  extern crate rust_embed_utils;

  let mut match_values = BTreeMap::new();
  let mut list_values = Vec::<String>::new();

  let includes: Vec<&str> = includes.iter().map(AsRef::as_ref).collect();
  let excludes: Vec<&str> = excludes.iter().map(AsRef::as_ref).collect();
  let matcher = PathMatcher::new(&includes, &excludes);
  for rust_embed_utils::FileEntry { rel_path, full_canonical_path } in rust_embed_utils::get_files(absolute_folder_path.clone(), matcher) {
    match_values.insert(
      rel_path.clone(),
      embed_file(relative_folder_path, ident, &rel_path, &full_canonical_path, metadata_only, crate_path)?,
    );

    list_values.push(if let Some(prefix) = prefix {
      format!("{}{}", prefix, rel_path)
    } else {
      rel_path
    });
  }

  let array_len = list_values.len();

  // If debug-embed is on, unconditionally include the code below. Otherwise,
  // make it conditional on cfg(not(debug_assertions)).
  let not_debug_attr = if cfg!(feature = "debug-embed") {
    quote! {}
  } else {
    quote! { #[cfg(not(debug_assertions))]}
  };

  let handle_prefix = if let Some(prefix) = prefix {
    quote! {
      let file_path = file_path.strip_prefix(#prefix)?;
    }
  } else {
    TokenStream2::new()
  };
  let match_values = match_values.into_iter().map(|(path, bytes)| {
    quote! {
        (#path, #bytes),
    }
  });
  let value_type = if cfg!(feature = "compression") {
    quote! { fn() -> #crate_path::EmbeddedFile }
  } else {
    quote! { #crate_path::EmbeddedFile }
  };
  let get_value = if cfg!(feature = "compression") {
    quote! {|idx| (ENTRIES[idx].1)()}
  } else {
    quote! {|idx| ENTRIES[idx].1.clone()}
  };
  Ok(quote! {
      #not_debug_attr
      impl #ident {
          /// Get an embedded file and its metadata.
          pub fn get(file_path: &str) -> ::std::option::Option<#crate_path::EmbeddedFile> {
            #handle_prefix
            let key = file_path.replace("\\", "/");
            const ENTRIES: &'static [(&'static str, #value_type)] = &[
                #(#match_values)*];
            let position = ENTRIES.binary_search_by_key(&key.as_str(), |entry| entry.0);
            position.ok().map(#get_value)

          }

          fn names() -> ::std::slice::Iter<'static, &'static str> {
              const ITEMS: [&str; #array_len] = [#(#list_values),*];
              ITEMS.iter()
          }

          /// Iterates over the file paths in the folder.
          pub fn iter() -> impl ::std::iter::Iterator<Item = ::std::borrow::Cow<'static, str>> + 'static {
              Self::names().map(|x| ::std::borrow::Cow::from(*x))
          }
      }

      #not_debug_attr
      impl #crate_path::RustEmbed for #ident {
        fn get(file_path: &str) -> ::std::option::Option<#crate_path::EmbeddedFile> {
          #ident::get(file_path)
        }
        fn iter() -> #crate_path::Filenames {
          #crate_path::Filenames::Embedded(#ident::names())
        }
      }
  })
}

fn dynamic(
  ident: &syn::Ident, folder_path: String, prefix: Option<&str>, includes: &[String], excludes: &[String], metadata_only: bool, crate_path: &syn::Path,
) -> TokenStream2 {
  let (handle_prefix, map_iter) = if let ::std::option::Option::Some(prefix) = prefix {
    (
      quote! { let file_path = file_path.strip_prefix(#prefix)?; },
      quote! { ::std::borrow::Cow::Owned(format!("{}{}", #prefix, e.rel_path)) },
    )
  } else {
    (TokenStream2::new(), quote! { ::std::borrow::Cow::from(e.rel_path) })
  };

  let declare_includes = quote! {
    const INCLUDES: &[&str] = &[#(#includes),*];
  };

  let declare_excludes = quote! {
    const EXCLUDES: &[&str] = &[#(#excludes),*];
  };

  // In metadata_only mode, we still need to read file contents to generate the
  // file hash, but then we drop the file data.
  let strip_contents = metadata_only.then_some(quote! {
      .map(|mut file| { file.data = ::std::default::Default::default(); file })
  });

  let non_canonical_folder_path = Path::new(&folder_path);
  let canonical_folder_path = non_canonical_folder_path
    .canonicalize()
    .or_else(|err| match err {
      err if err.kind() == ErrorKind::NotFound => Ok(non_canonical_folder_path.to_owned()),
      err => Err(err),
    })
    .expect("folder path must resolve to an absolute path");
  let canonical_folder_path = canonical_folder_path.to_str().expect("absolute folder path must be valid unicode");

  quote! {
      #[cfg(debug_assertions)]
      impl #ident {


        fn matcher() -> #crate_path::utils::PathMatcher {
            #declare_includes
            #declare_excludes
            static PATH_MATCHER: ::std::sync::OnceLock<#crate_path::utils::PathMatcher> = ::std::sync::OnceLock::new();
            PATH_MATCHER.get_or_init(|| #crate_path::utils::PathMatcher::new(INCLUDES, EXCLUDES)).clone()
        }
          /// Get an embedded file and its metadata.
          pub fn get(file_path: &str) -> ::std::option::Option<#crate_path::EmbeddedFile> {
              #handle_prefix

              let rel_file_path = file_path.replace("\\", "/");
              let file_path = ::std::path::Path::new(#folder_path).join(&rel_file_path);

              // Make sure the path requested does not escape the folder path
              let canonical_file_path = file_path.canonicalize().ok()?;
              if !canonical_file_path.starts_with(#canonical_folder_path) {
                  // Tried to request a path that is not in the embedded folder

                  // TODO: Currently it allows "path_traversal_attack" for the symlink files
                  // For it to be working properly we need to get absolute path first
                  // and check that instead if it starts with `canonical_folder_path`
                  // https://doc.rust-lang.org/std/path/fn.absolute.html (currently nightly)
                  // Should be allowed only if it was a symlink
                  let metadata = ::std::fs::symlink_metadata(&file_path).ok()?;
                  if !metadata.is_symlink() {
                    return ::std::option::Option::None;
                  }
              }
              let path_matcher = Self::matcher();
              if path_matcher.is_path_included(&rel_file_path) {
                #crate_path::utils::read_file_from_fs(&canonical_file_path).ok() #strip_contents
              } else {
                ::std::option::Option::None
              }
          }

          /// Iterates over the file paths in the folder.
          pub fn iter() -> impl ::std::iter::Iterator<Item = ::std::borrow::Cow<'static, str>> + 'static {
              use ::std::path::Path;


              #crate_path::utils::get_files(::std::string::String::from(#folder_path), Self::matcher())
                  .map(|e| #map_iter)
          }
      }

      #[cfg(debug_assertions)]
      impl #crate_path::RustEmbed for #ident {
        fn get(file_path: &str) -> ::std::option::Option<#crate_path::EmbeddedFile> {
          #ident::get(file_path)
        }
        fn iter() -> #crate_path::Filenames {
          // the return type of iter() is unnamable, so we have to box it
          #crate_path::Filenames::Dynamic(::std::boxed::Box::new(#ident::iter()))
        }
      }
  }
}

fn generate_assets(
  ident: &syn::Ident, relative_folder_path: Option<&str>, absolute_folder_path: String, prefix: Option<String>, includes: Vec<String>, excludes: Vec<String>,
  metadata_only: bool, crate_path: &syn::Path,
) -> syn::Result<TokenStream2> {
  let embedded_impl = embedded(
    ident,
    relative_folder_path,
    absolute_folder_path.clone(),
    prefix.as_deref(),
    &includes,
    &excludes,
    metadata_only,
    crate_path,
  );
  if cfg!(feature = "debug-embed") {
    return embedded_impl;
  }
  let embedded_impl = embedded_impl?;
  let dynamic_impl = dynamic(ident, absolute_folder_path, prefix.as_deref(), &includes, &excludes, metadata_only, crate_path);

  Ok(quote! {
      #embedded_impl
      #dynamic_impl
  })
}

fn embed_file(
  folder_path: Option<&str>, ident: &syn::Ident, rel_path: &str, full_canonical_path: &str, metadata_only: bool, crate_path: &syn::Path,
) -> syn::Result<TokenStream2> {
  let file = rust_embed_utils::read_file_from_fs(Path::new(full_canonical_path)).expect("File should be readable");
  let hash = file.metadata.sha256_hash();
  let (last_modified, created) = if cfg!(feature = "deterministic-timestamps") {
    (quote! { ::std::option::Option::Some(0u64) }, quote! { ::std::option::Option::Some(0u64) })
  } else {
    let last_modified = match file.metadata.last_modified() {
      Some(last_modified) => quote! { ::std::option::Option::Some(#last_modified) },
      None => quote! { ::std::option::Option::None },
    };
    let created = match file.metadata.created() {
      Some(created) => quote! { ::std::option::Option::Some(#created) },
      None => quote! { ::std::option::Option::None },
    };
    (last_modified, created)
  };
  #[cfg(feature = "mime-guess")]
  let mimetype_tokens = {
    let mt = file.metadata.mimetype();
    quote! { , #mt }
  };
  #[cfg(not(feature = "mime-guess"))]
  let mimetype_tokens = TokenStream2::new();

  let embedding_code = if metadata_only {
    quote! {
        const BYTES: &'static [u8] = &[];
    }
  } else if cfg!(feature = "compression") {
    let folder_path = folder_path.ok_or(syn::Error::new(ident.span(), "`folder` must be provided under `compression` feature."))?;
    // Print some debugging information
    let full_relative_path = PathBuf::from_iter([folder_path, rel_path]);
    let full_relative_path = full_relative_path.to_string_lossy();
    quote! {
      #crate_path::flate!(static BYTES: [u8] from #full_relative_path);
    }
  } else {
    quote! {
      const BYTES: &'static [u8] = include_bytes!(#full_canonical_path);
    }
  };
  let closure_args = if cfg!(feature = "compression") {
    quote! { || }
  } else {
    quote! {}
  };
  Ok(quote! {
       #closure_args {
        #embedding_code

        #crate_path::EmbeddedFile {
            data: ::std::borrow::Cow::Borrowed(&BYTES),
            metadata: #crate_path::Metadata::__rust_embed_new([#(#hash),*], #last_modified, #created #mimetype_tokens)
        }
      }
  })
}

/// Find all pairs of the `name = "value"` attribute from the derive input
fn find_attribute_values(ast: &syn::DeriveInput, attr_name: &str) -> Vec<String> {
  ast
    .attrs
    .iter()
    .filter(|value| value.path().is_ident(attr_name))
    .filter_map(|attr| match &attr.meta {
      Meta::NameValue(MetaNameValue {
        value: Expr::Lit(ExprLit { lit: Lit::Str(val), .. }),
        ..
      }) => Some(val.value()),
      _ => None,
    })
    .collect()
}

fn find_bool_attribute(ast: &syn::DeriveInput, attr_name: &str) -> Option<bool> {
  ast
    .attrs
    .iter()
    .find(|value| value.path().is_ident(attr_name))
    .and_then(|attr| match &attr.meta {
      Meta::NameValue(MetaNameValue {
        value: Expr::Lit(ExprLit { lit: Lit::Bool(val), .. }),
        ..
      }) => Some(val.value()),
      _ => None,
    })
}

fn impl_rust_embed(ast: &syn::DeriveInput) -> syn::Result<TokenStream2> {
  match ast.data {
    Data::Struct(ref data) => match data.fields {
      Fields::Unit => {}
      _ => return Err(syn::Error::new_spanned(ast, "RustEmbed can only be derived for unit structs")),
    },
    _ => return Err(syn::Error::new_spanned(ast, "RustEmbed can only be derived for unit structs")),
  };

  let crate_path: syn::Path = find_attribute_values(ast, "crate_path")
    .last()
    .map_or_else(|| syn::parse_str("rust_embed").unwrap(), |v| syn::parse_str(v).unwrap());

  let mut folder_paths = find_attribute_values(ast, "folder");
  if folder_paths.len() != 1 {
    return Err(syn::Error::new_spanned(
      ast,
      "#[derive(RustEmbed)] must contain one attribute like this #[folder = \"examples/public/\"]",
    ));
  }
  let folder_path = folder_paths.remove(0);

  let prefix = find_attribute_values(ast, "prefix").into_iter().next();
  let includes = find_attribute_values(ast, "include");
  let excludes = find_attribute_values(ast, "exclude");
  let metadata_only = find_bool_attribute(ast, "metadata_only").unwrap_or(false);
  let allow_missing = find_bool_attribute(ast, "allow_missing").unwrap_or(false);

  #[cfg(not(feature = "include-exclude"))]
  if !includes.is_empty() || !excludes.is_empty() {
    return Err(syn::Error::new_spanned(
      ast,
      "Please turn on the `include-exclude` feature to use the `include` and `exclude` attributes",
    ));
  }

  #[cfg(feature = "interpolate-folder-path")]
  let folder_path = shellexpand::full(&folder_path)
    .map_err(|v| syn::Error::new_spanned(ast, v.to_string()))?
    .to_string();

  // Base relative paths on the Cargo.toml location
  let (relative_path, absolute_folder_path) = if Path::new(&folder_path).is_relative() {
    let absolute_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
      .join(&folder_path)
      .to_str()
      .unwrap()
      .to_owned();
    (Some(folder_path.clone()), absolute_path)
  } else {
    if cfg!(feature = "compression") {
      return Err(syn::Error::new_spanned(ast, "`folder` must be a relative path under `compression` feature."));
    }
    (None, folder_path)
  };

  if !Path::new(&absolute_folder_path).exists() && !allow_missing {
    let mut message = format!(
      "#[derive(RustEmbed)] folder '{}' does not exist. cwd: '{}'",
      absolute_folder_path,
      std::env::current_dir().unwrap().to_str().unwrap()
    );

    // Add a message about the interpolate-folder-path feature if the path may
    // include a variable
    if absolute_folder_path.contains('$') && cfg!(not(feature = "interpolate-folder-path")) {
      message += "\nA variable has been detected. RustEmbed can expand variables \
                  when the `interpolate-folder-path` feature is enabled.";
    }

    return Err(syn::Error::new_spanned(ast, message));
  };

  generate_assets(
    &ast.ident,
    relative_path.as_deref(),
    absolute_folder_path,
    prefix,
    includes,
    excludes,
    metadata_only,
    &crate_path,
  )
}

#[proc_macro_derive(RustEmbed, attributes(folder, prefix, include, exclude, allow_missing, metadata_only, crate_path))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  match impl_rust_embed(&ast) {
    Ok(ok) => ok.into(),
    Err(e) => e.to_compile_error().into(),
  }
}
