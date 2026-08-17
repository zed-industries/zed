#ifndef TEST_OUTPUT_TEST_H
#define TEST_OUTPUT_TEST_H

#undef NDEBUG
#include <functional>
#include <initializer_list>
#include <memory>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "../src/re.h"
#include "benchmark/benchmark.h"

#define CONCAT2(x, y) x##y
#define CONCAT(x, y) CONCAT2(x, y)

#define ADD_CASES(...) int CONCAT(dummy, __LINE__) = ::AddCases(__VA_ARGS__)

#define SET_SUBSTITUTIONS(...) \
  int CONCAT(dummy, __LINE__) = ::SetSubstitutions(__VA_ARGS__)

enum MatchRules {
  MR_Default,  // Skip non-matching lines until a match is found.
  MR_Next,     // Match must occur on the next line.
  MR_Not  // No line between the current position and the next match matches
          // the regex
};

struct TestCase {
  TestCase(std::string re, int rule = MR_Default);

  std::string regex_str;
  int match_rule;
  std::string substituted_regex;
  std::shared_ptr<benchmark::Regex> regex;
};

enum TestCaseID {
  TC_ConsoleOut,
  TC_ConsoleErr,
  TC_JSONOut,
  TC_JSONErr,
  TC_CSVOut,
  TC_CSVErr,

  TC_NumID  // PRIVATE
};

// Add a list of test cases to be run against the output specified by
// 'ID'
int AddCases(TestCaseID ID, std::initializer_list<TestCase> il);

// Add or set a list of substitutions to be performed on constructed regex's
// See 'output_test_helper.cc' for a list of default substitutions.
int SetSubstitutions(
    std::initializer_list<std::pair<std::string, std::string>> il);

// Run all output tests.
void RunOutputTests(int argc, char* argv[]);

// Count the number of 'pat' substrings in the 'haystack' string.
int SubstrCnt(const std::string& haystack, const std::string& pat);

// Run registered benchmarks with file reporter enabled, and return the content
// outputted by the file reporter.
std::string GetFileReporterOutput(int argc, char* argv[]);

// ========================================================================= //
// ------------------------- Results checking ------------------------------ //
// ========================================================================= //

// Call this macro to register a benchmark for checking its results. This
// should be all that's needed. It subscribes a function to check the (CSV)
// results of a benchmark. This is done only after verifying that the output
// strings are really as expected.
// bm_name_pattern: a name or a regex pattern which will be matched against
//                  all the benchmark names. Matching benchmarks
//                  will be the subject of a call to checker_function
// checker_function: should be of type ResultsCheckFn (see below)
#define CHECK_BENCHMARK_RESULTS(bm_name_pattern, checker_function) \
  size_t CONCAT(dummy, __LINE__) = AddChecker(bm_name_pattern, checker_function)

struct Results;
typedef std::function<void(Results const&)> ResultsCheckFn;

size_t AddChecker(const std::string& bm_name_pattern, const ResultsCheckFn& fn);

// Class holding the results of a benchmark.
// It is passed in calls to checker functions.
struct Results {
  // the benchmark name
  std::string name;
  // the benchmark fields
  std::map<std::string, std::string> values;

  Results(const std::string& n) : name(n) {}

  int NumThreads() const;

  double NumIterations() const;

  typedef enum { kCpuTime, kRealTime } BenchmarkTime;

  // get cpu_time or real_time in seconds
  double GetTime(BenchmarkTime which) const;

  // get the real_time duration of the benchmark in seconds.
  // it is better to use fuzzy float checks for this, as the float
  // ASCII formatting is lossy.
  double DurationRealTime() const {
    return NumIterations() * GetTime(kRealTime);
  }
  // get the cpu_time duration of the benchmark in seconds
  double DurationCPUTime() const { return NumIterations() * GetTime(kCpuTime); }

  // get the string for a result by name, or nullptr if the name
  // is not found
  const std::string* Get(const std::string& entry_name) const {
    auto it = values.find(entry_name);
    if (it == values.end()) return nullptr;
    return &it->second;
  }

  // get a result by name, parsed as a specific type.
  // NOTE: for counters, use GetCounterAs instead.
  template <class T>
  T GetAs(const std::string& entry_name) const;

  // counters are written as doubles, so they have to be read first
  // as a double, and only then converted to the asked type.
  template <class T>
  T GetCounterAs(const std::string& entry_name) const {
    double dval = GetAs<double>(entry_name);
    T tval = static_cast<T>(dval);
    return tval;
  }
};

template <class T>
T Results::GetAs(const std::string& entry_name) const {
  auto* sv = Get(entry_name);
  BM_CHECK(sv != nullptr && !sv->empty());
  std::stringstream ss;
  ss << *sv;
  T out;
  ss >> out;
  BM_CHECK(!ss.fail());
  return out;
}

//----------------------------------
// Macros to help in result checking. Do not use them with arguments causing
// side-effects.

// clang-format off

#define CHECK_RESULT_VALUE_IMPL(entry, getfn, var_type, var_name, relationship, value) \
    CONCAT(BM_CHECK_, relationship)                                        \
    (entry.getfn< var_type >(var_name), (value)) << "\n"                \
    << __FILE__ << ":" << __LINE__ << ": " << (entry).name << ":\n"     \
    << __FILE__ << ":" << __LINE__ << ": "                              \
    << "expected (" << #var_type << ")" << (var_name)                   \
    << "=" << (entry).getfn< var_type >(var_name)                       \
    << " to be " #relationship " to " << (value) << "\n"

// check with tolerance. eps_factor is the tolerance window, which is
// interpreted relative to value (eg, 0.1 means 10% of value).
#define CHECK_FLOAT_RESULT_VALUE_IMPL(entry, getfn, var_type, var_name, relationship, value, eps_factor) \
    CONCAT(BM_CHECK_FLOAT_, relationship)                                  \
    (entry.getfn< var_type >(var_name), (value), (eps_factor) * (value)) << "\n" \
    << __FILE__ << ":" << __LINE__ << ": " << (entry).name << ":\n"     \
    << __FILE__ << ":" << __LINE__ << ": "                              \
    << "expected (" << #var_type << ")" << (var_name)                   \
    << "=" << (entry).getfn< var_type >(var_name)                       \
    << " to be " #relationship " to " << (value) << "\n"                \
    << __FILE__ << ":" << __LINE__ << ": "                              \
    << "with tolerance of " << (eps_factor) * (value)                   \
    << " (" << (eps_factor)*100. << "%), "                              \
    << "but delta was " << ((entry).getfn< var_type >(var_name) - (value)) \
    << " (" << (((entry).getfn< var_type >(var_name) - (value))         \
               /                                                        \
               ((value) > 1.e-5 || value < -1.e-5 ? value : 1.e-5)*100.) \
    << "%)"

#define CHECK_RESULT_VALUE(entry, var_type, var_name, relationship, value) \
    CHECK_RESULT_VALUE_IMPL(entry, GetAs, var_type, var_name, relationship, value)

#define CHECK_COUNTER_VALUE(entry, var_type, var_name, relationship, value) \
    CHECK_RESULT_VALUE_IMPL(entry, GetCounterAs, var_type, var_name, relationship, value)

#define CHECK_FLOAT_RESULT_VALUE(entry, var_name, relationship, value, eps_factor) \
    CHECK_FLOAT_RESULT_VALUE_IMPL(entry, GetAs, double, var_name, relationship, value, eps_factor)

#define CHECK_FLOAT_COUNTER_VALUE(entry, var_name, relationship, value, eps_factor) \
    CHECK_FLOAT_RESULT_VALUE_IMPL(entry, GetCounterAs, double, var_name, relationship, value, eps_factor)

// clang-format on

// ========================================================================= //
// --------------------------- Misc Utilities ------------------------------ //
// ========================================================================= //

namespace {

const char* const dec_re = "[0-9]*[.]?[0-9]+([eE][-+][0-9]+)?";

}  //  end namespace

#endif  // TEST_OUTPUT_TEST_H
