use std::sync::{Arc, LazyLock};

use collections::HashMap;
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
    /// The mapping of file stems to their associated icon keys.
    pub file_stems: HashMap<String, String>,
    /// The mapping of file suffixes to their associated icon keys.
    pub file_suffixes: HashMap<String, String>,
    /// The mapping of icon keys to icon definitions.
    pub file_icons: HashMap<String, IconDefinition>,
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
    ("docker", &["Dockerfile"]),
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
    ("bicep", &["bicep"]),
    ("bun", &["lockb"]),
    ("c", &["c", "h"]),
    ("cairo", &["cairo"]),
    ("code", &["handlebars", "metadata", "rkt", "scm"]),
    ("coffeescript", &["coffee"]),
    (
        "cpp",
        &["c++", "cc", "cpp", "cxx", "hh", "hpp", "hxx", "inl", "ixx"],
    ),
    ("crystal", &["cr", "ecr"]),
    ("csharp", &["cs"]),
    ("csproj", &["csproj"]),
    ("css", &["css", "pcss", "postcss"]),
    ("cue", &["cue"]),
    ("dart", &["dart"]),
    ("diff", &["diff"]),
    (
        "document",
        &[
            "doc", "docx", "mdx", "odp", "ods", "odt", "pdf", "ppt", "pptx", "rtf", "txt", "xls",
            "xlsx",
        ],
    ),
    ("elixir", &["eex", "ex", "exs", "heex"]),
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
    ("gitlab", &["gitlab-ci.yml"]),
    ("gleam", &["gleam"]),
    ("go", &["go", "mod", "work"]),
    ("graphql", &["gql", "graphql", "graphqls"]),
    ("haskell", &["hs"]),
    ("hcl", &["hcl"]),
    ("html", &["htm", "html"]),
    (
        "image",
        &[
            "avif", "bmp", "gif", "heic", "heif", "ico", "j2k", "jfif", "jp2", "jpeg", "jpg",
            "jxl", "png", "psd", "qoi", "svg", "tiff", "webp",
        ],
    ),
    ("java", &["java"]),
    ("javascript", &["cjs", "js", "mjs"]),
    ("json", &["json"]),
    ("julia", &["jl"]),
    ("kdl", &["kdl"]),
    ("kotlin", &["kt"]),
    ("lock", &["lock"]),
    ("log", &["log"]),
    ("lua", &["lua"]),
    ("luau", &["luau"]),
    ("markdown", &["markdown", "md"]),
    ("metal", &["metal"]),
    ("nim", &["nim"]),
    ("nix", &["nix"]),
    ("ocaml", &["ml", "mli"]),
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
    ("settings", &["conf", "ini", "yaml", "yml"]),
    ("solidity", &["sol"]),
    (
        "storage",
        &[
            "accdb", "csv", "dat", "db", "dbf", "dll", "fmp", "fp7", "frm", "gdb", "ib", "jsonc",
            "ldf", "mdb", "mdf", "myd", "myi", "pdb", "RData", "rdata", "sav", "sdf", "sql",
            "sqlite", "tsv",
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
    ("zig", &["zig"]),
];

/// A mapping of a file type identifier to its corresponding icon.
const FILE_ICONS: &[(&str, &str)] = &[
    ("astro", "icons/file_icons/astro.svg"),
    ("audio", "icons/file_icons/audio.svg"),
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
    ("elixir", "icons/file_icons/elixir.svg"),
    ("elm", "icons/file_icons/elm.svg"),
    ("erlang", "icons/file_icons/erlang.svg"),
    ("eslint", "icons/file_icons/eslint.svg"),
    ("font", "icons/file_icons/font.svg"),
    ("fsharp", "icons/file_icons/fsharp.svg"),
    ("fsproj", "icons/file_icons/file.svg"),
    ("gitlab", "icons/file_icons/settings.svg"),
    ("gleam", "icons/file_icons/gleam.svg"),
    ("go", "icons/file_icons/go.svg"),
    ("graphql", "icons/file_icons/graphql.svg"),
    ("haskell", "icons/file_icons/haskell.svg"),
    ("hcl", "icons/file_icons/hcl.svg"),
    ("heroku", "icons/file_icons/heroku.svg"),
    ("html", "icons/file_icons/html.svg"),
    ("image", "icons/file_icons/image.svg"),
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
pub(crate) const DEFAULT_ICON_THEME_NAME: &str = "Zed (Default)";

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
