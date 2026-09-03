use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use db::kvp::KeyValueStore;
use editor::Editor;
use extension_host::ExtensionStore;
use gpui::{AppContext as _, Context, Entity, SharedString, Window};
use language::Buffer;
use ui::prelude::*;
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::notifications::simple_message_notification::MessageNotification;
use workspace::{Workspace, notifications::NotificationId};

const SUGGESTIONS_BY_EXTENSION_ID: &[(&str, &[&str])] = &[
    ("asciidoc", &["adoc", "asciidoc"]),
    ("astro", &["astro"]),
    ("beancount", &["beancount", "bean"]),
    ("clojure", &["bb", "clj", "cljc", "cljd", "cljs", "edn"]),
    (
        "csharp",
        &["cs", "csproj", "proj", "props", "targets", "slnx"],
    ),
    ("csv", &["csv"]),
    ("cython", &["pyx", "pxd", "pxi"]),
    ("dart", &["dart"]),
    (
        "dockerfile",
        &[
            "Containerfile",
            "Dockerfile",
            "compose.yaml",
            "compose.yml",
            "docker-compose.yaml",
            "docker-compose.yml",
            "dockerfile",
        ],
    ),
    ("elisp", &["el"]),
    (
        "elixir",
        &["eex", "ex", "exs", "heex", "leex", "mix.lock", "neex"],
    ),
    ("elm", &["elm"]),
    ("env", &[".env", ".envrc", "env", "envrc"]),
    (
        "erlang",
        &[
            "Emakefile",
            "app.src",
            "erl",
            "escript",
            "erlang",
            "hrl",
            "rebar.config",
            "xrl",
            "yrl",
        ],
    ),
    ("fish", &["fish"]),
    (
        "gdscript",
        &[
            "gd",
            "gdextension",
            "gdshader",
            "gdshaderinc",
            "godot",
            "tres",
            "tscn",
        ],
    ),
    (
        "git-firefly",
        &[
            ".containerignore",
            ".cursorignore",
            ".dockerignore",
            ".eslintignore",
            ".fdignore",
            ".git-blame-ignore-revs",
            ".gitconfig",
            ".gitignore",
            ".gitignore_global",
            ".gitmodules",
            ".ignore",
            ".lfsconfig",
            ".npmignore",
            ".prettierignore",
            ".rgignore",
            ".vscodeignore",
            "config.worktree",
            "git-rebase-todo",
            "gitattributes",
            ".gitattributes",
        ],
    ),
    ("gleam", &["gleam"]),
    (
        "glsl",
        &[
            "comp", "frag", "geom", "glsl", "mesh", "rcall", "rgen", "rahit", "rchit", "rmiss",
            "rint", "task", "tesc", "tese", "vert",
        ],
    ),
    ("graphql", &["gql", "graphql", "graphqls"]),
    (
        "groovy",
        &["Jenkinsfile", "JenkinsFile", "gradle", "groovy"],
    ),
    ("haskell", &["cabal", "hs", "lhs"]),
    ("html", &["htm", "html", "shtml"]),
    ("ini", &["inf", "ini"]),
    ("java", &["java", "properties"]),
    ("json5", &["json5"]),
    ("julia", &["jl"]),
    ("just", &["JUSTFILE", "Justfile", "just", "justfile"]),
    ("kotlin", &["kt", "kts"]),
    (
        "latex",
        &[
            "bib", "biblatex", "bibtex", "cls", "dtx", "ins", "latex", "sty", "tex",
        ],
    ),
    ("log", &["log"]),
    ("lua", &["lua"]),
    (
        "make",
        &[
            "GNUmakefile",
            "Makefile",
            "OCamlMakefile",
            "mak",
            "makefile",
            "mk",
        ],
    ),
    ("neocmake", &["CMakeLists.txt", "cmake"]),
    ("nginx", &["nginx.conf"]),
    ("nim", &["nim", "nim_format_string", "nimble", "nims"]),
    ("nix", &["nix"]),
    ("nu", &["nu", "nuon"]),
    (
        "ocaml",
        &[
            "dune",
            "dune-project",
            "dune-workspace",
            "ml",
            "mld",
            "mli",
            "mll",
            "mlx",
            "mly",
            "re",
            "rei",
        ],
    ),
    ("odin", &["odin"]),
    ("perl", &["pl", "pm", "pod", "t"]),
    ("php", &["php", "phpt", "phtml"]),
    ("powershell", &["ps1", "psm1"]),
    ("prisma", &["prisma"]),
    ("proto", &["proto"]),
    ("purescript", &["purs"]),
    (
        "python-requirements",
        &["constraints.txt", "requirements.txt"],
    ),
    ("r", &["R", "Rmd", "qmd", "r", "rmd"]),
    ("racket", &["rkt"]),
    ("rescript", &["res", "resi"]),
    ("rst", &["rst"]),
    (
        "ruby",
        &[
            "Appfile",
            "Appraisals",
            "Berksfile",
            "Brewfile",
            "Capfile",
            "Cheffile",
            "Dangerfile",
            "Deliverfile",
            "Gemfile",
            "Guardfile",
            "Fastfile",
            "Gymfile",
            "Hobofile",
            "Matchfile",
            "Podfile",
            "Puppetfile",
            "Rakefile",
            "Rantfile",
            "Scanfile",
            "Snapfile",
            "Steepfile",
            "Thorfile",
            "Vagrantfile",
            "builder",
            "cap",
            "capfile",
            "erb",
            "gemspec",
            "irbrc",
            "jbuilder",
            "pryrc",
            "rabl",
            "rake",
            "rb",
            "rbs",
            "ru",
            "rxml",
            "simplecov",
            "thor",
        ],
    ),
    ("scala", &["mill", "scala", "sbt", "sc"]),
    ("scheme", &["scm", "ss"]),
    ("scss", &["sass", "scss"]),
    ("solidity", &["sol", "yul"]),
    ("sql", &["sql"]),
    ("svelte", &["svelte"]),
    ("swift", &["swift", "swiftinterface"]),
    ("templ", &["templ"]),
    ("terraform", &["hcl", "tf", "tfvars", "tofu"]),
    ("toml", &["Cargo.lock", "Pipfile", "toml", "uv.lock"]),
    ("typst", &["typ", "typst"]),
    ("vue", &["vue"]),
    ("wgsl", &["wgsl"]),
    ("windows-batch", &["bat", "cmd"]),
    ("wit", &["wit"]),
    ("xml", &["xml"]),
    ("zig", &["zig", "zon"]),
];

fn suggested_extensions() -> &'static HashMap<&'static str, Arc<str>> {
    static SUGGESTIONS_BY_PATH_SUFFIX: OnceLock<HashMap<&str, Arc<str>>> = OnceLock::new();
    SUGGESTIONS_BY_PATH_SUFFIX.get_or_init(|| {
        SUGGESTIONS_BY_EXTENSION_ID
            .iter()
            .flat_map(|(name, path_suffixes)| {
                let name = Arc::<str>::from(*name);
                path_suffixes
                    .iter()
                    .map(move |suffix| (*suffix, name.clone()))
            })
            .collect()
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SuggestedExtension {
    pub extension_id: Arc<str>,
    pub file_name_or_extension: Arc<str>,
}

/// Returns the suggested extension for the given [`Path`].
fn suggested_extension(path: &RelPath) -> Option<SuggestedExtension> {
    let file_extension: Option<Arc<str>> = path.extension().map(|extension| extension.into());
    let file_name: Option<Arc<str>> = path.file_name().map(|name| name.into());

    let (file_name_or_extension, extension_id) = None
        // We suggest against file names first, as these suggestions will be more
        // specific than ones based on the file extension.
        .or_else(|| {
            file_name.clone().zip(
                file_name
                    .as_deref()
                    .and_then(|file_name| suggested_extensions().get(file_name)),
            )
        })
        .or_else(|| {
            file_extension.clone().zip(
                file_extension
                    .as_deref()
                    .and_then(|file_extension| suggested_extensions().get(file_extension)),
            )
        })?;

    Some(SuggestedExtension {
        extension_id: extension_id.clone(),
        file_name_or_extension,
    })
}

fn language_extension_key(extension_id: &str) -> String {
    format!("{}_extension_suggest", extension_id)
}

pub(crate) fn suggest(buffer: Entity<Buffer>, window: &mut Window, cx: &mut Context<Workspace>) {
    let Some(file) = buffer.read(cx).file().cloned() else {
        return;
    };

    let Some(SuggestedExtension {
        extension_id,
        file_name_or_extension,
    }) = suggested_extension(file.path())
    else {
        return;
    };

    let key = language_extension_key(&extension_id);
    let kvp = KeyValueStore::global(cx);
    let Ok(None) = kvp.read_kvp(&key) else {
        return;
    };

    cx.on_next_frame(window, move |workspace, _, cx| {
        let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
            return;
        };

        if editor.read(cx).buffer().read(cx).as_singleton().as_ref() != Some(&buffer) {
            return;
        }

        struct ExtensionSuggestionNotification;

        let notification_id = NotificationId::composite::<ExtensionSuggestionNotification>(
            SharedString::from(extension_id.clone()),
        );

        workspace.show_notification(notification_id, cx, |cx| {
            cx.new(move |cx| {
                MessageNotification::new(
                    format!(
                        "Do you want to install the recommended '{}' extension for '{}' files?",
                        extension_id, file_name_or_extension
                    ),
                    cx,
                )
                .primary_message("Yes, install extension")
                .primary_icon(IconName::Check)
                .primary_icon_color(Color::Success)
                .primary_on_click({
                    let extension_id = extension_id.clone();
                    move |_window, cx| {
                        let extension_id = extension_id.clone();
                        let extension_store = ExtensionStore::global(cx);
                        extension_store.update(cx, move |store, cx| {
                            store.install_latest_extension(extension_id, cx);
                        });
                    }
                })
                .secondary_message("No, don't install it")
                .secondary_icon(IconName::Close)
                .secondary_icon_color(Color::Error)
                .secondary_on_click(move |_window, cx| {
                    let key = language_extension_key(&extension_id);
                    let kvp = KeyValueStore::global(cx);
                    cx.background_spawn(async move {
                        kvp.write_kvp(key, "dismissed".to_string()).await.log_err()
                    })
                    .detach();
                })
            })
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::rel_path::rel_path;

    #[test]
    pub fn test_suggested_extension() {
        assert_eq!(
            suggested_extension(rel_path("Cargo.toml")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "toml".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("Cargo.lock")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "Cargo.lock".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("Dockerfile")),
            Some(SuggestedExtension {
                extension_id: "dockerfile".into(),
                file_name_or_extension: "Dockerfile".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("a/b/c/d/.gitignore")),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                file_name_or_extension: ".gitignore".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("a/b/c/d/test.gleam")),
            Some(SuggestedExtension {
                extension_id: "gleam".into(),
                file_name_or_extension: "gleam".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("a/b/c/d/test.sol")),
            Some(SuggestedExtension {
                extension_id: "solidity".into(),
                file_name_or_extension: "sol".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("a/b/c/d/test.jl")),
            Some(SuggestedExtension {
                extension_id: "julia".into(),
                file_name_or_extension: "jl".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("script.pl")),
            Some(SuggestedExtension {
                extension_id: "perl".into(),
                file_name_or_extension: "pl".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("app/uv.lock")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "uv.lock".into()
            })
        );
        // Dotfiles have no `Path::extension`, so they match by name.
        assert_eq!(
            suggested_extension(rel_path(".envrc")),
            Some(SuggestedExtension {
                extension_id: "env".into(),
                file_name_or_extension: ".envrc".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path(".gitattributes")),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                file_name_or_extension: ".gitattributes".into()
            })
        );
    }

    fn assert_suggests(path: &str, extension_id: &str, matched: &str) {
        assert_eq!(
            suggested_extension(rel_path(path)),
            Some(SuggestedExtension {
                extension_id: extension_id.into(),
                file_name_or_extension: matched.into(),
            }),
            "unexpected suggestion for `{path}`",
        );
    }

    #[test]
    pub fn every_extension_has_a_representative_match() {
        assert_suggests("doc.adoc", "asciidoc", "adoc");
        assert_suggests("app.astro", "astro", "astro");
        assert_suggests("ledger.bean", "beancount", "bean");
        assert_suggests("deps.edn", "clojure", "edn");
        assert_suggests("App.csproj", "csharp", "csproj");
        assert_suggests("data.csv", "csv", "csv");
        assert_suggests("lib.pyx", "cython", "pyx");
        assert_suggests("main.dart", "dart", "dart");
        assert_suggests("compose.yaml", "dockerfile", "compose.yaml");
        assert_suggests("init.el", "elisp", "el");
        assert_suggests("mix.lock", "elixir", "mix.lock");
        assert_suggests("Main.elm", "elm", "elm");
        assert_suggests("rebar.config", "erlang", "rebar.config");
        assert_suggests("config.fish", "fish", "fish");
        assert_suggests("main.gd", "gdscript", "gd");
        assert_suggests("shader.frag", "glsl", "frag");
        assert_suggests("schema.graphqls", "graphql", "graphqls");
        assert_suggests("Jenkinsfile", "groovy", "Jenkinsfile");
        assert_suggests("proj.cabal", "haskell", "cabal");
        assert_suggests("index.html", "html", "html");
        assert_suggests("app.ini", "ini", "ini");
        assert_suggests("tsconfig.json5", "json5", "json5");
        assert_suggests("Justfile", "just", "Justfile");
        assert_suggests("build.gradle.kts", "kotlin", "kts");
        assert_suggests("gradle.properties", "java", "properties");
        assert_suggests("main.sty", "latex", "sty");
        assert_suggests("server.log", "log", "log");
        assert_suggests("init.lua", "lua", "lua");
        assert_suggests("CMakeLists.txt", "neocmake", "CMakeLists.txt");
        assert_suggests("shim.nimble", "nim", "nimble");
        assert_suggests("flake.nix", "nix", "nix");
        assert_suggests("script.nuon", "nu", "nuon");
        assert_suggests("dune", "ocaml", "dune");
        assert_suggests("main.odin", "odin", "odin");
        assert_suggests("index.phtml", "php", "phtml");
        assert_suggests("profile.ps1", "powershell", "ps1");
        assert_suggests("schema.prisma", "prisma", "prisma");
        assert_suggests("api.proto", "proto", "proto");
        assert_suggests("Main.purs", "purescript", "purs");
        assert_suggests("notebook.Rmd", "r", "Rmd");
        assert_suggests("main.rkt", "racket", "rkt");
        assert_suggests("App.res", "rescript", "res");
        assert_suggests("index.rst", "rst", "rst");
        assert_suggests("Gemfile", "ruby", "Gemfile");
        assert_suggests("build.sbt", "scala", "sbt");
        assert_suggests("main.ss", "scheme", "ss");
        assert_suggests("styles.sass", "scss", "sass");
        assert_suggests("query.sql", "sql", "sql");
        assert_suggests("App.svelte", "svelte", "svelte");
        assert_suggests("Model.swiftinterface", "swift", "swiftinterface");
        assert_suggests("home.templ", "templ", "templ");
        assert_suggests("terraform.tfvars", "terraform", "tfvars");
        assert_suggests("main.typst", "typst", "typst");
        assert_suggests("App.vue", "vue", "vue");
        assert_suggests("shader.wgsl", "wgsl", "wgsl");
        assert_suggests("run.bat", "windows-batch", "bat");
        assert_suggests("Makefile", "make", "Makefile");
        assert_suggests("nginx.conf", "nginx", "nginx.conf");
        assert_suggests(
            "requirements.txt",
            "python-requirements",
            "requirements.txt",
        );
        assert_suggests("lib.wit", "wit", "wit");
        assert_suggests("pom.xml", "xml", "xml");
        assert_suggests("build.zon", "zig", "zon");
    }

    #[test]
    pub fn suggested_path_suffixes_are_unique() {
        let mut claims: HashMap<&str, &str> = HashMap::new();
        for (extension_id, path_suffixes) in SUGGESTIONS_BY_EXTENSION_ID {
            for suffix in *path_suffixes {
                let previous = claims.insert(suffix, extension_id);
                assert!(
                    previous.is_none(),
                    "duplicate suffix `{suffix}` is claimed by both `{}` and `{extension_id}`",
                    previous.unwrap_or("?"),
                );
            }
        }
    }

    #[test]
    pub fn table_is_sorted_by_extension_id() {
        assert!(
            SUGGESTIONS_BY_EXTENSION_ID
                .iter()
                .map(|(extension_id, _)| *extension_id)
                .is_sorted(),
            "suggested extensions must be sorted by id"
        );
    }
}
