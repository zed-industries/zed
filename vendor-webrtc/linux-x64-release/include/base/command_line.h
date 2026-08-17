// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This class works with command lines: building and parsing.
// Arguments with prefixes ('--', '-', and on Windows, '/') are switches.
// Switches will precede all other arguments without switch prefixes.
// Switches can optionally have values, delimited by '=', e.g., "-switch=value".
// If a switch is specified multiple times, only the last value is used.
// An argument of "--" will terminate switch parsing during initialization,
// interpreting subsequent tokens as non-switch arguments, regardless of prefix.

// There is a singleton read-only CommandLine that represents the command line
// that the current process was started with.  It must be initialized in main().

#ifndef BASE_COMMAND_LINE_H_
#define BASE_COMMAND_LINE_H_

#include <stddef.h>

#include <functional>
#include <map>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/debug/debugging_buildflags.h"
#include "build/build_config.h"

#if BUILDFLAG(ENABLE_COMMANDLINE_SEQUENCE_CHECKS)
#include "base/sequence_checker.h"
#endif  // BUILDFLAG(ENABLE_COMMANDLINE_SEQUENCE_CHECKS)

namespace base {

class DuplicateSwitchHandler;
class FilePath;

class BASE_EXPORT CommandLine {
 public:
#if BUILDFLAG(IS_WIN)
  // The native command line string type.
  using StringType = std::wstring;
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  using StringType = std::string;
#endif

  using CharType = StringType::value_type;
  using StringVector = std::vector<StringType>;
  using StringViewType = std::basic_string_view<CharType>;
  using SwitchMap = std::map<std::string, StringType, std::less<>>;

  // Returns CommandLine object constructed with switches and keys alone.
  // NOTE: `argv` must NOT include the program path, and the switch arguments
  // must start from the index 0.
  static CommandLine FromArgvWithoutProgram(const StringVector& argv);

#if BUILDFLAG(IS_WIN)
  static CommandLine FromString(StringViewType command_line);
#endif

  // A constructor for CommandLines that only carry switches and arguments.
  enum NoProgram { NO_PROGRAM };
  explicit CommandLine(NoProgram no_program);

  // Construct a new command line with |program| as argv[0].
  explicit CommandLine(const FilePath& program);

  // Construct a new command line from an argument list.
  // TODO(tsepez): two-arg form should be UNSAFE_BUFFER_USAGE.
  CommandLine(int argc, const CharType* const* argv);
  explicit CommandLine(const StringVector& argv);

  // Allow the copy constructor. A common pattern is to copy of the current
  // process's command line and then add some flags to it. For example:
  //   CommandLine cl(*CommandLine::ForCurrentProcess());
  //   cl.AppendSwitch(...);
  CommandLine(const CommandLine& other);
  CommandLine& operator=(const CommandLine& other);

  CommandLine(CommandLine&& other) noexcept;
  CommandLine& operator=(CommandLine&& other) noexcept;

  ~CommandLine();

#if BUILDFLAG(IS_WIN)
  // By default this class will treat command-line arguments beginning with
  // slashes as switches on Windows, but not other platforms.
  //
  // If this behavior is inappropriate for your application, you can call this
  // function BEFORE initializing the current process' global command line
  // object and the behavior will be the same as Posix systems (only hyphens
  // begin switches, everything else will be an arg).
  static void set_slash_is_not_a_switch();

  // Normally when the CommandLine singleton is initialized it gets the command
  // line via the GetCommandLineW API and then uses the shell32 API
  // CommandLineToArgvW to parse the command line and convert it back to
  // argc and argv. Tests who don't want this dependency on shell32 and need
  // to honor the arguments passed in should use this function.
  // TODO(tsepez): should be UNSAFE_BUFFER_USAGE.
  static void InitUsingArgvForTesting(int argc, const char* const* argv);
#endif

  // Initialize the current process CommandLine singleton. On Windows, ignores
  // its arguments (we instead parse GetCommandLineW() directly) because we
  // don't trust the CRT's parsing of the command line, but it still must be
  // called to set up the command line. Returns false if initialization has
  // already occurred, and true otherwise. Only the caller receiving a 'true'
  // return value should take responsibility for calling Reset().
  // TODO(tsepez): should be UNSAFE_BUFFER_USAGE.
  static bool Init(int argc, const char* const* argv);

  // Destroys the current process CommandLine singleton. This is necessary if
  // you want to reset the base library to its initial state (for example, in an
  // outer library that needs to be able to terminate, and be re-initialized).
  // If Init is called only once, as in main(), Reset() is not necessary.
  // Do not call this in tests. Use base::test::ScopedCommandLine instead.
  static void Reset();

  // Get the singleton CommandLine representing the current process's
  // command line. Note: returned value is mutable, but not thread safe;
  // only mutate if you know what you're doing!
  static CommandLine* ForCurrentProcess();

  // Returns true if the CommandLine has been initialized for the given process.
  static bool InitializedForCurrentProcess();

  // Initialize from an argv vector.
  // TODO(tsepez): two-arg form should be UNSAFE_BUFFER_USAGE.
  void InitFromArgv(int argc, const CharType* const* argv);
  void InitFromArgv(const StringVector& argv);

  // Constructs and returns the represented command line string.
  // CAUTION! This should be avoided on POSIX because quoting behavior is
  // unclear.
  // CAUTION! If writing a command line to the Windows registry, use
  // GetCommandLineStringForShell() instead.
  StringType GetCommandLineString() const;

#if BUILDFLAG(IS_WIN)
  // Quotes and escapes `arg` if necessary so that it will be interpreted as a
  // single command-line parameter according to the following rules in line with
  // `::CommandLineToArgvW` and C++ `main`:
  // * Returns `arg` unchanged if `arg` does not include any characters that may
  // need encoding, which is spaces, tabs, backslashes, and double-quotes.
  // * Otherwise, double-quotes `arg` and in addition:
  //   * Escapes any double-quotes in `arg` with backslashes.
  //   * Escapes backslashes in `arg` if:
  //     * `arg` ends with backslashes , or
  //     * the backslashes end in a pre-existing double quote.
  //
  // https://learn.microsoft.com/en-us/search/?terms=CommandLineToArgvW and
  // http://msdn.microsoft.com/en-us/library/17w5ykft.aspx#parsing-c-command-line-arguments.
  static std::wstring QuoteForCommandLineToArgvW(const std::wstring& arg);

  // Returns the command-line string in the proper format for the Windows shell,
  // ending with the argument placeholder "--single-argument %1". The single-
  // argument switch prevents unexpected parsing of arguments from other
  // software that cannot be trusted to escape double quotes when substituting
  // into a placeholder (e.g., "%1" insert sequences populated by the Windows
  // shell).
  // NOTE: this must be used to generate the command-line string for the shell
  // even if this command line was parsed from a string with the proper syntax,
  // because the --single-argument switch is not preserved during parsing.
  StringType GetCommandLineStringForShell() const;

  // Returns the represented command-line string. Allows the use of unsafe
  // Windows insert sequences like "%1". Only use this method if
  // GetCommandLineStringForShell() is not adequate AND the processor inserting
  // the arguments is known to do so securely (i.e., is not the Windows shell).
  // If in doubt, do not use.
  StringType GetCommandLineStringWithUnsafeInsertSequences() const;
#endif

  // Constructs and returns the represented arguments string.
  // CAUTION! This should be avoided on POSIX because quoting behavior is
  // unclear.
  StringType GetArgumentsString() const;

  // Returns the original command line string as a vector of strings.
  const StringVector& argv() const LIFETIME_BOUND { return argv_; }

  // Get and Set the program part of the command line string (the first item).
  FilePath GetProgram() const;
  void SetProgram(const FilePath& program);

  // Returns true if this command line contains the given switch.
  // Switch names must be lowercase.
  // The second override provides an optimized version to avoid inlining codegen
  // at every callsite to find the length of the constant and construct a
  // std::string_view.
  bool HasSwitch(std::string_view switch_string) const;
  bool HasSwitch(const char switch_constant[]) const;

  // Returns the value associated with the given switch. If the switch has no
  // value or isn't present, this method returns the empty string.
  // Switch names must be lowercase.
  std::string GetSwitchValueASCII(std::string_view switch_string) const;
  std::string GetSwitchValueUTF8(std::string_view switch_string) const;
  FilePath GetSwitchValuePath(std::string_view switch_string) const;
  StringType GetSwitchValueNative(std::string_view switch_string) const;

  // Get a copy of all switches, along with their values.
  const SwitchMap& GetSwitches() const LIFETIME_BOUND { return switches_; }

  // Append a switch [with optional value] to the command line.
  // Note: Switches will precede arguments regardless of appending order.
  void AppendSwitch(std::string_view switch_string);
  void AppendSwitchPath(std::string_view switch_string, const FilePath& path);
  void AppendSwitchNative(std::string_view switch_string, StringViewType value);
  void AppendSwitchASCII(std::string_view switch_string,
                         std::string_view value);
  void AppendSwitchUTF8(std::string_view switch_string, std::string_view value);

  // Removes the switch that matches |switch_key_without_prefix|, regardless of
  // prefix and value. If no such switch is present, this has no effect.
  void RemoveSwitch(std::string_view switch_key_without_prefix);

  // Copies a set of switches (and any values) from another command line.
  // Commonly used when launching a subprocess.
  // If an entry in `switches` does not exist in `source`, then it is ignored.
  void CopySwitchesFrom(const CommandLine& source,
                        span<const char* const> switches);

  // Get the remaining arguments to the command.
  StringVector GetArgs() const;

  // Append an argument to the command line. Note that the argument is quoted
  // properly such that it is interpreted as one argument to the target command.
  // AppendArg is primarily for ASCII; non-ASCII input is interpreted as UTF-8.
  // Note: Switches will precede arguments regardless of appending order.
  void AppendArg(std::string_view value);
  void AppendArgPath(const FilePath& value);
  void AppendArgNative(StringViewType value);

  // Append the switches and arguments from another command line to this one.
  // If `include_program` is true, program will be overwritten by other's.
  void AppendArguments(const CommandLine& other, bool include_program);

  // Insert a command before the current command.
  // Common for debuggers, like "gdb --args".
  void PrependWrapper(StringViewType wrapper);

#if BUILDFLAG(IS_WIN)
  // Initialize by parsing the given command line string.
  // The program name is assumed to be the first item in the string.
  void ParseFromString(StringViewType command_line);

  // Returns true if the command line had the --single-argument switch, and
  // thus likely came from a Windows shell registration. This is only set if the
  // command line is parsed, and is not changed after it is parsed.
  bool HasSingleArgumentSwitch() const { return has_single_argument_switch_; }
#endif

  // Detaches this object from the current sequence in preparation for a move to
  // a different sequence.
  void DetachFromCurrentSequence();

  // Sets a delegate that's called when we encounter a duplicate switch
  static void SetDuplicateSwitchHandler(
      std::unique_ptr<DuplicateSwitchHandler>);

 private:
#if BUILDFLAG(ENABLE_COMMANDLINE_SEQUENCE_CHECKS)
  // A helper class that encapsulates a SEQUENCE_CHECKER but allows copy.
  // Copying this class will detach the sequence checker from the owning object.
  class InstanceBoundSequenceChecker {
   public:
    InstanceBoundSequenceChecker() = default;

    InstanceBoundSequenceChecker(const InstanceBoundSequenceChecker& other) {}

    InstanceBoundSequenceChecker& operator=(
        const InstanceBoundSequenceChecker& other) {
      return *this;
    }

    // Allow move as per SequenceChecker.
    InstanceBoundSequenceChecker(InstanceBoundSequenceChecker&&) = default;
    InstanceBoundSequenceChecker& operator=(InstanceBoundSequenceChecker&&) =
        default;

    void Detach() { DETACH_FROM_SEQUENCE(sequence_checker_); }
    void Check() { DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_); }

   private:
    SEQUENCE_CHECKER(sequence_checker_);
  };
#endif  // BUILDFLAG(ENABLE_COMMANDLINE_SEQUENCE_CHECKS)

  // Disallow default constructor; a program name must be explicitly specified.
  CommandLine() = delete;

  // Append switches and arguments, keeping switches before arguments.
  // NOTE: `argv` should not include the "program" element.
  void AppendSwitchesAndArguments(span<const StringType> argv);

  // Internal version of GetArgumentsString to support allowing unsafe insert
  // sequences in rare cases (see
  // GetCommandLineStringWithUnsafeInsertSequences).
  StringType GetArgumentsStringInternal(
      bool allow_unsafe_insert_sequences) const;

#if BUILDFLAG(IS_WIN)
  // Initializes by parsing |raw_command_line_string_|, treating everything
  // after |single_arg_switch_string| + <a single character> as the command
  // line's single argument, and dropping any arguments previously parsed. The
  // command line must contain |single_arg_switch_string|, and the argument, if
  // present, must be separated from |single_arg_switch_string| by one
  // character.
  // NOTE: the single-argument switch is not preserved after parsing;
  // GetCommandLineStringForShell() must be used to reproduce the original
  // command-line string with single-argument switch.
  void ParseAsSingleArgument(const StringType& single_arg_switch_string);

  // The string returned by GetCommandLineW(), to be parsed via
  // ParseFromString(). Empty if this command line was not parsed from a string,
  // or if ParseFromString() has finished executing.
  StringViewType raw_command_line_string_;

  // Set to true if the command line had --single-argument when initially
  // parsed. It does not change if the command line mutates after initial
  // parsing.
  bool has_single_argument_switch_ = false;
#endif

  // The singleton CommandLine representing the current process's command line.
  static CommandLine* current_process_commandline_;

  // The argv array: { program, [(--|-|/)switch[=value]]*, [--], [argument]* }
  StringVector argv_;

  // Parsed-out switch keys and values.
  SwitchMap switches_;

  // The index after the program and switches, any arguments start here.
  ptrdiff_t begin_args_;

#if BUILDFLAG(ENABLE_COMMANDLINE_SEQUENCE_CHECKS)
  InstanceBoundSequenceChecker sequence_checker_;
#endif
};

class BASE_EXPORT DuplicateSwitchHandler {
 public:
  // out_value contains the existing value of the switch
  virtual void ResolveDuplicate(std::string_view key,
                                CommandLine::StringViewType new_value,
                                CommandLine::StringType& out_value) = 0;
  virtual ~DuplicateSwitchHandler() = default;
};

}  // namespace base

#endif  // BASE_COMMAND_LINE_H_
