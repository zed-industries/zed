// Copyright 2017 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <grp.h>
#include <pthread.h>
#include <pwd.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <syslog.h>

#include <string>
#include <vector>

#define TOTP "TOTP"
#define AUTHZEN "AUTHZEN"
#define INTERNAL_TWO_FACTOR "INTERNAL_TWO_FACTOR"
#define IDV_PREREGISTERED_PHONE "IDV_PREREGISTERED_PHONE"
#define SECURITY_KEY_OTP "SECURITY_KEY_OTP"
#define INITGROUP_CACHE_EXPIRE_SECONDS 500


#ifdef DEBUG
#undef DEBUG
#define DEBUG(fmt, ...)                                                        \
  do {                                                                         \
      openlog("nss_oslogin", LOG_PID|LOG_PERROR, LOG_DAEMON);                  \
      syslog(LOG_ERR, fmt, ##__VA_ARGS__);                                     \
      closelog();                                                              \
  } while (0)
#else
#define DEBUG(fmt, ...)                                                        \
  do {                                                                         \
  } while (0)
#endif /* DEBUG */

using std::string;
using std::vector;

namespace oslogin_utils {

// Metadata server URL.
static const char kMetadataServerUrl[] =
    "http://169.254.169.254/computeMetadata/v1/oslogin/";

// BufferManager encapsulates and manages a buffer and length. This class is not
// thread safe.
class BufferManager {
 public:
  // Create a BufferManager that will dole out chunks of buf as requested.
  BufferManager(char* buf, size_t buflen);

  // Copies a string to the buffer and sets the buffer to point to that
  // string. Copied string is guaranteed to be null-terminated.
  // Returns false and sets errnop if there is not enough space left in the
  // buffer for the string.
  bool AppendString(const string& value, char** buffer, int* errnop);

  // Return a pointer to a buffer of size bytes. Returns NULL and sets errnop to
  // ERANGE if there is not enough space left in the buffer for the request.
  void* Reserve(size_t bytes, int* errnop);

 private:
  // Whether there is space available in the buffer.
  bool CheckSpaceAvailable(size_t bytes_to_write) const;

  char* buf_;
  size_t buflen_;

  // Not copyable or assignable.
  BufferManager& operator=(const BufferManager&);
  BufferManager(const BufferManager&);
};

// Challenge represents a security challenge available to the user.
class Challenge {
 public:
  int id;
  string type;
  string status;
};

class Group {
 public:
  int64_t gid;
  string name;
};

// NssCache caches passwd entries for getpwent_r. This is used to prevent making
// an HTTP call on every getpwent_r invocation. Stores up to cache_size entries
// at a time. This class is not thread safe.
class NssCache {
 public:
  explicit NssCache(int cache_size);

  // Clears and resets the NssCache.
  void Reset();

  // Whether the cache has a next entry.
  bool HasNextEntry();

  // Whether the cache has reached the last page of the database.
  bool OnLastPage() { return on_last_page_; }

  // Grabs the next passwd or group entry. Returns true on success. Sets errnop on
  // failure.
  bool GetNextPasswd(BufferManager* buf, struct passwd* result, int* errnop);
  bool GetNextGroup(BufferManager* buf, struct group* result, int* errnop);

  // Loads a json array of passwd or group entries in the cache, starting at the
  // beginning of the cache. This will remove all previous entries in the cache.
  // response is expected to be a JSON array of passwd or group entries. Returns
  // true on success.
  bool LoadJsonUsersToCache(string response);
  bool LoadJsonGroupsToCache(string response, int* errnop);

  // Helper method for get(pw|gr)ent nss methods. Each call will iterate through the
  // OsLogin database and return the next entry.  Internally, the cache will
  // keep track of pages of user or group entries, and will make an http call to
  // the server if necessary to retrieve additional entries. Returns whether
  // retrieval was successful. If true, the result will contain
  // valid data.
  bool NssGetpwentHelper(BufferManager* buf, struct passwd* result, int* errnop);
  bool NssGetgrentHelper(BufferManager* buf, struct group* result, int* errnop);

  // Returns the page token for requesting the next page of entries.
  string GetPageToken() { return page_token_; }

 private:
  // The maximum size of the cache.
  int cache_size_;

  // Vector of entries. These are represented as stringified json object.
  std::vector<std::string> entry_cache_;

  // The page token for requesting the next page of entries.
  std::string page_token_;

  // Index for requesting the next entry from the cache.
  uint32_t index_;

  // Whether the NssCache has reached the last page of the database.
  bool on_last_page_;

  // Not copyable or assignable.
  NssCache& operator=(const NssCache&);
  NssCache(const NssCache&);
};

// Auto locks and unlocks a given mutex on construction/destruction. Does NOT
// take ownership of the mutex.
class MutexLock {
 public:
  explicit MutexLock(pthread_mutex_t* mutex) : mutex_(mutex) {
    pthread_mutex_lock(mutex_);
  }

  ~MutexLock() { pthread_mutex_unlock(mutex_); }

 private:
  // The mutex to lock/unlock
  pthread_mutex_t* const mutex_;

  // Not copyable or assignable.
  MutexLock& operator=(const MutexLock);
  MutexLock(const MutexLock&);
};

// Callback invoked when Curl completes a request.
size_t OnCurlWrite(void* buf, size_t size, size_t nmemb, void* userp);

// Uses Curl to issue a GET request to the given url. Returns whether the
// request was successful. If successful, the result from the server will be
// stored in response, and the HTTP response code will be stored in http_code.
bool HttpGet(const string& url, string* response, long* http_code);
bool HttpPost(const string& url, const string& data, string* response,
              long* http_code);
// Based on known MDS status codes returns whether the HTTP request
// should be retried or not.
bool ShouldRetry(long http_code);

// Returns whether user_name is a valid OsLogin user name.
bool ValidateUserName(const string& user_name);

// URL encodes the given parameter. Returns the encoded parameter.
std::string UrlEncode(const string& param);

// Returns true if the given passwd contains valid fields. If pw_dir, pw_shell,
// or pw_passwd are not set, this will populate these entries with default
// values.
bool ValidatePasswd(struct passwd* result, BufferManager* buf, int* errnop);

// Adds users and associated array of char* to provided buffer and store pointer
// to array in result.gr_mem.
bool AddUsersToGroup(std::vector<string> users, struct group* result,
                     BufferManager* buf, int* errnop);

// Iterates through all groups until one matching provided group is found,
// replacing gr_name with a buffermanager provided string.
bool FindGroup(struct group* grp, BufferManager* buf, int* errnop);

// Iterates through all users for a group, storing results in a provided string
// vector.
bool GetUsersForGroup(string groupname, std::vector<string>* users,
                      int* errnop);

// Iterates through all groups for a user, storing results in a provided string
// vector.
bool GetGroupsForUser(string username, std::vector<Group>* groups, int* errnop);

// Parses a JSON groups response, storing results in a provided Group vector.
bool ParseJsonToGroups(const string& json, std::vector<Group>* groups);

// Parses a JSON users response, storing results in a provided string vector.
bool ParseJsonToUsers(const string& json, std::vector<string>* users);

// Gets group matching name.
bool GetGroupByName(string name, struct group* grp, BufferManager* buf, int* errnop);

// Gets group matching GID.
bool GetGroupByGID(int gid, struct group* grp, BufferManager* buf, int* errnop);

// Iterates through all users for a group, storing results in a provided string vector.
bool GetUsersForGroup(string groupname, std::vector<string>* users, int* errnop);

// Iterates through all groups for a user, storing results in a provided string vector.
bool GetGroupsForUser(string username, std::vector<Group>* groups, int* errnop);

// Parses a JSON groups response, storing results in a provided Group vector.
bool ParseJsonToGroups(const string& json, std::vector<Group>* groups);

// Parses a JSON users response, storing results in a provided string vector.
bool ParseJsonToUsers(const string& json, std::vector<string> *users);

// Parses a JSON LoginProfiles response for SSH keys. Returns a vector of valid
// ssh_keys. A key is considered valid if it's expiration date is greater than
// current unix time.
std::vector<string> ParseJsonToSshKeys(const string& json);
std::vector<string> ParseJsonToSshKeysSk(const string& json);

// Parses a JSON object and returns the value associated with a given key.
bool ParseJsonToKey(const string& json, const string& key, string* response);

// Parses a JSON LoginProfiles response and returns the email under the "name"
// field.
bool ParseJsonToEmail(const string& json, string* email);

// Parses a JSON LoginProfiles response and populates the passwd struct with the
// corresponding values set in the JSON object. Returns whether the parse was
// successful or not. If unsuccessful, errnop will also be set.
bool ParseJsonToPasswd(const string& response, struct passwd* result,
                       BufferManager* buf, int* errnop);
bool ParseJsonToGroup(const string& response, struct group* result,
                       BufferManager* buf, int* errnop);

// Parses a JSON adminLogin or login response and returns whether the user has
// the requested privilege.
bool ParseJsonToSuccess(const string& json);

// Parses a JSON startSession response into a vector of Challenge objects.
bool ParseJsonToChallenges(const string& json, vector<Challenge>* challenges);

// Calls the startSession API.
bool StartSession(const string& email, string* response);

// Calls the continueSession API.
bool ContinueSession(bool alt, const string& email, const string& user_token,
                     const string& session_id, const Challenge& challenge,
                     string* response);

// Returns user information from the metadata server.
bool GetUser(const string& username, string* response);

// Initializes the global sys logger instance setting it up with the
// provided ident and app, so the syslog entries will look like:
// <<ident>>: <<app>>: <<Message>>
// For google_authorized_keys for example, it would look like:
// sshd: google_authorized_keys: <<Message>>
extern void SetupSysLog(const char *ident, const char *app);

// Closes the sys logger.
extern void CloseSysLog();

// Prints out to sys logger with ERR severity.
extern void SysLogErr(const char *fmt, ...);

// AuthoOptions wraps authorization options.
struct AuthOptions {
  // security_key determines if the MDS "/users?..." should use
  // the view=securityKey parameter.
  bool security_key;

  // fingerprint is used when authorizing certificate based
  // authentication sessions.
  char *fingerprint;

  // fp_len is the fingerprint string length;
  size_t fp_len;
};

// Perform user authorization logic & create users files and google sudoers, returns true if successful,
// and false otherwise.
bool AuthorizeUser(const char *user_name, struct AuthOptions opts, string *user_response);

// Given a file_path extracts the file name only. file_path must be a null terminated string.
const char *FileName(const char *file_path);
}  // namespace oslogin_utils
