use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use path::rel_path::RelPath;

const SUGGESTIONS_BY_EXTENSION_ID: &[(&str, &[&str])] = &[
    ("asciidoc", &["adoc", "asciidoc"]),
    ("astro", &["astro"]),
    ("beancount", &["bean", "beancount"]),
    ("clojure", &["bb", "clj", "cljc", "cljd", "cljs", "edn"]),
    (
        "csharp",
        &["cs", "csproj", "proj", "props", "slnx", "targets"],
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
            "app.src",
            "Emakefile",
            "erl",
            "erlang",
            "escript",
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
            ".gitattributes",
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
            "mak",
            "Makefile",
            "makefile",
            "mk",
            "OCamlMakefile",
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
            "builder",
            "cap",
            "Capfile",
            "capfile",
            "Cheffile",
            "Dangerfile",
            "Deliverfile",
            "erb",
            "Fastfile",
            "Gemfile",
            "gemspec",
            "Guardfile",
            "Gymfile",
            "Hobofile",
            "irbrc",
            "jbuilder",
            "Matchfile",
            "Podfile",
            "pryrc",
            "Puppetfile",
            "rabl",
            "rake",
            "Rakefile",
            "Rantfile",
            "rb",
            "rbs",
            "ru",
            "rxml",
            "Scanfile",
            "simplecov",
            "Snapfile",
            "Steepfile",
            "thor",
            "Thorfile",
            "Vagrantfile",
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

pub struct LanguageAdditionSuggestion {
    pub extension_id: &'static str,
    pub languages: &'static [&'static str],
    pub title: &'static str,
    pub description: &'static str,
    pub docs_url: &'static str,
    pub install_message: &'static str,
}

const SUGGESTIONS_BY_LANGUAGE: &[LanguageAdditionSuggestion] = &[LanguageAdditionSuggestion {
    extension_id: "emmet",
    languages: &[
        "Angular",
        "Blade",
        "CSS",
        "Django",
        "ERB",
        "Elixir",
        "HEEx",
        "HTML",
        "HTML+ERB",
        "JavaScript",
        "Jinja2",
        "LESS",
        "Liquid",
        "Nunjucks",
        "PHP",
        "SCSS",
        "Statamic Antlers",
        "TSX",
        "Twig",
        "Vue.js",
    ],
    title: "Emmet is available for this file",
    description: "Emmet expands abbreviations such as `ul>li*3` into HTML and `m10` into CSS.",
    docs_url: "https://zed.dev/docs/languages/emmet",
    install_message: "Install Emmet",
}];

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
pub struct SuggestedExtension {
    pub extension_id: Arc<str>,
    pub file_name_or_extension: Arc<str>,
}

/// Returns the suggested extension for the given [`Path`].
pub fn suggest_extension(path: &RelPath) -> Option<SuggestedExtension> {
    let (file_name_or_extension, extension_id) = path
        .file_name()
        .and_then(|file_name| {
            // We suggest against file names first, as these suggestions will be more
            // specific than ones based on the file extension.
            suggested_extensions()
                .get(file_name)
                .map(|suggestion| (Arc::from(file_name), suggestion))
        })
        .or_else(|| {
            path.extension().and_then(|file_extension| {
                suggested_extensions()
                    .get(file_extension)
                    .map(|suggestion| (Arc::from(file_extension), suggestion))
            })
        })?;

    Some(SuggestedExtension {
        extension_id: extension_id.clone(),
        file_name_or_extension,
    })
}

pub fn additional_suggestion_for_language(
    language_name: &str,
) -> Option<&LanguageAdditionSuggestion> {
    SUGGESTIONS_BY_LANGUAGE
        .iter()
        .find(|suggestion| suggestion.languages.contains(&language_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use path::rel_path::rel_path;

    #[test]
    pub fn test_suggested_extension() {
        assert_eq!(
            suggest_extension(rel_path("Cargo.toml")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "toml".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("Cargo.lock")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "Cargo.lock".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("Dockerfile")),
            Some(SuggestedExtension {
                extension_id: "dockerfile".into(),
                file_name_or_extension: "Dockerfile".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("a/b/c/d/.gitignore")),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                file_name_or_extension: ".gitignore".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("a/b/c/d/test.gleam")),
            Some(SuggestedExtension {
                extension_id: "gleam".into(),
                file_name_or_extension: "gleam".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("a/b/c/d/test.sol")),
            Some(SuggestedExtension {
                extension_id: "solidity".into(),
                file_name_or_extension: "sol".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("a/b/c/d/test.jl")),
            Some(SuggestedExtension {
                extension_id: "julia".into(),
                file_name_or_extension: "jl".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("script.pl")),
            Some(SuggestedExtension {
                extension_id: "perl".into(),
                file_name_or_extension: "pl".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("app/uv.lock")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "uv.lock".into()
            })
        );
        // Dotfiles have no `Path::extension`, so they match by name.
        assert_eq!(
            suggest_extension(rel_path(".envrc")),
            Some(SuggestedExtension {
                extension_id: "env".into(),
                file_name_or_extension: ".envrc".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path(".gitattributes")),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                file_name_or_extension: ".gitattributes".into()
            })
        );
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
                    previous.unwrap()
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
