{
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
}:
{
  "brotli" = {
    package = brotli;
    path = "third_party/brotli/BUILD.gn";
  };
  "fontconfig" = {
    package = fontconfig;
    path = "third_party/fontconfig/BUILD.gn";
  };
  "freetype" = {
    package = freetype;
    path = "build/config/freetype/freetype.gni";
  };
  "harfbuzz-ng" = {
    package = harfbuzz;
    path = "third_party/harfbuzz-ng/harfbuzz.gni";
  };
  "jsoncpp" = {
    package = jsoncpp;
    path = "third_party/jsoncpp/BUILD.gn";
  };
  "icu" = {
    package = icu;
    path = "third_party/icu/BUILD.gn";
  };
  "libpng" = {
    package = libpng;
    path = "third_party/libpng/BUILD.gn";
  };
  "libwebp" = {
    package = libwebp;
    path = "third_party/libwebp/BUILD.gn";
  };
  "libxml" = {
    package = libxml2;
    path = "third_party/libxml/BUILD.gn";
  };
  "libxslt" = {
    package = libxslt;
    path = "third_party/libxslt/BUILD.gn";
  };
  "zlib" = {
    package = minizip;
    path = "third_party/zlib/BUILD.gn";
  };
  "ffmpeg" = {
    package = ffmpeg_6;
    path = "third_party/ffmpeg/BUILD.gn";
  };
}
