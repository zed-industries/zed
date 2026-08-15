use proc_macro2::{Delimiter, TokenTree};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn collect_macro(mac: &syn::Macro, commands: &mut BTreeSet<String>) {
    if !mac.path.is_ident("actions") {
        return;
    }
    let mut tokens = mac.tokens.clone().into_iter();
    let namespace = match tokens.next() {
        Some(TokenTree::Ident(namespace)) => namespace.to_string(),
        _ => {
            eprintln!("    FAILED: expected namespace");
            return;
        }
    };
    match tokens.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => {}
        _ => {
            eprintln!("    FAILED: expected comma");
            return;
        }
    }
    let group = match tokens.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => group,
        _ => {
            eprintln!("    FAILED: expected action list");
            return;
        }
    };
    let mut tokens = group.stream().into_iter();
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Punct(p) if p.as_char() == '#' => {
                tokens.next();
            }
            TokenTree::Ident(action) => {
                commands.insert(format!("{namespace}::{action}"));
            }
            _ => {}
        }
    }
}

pub fn collect_file(path: &Path, commands: &mut BTreeSet<String>) {
    eprintln!("  reading: {}", path.display());
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("  failed to read: {err}");
            return;
        }
    };
    eprintln!("  read complete: {} bytes", source.len());
    if !source.contains("actions!") {
        eprintln!("  no actions!");
        return;
    }
    eprintln!("  parsing file...");
    let file = match syn::parse_file(&source) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("  failed to parse: {err}");
            return;
        }
    };
    eprintln!("  parse complete");
    collect_items(file.items, commands);
    eprintln!("  collect complete");
}
pub fn collect_items(items: Vec<syn::Item>, commands: &mut BTreeSet<String>) {
    for item in items {
        match item {
            syn::Item::Macro(item_macro) => {
                eprintln!("  found macro: {:#?}", item_macro.mac.path.get_ident());
                collect_macro(&item_macro.mac, commands);
            }
            syn::Item::Mod(item_mod) => {
                eprintln!("  entering module: {}", item_mod.ident);
                if let Some((_, items)) = item_mod.content {
                    collect_items(items, commands);
                }
                eprintln!("  leaving module: {}", item_mod.ident);
            }
            _ => {}
        }
    }
}
pub fn find_action_files(root: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();
        if path.components().any(|c| {
            matches!(
                c.as_os_str().to_str(),
                Some("target" | "node_modules" | ".git")
            )
        }) {
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let source = match fs::read_to_string(path) {
            Ok(source) => source,
            Err(_) => continue,
        };
        if source.contains("actions!") {
            files.push(path.to_path_buf());
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use std::fs;
    use tempfile::tempdir;

    fn get_macro(source: proc_macro2::TokenStream) -> syn::Macro {
        let file = syn::parse2::<syn::File>(source).unwrap();

        match file.items.into_iter().next().unwrap() {
            syn::Item::Macro(item) => item.mac,
            other => panic!("expected macro, got {:#?}", other),
        }
    }

    #[test]
    fn collects_actions() {
        let item: syn::ItemMacro = syn::parse2(quote! {
            actions!(editor, [
                OpenFile,
                CloseFile,
                SaveFile,
            ]);
        })
        .unwrap();

        let mut commands = BTreeSet::new();

        collect_macro(&item.mac, &mut commands);

        assert_eq!(
            commands,
            BTreeSet::from([
                "editor::OpenFile".to_string(),
                "editor::CloseFile".to_string(),
                "editor::SaveFile".to_string(),
            ])
        );
    }

    #[test]
    fn collects_actions_directly() {
        let file = syn::parse2::<syn::File>(quote! {
            actions!(editor, [
                OpenFile,
                CloseFile,
                SaveFile,
            ]);
        })
        .unwrap();

        let item_macro = match &file.items[0] {
            syn::Item::Macro(item_macro) => item_macro,
            _ => panic!("expected macro"),
        };

        let mut commands = BTreeSet::new();

        collect_macro(&item_macro.mac, &mut commands);

        assert_eq!(
            commands,
            BTreeSet::from([
                "editor::CloseFile".to_string(),
                "editor::OpenFile".to_string(),
                "editor::SaveFile".to_string(),
            ])
        );
    }

    #[test]
    fn collects_actions_in_sorted_order() {
        let mac = get_macro(quote! {
            actions!(editor, [
                Zebra,
                Alpha,
                Middle,
            ]);
        });

        let mut commands = BTreeSet::new();

        collect_macro(&mac, &mut commands);

        assert_eq!(
            commands,
            BTreeSet::from([
                "editor::Alpha".to_string(),
                "editor::Middle".to_string(),
                "editor::Zebra".to_string(),
            ])
        );
    }

    #[test]
    fn ignores_non_actions_macros() {
        let mac = get_macro(quote! {
            something!(editor, [
                OpenFile,
            ]);
        });

        let mut commands = BTreeSet::new();

        collect_macro(&mac, &mut commands);

        assert!(commands.is_empty());
    }

    #[test]
    fn handles_action_attributes() {
        let mac = get_macro(quote! {
            actions!(editor, [
                #[deprecated]
                OpenFile,

                #[cfg(feature = "foo")]
                CloseFile,
            ]);
        });

        let mut commands = BTreeSet::new();

        collect_macro(&mac, &mut commands);

        assert_eq!(
            commands,
            BTreeSet::from([
                "editor::OpenFile".to_string(),
                "editor::CloseFile".to_string(),
            ])
        );
    }

    #[test]
    fn handles_empty_action_list() {
        let mac = get_macro(quote! {
            actions!(editor, []);
        });

        let mut commands = BTreeSet::new();

        collect_macro(&mac, &mut commands);

        assert!(commands.is_empty());
    }

    #[test]
    fn duplicate_actions_are_deduplicated() {
        let mac = get_macro(quote! {
            actions!(editor, [
                OpenFile,
                OpenFile,
                SaveFile,
                OpenFile,
            ]);
        });

        let mut commands = BTreeSet::new();

        collect_macro(&mac, &mut commands);

        assert_eq!(
            commands,
            BTreeSet::from([
                "editor::OpenFile".to_string(),
                "editor::SaveFile".to_string(),
            ])
        );
    }

    #[test]
    fn collects_nested_modules() {
        let file: syn::File = syn::parse2(quote! {
            actions!(root, [
                RootAction,
            ]);

            mod foo {
                actions!(foo, [
                    FooAction,
                ]);

                mod bar {
                    actions!(bar, [
                        BarAction,
                    ]);
                }
            }
        })
        .unwrap();

        let mut commands = BTreeSet::new();

        collect_items(file.items, &mut commands);

        assert_eq!(
            commands,
            BTreeSet::from([
                "root::RootAction".to_string(),
                "foo::FooAction".to_string(),
                "bar::BarAction".to_string(),
            ])
        );
    }

    #[test]
    fn ignores_non_macro_items() {
        let file: syn::File = syn::parse2(quote! {
            fn foo() {}

            struct Foo;

            const BAR: u32 = 1;

            actions!(editor, [
                OpenFile,
            ]);
        })
        .unwrap();

        let mut commands = BTreeSet::new();

        collect_items(file.items, &mut commands);

        assert_eq!(commands, BTreeSet::from(["editor::OpenFile".to_string(),]));
    }

    #[test]
    fn collect_file_reads_and_collects_actions() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("actions.rs");

        fs::write(
            &path,
            r#"
                actions!(editor, [
                    OpenFile,
                    CloseFile,
                ]);
            "#,
        )
        .unwrap();

        let mut commands = BTreeSet::new();

        collect_file(&path, &mut commands);

        assert_eq!(
            commands,
            BTreeSet::from([
                "editor::OpenFile".to_string(),
                "editor::CloseFile".to_string(),
            ])
        );
    }

    #[test]
    fn collect_file_ignores_files_without_actions() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("other.rs");

        fs::write(
            &path,
            r#"
                fn main() {
                    println!("hello");
                }
            "#,
        )
        .unwrap();

        let mut commands = BTreeSet::new();

        collect_file(&path, &mut commands);

        assert!(commands.is_empty());
    }

    #[test]
    fn find_action_files_finds_rust_files_with_actions() {
        let dir = tempdir().unwrap();

        let action_file = dir.path().join("actions.rs");
        let normal_file = dir.path().join("normal.rs");

        fs::write(
            &action_file,
            r#"
                actions!(editor, [OpenFile]);
            "#,
        )
        .unwrap();

        fs::write(
            &normal_file,
            r#"
                fn main() {}
            "#,
        )
        .unwrap();

        let files = find_action_files(dir.path().to_str().unwrap());

        assert_eq!(files, vec![action_file]);
    }

    #[test]
    fn find_action_files_ignores_non_rust_files() {
        let dir = tempdir().unwrap();

        fs::write(
            dir.path().join("actions.txt"),
            "actions!(editor, [OpenFile]);",
        )
        .unwrap();

        fs::write(
            dir.path().join("actions.json"),
            "actions!(editor, [OpenFile]);",
        )
        .unwrap();

        let files = find_action_files(dir.path().to_str().unwrap());

        assert!(files.is_empty());
    }

    #[test]
    fn find_action_files_ignores_target_directory() {
        let dir = tempdir().unwrap();

        let target = dir.path().join("target");
        fs::create_dir_all(&target).unwrap();

        fs::write(
            target.join("generated.rs"),
            "actions!(editor, [ShouldNotBeFound]);",
        )
        .unwrap();

        let files = find_action_files(dir.path().to_str().unwrap());

        assert!(files.is_empty());
    }

    #[test]
    fn find_action_files_ignores_node_modules() {
        let dir = tempdir().unwrap();

        let node_modules = dir.path().join("node_modules");
        fs::create_dir_all(&node_modules).unwrap();

        fs::write(
            node_modules.join("generated.rs"),
            "actions!(editor, [ShouldNotBeFound]);",
        )
        .unwrap();

        let files = find_action_files(dir.path().to_str().unwrap());

        assert!(files.is_empty());
    }

    #[test]
    fn find_action_files_ignores_git_directory() {
        let dir = tempdir().unwrap();

        let git = dir.path().join(".git");
        fs::create_dir_all(&git).unwrap();
        fs::write(
            git.join("generated.rs"),
            "actions!(editor, [ShouldNotBeFound]);",
        )
        .unwrap();
        let files = find_action_files(dir.path().to_str().unwrap());
        assert!(files.is_empty());
    }
}
