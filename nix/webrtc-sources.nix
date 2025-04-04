# TODO(nixpkgs-livekit-bump): see https://github.com/NixOS/nixpkgs/pull/396016;
# drop once in `nixpkgs-unstable`
#
# This file is from the above PR (courtesy of @WeetHet).
{
  fetchFromGitHub,
  fetchFromGitiles,
  fetchgit,
  fetchurl,
  runCommand,
  lib,
}:
let
  sourceDerivations = {
    "src" = fetchFromGitHub {
      owner = "webrtc-sdk";
      repo = "webrtc";
      rev = "7ec4c03bff7f7ce117dc9100f081d031d946d995"; # m125_release
      hash = "sha256-LUncFGXaYVUrBdWD1Xx3MZe5GzmjJuJtDebAMb8jass=";
    };
    "src/base" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/src/base";
      rev = "738cf0c976fd3d07c5f1853f050594c5295300d8";
      hash = "sha256-Hw0cXws+0M2UcvcnJZGkUtH28ZEDfxNl0e8ngWlAZnA=";
    };
    "src/build" = fetchFromGitHub {
      owner = "webrtc-sdk";
      repo = "build";
      rev = "6978bac6466311e4bee4c7a9fd395faa939e0fcd";
      hash = "sha256-mPjb7/TTJ7/oatBdIRGhSsacjbyu5ZilUgyplAtji1s=";
    };
    "src/buildtools" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/src/buildtools";
      rev = "5eb927f0a922dfacf10cfa84ee76f39dcf2a7311";
      hash = "sha256-OS9k7sDzAVH+NU9P4ilKJavkiov/1qq1fG5AWq9kH/Y=";
    };
    "src/testing" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/src/testing";
      rev = "d6e731571c33f30e5dc46f54c69e6d432566e55c";
      hash = "sha256-VisK7NDR/xDC3OM7LD9Gyo58rs1GE37i7QRYC/Kk12k=";
    };
    "src/third_party" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/src/third_party";
      rev = "f36c4b6e56aaa94606c87fa0c3f7cbdbb5c70546";
      hash = "sha256-TdB8qMcmXO3xgYyJkHHwn/8tVg1pFMlrNABpQQ80bOY=";
    };
    "src/third_party/clang-format/script" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/llvm/llvm-project/clang/tools/clang-format";
      rev = "3c0acd2d4e73dd911309d9e970ba09d58bf23a62";
      hash = "sha256-whD8isX2ZhLrFzdxHhFP1S/sZDRgyrzLFaVd7OEFqYo=";
    };
    "src/third_party/libc++/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/llvm/llvm-project/libcxx";
      rev = "e3b94d0e5b86883fd77696bf10dc33ba250ba99b";
      hash = "sha256-ocJqlENHw19VpkFxKwHneGw3aNh56nt+/JeopxLj2M8=";
    };
    "src/third_party/libc++abi/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/llvm/llvm-project/libcxxabi";
      rev = "932d253fedb390a08b17ec3a92469a4553934a6a";
      hash = "sha256-qBupfCAnSNpvqcwFycQEi5v6TBAH5LdQI5YcLeQD2y8=";
    };
    "src/third_party/libunwind/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/llvm/llvm-project/libunwind";
      rev = "419b03c0b8f20d6da9ddcb0d661a94a97cdd7dad";
      hash = "sha256-/4/Trextb4F9UMDVrg4uG9QZl6S0H9FiwnL+2S5+ZpE=";
    };
    "src/third_party/boringssl/src" = fetchFromGitiles {
      url = "https://boringssl.googlesource.com/boringssl";
      rev = "f94f3ed3965ea033001fb9ae006084eee408b861";
      hash = "sha256-baa6L6h1zVBHen/YFVtF+9fhYWC4ZGbMUSO8L1VNFjw=";
    };
    "src/third_party/breakpad/breakpad" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/breakpad/breakpad";
      rev = "76788faa4ef163081f82273bfca7fae8a734b971";
      hash = "sha256-qAIXZ1jZous0Un0jVkOQ66nA2525NziV3Lbso2/+Z1Y=";
    };
    "src/third_party/catapult" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/catapult";
      rev = "88367fd8c736a2601fc183920c9ffe9ac2ec32ac";
      hash = "sha256-uqtyxO7Ge3egBsYmwcRGiV1lqm4iYVHrqYfDz7r6Byo=";
    };
    "src/third_party/ced/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/google/compact_enc_det";
      rev = "ba412eaaacd3186085babcd901679a48863c7dd5";
      hash = "sha256-ySG74Rj2i2c/PltEgHVEDq+N8yd9gZmxNktc56zIUiY=";
    };
    "src/third_party/colorama/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/colorama";
      rev = "3de9f013df4b470069d03d250224062e8cf15c49";
      hash = "sha256-6ZTdPYSHdQOLYMSnE+Tp7PgsVTs3U2awGu9Qb4Rg/tk=";
    };
    "src/third_party/crc32c/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/google/crc32c";
      rev = "fa5ade41ee480003d9c5af6f43567ba22e4e17e6";
      hash = "sha256-urg0bmnfMfHagLPELp4WrNCz1gBZ6DFOWpDue1KsMtc=";
    };
    "src/third_party/depot_tools" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/tools/depot_tools";
      rev = "495b23b39aaba2ca3b55dd27cadc523f1cb17ee6";
      hash = "sha256-RguGUaIpxtxrY+LksFmeNbZuitZpB6O9HJc1c4TMXeQ=";
    };
    "src/third_party/ffmpeg" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/third_party/ffmpeg";
      rev = "901248a373cbbe7af68fb92faf3be7d4f679150d";
      hash = "sha256-6+Sc5DsPaKW68PSUS4jlpzRXjPhEN7LFQATVVL9Xhfo=";
    };
    "src/third_party/flatbuffers/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/google/flatbuffers";
      rev = "bcb9ef187628fe07514e57756d05e6a6296f7dc5";
      hash = "sha256-LecJwLDG6szZZ/UOCFD+MDqH3NKawn0sdEwgnMt8wMM=";
    };
    "src/third_party/grpc/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/grpc/grpc";
      rev = "822dab21d9995c5cf942476b35ca12a1aa9d2737";
      hash = "sha256-64JEVCx/PCM0dvv7kAQvSjLc0QbRAZVBDzwD/FAV6T8=";
    };
    "src/third_party/fontconfig/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/fontconfig";
      rev = "14d466b30a8ab4a9d789977ed94f2c30e7209267";
      hash = "sha256-W5WIgC6A52kY4fNkbsDEa0o+dfd97Rl5NKfgnIRpI00=";
    };
    "src/third_party/freetype/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/src/third_party/freetype2";
      rev = "b3a6a20a805366e0bc7044d1402d04c53f9c1660";
      hash = "sha256-XBHWUw28bsCpwUXb+faE36DRdujuKiWoJ+dEmUk07s4=";
    };
    "src/third_party/harfbuzz-ng/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/harfbuzz/harfbuzz";
      rev = "155015f4bec434ecc2f94621665844218f05ce51";
      hash = "sha256-VAan6P8PHSq8RsGE4YbI/wCfFAhzl3nJMt0cQBYi5Ls=";
    };
    "src/third_party/google_benchmark/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/google/benchmark";
      rev = "344117638c8ff7e239044fd0fa7085839fc03021";
      hash = "sha256-gztnxui9Fe/FTieMjdvfJjWHjkImtlsHn6fM1FruyME=";
    };
    "src/third_party/gtest-parallel" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/google/gtest-parallel";
      rev = "96f4f904922f9bf66689e749c40f314845baaac8";
      hash = "sha256-VUuk5tBTh+aU2dxVWUF1FePWlKUJaWSiGSXk/J5zgHw=";
    };
    "src/third_party/googletest/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/google/googletest";
      rev = "5197b1a8e6a1ef9f214f4aa537b0be17cbf91946";
      hash = "sha256-JCIJrjN/hH6oAgvJRuv3aJA+z6Qe7yefyRbAhP5bZDc=";
    };
    "src/third_party/icu" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/deps/icu";
      rev = "364118a1d9da24bb5b770ac3d762ac144d6da5a4";
      hash = "sha256-frsmwYMiFixEULsE91x5+p98DvkyC0s0fNupqjoRnvg=";
    };
    "src/third_party/jsoncpp/source" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/open-source-parsers/jsoncpp";
      rev = "42e892d96e47b1f6e29844cc705e148ec4856448";
      hash = "sha256-bSLNcoYBz3QCt5VuTR056V9mU2PmBuYBa0W6hFg2m8Q=";
    };
    "src/third_party/libFuzzer/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/llvm/llvm-project/compiler-rt/lib/fuzzer";
      rev = "758bd21f103a501b362b1ca46fa8fcb692eaa303";
      hash = "sha256-T0dO+1A0r6kLFoleMkY8heu80biPntCpvA6YfqA7b+E=";
    };
    "src/third_party/fuzztest/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/google/fuzztest";
      rev = "65354bf09a2479945b4683c42948695d4f2f7c07";
      hash = "sha256-8w4yIW15VamdjevMO27NYuf+GFu5AvHSooDZH0PbS6s=";
    };
    "src/third_party/libjpeg_turbo" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/deps/libjpeg_turbo";
      rev = "9b894306ec3b28cea46e84c32b56773a98c483da";
      hash = "sha256-+t75ZAdOXc7Nd1/8zEQLX+enZb8upqIQuR6qzb9z7Cg=";
    };
    "src/third_party/libsrtp" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/deps/libsrtp";
      rev = "7a7e64c8b5a632f55929cb3bb7d3e6fb48c3205a";
      hash = "sha256-XOPiDAOHpWyCiXI+fi1CAie0Zaj4v14m9Kc8+jbzpUY=";
    };
    "src/third_party/dav1d/libdav1d" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/videolan/dav1d";
      rev = "006ca01d387ac6652825d6cce1a57b2de67dbf8d";
      hash = "sha256-AA2bcrsW1xFspyl5TqYUJeAwKM06rWTNtXr/uMVIJmw=";
    };
    "src/third_party/libaom/source/libaom" = fetchFromGitiles {
      url = "https://aomedia.googlesource.com/aom";
      rev = "eefd5585a0c4c204fcf7d30065f8c2ca35c38a82";
      hash = "sha256-0tLfbfYyCnG89DHNIoYoiitN9pFFcuX/Nymp3Q5xhBg=";
    };
    "src/third_party/perfetto" = fetchFromGitiles {
      url = "https://android.googlesource.com/platform/external/perfetto";
      rev = "0e424063dbfd4e7400aa3b77b5c00b84893aee7b";
      hash = "sha256-fS0P/0Bqn9EreCPRC65Lw7/zcpMquo7RDf6dmbMDa74=";
    };
    "src/third_party/protobuf-javascript/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/protocolbuffers/protobuf-javascript";
      rev = "e34549db516f8712f678fcd4bc411613b5cc5295";
      hash = "sha256-TmP6xftUVTD7yML7UEM/DB8bcsL5RFlKPyCpcboD86U=";
    };
    "src/third_party/libvpx/source/libvpx" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/webm/libvpx";
      rev = "8762f5efb2917765316a198e6713f0bc93b07c9b";
      hash = "sha256-JbeUgX8Dx8GkQ79ElZHK8gYI3/4o6NrTV+HpblwLvIE=";
    };
    "src/third_party/libyuv" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/libyuv/libyuv";
      rev = "a6a2ec654b1be1166b376476a7555c89eca0c275";
      hash = "sha256-hD5B9fPNwf8M98iS/PYeUJgJxtBvvf2BrrlnBNYXSg0=";
    };
    "src/third_party/lss" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/linux-syscall-support";
      rev = "ce877209e11aa69dcfffbd53ef90ea1d07136521";
      hash = "sha256-hE8uZf9Fst66qJkoVYChiB8G41ie+k9M4X0W+5JUSdw=";
    };
    "src/third_party/instrumented_libs" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/third_party/instrumented_libraries";
      rev = "0172d67d98df2d30bd2241959d0e9569ada25abe";
      hash = "sha256-SGEB74fK9e0WWT77ZNISE9fVlXGGPvZMBUsQ3XD+DsA=";
    };
    "src/third_party/nasm" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/deps/nasm";
      rev = "f477acb1049f5e043904b87b825c5915084a9a29";
      hash = "sha256-SiRXHsUlWXtH6dbDjDjqNAm105ibEB3jOfNtQAM4CaY=";
    };
    "src/third_party/openh264/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/cisco/openh264";
      rev = "09a4f3ec842a8932341b195c5b01e141c8a16eb7";
      hash = "sha256-J7Eqe2QevZh1xfap19W8AVCcwfRu7ztknnbKFJUAH1c=";
    };
    "src/third_party/re2/src" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/external/github.com/google/re2";
      rev = "b84e3ff189980a33d4a0c6fa1201aa0b3b8bab4a";
      hash = "sha256-FA9wAZwqLx7oCPf+qeqZ7hhpJ9J2DSMXZAWllHIX/qY=";
    };
    "src/tools" = fetchFromGitiles {
      url = "https://chromium.googlesource.com/chromium/src/tools";
      rev = "0d6482e40fe26f738a0acf6ebb0f797358538b48";
      hash = "sha256-19oGSveaPv8X+/hsevUe4fFtLASC3HfPtbnw3TWpYQk=";
    };
  };
  namedSourceDerivations = builtins.mapAttrs (
    path: drv:
    drv.overrideAttrs {
      name = lib.strings.sanitizeDerivationName path;
    }
  ) sourceDerivations;
in
runCommand "combined-sources" { } (
  lib.concatLines (
    [ "mkdir $out" ]
    ++ (lib.mapAttrsToList (path: drv: ''
      mkdir -p $out/${path}
      cp --no-preserve=mode --reflink=auto -rfT ${drv} $out/${path}
    '') namedSourceDerivations)
  )
)
