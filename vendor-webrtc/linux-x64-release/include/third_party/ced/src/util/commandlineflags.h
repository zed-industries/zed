// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

#ifndef UTIL_COMMANDLINEFLAGS_H_
#define UTIL_COMMANDLINEFLAGS_H_


#undef DEFINE_bool
#define DEFINE_bool(name, default_value, comment) \
    bool FLAGS_##name = default_value
#undef DEFINE_int32
#define DEFINE_int32(name, default_value, comment) \
    int32 FLAGS_##name = default_value
#undef DEFINE_string
#define DEFINE_string(name, default_value, comment) \
    string FLAGS_##name = default_value

#undef DECLARE_bool
#define DECLARE_bool(name) extern bool FLAGS_##name
#undef DECLARE_int32
#define DECLARE_int32(name) extern int32 FLAGS_##name
#undef DECLARE_string
#define DECLARE_string(name) extern string FLAGS_##name


#endif  // UTIL_COMMANDLINEFLAGS_H_
