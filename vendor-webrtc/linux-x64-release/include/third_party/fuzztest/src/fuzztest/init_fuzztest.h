#ifndef FUZZTEST_FUZZTEST_INIT_FUZZTEST_H_
#define FUZZTEST_FUZZTEST_INIT_FUZZTEST_H_

#include <string>
#include <string_view>
#include <vector>

namespace fuzztest {

// Initializes Abseil flags such that FuzzTest can rely upon them.
// It initializes all Abseil flags (not just FuzzTest ones).
// It does not do anything with non-Abseil flags and will not result
// in an error or failure if there are other flags on the command line
// (as there might be especially if you are using a non-centipede
// fuzzing engine).
// This should be called only if you have no other code initializing
// Abseil flags from the command line elsewhere (such as
// absl::ParseCommandLine()).
// It does not change the command line (argv); only reads it.
void ParseAbslFlags(int argc, char** argv);

// Initializes FuzzTest. Handles the FuzzTest related flags and registers
// FUZZ_TEST-s in the binary as GoogleTest TEST-s.
//
// The command line arguments (argc, argv) are passed only to support the
// "compatibility mode" with external engines via the LLVMFuzzerRunDriver
// interface:
// https://llvm.org/docs/LibFuzzer.html#using-libfuzzer-as-a-library
//
// The optional `binary_id` can be used to override the default "binary
// identifier", which is used to find the saved corpus in the corpus database
// for the fuzz tests in the given binary. (By default, the filename of the
// binary is used as an identifier).
//
// REQUIRES: `main()` has started before calling this function.
// REQUIRES: Abseil flags have been inited, either using
//           ParseAbslFlags or some other means
//
void InitFuzzTest(int* argc, char*** argv, std::string_view binary_id = {});

// Returns a list of all registered fuzz test names in the form of
// "<suite_name>.<property_function_name>", e.g., `MySuite.MyFuzzTest".
//
// REQUIRES: `main()` has started before calling this function.
std::vector<std::string> ListRegisteredTests();

// Returns the full name of the fuzz test that "matches" the provided `name`
// specification. If no match is found, it exists.
//
// 1) The provided `name` specification can be a full name, e.g.,
// "MySuite.MyFuzzTest". If such fuzz test exists, the full name is returned.
//
// 2) The `name` specification can also be a strict sub-string of a full name,
// e.g., "MyFuzz". If there's exactly one fuzz test that contains the (strict)
// sub-string, its full name is returned.
//
// 3) The `name` specification can also be an empty string. If there's only one
// fuzz test in the binary, its full name is returned.
//
// If no single match is found, it exits with an error message.
//
// REQUIRES: `main()` has started before calling this function.
std::string GetMatchingFuzzTestOrExit(std::string_view name);

// Runs the FUZZ_TEST specified by `name` in fuzzing mode.
//
// Selects the fuzz test to run using GetMatchingFuzzTestOrExit(name).
//
// If `name` matches exactly one FUZZ_TEST, it runs the selected test in fuzzing
// mode, until a bug is found or until manually stopped. Otherwise, it exits.
//
// `binary_id` used to find the saved corpus in the corpus database for the fuzz
// test in the given binary.
//
// REQUIRES: `main()` has started before calling this function.
// REQUIRES: Binary must be built with SanCov instrumentation on.
// TODO(b/346833936): Make `binary_id` optional.
void RunSpecifiedFuzzTest(std::string_view name, std::string_view binary_id);

}  // namespace fuzztest

#endif  // FUZZTEST_FUZZTEST_INIT_FUZZTEST_H_
