/**
 * \file wasi.h
 *
 * C API for WASI
 */

#ifndef WASI_H
#define WASI_H

#include "wasm.h"
#include <stdint.h>
#include <wasmtime/conf.h>

#ifdef WASMTIME_FEATURE_WASI

#ifndef WASI_API_EXTERN
#ifdef _WIN32
#define WASI_API_EXTERN __declspec(dllimport)
#else
#define WASI_API_EXTERN
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define own

#define WASI_DECLARE_OWN(name)                                                 \
  typedef struct wasi_##name##_t wasi_##name##_t;                              \
  WASI_API_EXTERN void wasi_##name##_delete(own wasi_##name##_t *);

/**
 * \typedef wasi_config_t
 * \brief Convenience alias for #wasi_config_t
 *
 * \struct wasi_config_t
 * \brief TODO
 *
 * \fn void wasi_config_delete(wasi_config_t *);
 * \brief Deletes a configuration object.
 */
WASI_DECLARE_OWN(config)

/**
 * \brief Creates a new empty configuration object.
 *
 * The caller is expected to deallocate the returned configuration
 */
WASI_API_EXTERN own wasi_config_t *wasi_config_new();

/**
 * \brief Sets the argv list for this configuration object.
 *
 * By default WASI programs have an empty argv list, but this can be used to
 * explicitly specify what the argv list for the program is.
 *
 * The arguments are copied into the `config` object as part of this function
 * call, so the `argv` pointer only needs to stay alive for this function call.
 *
 * This function returns `true` if all arguments were registered successfully,
 * or `false` if an argument was not valid UTF-8.
 */
WASI_API_EXTERN bool wasi_config_set_argv(wasi_config_t *config, size_t argc,
                                          const char *argv[]);

/**
 * \brief Indicates that the argv list should be inherited from this process's
 * argv list.
 */
WASI_API_EXTERN void wasi_config_inherit_argv(wasi_config_t *config);

/**
 * \brief Sets the list of environment variables available to the WASI instance.
 *
 * By default WASI programs have a blank environment, but this can be used to
 * define some environment variables for them.
 *
 * It is required that the `names` and `values` lists both have `envc` entries.
 *
 * The env vars are copied into the `config` object as part of this function
 * call, so the `names` and `values` pointers only need to stay alive for this
 * function call.
 *
 * This function returns `true` if all environment variables were successfully
 * registered. This returns `false` if environment variables are not valid
 * UTF-8.
 */
WASI_API_EXTERN bool wasi_config_set_env(wasi_config_t *config, size_t envc,
                                         const char *names[],
                                         const char *values[]);

/**
 * \brief Indicates that the entire environment of the calling process should be
 * inherited by this WASI configuration.
 */
WASI_API_EXTERN void wasi_config_inherit_env(wasi_config_t *config);

/**
 * \brief Configures standard input to be taken from the specified file.
 *
 * By default WASI programs have no stdin, but this configures the specified
 * file to be used as stdin for this configuration.
 *
 * If the stdin location does not exist or it cannot be opened for reading then
 * `false` is returned. Otherwise `true` is returned.
 */
WASI_API_EXTERN bool wasi_config_set_stdin_file(wasi_config_t *config,
                                                const char *path);

/**
 * \brief Configures standard input to be taken from the specified
 * #wasm_byte_vec_t.
 *
 * By default WASI programs have no stdin, but this configures the specified
 * bytes to be used as stdin for this configuration.
 *
 * This function takes ownership of the `binary` argument.
 */
WASI_API_EXTERN void wasi_config_set_stdin_bytes(wasi_config_t *config,
                                                 wasm_byte_vec_t *binary);

/**
 * \brief Configures this process's own stdin stream to be used as stdin for
 * this WASI configuration.
 */
WASI_API_EXTERN void wasi_config_inherit_stdin(wasi_config_t *config);

/**
 * \brief Configures standard output to be written to the specified file.
 *
 * By default WASI programs have no stdout, but this configures the specified
 * file to be used as stdout.
 *
 * If the stdout location could not be opened for writing then `false` is
 * returned. Otherwise `true` is returned.
 */
WASI_API_EXTERN bool wasi_config_set_stdout_file(wasi_config_t *config,
                                                 const char *path);

/**
 * \brief Configures this process's own stdout stream to be used as stdout for
 * this WASI configuration.
 */
WASI_API_EXTERN void wasi_config_inherit_stdout(wasi_config_t *config);

/**
 * \brief Configures standard output to be written to the specified file.
 *
 * By default WASI programs have no stderr, but this configures the specified
 * file to be used as stderr.
 *
 * If the stderr location could not be opened for writing then `false` is
 * returned. Otherwise `true` is returned.
 */
WASI_API_EXTERN bool wasi_config_set_stderr_file(wasi_config_t *config,
                                                 const char *path);

/**
 * \brief Configures this process's own stderr stream to be used as stderr for
 * this WASI configuration.
 */
WASI_API_EXTERN void wasi_config_inherit_stderr(wasi_config_t *config);

/**
 * \brief The permissions granted for a directory when preopening it.
 */
enum wasi_dir_perms_flags {
  /**
   * \brief This directory can be read, for example its entries can be iterated
   */
  WASMTIME_WASI_DIR_PERMS_READ = 1,

  /**
   * \brief This directory can be written to, for example new files can be
   * created within it.
   */
  WASMTIME_WASI_DIR_PERMS_WRITE = 2,
};

/**
 * \brief The permissions granted for directories when preopening them,
 * which is a bitmask with flag values from wasi_dir_perms_flags.
 */
typedef size_t wasi_dir_perms;

/**
 * \brief The permissions granted for files when preopening a directory.
 */
enum wasi_file_perms_flags {
  /**
   * \brief Files can be read.
   */
  WASMTIME_WASI_FILE_PERMS_READ = 1,

  /**
   * \brief Files can be written to.
   */
  WASMTIME_WASI_FILE_PERMS_WRITE = 2,
};

/**
 * \brief The max permissions granted a file within a preopened directory,
 * which is a bitmask with flag values from wasi_file_perms_flags.
 */
typedef size_t wasi_file_perms;

/**
 * \brief Configures a "preopened directory" to be available to WASI APIs.
 *
 * By default WASI programs do not have access to anything on the filesystem.
 * This API can be used to grant WASI programs access to a directory on the
 * filesystem, but only that directory (its whole contents but nothing above
 * it).
 *
 * The `host_path` argument here is a path name on the host filesystem, and
 * `guest_path` is the name by which it will be known in wasm.
 *
 * The `dir_perms` argument is the permissions that wasm will have to operate on
 * `guest_path`. This can be used, for example, to provide readonly access to a
 * directory. This argument is a bitmask with the following flag values:
 * - WASMTIME_WASI_DIR_PERMS_READ
 * - WASMTIME_WASI_DIR_PERMS_WRITE
 *
 * The `file_perms` argument is similar to `dir_perms` but corresponds to the
 * maximum set of permissions that can be used for any file in this directory.
 * This argument is a bitmask with the following flag values:
 * - WASMTIME_WASI_FILE_PERMS_READ
 * - WASMTIME_WASI_FILE_PERMS_WRITE
 */
WASI_API_EXTERN bool wasi_config_preopen_dir(wasi_config_t *config,
                                             const char *host_path,
                                             const char *guest_path,
                                             wasi_dir_perms dir_perms,
                                             wasi_file_perms file_perms);

#undef own

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_FEATURE_WASI

#endif // #ifdef WASI_H
