{
  lib,
  writeText,
  livekit-libwebrtc,
  pkg-config,
  libx11,
  libxcb,
  cmake,
  libxkbcommon,
}:
let
  ZED_UPDATE_EXPLANATION = "Zed has been installed using Nix. Auto-updates have thus been disabled.";
in
{
  documented = attrs: {
    readme = "README.md";
  };
  x11 = attrs: {
    nativeBuildInputs = (attrs.nativeBuildInputs or [ ]) ++ [ pkg-config ];
    buildInputs = (attrs.buildInputs or [ ]) ++ [ libx11 ];
  };
  rav1e = attrs: {
    CARGO_ENCODED_RUSTFLAGS = "";
  };
  wasmtime-c-api-impl = attrs: {
    nativeBuildInputs = (attrs.nativeBuildInputs or [ ]) ++ [ cmake ];
  };
  webrtc-sys = attrs: {
    LK_CUSTOM_WEBRTC = "${livekit-libwebrtc}";

    postPatch = ''
      substituteInPlace webrtc-sys/build.rs --replace-fail \
        "cargo:rustc-link-lib=static=webrtc" "cargo:rustc-link-lib=dylib=webrtc"
    '';
    # TODO: ideally, we should fix these bindings to have non-dangling symlinks, but I don't think
    # we actually need them for our build, so just remove them for now. If we decide to upstream
    # these overrides, we'll need a real fix.
    postInstall = ''
      rm -rf $lib/lib/webrtc-sys.out/cxxbridge
    '';
  };
  prompt_store = attrs: {
    postPatch = (attrs.postPatch or "") + ''
      substituteInPlace \
        src/prompt_store.rs \
        --replace-fail \
        "../../git_ui" \
        "${../crates/git_ui}"
    '';
  };
  release_channel = attrs: {
    postPatch = (attrs.postPatch or "") + ''
      substituteInPlace \
        src/lib.rs \
        --replace-fail \
        "../../zed/RELEASE_CHANNEL" \
        "${writeText "RELEASE_CHANNEL" "nightly"}"
    '';
  };
  tree-sitter =
    attrs:
    let
      wasmtime-c-api-impl = lib.findFirst (p: p.libName == "wasmtime_c_api") null attrs.dependencies;
    in
    {
      DEP_WASMTIME_C_API_INCLUDE = "${wasmtime-c-api-impl.lib}/lib/wasmtime-c-api-impl.out/include";
    };
  assets = attrs: {
    postPatch = (attrs.postPatch or "") + ''
      substituteInPlace \
        src/assets.rs \
        --replace-fail \
        '../../assets' \
        '${../assets}'
    '';
  };
  blade-macros = attrs: {
    type = [ "proc-macro" ];
  };
  inspector_ui = attrs: {
    postPatch = (attrs.postPatch or "") + ''
      substituteInPlace \
        build.rs \
        --replace-fail \
        'std::env::var("CARGO_MANIFEST_DIR").unwrap()' \
        '"${../.}/crates/inspector_ui".to_string()'
    '';
  };
  zed = attrs: {
    buildInputs = [
      libxcb
      libxkbcommon
    ];
  };
  auto_update = attrs: {
    inherit ZED_UPDATE_EXPLANATION;
  };
  gpui = attrs: {
    features = (attrs.features or [ ]) ++ [ "runtime_shaders" ];
  };
  cli = attrs: {
    inherit ZED_UPDATE_EXPLANATION;
    buildInputs = [
      libxcb
      libxkbcommon
    ];
    postPatch = (attrs.postPatch or "") + ''
      substituteInPlace \
        src/main.rs \
        --replace-fail \
        '../../../script' \
        '${../script}'
    '';
  };
  settings = attrs: {
    postPatch = (attrs.postPatch or "") + ''
      substituteInPlace \
        src/settings.rs \
        --replace-fail \
        '../../assets' \
        '${../assets}'
    '';
  };
  extension_host = attrs: {
    postPatch = (attrs.postPatch or "") + ''
      ${lib.concatMapStringsSep "\n"
        (f: ''
          substituteInPlace \
            ${f} \
            --replace-fail \
            '../extension_api' \
            '${../crates/extension_api}'
        '')
        (
          [ "build.rs" ]
          ++ (lib.map (v: "src/wasm_host/wit/${v}") (
            lib.attrNames (builtins.readDir ../crates/extension_host/src/wasm_host/wit)
          ))
        )
      }
    '';
  };
}
