# GDScript

Godot [GDScript](https://gdscript.com/) language support in Zed is provided by the community-maintained [GDScript extension](https://github.com/JuliaEditorSupport/zed-julia).
Report issues to: [https://github.com/JuliaEditorSupport/zed-julia/issues](https://github.com/JuliaEditorSupport/zed-julia/issues)

- Tree Sitter: [tree-sitter-julia](https://github.com/tree-sitter/tree-sitter-julia)
- Language Server: [LanguageServer.jl](https://github.com/julia-vscode/LanguageServer.jl)

## Setup

1. Download and install [Godot for MacOS](https://godotengine.org/download/macos/).
2. Unzip the Godot.app and drag it into your /Applications folder.
3. Open Godot.app and open your project (an example project is fine)
4. In Godot, Editor Menu -> Editor Settings; scroll down the left sidebar to `Text Editor -> External`
   1. Use External Editor: "✅ On"
   2. Exec path: `/Applications/Zed.app/Contents/MacOS/zed`
   3. Exec flags: `{project} {file}`
   4. Close settings to save.
5. In Godot double click on a \*.gd script and Zed will launch

## Usage

When Godot is running, the GDScript extension will connect to the language server provided by the Godot runtime and will provide `jump to definition`, hover states when you hold cmd and other language server features.

> Note: If Zed is already running with an existing workspace, spawning from Godot will fail. Quit Zed and it should work again.
