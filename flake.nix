{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.xz";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      crane,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      mkWorkspace =
        pkgs:
        let
          rustBin = rust-overlay.lib.mkRustBin { } pkgs;
          toolchain = rustBin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        (pkgs.callPackage ./Cargo.nix {
          buildRustCrateForPkgs =
            pkgs:
            pkgs.buildRustCrate.override {
              rustc = toolchain;
              cargo = toolchain;
              defaultCodegenUnits = 16;
              defaultCrateOverrides = pkgs.defaultCrateOverrides // {
                documented = attrs: {
                  readme = "README.md";
                };
                x11 = attrs: {
                  nativeBuildInputs = (attrs.nativeBuildInputs or [ ]) ++ [ pkgs.pkg-config ];
                  buildInputs = (attrs.buildInputs or [ ]) ++ [ pkgs.libx11 ];
                };
                rav1e = attrs: {
                  CARGO_ENCODED_RUSTFLAGS = "";
                };
                wasmtime-c-api-impl = attrs: {
                  nativeBuildInputs = (attrs.nativeBuildInputs or [ ]) ++ [ pkgs.cmake ];
                };
                webrtc-sys = attrs: {
                  LK_CUSTOM_WEBRTC = "${pkgs.livekit-libwebrtc}";

                  postPatch = ''
                    substituteInPlace webrtc-sys/build.rs --replace-fail \
                      "cargo:rustc-link-lib=static=webrtc" "cargo:rustc-link-lib=dylib=webrtc"
                  '';
                  postInstall = ''
                    ls -R
                    rm -rf $lib/lib/webrtc-sys.out/cxxbridge
                  '';
                };
                release_channel = attrs: {
                  postPatch = (attrs.postPatch or "") + ''
                    substituteInPlace \
                      src/lib.rs \
                      --replace-fail \
                      "../../zed/RELEASE_CHANNEL" \
                      "${pkgs.writeText "RELEASE_CHANNEL" "nightly"}"
                  '';
                };
                tree-sitter =
                  attrs:
                  let
                    lib = pkgs.lib;
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
                      '${./assets}'
                  '';
                };
                inspector_ui = attrs: {
                  postPatch = (attrs.postPatch or "") + ''
                    substituteInPlace \
                      build.rs \
                      --replace-fail \
                      'std::env::var("CARGO_MANIFEST_DIR").unwrap()' \
                      '"${self}/crates/inspector_ui".to_string()'
                  '';
                };
                zed = attrs: {
                  buildInputs = [
                    pkgs.libxcb
                    pkgs.libxkbcommon
                  ];
                };
                cli = attrs: {
                  buildInputs = [
                    pkgs.libxcb
                    pkgs.libxkbcommon
                  ];
                  postPatch = (attrs.postPatch or "") + ''
                    substituteInPlace \
                      src/main.rs \
                      --replace-fail \
                      '../../../script' \
                      '${./script}'
                  '';
                };
                settings = attrs: {
                  postPatch = (attrs.postPatch or "") + ''
                    substituteInPlace \
                      src/settings.rs \
                      --replace-fail \
                      '../../assets' \
                      '${./assets}'
                  '';
                };
                extension_host = attrs: {
                  postPatch = (attrs.postPatch or "") + ''
                    ${builtins.foldl'
                      (acc: f: ''
                        ${acc}
                        substituteInPlace \
                          ${f} \
                          --replace-fail \
                          '../extension_api' \
                          '${./crates/extension_api}'
                      '')
                      ""
                      (
                        [ "build.rs" ]
                        ++ (builtins.map (v: "src/wasm_host/wit/since_${v}.rs") [
                          "v0_0_1"
                          "v0_0_4"
                          "v0_0_6"
                          "v0_1_0"
                          "v0_2_0"
                          "v0_3_0"
                          "v0_4_0"
                          "v0_5_0"
                          "v0_6_0"
                        ])
                      )
                    }
                  '';
                };
              };
            };
        });
      # Pull just the zed binary out of the workspace
      mkZed = pkgs: (mkWorkspace pkgs).workspaceMembers.zed.build;
    in
    rec {
      workspace = forAllSystems mkWorkspace;
      packages = forAllSystems (pkgs: rec {
        default = mkZed pkgs;
        debug = default.override { profile = "dev"; };
      });
      devShells = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/shell.nix {
          zed-editor = packages.${pkgs.hostPlatform.system}.default;
        };
      });
      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
      overlays.default = final: _: {
        zed-editor = mkZed final;
      };
    };

  nixConfig = {
    extra-substituters = [
      "https://zed.cachix.org"
      "https://cache.garnix.io"
    ];
    extra-trusted-public-keys = [
      "zed.cachix.org-1:/pHQ6dpMsAZk2DiP4WCL0p9YDNKWj2Q5FL20bNmw1cU="
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
    ];
  };
}
