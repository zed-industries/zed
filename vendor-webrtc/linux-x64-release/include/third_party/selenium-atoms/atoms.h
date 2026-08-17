/*
 * Copyright 2011-2014 Software Freedom Conservancy
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/* AUTO GENERATED - DO NOT EDIT BY HAND */
#ifndef WEBDRIVER_ATOMS_H
#define WEBDRIVER_ATOMS_H

#include <string>  // For std::(w)string.

namespace webdriver {
namespace atoms {

extern const char* const CLEAR[];
extern const char* const CLEAR_LOCAL_STORAGE[];
extern const char* const CLEAR_SESSION_STORAGE[];
extern const char* const CLICK[];
extern const char* const EXECUTE_ASYNC_SCRIPT[];
extern const char* const EXECUTE_SCRIPT[];
extern const char* const EXECUTE_SQL[];
extern const char* const FIND_ELEMENT[];
extern const char* const FIND_ELEMENTS[];
extern const char* const GET_APPCACHE_STATUS[];
extern const char* const GET_ATTRIBUTE[];
extern const char* const GET_EFFECTIVE_STYLE[];
extern const char* const GET_FIRST_CLIENT_RECT[];
extern const char* const GET_LOCAL_STORAGE_ITEM[];
extern const char* const GET_LOCAL_STORAGE_KEY[];
extern const char* const GET_LOCAL_STORAGE_KEYS[];
extern const char* const GET_LOCAL_STORAGE_SIZE[];
extern const char* const GET_LOCATION[];
extern const char* const GET_LOCATION_IN_VIEW[];
extern const char* const GET_PAGE_ZOOM[];
extern const char* const GET_SESSION_STORAGE_ITEM[];
extern const char* const GET_SESSION_STORAGE_KEY[];
extern const char* const GET_SESSION_STORAGE_KEYS[];
extern const char* const GET_SESSION_STORAGE_SIZE[];
extern const char* const GET_SIZE[];
extern const char* const GET_TEXT[];
extern const char* const IS_DISPLAYED[];
extern const char* const IS_ELEMENT_CLICKABLE[];
extern const char* const IS_ELEMENT_DISPLAYED[];
extern const char* const IS_ENABLED[];
extern const char* const IS_SELECTED[];
extern const char* const REMOVE_LOCAL_STORAGE_ITEM[];
extern const char* const REMOVE_SESSION_STORAGE_ITEM[];
extern const char* const SET_LOCAL_STORAGE_ITEM[];
extern const char* const SET_SESSION_STORAGE_ITEM[];
extern const char* const SUBMIT[];

static inline std::string asString(const char* const atom[]) {
  std::string source;
  for (int i = 0; atom[i] != NULL; i++) {
    source += atom[i];
  }
  return source;
}

}  // namespace atoms
}  // namespace webdriver

#endif  // WEBDRIVER_ATOMS_H
