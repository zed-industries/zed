pub(crate) static SUGGESTIONS_BY_PATH_SUFFIX: phf::Map<&str, &str> = phf::phf_map! {
    "as" => "actionscript",
    "adb" | "ads" | "gpr" => "ada",
    "agda" => "agda",
    "ak" => "aiken",
    "ab" => "amber",
    "component.html" | "ng.html" => "angular",
    "ansible" => "ansible",
    "avdl" => "apache-avro",
    "ino" => "arduino",
    "ets" => "arkts",
    "ad" | "adoc" | "asc" | "asciidoc" => "asciidoc",
    "mib" => "asn1",
    "ass" | "ssa" => "ass",
    "S" | "asm" | "s" => "assembly",
    "astro" => "astro",
    "zed" => "authzed",
    "ahk" => "autohotkey",
    "awk" => "awk",
    "baml" => "baml",
    "bean" | "beancount" => "beancount",
    "bend" => "bend",
    "bicep" | "bicepparam" => "bicep",
    "grit" => "biome",
    "y" | "yy" => "bison",
    "bbappend" | "bbclass" | "inc" => "bitbake",
    "blade.php" => "blade",
    "blp" => "blueprint",
    "bsv" => "bluespec-systemverilog",
    "bqn" => "bqn",
    "bf" => "brainfuck",
    "bsl" => "bsl",
    "c3" | "c3i" => "c3",
    "Caddyfile" | "caddyfile" => "caddyfile",
    "cdc" => "cadence",
    "cairo" => "cairo",
    "capnp" => "capnp",
    "cds" => "cds-lsp",
    "cedar" => "cedar",
    "cf" | "cf.sub" | "cf3" | "cfengine" | "cfengine3" => "cfengine",
    "cfc" | "cfm" | "cfml" | "cfs" => "cfml",
    "cherri" => "cherri",
    "circom" => "circom",
    "clar" => "clarity",
    "bb" | "clj" | "cljc" | "cljd" | "cljs" | "edn" => "clojure",
    "cbl" | "cob" => "cobol",
    "CODEOWNERS" | "CODEOWNERS.txt" => "codeowners",
    "coffee" => "coffeescript",
    "coi" | "d.coi" => "coi",
    "conl" => "conl",
    "cook" => "cooklang",
    "cpp2" | "h2" => "cpp2",
    "cr" | "ecr" => "crystal",
    "cs" | "csproj" | "proj" | "props" | "slnx" | "targets" => "csharp",
    "csd" | "orc" | "sco" | "udo" => "csound",
    "csv" => "csv",
    "feature" | "gherkin" => "cucumber",
    "cue" => "cue",
    "curry" => "curry",
    "cylc" => "cylc",
    "cql" | "cyp" | "cypher" => "cypher",
    "pxd" | "pxi" | "pyx" => "cython",
    "d" | "dd" | "di" => "d",
    "d2" => "d2",
    "dfy" => "dafny",
    "dang" => "dang",
    "dart" => "dart",
    "dbml" => "dbml",
    "gui_script" | "render_script" | "script" => "defold",
    "desktop" | "directory" => "desktop",
    "dts" | "dtsi" | "dtso" | "its" | "overlay" => "devicetree",
    "dj.html" | "dj.md" | "dj.txt" => "django",
    "Containerfile"
    | "Dockerfile"
    | "compose.yaml"
    | "compose.yml"
    | "docker-compose.yaml"
    | "docker-compose.yml"
    | "dockerfile" => "dockerfile",
    "txt" => "duckyscript",
    "duper" => "duper",
    "Earthfile" => "earthfile",
    "edge" => "edge",
    "x12" => "edi",
    "editorconfig" => "editorconfig",
    "ejs" | "eta" => "ejs",
    "el" => "elisp",
    "eex" | "ex" | "exs" | "heex" | "html.eex" | "leex" | "mix.lock" | "neex" => "elixir",
    "le" => "elle",
    "elm" => "elm",
    "gjs" | "gts" | "handlebars" | "hbs" => "ember",
    "lua.txt" => "emmylua",
    "env" | "envrc" | "example" | "local" | "test" => "env",
    "Emakefile"
    | "app.src"
    | "erl"
    | "erlang"
    | "escript"
    | "hrl"
    | "rebar.config"
    | "xrl"
    | "yrl" => "erlang",
    "exo" => "exograph",
    "fnl" => "fennel",
    "fer" | "fer.ret" | "ferret.lock" => "ferret",
    "firebase.rules" | "rules" | "storage.rules" => "firebase-security-rules",
    "fish" => "fish",
    "fbs" => "flatbuffers",
    "F" | "F03" | "F08" | "F90" | "F95" | "f" | "f03" | "f08" | "f90" | "f95" => "fortran",
    "fountain" | "spmd" => "fountain",
    "ftl" => "freemarker",
    "fs" | "fsi" | "fsscript" | "fsx" => "fsharp",
    "fsm" => "fsm",
    "001"
    | "anc"
    | "apt"
    | "aptcl"
    | "bfb"
    | "cnc"
    | "din"
    | "dnc"
    | "ecs"
    | "eia"
    | "fan"
    | "fgc"
    | "fnc"
    | "g"
    | "g00"
    | "gc"
    | "gcd"
    | "gco"
    | "gcode"
    | "gp"
    | "hnc"
    | "knc"
    | "lib"
    | "min"
    | "mmg"
    | "mpf"
    | "mpt"
    | "nc"
    | "ncd"
    | "ncf"
    | "ncg"
    | "nci"
    | "ncp"
    | "ngc"
    | "out"
    | "pim"
    | "pit"
    | "plt"
    | "ply"
    | "prg"
    | "pu1"
    | "rol"
    | "sbp"
    | "spf"
    | "ssb"
    | "sub"
    | "tap"
    | "tcn"
    | "xpi" => "gcode",
    "gd"
    | "gdextension"
    | "gdshader"
    | "gdshaderinc"
    | "godot"
    | "import"
    | "tres"
    | "tscn" => "gdscript",
    "com.mitchellh.ghostty/config"
    | "config/ghostty/config"
    | "ghostty"
    | "ghostty/config" => "ghostty",
    ".containerignore"
    | ".cursorignore"
    | ".dockerignore"
    | ".eslintignore"
    | ".fdignore"
    | ".git-blame-ignore-revs"
    | ".gitattributes"
    | ".gitconfig"
    | ".gitignore"
    | ".gitignore_global"
    | ".gitmodules"
    | ".ignore"
    | ".lfsconfig"
    | ".npmignore"
    | ".prettierignore"
    | ".rgignore"
    | ".vscodeignore"
    | "config.worktree"
    | "git-rebase-todo"
    | "gitattributes" => "git-firefly",
    "gleam" => "gleam",
    "comp"
    | "frag"
    | "geom"
    | "glsl"
    | "mesh"
    | "rahit"
    | "rcall"
    | "rchit"
    | "rgen"
    | "rint"
    | "rmiss"
    | "task"
    | "tesc"
    | "tese"
    | "vert" => "glsl",
    "go.sum" => "gosum",
    "go.tpl"
    | "go.txt"
    | "gohtml"
    | "gotmpl"
    | "gtpl"
    | "html.gotmpl"
    | "html.gotpl"
    | "txt.gotmpl"
    | "txt.gotpl" => "gotmpl",
    "gql" | "graphql" | "graphqls" => "graphql",
    "DOT" | "dot" | "gv" => "graphviz",
    "gren" => "gren",
    "gritqlsnippet" => "grit",
    "JenkinsFile" | "Jenkinsfile" | "groovy" => "groovy",
    "groq" => "groq",
    "haml" | "html.haml" => "haml",
    "ha" => "hare",
    "cabal" | "hs" | "lhs" | "x" => "haskell",
    "hx" | "hxml" => "haxe",
    ".helmignore"
    | "helmfile.yaml"
    | "helmfile.yml"
    | "tpl"
    | "yaml.gotmpl"
    | "yml.gotmpl" => "helm",
    "hlsl" => "hlsl",
    "hocon" => "hocon",
    "42s" | "hp42s" => "hp42s",
    "htm" | "html" | "shtml" => "html",
    "j2" => "html-jinja",
    "http" => "http",
    "huff" => "huff",
    "hurl" => "hurl",
    "hl" | "hyprland.conf" | "hyprland.hl" | "hyprlandd.conf" | "hyprlandd.hl" => "hyprlang",
    "ics" => "ical",
    "h" => "inform6",
    "automount"
    | "build"
    | "cfg"
    | "conf"
    | "container"
    | "image"
    | "inf"
    | "ini"
    | "kube"
    | "mount"
    | "network"
    | "path"
    | "pod"
    | "scope"
    | "service"
    | "slice"
    | "socket"
    | "swap"
    | "target"
    | "timer"
    | "volume" => "ini",
    "ink" => "ink",
    "ion" | "isl" => "ion",
    "isle" => "isle",
    "jai" => "jai",
    "janet" => "janet",
    "gradle" | "gradle.kts" | "java" | "properties" => "java",
    "jdl" => "jdl",
    "jinja" | "jinja2" => "jinja2",
    "jq" => "jq",
    "json5" => "json5",
    "jsonl" | "ndjson" => "jsonl",
    "jsonnet" | "libsonnet" => "jsonnet",
    "jsp" | "jspf" | "tag" => "jsp",
    "jl" => "julia",
    "JUSTFILE" | "Justfile" | "just" | "justfile" => "just",
    "k" => "kcl",
    "Kconfig" => "kconfig",
    "kdl" => "kdl",
    "kt" | "kts" => "kotlin",
    "koto" => "koto",
    "bib" | "biblatex" | "bibtex" | "cls" | "latex" | "sty" | "tex" => "latex",
    "latte" => "latte",
    "lean" => "lean4",
    "journal" | "ldg" | "ldgr" | "ledger" => "ledger",
    "less" => "less",
    "ily" | "ly" => "lilypond",
    "ld" => "linkerscript",
    "liquid" => "liquid",
    "ll" => "llvm-ir",
    "log" => "log",
    "logcat" => "logcat",
    "lox" => "lox",
    "lua" => "lua",
    "luau" => "luau",
    "GNUmakefile" | "Makefile" | "OCamlMakefile" | "mak" | "makefile" | "mk" => "make",
    "m" => "matlab",
    "mcfunction" => "mcfunction",
    "mdx" => "mdx",
    "mermaid" | "mmd" => "mermaid",
    "meson.build" | "meson.options" | "meson_options.txt" => "meson",
    "ms" => "microscript",
    "mbt" => "moonbit",
    "did" | "mo" => "motoko",
    "move" => "move",
    "mustache" => "mustache",
    "navi" | "nv" | "nvs" => "navi",
    "CMakeLists.txt" | "cmake" => "neocmake",
    "axb" | "axi" | "axs" => "netlinx",
    "nginx.conf" => "nginx",
    "ncl" => "nickel",
    "nim" | "nim_format_string" | "nims" => "nim",
    "nix" => "nix",
    "nr" => "noir",
    "nomad" | "nomad.hcl" => "nomad",
    "nu" | "nuon" => "nu",
    "num" | "numscript" => "numscript",
    "njk" => "nunjucks",
    "oat" => "oat",
    "hh" | "mm" => "objective-c",
    "int" | "mac" | "rtn" => "objectscript",
    "dune"
    | "dune-project"
    | "dune-workspace"
    | "ml"
    | "mld"
    | "mli"
    | "mll"
    | "mlx"
    | "mly"
    | "re"
    | "rei" => "ocaml",
    "odin" => "odin",
    "msg" | "ned" => "omnetpp",
    "fga" => "openfga",
    "scad" => "openscad",
    "org" => "org",
    "polar" => "oso",
    "pact" | "repl" => "pact",
    "dpr" | "lpr" | "p" | "pas" => "pascal",
    "filter" => "path-of-exile",
    "pbxproj" | "strings" => "pbxproj",
    "pl" | "pm" | "t" => "perl",
    "perm" => "perm",
    "pest" => "pest",
    "php" | "phpt" | "phtml" => "php",
    "pica" => "pica200",
    "pcf" | "pkl" => "pkl",
    "iuml" | "plantuml" | "pu" | "puml" | "wsd" => "plantuml",
    "animation.txt" | "pdxinfo" => "playdate",
    "po" | "pot" => "po",
    "pony" => "pony",
    "ps1" | "psm1" => "powershell",
    "prisma" => "prisma",
    "pde" => "processing",
    "P" | "pro" => "prolog",
    "proto" => "proto",
    "jade" | "pug" => "pug",
    "epp" | "pp" => "puppet",
    "purs" => "purescript",
    "constraints.txt" | "requirements.txt" => "python-requirements",
    "qml" => "qml",
    "artifact" => "quadlet",
    "qc" => "quakec",
    "qmd" => "quarto",
    "R" | "r" => "r",
    "rkt" => "racket",
    "tsv" => "rainbow-csv",
    "rcl" => "rcl",
    "red" => "red",
    "reds" => "redscript",
    "reg" => "regedit",
    "res" | "resi" => "rescript",
    "rhai" => "rhai",
    "risor" => "risor",
    "roc" => "roc",
    "ron" => "ron",
    "roto" => "roto",
    "spec" => "rpmspec",
    "rs.html" => "rshtml",
    "rst" => "rst",
    "Appfile"
    | "Appraisals"
    | "Berksfile"
    | "Berksfile.lock"
    | "Brewfile"
    | "Capfile"
    | "Cheffile"
    | "Dangerfile"
    | "Deliverfile"
    | "Fastfile"
    | "Gemfile"
    | "Guardfile"
    | "Gymfile"
    | "Hobofile"
    | "Matchfile"
    | "Podfile"
    | "Puppetfile"
    | "Rakefile"
    | "Rantfile"
    | "Scanfile"
    | "Snapfile"
    | "Steepfile"
    | "Thorfile"
    | "Vagrantfile"
    | "builder"
    | "cap"
    | "capfile"
    | "erb"
    | "gemspec"
    | "html.erb"
    | "irbrc"
    | "jbuilder"
    | "js.erb"
    | "pryrc"
    | "rabl"
    | "rake"
    | "rb"
    | "rbi"
    | "rbs"
    | "ru"
    | "rxml"
    | "simplecov"
    | "thor"
    | "yaml.erb"
    | "yml.erb" => "ruby",
    "sage" => "sagemath",
    "mill" | "sbt" | "sc" | "scala" => "scala",
    "scm" | "ss" => "scheme",
    "sass" | "scss" => "scss",
    "sieve" | "sieveinterface" => "sieve",
    "SIM" | "SIm" | "SiM" | "Sim" | "sIM" | "sIm" | "siM" | "sim" => "simula",
    "slang" => "slang",
    "html.slim" | "slim" => "slim",
    "slint" => "slint",
    "smali" => "smalisp",
    "smithy" => "smithy",
    "fun" | "sig" | "sml" => "sml",
    "Snakefile" | "smk" | "snakefile" => "snakemake",
    "sol" | "yul" => "solidity",
    "soma" => "soma",
    "rdoc" => "sorbet",
    "sp" => "sourcepawn",
    "sql" => "sql",
    "nut" => "squirrel",
    "config" | "ssh_config" => "ssh-config",
    "stan" | "stanfunctions" => "stan",
    ".tilt"
    | ".tiltfile"
    | "BUCK"
    | "BUILD"
    | "BUILD.bazel"
    | "MODULE.bazel"
    | "PACKAGE"
    | "REPO.bazel"
    | "Tiltfile"
    | "WORKSPACE"
    | "WORKSPACE.bzlmod"
    | "bazelrc"
    | "bxl"
    | "bzl"
    | "star" => "starlark",
    "antlers.html" => "statamic-antlers",
    "strace" => "strace",
    "st" | "stx" => "structured-text",
    "styx" => "styx",
    "surql" => "surrealql",
    "svelte" => "svelte",
    "sw" => "sway",
    "swift" | "swiftinterface" => "swift",
    "rdl" => "systemrdl",
    "tact" => "tact",
    "spthy" => "tamarin",
    "tcl" | "tm" => "tcl",
    "templ" => "templ",
    "css.tera"
    | "html.tera"
    | "json.tera"
    | "tera"
    | "toml.tera"
    | "xml.tera"
    | "yaml.tera" => "tera",
    "ebuild" => "termux",
    "hcl" | "tf" | "tfvars" | "tofu" => "terraform",
    "pbtxt" | "textpb" | "textproto" | "txtpb" => "textproto",
    "thrift" => "thrift",
    "tmux" | "tmux.conf" => "tmux",
    "todo.txt" => "todotxt",
    "Cargo.lock" | "Pipfile" | "toml" | "uv.lock" => "toml",
    "fc" | "fif" | "tlb" | "tolk" => "ton",
    "toon" => "toon",
    "tql" => "tql",
    "ttl" | "turtle" => "turtle",
    "html.twig" | "twig" | "twig.html" => "twig",
    "txtar" => "txtar",
    "tsp" | "typespec" => "typespec",
    "typ" | "typst" => "typst",
    "uc" => "ucode",
    "ua" => "uiua",
    "um" => "umka",
    "u" => "unison",
    "mod" | "vsh" | "vv" => "v",
    "vala" | "vapi" => "vala",
    "vcf" => "vcard",
    "vento" | "vto" => "vento",
    "sv" | "svh" | "v" | "vh" => "verilog",
    "veryl" => "veryl",
    "vhd" | "vhdl" => "vhdl",
    "tape" => "vhs",
    "view.tree" => "viewtree",
    "vrl" => "vrl",
    "vue" => "vue",
    "wat" => "wat",
    "wdl" => "wdl",
    "webidl" => "webidl",
    "wgsl" => "wgsl",
    "wesl" => "wgsl-wesl",
    "whkdrc" => "whkd",
    "mediawiki" | "wikimedia" | "wikitext" => "wikitext",
    "bat" | "cmd" => "windows-batch",
    "wit" => "wit",
    "toc" => "wow-toc",
    "wren" => "wren",
    "wxml" => "wxml",
    "xdr" => "xdr-naive",
    "xml" => "xml",
    "yang" => "yang",
    "yar" | "yara" => "yara",
    "yarn" => "yarn-spinner",
    "zig" | "zon" => "zig",
    "zgy" | "zgy-schema" | "ziggy" | "ziggy-schema" => "ziggy",
    "zok" => "zokrates",
    "zwirn" => "zwirn",
};
