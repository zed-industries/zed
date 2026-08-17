// Copyright 2008 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---
// Author: Dave Nicponski
//
// Implement helpful bash-style command line flag completions
//
// ** Functional API:
// HandleCommandLineCompletions() should be called early during
// program startup, but after command line flag code has been
// initialized, such as the beginning of HandleCommandLineHelpFlags().
// It checks the value of the flag --tab_completion_word.  If this
// flag is empty, nothing happens here.  If it contains a string,
// however, then HandleCommandLineCompletions() will hijack the
// process, attempting to identify the intention behind this
// completion.  Regardless of the outcome of this deduction, the
// process will be terminated, similar to --helpshort flag
// handling.
//
// ** Overview of Bash completions:
// Bash can be told to programatically determine completions for the
// current 'cursor word'.  It does this by (in this case) invoking a
// command with some additional arguments identifying the command
// being executed, the word being completed, and the previous word
// (if any).  Bash then expects a sequence of output lines to be
// printed to stdout.  If these lines all contain a common prefix
// longer than the cursor word, bash will replace the cursor word
// with that common prefix, and display nothing.  If there isn't such
// a common prefix, bash will display the lines in pages using 'more'.
//
// ** Strategy taken for command line completions:
// If we can deduce either the exact flag intended, or a common flag
// prefix, we'll output exactly that.  Otherwise, if information
// must be displayed to the user, we'll take the opportunity to add
// some helpful information beyond just the flag name (specifically,
// we'll include the default flag value and as much of the flag's
// description as can fit on a single terminal line width, as specified
// by the flag --tab_completion_columns).  Furthermore, we'll try to
// make bash order the output such that the most useful or relevent
// flags are the most likely to be shown at the top.
//
// ** Additional features:
// To assist in finding that one really useful flag, substring matching
// was implemented.  Before pressing a <TAB> to get completion for the
// current word, you can append one or more '?' to the flag to do
// substring matching.  Here's the semantics:
//   --foo<TAB>     Show me all flags with names prefixed by 'foo'
//   --foo?<TAB>    Show me all flags with 'foo' somewhere in the name
//   --foo??<TAB>   Same as prior case, but also search in module
//                  definition path for 'foo'
//   --foo???<TAB>  Same as prior case, but also search in flag
//                  descriptions for 'foo'
// Finally, we'll trim the output to a relatively small number of
// flags to keep bash quiet about the verbosity of output.  If one
// really wanted to see all possible matches, appending a '+' to the
// search word will force the exhaustive list of matches to be printed.
//
// ** How to have bash accept completions from a binary:
// Bash requires that it be informed about each command that programmatic
// completion should be enabled for.  Example addition to a .bashrc
// file would be (your path to gflags_completions.sh file may differ):

/*
$ complete -o bashdefault -o default -o nospace -C                        \
 '/usr/local/bin/gflags_completions.sh --tab_completion_columns $COLUMNS' \
  time  env  binary_name  another_binary  [...]
*/

// This would allow the following to work:
//   $ /path/to/binary_name --vmodule<TAB>
// Or:
//   $ ./bin/path/another_binary --gfs_u<TAB>
// (etc)
//
// Sadly, it appears that bash gives no easy way to force this behavior for
// all commands.  That's where the "time" in the above example comes in.
// If you haven't specifically added a command to the list of completion
// supported commands, you can still get completions by prefixing the
// entire command with "env".
//   $ env /some/brand/new/binary --vmod<TAB>
// Assuming that "binary" is a newly compiled binary, this should still
// produce the expected completion output.


#ifndef GOOGLE_GFLAGS_COMPLETIONS_H_
#define GOOGLE_GFLAGS_COMPLETIONS_H_

namespace google {

void HandleCommandLineCompletions(void);

}

#endif  // GOOGLE_GFLAGS_COMPLETIONS_H_
