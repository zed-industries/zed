#ifndef BENCHMARK_EXPORT_H
#define BENCHMARK_EXPORT_H

#if defined(_WIN32)
#define EXPORT_ATTR __declspec(dllexport)
#define IMPORT_ATTR __declspec(dllimport)
#define NO_EXPORT_ATTR
#define DEPRECATED_ATTR __declspec(deprecated)
#else  // _WIN32
#define EXPORT_ATTR __attribute__((visibility("default")))
#define IMPORT_ATTR __attribute__((visibility("default")))
#define NO_EXPORT_ATTR __attribute__((visibility("hidden")))
#define DEPRECATE_ATTR __attribute__((__deprecated__))
#endif  // _WIN32

#ifdef BENCHMARK_STATIC_DEFINE
#define BENCHMARK_EXPORT
#define BENCHMARK_NO_EXPORT
#else  // BENCHMARK_STATIC_DEFINE
#ifndef BENCHMARK_EXPORT
#ifdef benchmark_EXPORTS
/* We are building this library */
#define BENCHMARK_EXPORT EXPORT_ATTR
#else  // benchmark_EXPORTS
/* We are using this library */
#define BENCHMARK_EXPORT IMPORT_ATTR
#endif  // benchmark_EXPORTS
#endif  // !BENCHMARK_EXPORT

#ifndef BENCHMARK_NO_EXPORT
#define BENCHMARK_NO_EXPORT NO_EXPORT_ATTR
#endif  // !BENCHMARK_NO_EXPORT
#endif  // BENCHMARK_STATIC_DEFINE

#ifndef BENCHMARK_DEPRECATED
#define BENCHMARK_DEPRECATED DEPRECATE_ATTR
#endif  // BENCHMARK_DEPRECATED

#ifndef BENCHMARK_DEPRECATED_EXPORT
#define BENCHMARK_DEPRECATED_EXPORT BENCHMARK_EXPORT BENCHMARK_DEPRECATED
#endif  // BENCHMARK_DEPRECATED_EXPORT

#ifndef BENCHMARK_DEPRECATED_NO_EXPORT
#define BENCHMARK_DEPRECATED_NO_EXPORT BENCHMARK_NO_EXPORT BENCHMARK_DEPRECATED
#endif  // BENCHMARK_DEPRECATED_EXPORT

#endif /* BENCHMARK_EXPORT_H */
