/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#include <cstdio>

// File utilities for testing purposes.
//
// The ProjectRootPath() method is a convenient way of getting an absolute
// path to the project source tree root directory. Using this, it is easy to
// refer to test resource files in a portable way.
//
// Notice that even if Windows platforms use backslash as path delimiter, it is
// also supported to use slash, so there's no need for #ifdef checks in test
// code for setting up the paths to the resource files.
//
// Example use:
// Assume we have the following code being used in a test source file:
// const std::string kInputFile = webrtc::test::ProjectRootPath() +
//     "test/data/voice_engine/audio_long16.wav";
// // Use the kInputFile for the tests...
//
// Then here's some example outputs for different platforms:
// Linux:
// * Source tree located in /home/user/webrtc/trunk
// * Test project located in /home/user/webrtc/trunk/src/testproject
// * Test binary compiled as:
//   /home/user/webrtc/trunk/out/Debug/testproject_unittests
// Then ProjectRootPath() will return /home/user/webrtc/trunk/ no matter if
// the test binary is executed from standing in either of:
// /home/user/webrtc/trunk
// or
// /home/user/webrtc/trunk/out/Debug
// (or any other directory below the trunk for that matter).
//
// Windows:
// * Source tree located in C:\Users\user\webrtc\trunk
// * Test project located in C:\Users\user\webrtc\trunk\src\testproject
// * Test binary compiled as:
//   C:\Users\user\webrtc\trunk\src\testproject\Debug\testproject_unittests.exe
// Then ProjectRootPath() will return C:\Users\user\webrtc\trunk\ when the
// test binary is executed from inside Visual Studio.
// It will also return the same path if the test is executed from a command
// prompt standing in C:\Users\user\webrtc\trunk\src\testproject\Debug
//
// Mac:
// * Source tree located in /Users/user/webrtc/trunk
// * Test project located in /Users/user/webrtc/trunk/src/testproject
// * Test binary compiled as:
//   /Users/user/webrtc/trunk/xcodebuild/Debug/testproject_unittests
// Then ProjectRootPath() will return /Users/user/webrtc/trunk/ no matter if
// the test binary is executed from standing in either of:
// /Users/user/webrtc/trunk
// or
// /Users/user/webrtc/trunk/out/Debug
// (or any other directory below the trunk for that matter).

#ifndef WEBRTC_TEST_TESTSUPPORT_FILEUTILS_H_
#define WEBRTC_TEST_TESTSUPPORT_FILEUTILS_H_

#include <string>

namespace webrtc {
namespace test {

// This is the "directory" returned if the ProjectPath() function fails
// to find the project root.
extern const char* kCannotFindProjectRootDir;

// Finds the root dir of the project, to be able to set correct paths to
// resource files used by tests.
// The implementation is simple: it just looks for the file defined by
// kProjectRootFileName, starting in the current directory (the working
// directory) and then steps upward until it is found (or it is at the root of
// the file system).
// If the current working directory is above the project root dir, it will not
// be found.
//
// If symbolic links occur in the path they will be resolved and the actual
// directory will be returned.
//
// Returns the absolute path to the project root dir (usually the trunk dir)
// WITH a trailing path delimiter.
// If the project root is not found, the string specified by
// kCannotFindProjectRootDir is returned.
std::string ProjectRootPath();

// Creates and returns the absolute path to the output directory where log files
// and other test artifacts should be put. The output directory is generally a
// directory named "out" at the top-level of the project, i.e. a subfolder to
// the path returned by ProjectRootPath(). The exception is Android where we use
// /sdcard/ instead.
//
// Details described for ProjectRootPath() apply here too.
//
// Returns the path WITH a trailing path delimiter. If the project root is not
// found, the current working directory ("./") is returned as a fallback.
std::string OutputPath();

// Returns a path to a resource file for the currently executing platform.
// Adapts to what filenames are currently present in the
// [project-root]/resources/ dir.
// Returns an absolute path according to this priority list (the directory
// part of the path is left out for readability):
// 1. [name]_[platform]_[architecture].[extension]
// 2. [name]_[platform].[extension]
// 3. [name]_[architecture].[extension]
// 4. [name].[extension]
// Where
// * platform is either of "win", "mac" or "linux".
// * architecture is either of "32" or "64".
//
// Arguments:
//    name - Name of the resource file. If a plain filename (no directory path)
//           is supplied, the file is assumed to be located in resources/
//           If a directory path is prepended to the filename, a subdirectory
//           hierarchy reflecting that path is assumed to be present.
//    extension - File extension, without the dot, i.e. "bmp" or "yuv".
std::string ResourcePath(std::string name, std::string extension);

// Gets the current working directory for the executing program.
// Returns "./" if for some reason it is not possible to find the working
// directory.
std::string WorkingDir();

// Creates a directory if it not already exists.
// Returns true if successful. Will print an error message to stderr and return
// false if a file with the same name already exists.
bool CreateDirectory(std::string directory_name);

// File size of the supplied file in bytes. Will return 0 if the file is
// empty or if the file does not exist/is readable.
size_t GetFileSize(std::string filename);

// Sets the executable path, i.e. the path to the executable that is being used
// when launching it. This is usually the path relative to the working directory
// but can also be an absolute path. The intention with this function is to pass
// the argv[0] being sent into the main function to make it possible for
// fileutils.h to find the correct project paths even when the working directory
// is outside the project tree (which happens in some cases).
void SetExecutablePath(const std::string& path_to_executable);

}  // namespace test
}  // namespace webrtc

#endif  // WEBRTC_TEST_TESTSUPPORT_FILEUTILS_H_
