{
  lib,
  stdenv,
  rustPlatform,
  zed-editor,
  nix-gitignore,
}:
let
  includeFilter =
    path: type:
    !(
      (type == "directory" && (dirOf path) == ../.)
      && (builtins.elem (baseNameOf (toString path)) [
        "docs"
        ".github"
        ".git"
        "target"
      ])
    );
  disableSandboxDarwin =
    drv:
    drv.overrideAttrs (
      lib.optionalAttrs stdenv.buildPlatform.isDarwin {
        __noChroot = true;
      }
    );
in
disableSandboxDarwin (
  zed-editor.overrideAttrs (old: {
    version = "nightly";
    src = lib.cleanSourceWith {
      src = nix-gitignore.gitignoreSource [ ] ../.;
      filter = includeFilter;
      name = "source";
    };

    postPatch = ''
      substituteInPlace ../cargo-vendor-dir/webrtc-sys-*/build.rs \
        --replace-fail "cargo:rustc-link-lib=static=webrtc" "cargo:rustc-link-lib=dylib=webrtc"
    '';

    cargoDeps = disableSandboxDarwin (
      rustPlatform.importCargoLock {
        lockFile = ../Cargo.lock;
        allowBuiltinFetchGit = true;
      }
    );
    doInstallCheck = false;
  })
)
