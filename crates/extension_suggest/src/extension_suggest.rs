use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use path::rel_path::RelPath;

const SUGGESTIONS_BY_EXTENSION_ID: &[(&str, &[&str])] = &[
    ("actionscript", &["as"]),
    ("ada", &["adb", "ads", "gpr"]),
    ("agda", &["agda"]),
    ("aiken", &["ak"]),
    ("amber", &["ab"]),
    ("angular", &["component.html", "ng.html"]),
    ("ansible", &["ansible"]),
    ("apache-avro", &["avdl"]),
    ("arduino", &["ino"]),
    ("arkts", &["ets"]),
    ("asciidoc", &["ad", "adoc", "asc", "asciidoc"]),
    ("asn1", &["mib"]),
    ("ass", &["ass", "ssa"]),
    ("assembly", &["S", "asm", "s"]),
    ("astro", &["astro"]),
    ("authzed", &["zed"]),
    ("autohotkey", &["ahk"]),
    ("awk", &["awk"]),
    ("baml", &["baml"]),
    ("beancount", &["bean", "beancount"]),
    ("bend", &["bend"]),
    ("bicep", &["bicep", "bicepparam"]),
    ("biome", &["grit"]),
    ("bison", &["y", "yy"]),
    ("bitbake", &["bbappend", "bbclass", "inc"]),
    ("blade", &["blade.php"]),
    ("blueprint", &["blp"]),
    ("bluespec-systemverilog", &["bsv"]),
    ("bqn", &["bqn"]),
    ("brainfuck", &["bf"]),
    ("bsl", &["bsl"]),
    ("c3", &["c3", "c3i"]),
    ("caddyfile", &["Caddyfile", "caddyfile"]),
    ("cadence", &["cdc"]),
    ("cairo", &["cairo"]),
    ("capnp", &["capnp"]),
    ("cds-lsp", &["cds"]),
    ("cedar", &["cedar"]),
    (
        "cfengine",
        &["cf", "cf.sub", "cf3", "cfengine", "cfengine3"],
    ),
    ("cfml", &["cfc", "cfm", "cfml", "cfs"]),
    ("cherri", &["cherri"]),
    ("circom", &["circom"]),
    ("clarity", &["clar"]),
    ("clojure", &["bb", "clj", "cljc", "cljd", "cljs", "edn"]),
    ("cobol", &["cbl", "cob"]),
    ("codeowners", &["CODEOWNERS", "CODEOWNERS.txt"]),
    ("coffeescript", &["coffee"]),
    ("coi", &["coi", "d.coi"]),
    ("conl", &["conl"]),
    ("cooklang", &["cook"]),
    ("cpp2", &["cpp2", "h2"]),
    ("crystal", &["cr", "ecr"]),
    (
        "csharp",
        &["cs", "csproj", "proj", "props", "slnx", "targets"],
    ),
    ("csound", &["csd", "orc", "sco", "udo"]),
    ("csv", &["csv"]),
    ("cucumber", &["feature", "gherkin"]),
    ("cue", &["cue"]),
    ("curry", &["curry"]),
    ("cylc", &["cylc"]),
    ("cypher", &["cql", "cyp", "cypher"]),
    ("cython", &["pxd", "pxi", "pyx"]),
    ("d", &["d", "dd", "di"]),
    ("d2", &["d2"]),
    ("dafny", &["dfy"]),
    ("dang", &["dang"]),
    ("dart", &["dart"]),
    ("dbml", &["dbml"]),
    ("defold", &["gui_script", "render_script", "script"]),
    ("desktop", &["desktop", "directory"]),
    ("devicetree", &["dts", "dtsi", "dtso", "its", "overlay"]),
    ("django", &["dj.html", "dj.md", "dj.txt"]),
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
    ("duckyscript", &["txt"]),
    ("duper", &["duper"]),
    ("earthfile", &["Earthfile"]),
    ("edge", &["edge"]),
    ("edi", &["x12"]),
    ("editorconfig", &["editorconfig"]),
    ("ejs", &["ejs", "eta"]),
    ("elisp", &["el"]),
    (
        "elixir",
        &[
            "eex", "ex", "exs", "heex", "html.eex", "leex", "mix.lock", "neex",
        ],
    ),
    ("elle", &["le"]),
    ("elm", &["elm"]),
    ("ember", &["gjs", "gts", "handlebars", "hbs"]),
    ("emmylua", &["lua.txt"]),
    ("env", &["env", "envrc", "example", "local", "test"]),
    (
        "erlang",
        &[
            "Emakefile",
            "app.src",
            "erl",
            "erlang",
            "escript",
            "hrl",
            "rebar.config",
            "xrl",
            "yrl",
        ],
    ),
    ("exograph", &["exo"]),
    ("fennel", &["fnl"]),
    ("ferret", &["fer", "fer.ret", "ferret.lock"]),
    (
        "firebase-security-rules",
        &["firebase.rules", "rules", "storage.rules"],
    ),
    ("fish", &["fish"]),
    ("flatbuffers", &["fbs"]),
    (
        "fortran",
        &[
            "F", "F03", "F08", "F90", "F95", "f", "f03", "f08", "f90", "f95",
        ],
    ),
    ("fountain", &["fountain", "spmd"]),
    ("freemarker", &["ftl"]),
    ("fsharp", &["fs", "fsi", "fsscript", "fsx"]),
    ("fsm", &["fsm"]),
    (
        "gcode",
        &[
            "001", "anc", "apt", "aptcl", "bfb", "cnc", "din", "dnc", "ecs", "eia", "fan", "fgc",
            "fnc", "g", "g00", "gc", "gcd", "gco", "gcode", "gp", "hnc", "knc", "lib", "min",
            "mmg", "mpf", "mpt", "nc", "ncd", "ncf", "ncg", "nci", "ncp", "ngc", "out", "pim",
            "pit", "plt", "ply", "prg", "pu1", "rol", "sbp", "spf", "ssb", "sub", "tap", "tcn",
            "xpi",
        ],
    ),
    (
        "gdscript",
        &[
            "gd",
            "gdextension",
            "gdshader",
            "gdshaderinc",
            "godot",
            "import",
            "tres",
            "tscn",
        ],
    ),
    (
        "ghostty",
        &[
            "com.mitchellh.ghostty/config",
            "config/ghostty/config",
            "ghostty",
            "ghostty/config",
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
            "comp", "frag", "geom", "glsl", "mesh", "rahit", "rcall", "rchit", "rgen", "rint",
            "rmiss", "task", "tesc", "tese", "vert",
        ],
    ),
    ("gosum", &["go.sum"]),
    (
        "gotmpl",
        &[
            "go.tpl",
            "go.txt",
            "gohtml",
            "gotmpl",
            "gtpl",
            "html.gotmpl",
            "html.gotpl",
            "txt.gotmpl",
            "txt.gotpl",
        ],
    ),
    ("graphql", &["gql", "graphql", "graphqls"]),
    ("graphviz", &["DOT", "dot", "gv"]),
    ("gren", &["gren"]),
    ("grit", &["gritqlsnippet"]),
    ("groovy", &["JenkinsFile", "Jenkinsfile", "groovy"]),
    ("groq", &["groq"]),
    ("haml", &["haml", "html.haml"]),
    ("hare", &["ha"]),
    ("haskell", &["cabal", "hs", "lhs", "x"]),
    ("haxe", &["hx", "hxml"]),
    (
        "helm",
        &[
            ".helmignore",
            "helmfile.yaml",
            "helmfile.yml",
            "tpl",
            "yaml.gotmpl",
            "yml.gotmpl",
        ],
    ),
    ("hlsl", &["hlsl"]),
    ("hocon", &["hocon"]),
    ("hp42s", &["42s", "hp42s"]),
    ("html", &["htm", "html", "shtml"]),
    ("html-jinja", &["j2"]),
    ("http", &["http"]),
    ("huff", &["huff"]),
    ("hurl", &["hurl"]),
    (
        "hyprlang",
        &[
            "hl",
            "hyprland.conf",
            "hyprland.hl",
            "hyprlandd.conf",
            "hyprlandd.hl",
        ],
    ),
    ("ical", &["ics"]),
    ("inform6", &["h"]),
    (
        "ini",
        &[
            "automount",
            "build",
            "cfg",
            "conf",
            "container",
            "image",
            "inf",
            "ini",
            "kube",
            "mount",
            "network",
            "path",
            "pod",
            "scope",
            "service",
            "slice",
            "socket",
            "swap",
            "target",
            "timer",
            "volume",
        ],
    ),
    ("ink", &["ink"]),
    ("ion", &["ion", "isl"]),
    ("isle", &["isle"]),
    ("jai", &["jai"]),
    ("janet", &["janet"]),
    ("java", &["gradle", "gradle.kts", "java", "properties"]),
    ("jdl", &["jdl"]),
    ("jinja2", &["jinja", "jinja2"]),
    ("jq", &["jq"]),
    ("json5", &["json5"]),
    ("jsonl", &["jsonl", "ndjson"]),
    ("jsonnet", &["jsonnet", "libsonnet"]),
    ("jsp", &["jsp", "jspf", "tag"]),
    ("julia", &["jl"]),
    ("just", &["JUSTFILE", "Justfile", "just", "justfile"]),
    ("kcl", &["k"]),
    ("kconfig", &["Kconfig"]),
    ("kdl", &["kdl"]),
    ("kotlin", &["kt", "kts"]),
    ("koto", &["koto"]),
    (
        "latex",
        &["bib", "biblatex", "bibtex", "cls", "latex", "sty", "tex"],
    ),
    ("latte", &["latte"]),
    ("lean4", &["lean"]),
    ("ledger", &["journal", "ldg", "ldgr", "ledger"]),
    ("less", &["less"]),
    ("lilypond", &["ily", "ly"]),
    ("linkerscript", &["ld"]),
    ("liquid", &["liquid"]),
    ("llvm-ir", &["ll"]),
    ("log", &["log"]),
    ("logcat", &["logcat"]),
    ("lox", &["lox"]),
    ("lua", &["lua"]),
    ("luau", &["luau"]),
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
    ("matlab", &["m"]),
    ("mcfunction", &["mcfunction"]),
    ("mdx", &["mdx"]),
    ("mermaid", &["mermaid", "mmd"]),
    (
        "meson",
        &["meson.build", "meson.options", "meson_options.txt"],
    ),
    ("microscript", &["ms"]),
    ("moonbit", &["mbt"]),
    ("motoko", &["did", "mo"]),
    ("move", &["move"]),
    ("mustache", &["mustache"]),
    ("navi", &["navi", "nv", "nvs"]),
    ("neocmake", &["CMakeLists.txt", "cmake"]),
    ("netlinx", &["axb", "axi", "axs"]),
    ("nginx", &["nginx.conf"]),
    ("nickel", &["ncl"]),
    ("nim", &["nim", "nim_format_string", "nims"]),
    ("nix", &["nix"]),
    ("noir", &["nr"]),
    ("nomad", &["nomad", "nomad.hcl"]),
    ("nu", &["nu", "nuon"]),
    ("numscript", &["num", "numscript"]),
    ("nunjucks", &["njk"]),
    ("oat", &["oat"]),
    ("objective-c", &["hh", "mm"]),
    ("objectscript", &["int", "mac", "rtn"]),
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
    ("omnetpp", &["msg", "ned"]),
    ("openfga", &["fga"]),
    ("openscad", &["scad"]),
    ("org", &["org"]),
    ("oso", &["polar"]),
    ("pact", &["pact", "repl"]),
    ("pascal", &["dpr", "lpr", "p", "pas"]),
    ("path-of-exile", &["filter"]),
    ("pbxproj", &["pbxproj", "strings"]),
    ("perl", &["pl", "pm", "t"]),
    ("perm", &["perm"]),
    ("pest", &["pest"]),
    ("php", &["php", "phpt", "phtml"]),
    ("pica200", &["pica"]),
    ("pkl", &["pcf", "pkl"]),
    ("plantuml", &["iuml", "plantuml", "pu", "puml", "wsd"]),
    ("playdate", &["animation.txt", "pdxinfo"]),
    ("po", &["po", "pot"]),
    ("pony", &["pony"]),
    ("powershell", &["ps1", "psm1"]),
    ("prisma", &["prisma"]),
    ("processing", &["pde"]),
    ("prolog", &["P", "pro"]),
    ("proto", &["proto"]),
    ("pug", &["jade", "pug"]),
    ("puppet", &["epp", "pp"]),
    ("purescript", &["purs"]),
    (
        "python-requirements",
        &["constraints.txt", "requirements.txt"],
    ),
    ("qml", &["qml"]),
    ("quadlet", &["artifact"]),
    ("quakec", &["qc"]),
    ("quarto", &["qmd"]),
    ("r", &["R", "r"]),
    ("racket", &["rkt"]),
    ("rainbow-csv", &["tsv"]),
    ("rcl", &["rcl"]),
    ("red", &["red"]),
    ("redscript", &["reds"]),
    ("regedit", &["reg"]),
    ("rescript", &["res", "resi"]),
    ("rhai", &["rhai"]),
    ("risor", &["risor"]),
    ("roc", &["roc"]),
    ("ron", &["ron"]),
    ("roto", &["roto"]),
    ("rpmspec", &["spec"]),
    ("rshtml", &["rs.html"]),
    ("rst", &["rst"]),
    (
        "ruby",
        &[
            "Appfile",
            "Appraisals",
            "Berksfile",
            "Berksfile.lock",
            "Brewfile",
            "Capfile",
            "Cheffile",
            "Dangerfile",
            "Deliverfile",
            "Fastfile",
            "Gemfile",
            "Guardfile",
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
            "html.erb",
            "irbrc",
            "jbuilder",
            "js.erb",
            "pryrc",
            "rabl",
            "rake",
            "rb",
            "rbi",
            "rbs",
            "ru",
            "rxml",
            "simplecov",
            "thor",
            "yaml.erb",
            "yml.erb",
        ],
    ),
    ("sagemath", &["sage"]),
    ("scala", &["mill", "sbt", "sc", "scala"]),
    ("scheme", &["scm", "ss"]),
    ("scss", &["sass", "scss"]),
    ("sieve", &["sieve", "sieveinterface"]),
    (
        "simula",
        &["SIM", "SIm", "SiM", "Sim", "sIM", "sIm", "siM", "sim"],
    ),
    ("slang", &["slang"]),
    ("slim", &["html.slim", "slim"]),
    ("slint", &["slint"]),
    ("smalisp", &["smali"]),
    ("smithy", &["smithy"]),
    ("sml", &["fun", "sig", "sml"]),
    ("snakemake", &["Snakefile", "smk", "snakefile"]),
    ("solidity", &["sol", "yul"]),
    ("soma", &["soma"]),
    ("sorbet", &["rdoc"]),
    ("sourcepawn", &["sp"]),
    ("sql", &["sql"]),
    ("squirrel", &["nut"]),
    ("ssh-config", &["config", "ssh_config"]),
    ("stan", &["stan", "stanfunctions"]),
    (
        "starlark",
        &[
            ".tilt",
            ".tiltfile",
            "BUCK",
            "BUILD",
            "BUILD.bazel",
            "MODULE.bazel",
            "PACKAGE",
            "REPO.bazel",
            "Tiltfile",
            "WORKSPACE",
            "WORKSPACE.bzlmod",
            "bazelrc",
            "bxl",
            "bzl",
            "star",
        ],
    ),
    ("statamic-antlers", &["antlers.html"]),
    ("strace", &["strace"]),
    ("structured-text", &["st", "stx"]),
    ("styx", &["styx"]),
    ("surrealql", &["surql"]),
    ("svelte", &["svelte"]),
    ("sway", &["sw"]),
    ("swift", &["swift", "swiftinterface"]),
    ("systemrdl", &["rdl"]),
    ("tact", &["tact"]),
    ("tamarin", &["spthy"]),
    ("tcl", &["tcl", "tm"]),
    ("templ", &["templ"]),
    (
        "tera",
        &[
            "css.tera",
            "html.tera",
            "json.tera",
            "tera",
            "toml.tera",
            "xml.tera",
            "yaml.tera",
        ],
    ),
    ("termux", &["ebuild"]),
    ("terraform", &["hcl", "tf", "tfvars", "tofu"]),
    ("textproto", &["pbtxt", "textpb", "textproto", "txtpb"]),
    ("thrift", &["thrift"]),
    ("tmux", &["tmux", "tmux.conf"]),
    ("todotxt", &["todo.txt"]),
    ("toml", &["Cargo.lock", "Pipfile", "toml", "uv.lock"]),
    ("ton", &["fc", "fif", "tlb", "tolk"]),
    ("toon", &["toon"]),
    ("tql", &["tql"]),
    ("turtle", &["ttl", "turtle"]),
    ("twig", &["html.twig", "twig", "twig.html"]),
    ("txtar", &["txtar"]),
    ("typespec", &["tsp", "typespec"]),
    ("typst", &["typ", "typst"]),
    ("ucode", &["uc"]),
    ("uiua", &["ua"]),
    ("umka", &["um"]),
    ("unison", &["u"]),
    ("v", &["mod", "vsh", "vv"]),
    ("vala", &["vala", "vapi"]),
    ("vcard", &["vcf"]),
    ("vento", &["vento", "vto"]),
    ("verilog", &["sv", "svh", "v", "vh"]),
    ("veryl", &["veryl"]),
    ("vhdl", &["vhd", "vhdl"]),
    ("vhs", &["tape"]),
    ("viewtree", &["view.tree"]),
    ("vrl", &["vrl"]),
    ("vue", &["vue"]),
    ("wat", &["wat"]),
    ("wdl", &["wdl"]),
    ("webidl", &["webidl"]),
    ("wgsl", &["wgsl"]),
    ("wgsl-wesl", &["wesl"]),
    ("whkd", &["whkdrc"]),
    ("wikitext", &["mediawiki", "wikimedia", "wikitext"]),
    ("windows-batch", &["bat", "cmd"]),
    ("wit", &["wit"]),
    ("wow-toc", &["toc"]),
    ("wren", &["wren"]),
    ("wxml", &["wxml"]),
    ("xdr-naive", &["xdr"]),
    ("xml", &["xml"]),
    ("yang", &["yang"]),
    ("yara", &["yar", "yara"]),
    ("yarn-spinner", &["yarn"]),
    ("zig", &["zig", "zon"]),
    ("ziggy", &["zgy", "zgy-schema", "ziggy", "ziggy-schema"]),
    ("zokrates", &["zok"]),
    ("zwirn", &["zwirn"]),
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
            suggest_extension(rel_path(".helmignore")),
            Some(SuggestedExtension {
                extension_id: "helm".into(),
                file_name_or_extension: ".helmignore".into()
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
