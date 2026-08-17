/**
 * \file wasmtime/wasi.hh
 */

#ifndef WASMTIME_WASI_HH
#define WASMTIME_WASI_HH

#include <memory>
#include <string>
#include <vector>
#include <wasi.h>
#include <wasmtime/conf.h>

#ifdef WASMTIME_FEATURE_WASI

namespace wasmtime {

/**
 * \brief Configuration for an instance of WASI.
 *
 * This is inserted into a store with `Store::Context::set_wasi`.
 */
class WasiConfig {
  friend class Store;

  struct deleter {
    void operator()(wasi_config_t *p) const { wasi_config_delete(p); }
  };

  std::unique_ptr<wasi_config_t, deleter> ptr;

public:
  /// Creates a new configuration object with default settings.
  WasiConfig() : ptr(wasi_config_new()) {}

  /// Configures the argv explicitly with the given string array.
  void argv(const std::vector<std::string> &args) {
    std::vector<const char *> ptrs;
    ptrs.reserve(args.size());
    for (const auto &arg : args) {
      ptrs.push_back(arg.c_str());
    }

    wasi_config_set_argv(ptr.get(), (int)args.size(), ptrs.data());
  }

  /// Configures the argv for wasm to be inherited from this process itself.
  void inherit_argv() { wasi_config_inherit_argv(ptr.get()); }

  /// Configures the environment variables available to wasm, specified here as
  /// a list of pairs where the first element of the pair is the key and the
  /// second element is the value.
  void env(const std::vector<std::pair<std::string, std::string>> &env) {
    std::vector<const char *> names;
    std::vector<const char *> values;
    for (const auto &[name, value] : env) {
      names.push_back(name.c_str());
      values.push_back(value.c_str());
    }
    wasi_config_set_env(ptr.get(), (int)env.size(), names.data(),
                        values.data());
  }

  /// Indicates that the entire environment of this process should be inherited
  /// by the wasi configuration.
  void inherit_env() { wasi_config_inherit_env(ptr.get()); }

  /// Configures the provided file to be used for the stdin of this WASI
  /// configuration.
  [[nodiscard]] bool stdin_file(const std::string &path) {
    return wasi_config_set_stdin_file(ptr.get(), path.c_str());
  }

  /// Configures this WASI configuration to inherit its stdin from the host
  /// process.
  void inherit_stdin() { return wasi_config_inherit_stdin(ptr.get()); }

  /// Configures the provided file to be created and all stdout output will be
  /// written there.
  [[nodiscard]] bool stdout_file(const std::string &path) {
    return wasi_config_set_stdout_file(ptr.get(), path.c_str());
  }

  /// Configures this WASI configuration to inherit its stdout from the host
  /// process.
  void inherit_stdout() { return wasi_config_inherit_stdout(ptr.get()); }

  /// Configures the provided file to be created and all stderr output will be
  /// written there.
  [[nodiscard]] bool stderr_file(const std::string &path) {
    return wasi_config_set_stderr_file(ptr.get(), path.c_str());
  }

  /// Configures this WASI configuration to inherit its stdout from the host
  /// process.
  void inherit_stderr() { return wasi_config_inherit_stderr(ptr.get()); }

  /// Opens `path` to be opened as `guest_path` in the WASI pseudo-filesystem.
  [[nodiscard]] bool preopen_dir(const std::string &path,
                                 const std::string &guest_path,
                                 size_t dir_perms, size_t file_perms) {
    return wasi_config_preopen_dir(ptr.get(), path.c_str(), guest_path.c_str(),
                                   dir_perms, file_perms);
  }
};

} // namespace wasmtime

#endif // WASMTIME_FEATURE_WASI

#endif // WASMTIME_WASI_HH
