{
  stdenv,
  clang,
  gclient2nix,
  lib,
  gn,
  fetchurl,
  fetchpatch,
  xcbuild,
  python3,
  ninja,
  git,
  cpio,
  pkg-config,
  glib,
  alsa-lib,
  pulseaudio,
  nasm,
  brotli,
  fontconfig,
  freetype,
  harfbuzz,
  icu,
  jsoncpp,
  libpng,
  libwebp,
  libxml2,
  libxslt,
  minizip,
  ffmpeg_6,
  libepoxy,
  libgbm,
  libGL,
  libxcomposite,
  libxdamage,
  libxext,
  libxfixes,
  libxrandr,
  libxtst,
  pipewire,
  xorg,
}:
let
  platformMap = {
    "x86_64" = "x64";
    "i686" = "x86";
    "arm" = "arm";
    "aarch64" = "arm64";
  };
  cpuName = stdenv.hostPlatform.parsed.cpu.name;
  gnArch = platformMap."${cpuName}" or (throw "unsupported arch ${cpuName}");
  gnOs =
    if stdenv.hostPlatform.isLinux then
      "linux"
    else if stdenv.hostPlatform.isDarwin then
      "mac"
    else
      throw "unknown platform ${stdenv.hostPlatform.config}";
  boringSslSymbols = fetchurl {
    url = "https://raw.githubusercontent.com/livekit/rust-sdks/refs/tags/webrtc-dac8015-6/webrtc-sys/libwebrtc/boringssl_prefix_symbols.txt";
    hash = "sha256-dAweArv8zjsFPENEKi9mNBQkt4y+hh3rCqG6QZjRC20=";
  };
  gnSystemLibraries = import ./mkSystemLibraries.nix {
    inherit
      brotli
      fontconfig
      freetype
      harfbuzz
      icu
      jsoncpp
      libpng
      libwebp
      libxml2
      libxslt
      minizip
      ffmpeg_6
      ;
  };
in
stdenv.mkDerivation {
  pname = "livekit-libwebrtc";
  version = "137-unstable-2025-11-24";

  gclientDeps = gclient2nix.importGclientDeps ./sources.json;
  sourceRoot = "src";

  patches = [
    # Adds missing dependencies to generated LICENSE
    (fetchpatch {
      url = "https://raw.githubusercontent.com/livekit/rust-sdks/a4343fe9d88fcc96f8e88959c90d509abbd0307b/webrtc-sys/libwebrtc/patches/add_licenses.patch";
      hash = "sha256-9A4KyRW1K3eoQxsTbPX0vOnj66TCs2Fxjpsu5wO8mGI=";
    })
    # Fixes the certificate chain, required for Let's Encrypt certs
    (fetchpatch {
      url = "https://raw.githubusercontent.com/livekit/rust-sdks/a4343fe9d88fcc96f8e88959c90d509abbd0307b/webrtc-sys/libwebrtc/patches/ssl_verify_callback_with_native_handle.patch";
      hash = "sha256-RBvRcJzoKItpEbqpe07YZe1D1ZVGS12EnDSISldGy+0=";
    })
    # Adds dependencies and features required by livekit
    (fetchpatch {
      url = "https://raw.githubusercontent.com/livekit/rust-sdks/a4343fe9d88fcc96f8e88959c90d509abbd0307b/webrtc-sys/libwebrtc/patches/add_deps.patch";
      hash = "sha256-DwRtGdU5sppmiFsVuyhJoVCQrRl5JFmZJfxgUPhYXBg=";
    })
    # Fix gcc-related errors
    (fetchpatch {
      url = "https://raw.githubusercontent.com/livekit/rust-sdks/a4343fe9d88fcc96f8e88959c90d509abbd0307b/webrtc-sys/libwebrtc/patches/force_gcc.patch";
      hash = "sha256-1d73Pi1HkbunjYvp1NskUNE4xXbCmnh++rC6NrCJHbY=";
      stripLen = 1;
      extraPrefix = "build/";
    })
    # fix a gcc-related dav1d compile option
    (fetchpatch {
      url = "https://raw.githubusercontent.com/livekit/rust-sdks/a4343fe9d88fcc96f8e88959c90d509abbd0307b/webrtc-sys/libwebrtc/patches/david_disable_gun_source_macro.patch";
      hash = "sha256-RCZpeeSQHaxkL3dY2oFFXDjYeU0KHw7idQFONGge8+0=";
      stripLen = 1;
      extraPrefix = "third_party/";
    })
    # Required for dynamically linking to ffmpeg libraries and exposing symbols
    ./0001-shared-libraries.patch
    # Borrow a patch from chromium to prevent a build failure due to missing libclang libraries
    ./applications/networking/browsers/chromium/patches/chromium-129-rust.patch
  ];

  postPatch = ''
    substituteInPlace .gn \
      --replace-fail "vpython3" "python3"

    substituteInPlace tools/generate_shim_headers/generate_shim_headers.py \
      --replace-fail "OFFICIAL_BUILD" "GOOGLE_CHROME_BUILD"

    substituteInPlace BUILD.gn \
      --replace-fail "rtc_static_library" "rtc_shared_library" \
      --replace-fail "complete_static_lib = true" ""

    substituteInPlace webrtc.gni \
      --replace-fail "!build_with_chromium && is_component_build" "false"

    substituteInPlace rtc_tools/BUILD.gn \
      --replace-fail "\":frame_analyzer\"," ""

    for lib in ${toString (builtins.attrNames gnSystemLibraries)}; do
      if [ -d "third_party/$lib" ]; then
        find "third_party/$lib" -type f \
          \! -path "third_party/$lib/chromium/*" \
          \! -path "third_party/$lib/google/*" \
          \! -path "third_party/harfbuzz-ng/utils/hb_scoped.h" \
          \! -regex '.*\.\(gn\|gni\|isolate\)' \
          \! -name 'LICENSE*' \
          \! -name 'COPYING*' \
          -delete
      fi
    done

    # Trick the update_rust.py script into thinking we have *this specfic* rust available.
    # It isn't actually needed for the libwebrtc build, but GN will fail if it isn't there.
    mkdir -p third_party/rust-toolchain
    (python3 tools/rust/update_rust.py --print-package-version || true) \
      | head -n 1 \
      | sed 's/.* expected Rust version is \([^ ]*\) .*/rustc 1.0 1234 (\1 chromium)/' \
      > third_party/rust-toolchain/VERSION
  ''
  + lib.optionalString stdenv.hostPlatform.isLinux ''
    mkdir -p buildtools/linux64
    ln -sf ${lib.getExe gn} buildtools/linux64/gn
    substituteInPlace build/toolchain/linux/BUILD.gn \
      --replace 'toolprefix = "aarch64-linux-gnu-"' 'toolprefix = ""'
  ''
  + lib.optionalString stdenv.hostPlatform.isDarwin ''
    mkdir -p buildtools/mac
    ln -sf ${lib.getExe gn} buildtools/mac/gn
    chmod +x build/toolchain/apple/linker_driver.py
    patchShebangs build/toolchain/apple/linker_driver.py
    substituteInPlace build/toolchain/apple/toolchain.gni --replace-fail "/bin/cp -Rc" "cp -a"
  '';

  outputs = [
    "dev"
    "out"
  ];

  nativeBuildInputs =
    (builtins.concatLists (
      lib.mapAttrsToList (
        _: library: if (library.package ? dev) then [ library.package.dev ] else [ ]
      ) gnSystemLibraries
    ))
    ++ [
      gclient2nix.gclientUnpackHook
      gn
      (python3.withPackages (ps: [ ps.setuptools ]))
      ninja
      git
      cpio
      pkg-config
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [ xcbuild ];

  buildInputs = [
    nasm
  ]
  ++ (lib.mapAttrsToList (_: library: library.package) gnSystemLibraries)
  ++ (lib.optionals stdenv.hostPlatform.isLinux [
    glib
    alsa-lib
    pulseaudio
    libepoxy
    libgbm
    libGL
    libxcomposite
    libxdamage
    libxext
    libxfixes
    libxrandr
    libxtst
    pipewire
    xorg.libX11
    xorg.libXi
  ]);

  preConfigure = ''
    echo "generate_location_tags = true" >> build/config/gclient_args.gni
    echo "0" > build/util/LASTCHANGE.committime

    python build/linux/unbundle/replace_gn_files.py \
        --system-libraries ${toString (builtins.attrNames gnSystemLibraries)}
  '';

  gnFlags = [
    "is_debug=false"
    "rtc_include_tests=false"
    ''target_os="${gnOs}"''
    ''target_cpu="${gnArch}"''
    "treat_warnings_as_errors=false"
    "rtc_enable_protobuf=false"
    "rtc_include_tests=false"
    "rtc_build_examples=false"
    "rtc_build_tools=false"
    "rtc_libvpx_build_vp9=true"
    "enable_libaom=true"
    "use_dummy_lastchange=true"
    "is_component_build=true"
    "enable_stripping=true"
    "rtc_use_h264=true"
    "rtc_use_h265=true"
    "use_custom_libcxx=false"
    "use_rtti=true"
  ]
  ++ (lib.optionals stdenv.hostPlatform.isLinux [
    "rtc_use_pipewire=true"
    "symbol_level=0"
    "enable_iterator_debugging=false"
    "rtc_use_x11=true"
    "use_sysroot=false"
    "use_custom_libcxx_for_host=false"
    "use_libcxx_modules=false"
    "use_llvm_libatomic=false"
    "is_clang=false"
  ])
  ++ (lib.optionals stdenv.hostPlatform.isDarwin [
    ''mac_deployment_target="${stdenv.hostPlatform.darwinMinVersion}"''
    "rtc_enable_symbol_export=true"
    "rtc_enable_objc_symbol_export=true"
    "rtc_include_dav1d_in_internal_decoder_factory=true"
    "clang_use_chrome_plugins=false"
    "use_lld=false"
    ''clang_base_path="${clang}"''
  ]);

  ninjaFlags = [
    ":default"
  ]
  ++ lib.optionals stdenv.hostPlatform.isDarwin [
    "api/audio_codecs:builtin_audio_decoder_factory"
    "api/task_queue:default_task_queue_factory"
    "sdk:native_api"
    "sdk:default_codec_factory_objc"
    "pc:peer_connection"
    "sdk:videocapture_objc"
    "sdk:mac_framework_objc"
    "desktop_capture_objc"
  ];

  postBuild =
    lib.optionalString stdenv.hostPlatform.isLinux ''
      objcopy --redefine-syms="${boringSslSymbols}" "libwebrtc.so"
    ''
    + ''
      # Generate licenses
      python3 "../../tools_webrtc/libs/generate_licenses.py" \
          --target ${if stdenv.hostPlatform.isDarwin then ":webrtc" else ":default"} $PWD $PWD
    '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/lib
    mkdir -p $dev/include

    install -m0644 obj/webrtc.ninja obj/modules/desktop_capture/desktop_capture.ninja args.gn LICENSE.md $dev

    pushd ../..
    find . -name "*.h" -print | cpio -pd $dev/include
    find . -name "*.inc" -print | cpio -pd $dev/include
    popd
  ''
  + lib.optionalString stdenv.hostPlatform.isLinux ''
    install -m0644 libwebrtc.so libthird_party_boringssl.so $out/lib
  ''
  + lib.optionalString stdenv.hostPlatform.isDarwin ''
    install -m0644 WebRTC.framework/Versions/A/WebRTC $out/lib/libwebrtc.dylib
    install -m0644 libthird_party_boringssl.dylib $out/lib
  ''
  + ''
    ln -s $out/lib $dev/lib

    runHook postInstall
  '';

  postFixup = lib.optionalString stdenv.hostPlatform.isDarwin ''
    boringssl="$out/lib/libthird_party_boringssl.dylib"
    webrtc="$out/lib/libwebrtc.dylib"

    install_name_tool -id "$boringssl" "$boringssl"
    install_name_tool -id "$webrtc" "$webrtc"
    install_name_tool -change @rpath/libthird_party_boringssl.dylib "$boringssl" "$webrtc"
  '';

  passthru.updateScript = ./update.sh;

  meta = {
    description = "WebRTC library used by livekit";
    homepage = "https://github.com/livekit/rust-sdks/";
    license = lib.licenses.bsd3;
    maintainers = with lib.maintainers; [
      WeetHet
      niklaskorz
    ];
    platforms = lib.platforms.linux ++ lib.platforms.darwin;
  };
}
