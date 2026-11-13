use std::path::Path;
use std::sync::{Arc, LazyLock};

use collections::HashMap;
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use gpui::SharedString;

use crate::Appearance;

/// A family of icon themes.
pub struct IconThemeFamily {
    /// The unique ID for the icon theme family.
    pub id: String,
    /// The name of the icon theme family.
    pub name: SharedString,
    /// The author of the icon theme family.
    pub author: SharedString,
    /// The list of icon themes in the family.
    pub themes: Vec<IconTheme>,
}

/// An icon theme.
#[derive(Debug, PartialEq)]
pub struct IconTheme {
    /// The unique ID for the icon theme.
    pub id: String,
    /// The name of the icon theme.
    pub name: SharedString,
    /// The appearance of the icon theme (e.g., light or dark).
    pub appearance: Appearance,
    /// The icons used for directories.
    pub directory_icons: DirectoryIcons,
    /// The icons used for named directories.
    pub named_directory_icons: HashMap<String, DirectoryIcons>,
    /// The icons used for chevrons.
    pub chevron_icons: ChevronIcons,
    /// The mapping of file path patterns to their associated icon keys.
    pub file_patterns: FilePatterns,
    /// The mapping of file stems to their associated icon keys.
    pub file_stems: HashMap<String, String>,
    /// The mapping of file suffixes to their associated icon keys.
    pub file_suffixes: HashMap<String, String>,
    /// The mapping of icon keys to icon definitions.
    pub file_icons: HashMap<String, IconDefinition>,
}

/// Icon keys for files whose path matches a glob pattern.
///
/// Patterns are kept in the order the icon theme declares them, so that when
/// several patterns match the same path, the first one wins.
#[derive(Debug, Default)]
pub struct FilePatterns {
    patterns: Vec<String>,
    icon_keys: Vec<String>,
    glob_set: GlobSet,
}

impl FilePatterns {
    /// Compiles the given `(pattern, icon key)` pairs.
    ///
    /// A pattern that does not start with `**` is matched at any depth, as if
    /// it started with `**/`, because callers pass paths relative to a
    /// worktree, prefixed with the worktree name, or absolute. `*` does not
    /// cross a `/`. Invalid patterns are skipped and logged.
    pub fn new(patterns: impl IntoIterator<Item = (String, String)>) -> Self {
        let mut builder = GlobSetBuilder::new();
        let mut kept_patterns = Vec::new();
        let mut icon_keys = Vec::new();
        for (pattern, icon_key) in patterns {
            let anchored = if pattern.starts_with("**") {
                pattern.clone()
            } else {
                format!("**/{}", pattern.trim_start_matches('/'))
            };
            match GlobBuilder::new(&anchored).literal_separator(true).build() {
                Ok(glob) => {
                    builder.add(glob);
                    kept_patterns.push(pattern);
                    icon_keys.push(icon_key);
                }
                Err(error) => {
                    log::warn!("skipping invalid file pattern {pattern:?} in icon theme: {error}");
                }
            }
        }

        let glob_set = builder.build().unwrap_or_else(|error| {
            log::warn!("failed to compile file patterns of icon theme: {error}");
            kept_patterns.clear();
            icon_keys.clear();
            GlobSet::empty()
        });

        Self {
            patterns: kept_patterns,
            icon_keys,
            glob_set,
        }
    }

    /// Returns the icon keys of the patterns matching `path`, in declaration order.
    pub fn icon_keys_for<'a>(&'a self, path: &Path) -> impl Iterator<Item = &'a str> + 'a {
        let matches = if self.icon_keys.is_empty() {
            Vec::new()
        } else {
            self.glob_set.matches(path)
        };
        matches
            .into_iter()
            .map(|index| self.icon_keys[index].as_str())
    }
}

impl PartialEq for FilePatterns {
    fn eq(&self, other: &Self) -> bool {
        self.patterns == other.patterns && self.icon_keys == other.icon_keys
    }
}

/// The icons used for directories.
#[derive(Debug, PartialEq, Clone)]
pub struct DirectoryIcons {
    /// The path to the icon to use for a collapsed directory.
    pub collapsed: Option<SharedString>,
    /// The path to the icon to use for an expanded directory.
    pub expanded: Option<SharedString>,
}

/// The icons used for chevrons.
#[derive(Debug, PartialEq)]
pub struct ChevronIcons {
    /// The path to the icon to use for a collapsed chevron.
    pub collapsed: Option<SharedString>,
    /// The path to the icon to use for an expanded chevron.
    pub expanded: Option<SharedString>,
}

/// An icon definition.
#[derive(Debug, PartialEq)]
pub struct IconDefinition {
    /// The path to the icon file.
    pub path: SharedString,
}

const FILE_STEMS_BY_ICON_KEY: &[(&str, &[&str])] = &[
    ("docker", &["Containerfile", "Dockerfile", ".dockerignore"]),
    ("ruby", &["Podfile"]),
    ("heroku", &["Procfile"]),
];

const FILE_SUFFIXES_BY_ICON_KEY: &[(&str, &[&str])] = &[
    ("astro", &["astro"]),
    (
        "audio",
        &[
            "aac", "flac", "m4a", "mka", "mp3", "ogg", "opus", "wav", "wma", "wv",
        ],
    ),
    ("backup", &["bak"]),
    ("ballerina", &["bal"]),
    ("bicep", &["bicep"]),
    ("bun", &["lockb"]),
    ("c", &["c", "h"]),
    ("cairo", &["cairo"]),
    ("code", &["handlebars", "metadata", "rkt", "scm"]),
    ("coffeescript", &["coffee"]),
    (
        "cpp",
        &[
            "c++", "h++", "cc", "cpp", "cppm", "cxx", "hh", "hpp", "hxx", "inl", "ixx",
        ],
    ),
    ("crystal", &["cr", "ecr"]),
    ("csharp", &["cs"]),
    ("csproj", &["csproj"]),
    ("css", &["css", "pcss", "postcss"]),
    ("cue", &["cue"]),
    ("dart", &["dart"]),
    ("diff", &["diff"]),
    (
        "docker",
        &[
            "docker-compose.yml",
            "docker-compose.yaml",
            "compose.yml",
            "compose.yaml",
        ],
    ),
    (
        "document",
        &[
            "doc", "docx", "mdx", "odp", "ods", "odt", "pdf", "ppt", "pptx", "rtf", "txt", "xls",
            "xlsx",
        ],
    ),
    ("editorconfig", &["editorconfig"]),
    ("elixir", &["eex", "ex", "exs", "heex", "leex", "neex"]),
    ("elm", &["elm"]),
    (
        "erlang",
        &[
            "Emakefile",
            "app.src",
            "erl",
            "escript",
            "hrl",
            "rebar.config",
            "xrl",
            "yrl",
        ],
    ),
    (
        "eslint",
        &[
            "eslint.config.cjs",
            "eslint.config.cts",
            "eslint.config.js",
            "eslint.config.mjs",
            "eslint.config.mts",
            "eslint.config.ts",
            "eslintrc",
            "eslintrc.js",
            "eslintrc.json",
        ],
    ),
    ("font", &["otf", "ttf", "woff", "woff2"]),
    ("fsharp", &["fs"]),
    ("fsproj", &["fsproj"]),
    ("gitlab", &["gitlab-ci.yml", "gitlab-ci.yaml"]),
    ("gleam", &["gleam"]),
    ("go", &["go", "mod", "work"]),
    ("graphql", &["gql", "graphql", "graphqls"]),
    ("haskell", &["hs"]),
    ("hcl", &["hcl"]),
    (
        "helm",
        &[
            "helmfile.yaml",
            "helmfile.yml",
            "Chart.yaml",
            "Chart.yml",
            "Chart.lock",
            "values.yaml",
            "values.yml",
            "requirements.yaml",
            "requirements.yml",
            "tpl",
        ],
    ),
    ("html", &["htm", "html"]),
    (
        "image",
        &[
            "avif", "bmp", "gif", "heic", "heif", "ico", "j2k", "jfif", "jp2", "jpeg", "jpg",
            "jxl", "png", "psd", "qoi", "svg", "tiff", "webp",
        ],
    ),
    ("ipynb", &["ipynb"]),
    ("java", &["java"]),
    ("javascript", &["cjs", "js", "mjs"]),
    ("json", &["json", "jsonc"]),
    ("julia", &["jl"]),
    ("kdl", &["kdl"]),
    ("kotlin", &["kt"]),
    ("lock", &["lock"]),
    ("log", &["log"]),
    ("lua", &["lua"]),
    ("luau", &["luau"]),
    ("markdown", &["markdown", "md"]),
    ("metal", &["metal"]),
    ("nim", &["nim", "nims", "nimble"]),
    ("nix", &["nix"]),
    ("ocaml", &["ml", "mli", "mlx"]),
    ("odin", &["odin"]),
    ("php", &["php"]),
    (
        "prettier",
        &[
            "prettier.config.cjs",
            "prettier.config.js",
            "prettier.config.mjs",
            "prettierignore",
            "prettierrc",
            "prettierrc.cjs",
            "prettierrc.js",
            "prettierrc.json",
            "prettierrc.json5",
            "prettierrc.mjs",
            "prettierrc.toml",
            "prettierrc.yaml",
            "prettierrc.yml",
        ],
    ),
    ("prisma", &["prisma"]),
    ("puppet", &["pp"]),
    ("python", &["py"]),
    ("r", &["r", "R"]),
    ("react", &["cjsx", "ctsx", "jsx", "mjsx", "mtsx", "tsx"]),
    ("roc", &["roc"]),
    ("ruby", &["rb"]),
    ("rust", &["rs"]),
    ("sass", &["sass", "scss"]),
    ("scala", &["scala", "sc"]),
    ("settings", &["conf", "ini"]),
    ("solidity", &["sol"]),
    (
        "storage",
        &[
            "accdb", "csv", "dat", "db", "dbf", "dll", "fmp", "fp7", "frm", "gdb", "ib", "ldf",
            "mdb", "mdf", "myd", "myi", "pdb", "psv", "RData", "rdata", "sav", "sdf", "sql",
            "sqlite", "ssv", "tsv",
        ],
    ),
    (
        "stylelint",
        &[
            "stylelint.config.cjs",
            "stylelint.config.js",
            "stylelint.config.mjs",
            "stylelintignore",
            "stylelintrc",
            "stylelintrc.cjs",
            "stylelintrc.js",
            "stylelintrc.json",
            "stylelintrc.mjs",
            "stylelintrc.yaml",
            "stylelintrc.yml",
        ],
    ),
    ("surrealql", &["surql"]),
    ("svelte", &["svelte"]),
    ("swift", &["swift"]),
    ("tcl", &["tcl"]),
    ("template", &["hbs", "plist", "xml"]),
    (
        "terminal",
        &[
            "bash",
            "bash_aliases",
            "bash_login",
            "bash_logout",
            "bash_profile",
            "bashrc",
            "brushrc",
            "fish",
            "nu",
            "profile",
            "ps1",
            "sh",
            "zlogin",
            "zlogout",
            "zprofile",
            "zsh",
            "zsh_aliases",
            "zsh_histfile",
            "zsh_history",
            "zshenv",
            "zshrc",
        ],
    ),
    ("terraform", &["tf", "tfvars"]),
    ("toml", &["toml"]),
    ("typescript", &["cts", "mts", "ts"]),
    ("v", &["v", "vsh", "vv"]),
    (
        "vcs",
        &[
            "COMMIT_EDITMSG",
            "EDIT_DESCRIPTION",
            "MERGE_MSG",
            "NOTES_EDITMSG",
            "TAG_EDITMSG",
            "gitattributes",
            "gitignore",
            "gitkeep",
            "gitmodules",
        ],
    ),
    ("vbproj", &["vbproj"]),
    ("video", &["avi", "m4v", "mkv", "mov", "mp4", "webm", "wmv"]),
    ("vs_sln", &["sln"]),
    ("vs_suo", &["suo"]),
    ("vue", &["vue"]),
    ("vyper", &["vy", "vyi"]),
    ("wgsl", &["wgsl"]),
    ("yaml", &["yaml", "yml"]),
    ("zig", &["zig"]),
];

/// A mapping of a file type identifier to its corresponding icon.
const FILE_ICONS: &[(&str, &str)] = &[
    ("astro", "icons/file_icons/astro.svg"),
    ("audio", "icons/file_icons/audio.svg"),
    ("ballerina", "icons/file_icons/ballerina.svg"),
    ("bicep", "icons/file_icons/file.svg"),
    ("bun", "icons/file_icons/bun.svg"),
    ("c", "icons/file_icons/c.svg"),
    ("cairo", "icons/file_icons/cairo.svg"),
    ("code", "icons/file_icons/code.svg"),
    ("coffeescript", "icons/file_icons/coffeescript.svg"),
    ("cpp", "icons/file_icons/cpp.svg"),
    ("crystal", "icons/file_icons/file.svg"),
    ("csharp", "icons/file_icons/file.svg"),
    ("csproj", "icons/file_icons/file.svg"),
    ("css", "icons/file_icons/css.svg"),
    ("cue", "icons/file_icons/file.svg"),
    ("dart", "icons/file_icons/dart.svg"),
    ("default", "icons/file_icons/file.svg"),
    ("diff", "icons/file_icons/diff.svg"),
    ("docker", "icons/file_icons/docker.svg"),
    ("document", "icons/file_icons/book.svg"),
    ("editorconfig", "icons/file_icons/editorconfig.svg"),
    ("elixir", "icons/file_icons/elixir.svg"),
    ("elm", "icons/file_icons/elm.svg"),
    ("erlang", "icons/file_icons/erlang.svg"),
    ("eslint", "icons/file_icons/eslint.svg"),
    ("font", "icons/file_icons/font.svg"),
    ("fsharp", "icons/file_icons/fsharp.svg"),
    ("fsproj", "icons/file_icons/file.svg"),
    ("gitlab", "icons/file_icons/gitlab.svg"),
    ("gleam", "icons/file_icons/gleam.svg"),
    ("go", "icons/file_icons/go.svg"),
    ("graphql", "icons/file_icons/graphql.svg"),
    ("haskell", "icons/file_icons/haskell.svg"),
    ("hcl", "icons/file_icons/hcl.svg"),
    ("helm", "icons/file_icons/helm.svg"),
    ("heroku", "icons/file_icons/heroku.svg"),
    ("html", "icons/file_icons/html.svg"),
    ("image", "icons/file_icons/image.svg"),
    ("ipynb", "icons/file_icons/jupyter.svg"),
    ("java", "icons/file_icons/java.svg"),
    ("javascript", "icons/file_icons/javascript.svg"),
    ("json", "icons/file_icons/code.svg"),
    ("julia", "icons/file_icons/julia.svg"),
    ("kdl", "icons/file_icons/kdl.svg"),
    ("kotlin", "icons/file_icons/kotlin.svg"),
    ("lock", "icons/file_icons/lock.svg"),
    ("log", "icons/file_icons/info.svg"),
    ("lua", "icons/file_icons/lua.svg"),
    ("luau", "icons/file_icons/luau.svg"),
    ("markdown", "icons/file_icons/book.svg"),
    ("metal", "icons/file_icons/metal.svg"),
    ("nim", "icons/file_icons/nim.svg"),
    ("nix", "icons/file_icons/nix.svg"),
    ("ocaml", "icons/file_icons/ocaml.svg"),
    ("odin", "icons/file_icons/odin.svg"),
    ("phoenix", "icons/file_icons/phoenix.svg"),
    ("php", "icons/file_icons/php.svg"),
    ("prettier", "icons/file_icons/prettier.svg"),
    ("prisma", "icons/file_icons/prisma.svg"),
    ("puppet", "icons/file_icons/puppet.svg"),
    ("python", "icons/file_icons/python.svg"),
    ("r", "icons/file_icons/r.svg"),
    ("react", "icons/file_icons/react.svg"),
    ("roc", "icons/file_icons/roc.svg"),
    ("ruby", "icons/file_icons/ruby.svg"),
    ("rust", "icons/file_icons/rust.svg"),
    ("sass", "icons/file_icons/sass.svg"),
    ("scala", "icons/file_icons/scala.svg"),
    ("settings", "icons/file_icons/settings.svg"),
    ("solidity", "icons/file_icons/file.svg"),
    ("storage", "icons/file_icons/database.svg"),
    ("stylelint", "icons/file_icons/javascript.svg"),
    ("surrealql", "icons/file_icons/surrealql.svg"),
    ("svelte", "icons/file_icons/html.svg"),
    ("swift", "icons/file_icons/swift.svg"),
    ("tcl", "icons/file_icons/tcl.svg"),
    ("template", "icons/file_icons/html.svg"),
    ("terminal", "icons/file_icons/terminal.svg"),
    ("terraform", "icons/file_icons/terraform.svg"),
    ("toml", "icons/file_icons/toml.svg"),
    ("typescript", "icons/file_icons/typescript.svg"),
    ("v", "icons/file_icons/v.svg"),
    ("vbproj", "icons/file_icons/file.svg"),
    ("vcs", "icons/file_icons/git.svg"),
    ("video", "icons/file_icons/video.svg"),
    ("vs_sln", "icons/file_icons/file.svg"),
    ("vs_suo", "icons/file_icons/file.svg"),
    ("vue", "icons/file_icons/vue.svg"),
    ("vyper", "icons/file_icons/vyper.svg"),
    ("wgsl", "icons/file_icons/wgsl.svg"),
    ("yaml", "icons/file_icons/yaml.svg"),
    ("zig", "icons/file_icons/zig.svg"),
];

/// Returns a mapping of file associations to icon keys.
fn icon_keys_by_association(
    associations_by_icon_key: &[(&str, &[&str])],
) -> HashMap<String, String> {
    let mut icon_keys_by_association = HashMap::default();
    for (icon_key, associations) in associations_by_icon_key {
        for association in *associations {
            icon_keys_by_association.insert(association.to_string(), icon_key.to_string());
        }
    }

    icon_keys_by_association
}

/// The name of the default icon theme.
pub const DEFAULT_ICON_THEME_NAME: &str = "Zed (Default)";

static DEFAULT_ICON_THEME: LazyLock<Arc<IconTheme>> = LazyLock::new(|| {
    Arc::new(IconTheme {
        id: "zed".into(),
        name: DEFAULT_ICON_THEME_NAME.into(),
        appearance: Appearance::Dark,
        directory_icons: DirectoryIcons {
            collapsed: Some("icons/file_icons/folder.svg".into()),
            expanded: Some("icons/file_icons/folder_open.svg".into()),
        },
        named_directory_icons: HashMap::default(),
        chevron_icons: ChevronIcons {
            collapsed: Some("icons/file_icons/chevron_right.svg".into()),
            expanded: Some("icons/file_icons/chevron_down.svg".into()),
        },
        file_patterns: FilePatterns::default(),
        file_stems: icon_keys_by_association(FILE_STEMS_BY_ICON_KEY),
        file_suffixes: icon_keys_by_association(FILE_SUFFIXES_BY_ICON_KEY),
        file_icons: HashMap::from_iter(FILE_ICONS.iter().map(|(ty, path)| {
            (
                ty.to_string(),
                IconDefinition {
                    path: (*path).into(),
                },
            )
        })),
    })
});

/// Returns the default icon theme.
pub fn default_icon_theme() -> Arc<IconTheme> {
    DEFAULT_ICON_THEME.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patterns(pairs: &[(&str, &str)]) -> FilePatterns {
        FilePatterns::new(
            pairs
                .iter()
                .map(|(pattern, key)| (pattern.to_string(), key.to_string())),
        )
    }

    fn keys(patterns: &FilePatterns, path: &str) -> Vec<String> {
        patterns
            .icon_keys_for(Path::new(path))
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn test_matches_in_declaration_order() {
        let patterns = patterns(&[
            ("app/models/**/*.rb", "model"),
            ("**/*_controller.rb", "controller"),
            ("app/**/*.rb", "app"),
        ]);

        assert_eq!(keys(&patterns, "app/models/user.rb"), ["model", "app"]);
        assert_eq!(
            keys(&patterns, "app/controllers/users_controller.rb"),
            ["controller", "app"]
        );
        assert_eq!(keys(&patterns, "app/helpers/users_helper.rb"), ["app"]);
        assert!(keys(&patterns, "config/routes.rb").is_empty());
    }

    #[test]
    fn test_patterns_match_at_any_depth() {
        let patterns = patterns(&[("app/models/**/*.rb", "model"), ("/lib/*.rb", "lib")]);

        // Worktree-relative, prefixed with the worktree name, nested (an engine).
        assert_eq!(keys(&patterns, "app/models/user.rb"), ["model"]);
        assert_eq!(keys(&patterns, "my-app/app/models/user.rb"), ["model"]);
        assert_eq!(
            keys(&patterns, "engines/billing/app/models/invoice.rb"),
            ["model"]
        );
        assert_eq!(keys(&patterns, "app/models/concerns/named.rb"), ["model"]);
        // A leading slash does not anchor to anything: paths are not absolute.
        assert_eq!(keys(&patterns, "lib/tasks.rb"), ["lib"]);
        // Only the file name: directory patterns have nothing to match.
        assert!(keys(&patterns, "user.rb").is_empty());
    }

    #[test]
    fn test_single_star_does_not_cross_directories() {
        let patterns = patterns(&[("app/*.rb", "app")]);

        assert_eq!(keys(&patterns, "app/application.rb"), ["app"]);
        assert!(keys(&patterns, "app/models/user.rb").is_empty());
    }

    #[test]
    fn test_matching_is_case_sensitive() {
        let patterns = patterns(&[("app/models/*.rb", "model")]);

        assert!(keys(&patterns, "App/Models/user.rb").is_empty());
    }

    #[test]
    fn test_invalid_patterns_are_skipped() {
        let patterns = patterns(&[("app/[models/*.rb", "broken"), ("**/*_job.rb", "job")]);

        assert_eq!(keys(&patterns, "app/jobs/sync_job.rb"), ["job"]);
        assert_eq!(patterns, self::patterns(&[("**/*_job.rb", "job")]));
    }

    #[test]
    fn test_no_patterns_match_nothing() {
        assert!(keys(&FilePatterns::default(), "app/models/user.rb").is_empty());
        assert!(keys(&patterns(&[]), "app/models/user.rb").is_empty());
    }
}
