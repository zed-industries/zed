/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_SQLITE_MODULE_STATE_MANAGER_H_
#define SRC_TRACE_PROCESSOR_SQLITE_MODULE_STATE_MANAGER_H_

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"

namespace perfetto::trace_processor::sqlite {

// Base class for ModuleStateManager. Used to reduce the binary size of
// ModuleStateManager and also provide a type-erased interface for the
// engines to hold onto (e.g. to call OnCommit, OnRollback, etc).
class ModuleStateManagerBase {
 public:
  // Per-vtab state. The pointer to this class should be stored in the Vtab.
  struct PerVtabState {
   private:
    // The below fields should only be accessed by the manager, use GetState to
    // access the state from outside this class.
    friend class ModuleStateManagerBase;

    // The name of the vtab.
    std::string name;

    // A hash of all the arguments passed to the module from SQLite. This
    // acts as the unique identifier for the vtab state.
    uint64_t argv_hash;

    // A pointer to the manager object. Backreference for use by static
    // functions in this class.
    ModuleStateManagerBase* manager;

    // The actual state object which will be used by the module.
    // The deleter is a function pointer which will be set by the templated
    // manager class.
    std::unique_ptr<void, void (*)(void*)> state{nullptr, nullptr};

    enum {
      kCommitted,
      kCreatedButNotCommitted,
      kDestroyedButNotCommitted
    } lifecycle;
  };

  // Called by the engine when a transaction is committed.
  //
  // This is used to finalize all the destroys performed since a previous
  // rollback or commit.
  void OnCommit() {
    std::vector<uint64_t> to_erase;
    for (auto it = state_by_args_hash_.GetIterator(); it; ++it) {
      if (it.value()->lifecycle == PerVtabState::kDestroyedButNotCommitted) {
        to_erase.push_back(it.key());
      } else {
        it.value()->lifecycle = PerVtabState::kCommitted;
      }
    }
    for (const auto& hash : to_erase) {
      state_by_args_hash_.Erase(hash);
    }
  }

  // Called by the engine when a transaction is rolled back.
  //
  // This is used to undo the effects of all the destroys performed since a
  // previous rollback or commit.
  void OnRollback() {
    std::vector<uint64_t> to_erase;
    for (auto it = state_by_args_hash_.GetIterator(); it; ++it) {
      if (it.value()->lifecycle == PerVtabState::kCreatedButNotCommitted) {
        to_erase.push_back(it.key());
      } else {
        it.value()->lifecycle = PerVtabState::kCommitted;
      }
    }
    for (const auto& hash : to_erase) {
      state_by_args_hash_.Erase(hash);
    }
  }

 protected:
  // Enforce that anyone who wants to use this class inherits from it.
  ModuleStateManagerBase() = default;

  // Type-erased counterparts of ModuleStateManager functions. See below for
  // documentation of these functions.
  [[nodiscard]] PerVtabState* OnCreate(
      int argc,
      const char* const* argv,
      std::unique_ptr<void, void (*)(void*)> state);
  [[nodiscard]] PerVtabState* OnConnect(int argc, const char* const* argv);
  static void OnDisconnect(PerVtabState* state);
  static void OnDestroy(PerVtabState* state);
  static void* GetState(PerVtabState* s);
  void* FindStateByNameSlow(std::string_view name);

 private:
  using StateMap = base::FlatHashMap<uint64_t,
                                     std::unique_ptr<PerVtabState>,
                                     base::AlreadyHashed<uint64_t>>;
  static uint64_t ComputeHash(int argc, const char* const* argv);

  StateMap state_by_args_hash_;
};

// Helper class which abstracts away management of per-vtab state of an SQLite
// virtual table module.
//
// SQLite has some subtle semantics around lifecycle of vtabs which makes state
// management complex. This class attempts to encapsulate some of that
// complexity as a central place where we can document the quirks.
//
// Usage of this class:
// struct MyModule : sqlite::Module<MyModule> {
//   // Make the context object inherit from ModuleStateManager.
//   struct Context : ModuleStateManager<MyModule> {
//     ... (other fields)
//   }
//   struct Vtab : sqlite::Module<MyModule>::Vtab {
//     // Store the per-vtab-state pointer in the vtab object.
//     ModuleStateManager<MyModule>::PerVtabState* state;
//     ... (other fields)
//   }
//   static void OnCreate(...) {
//     ...
//     // Call OnCreate on the manager object and store the returned pointer.
//     tab->state = ctx->OnCreate(argv);
//     ...
//   }
//   static void OnDestroy(...) {
//     ...
//     // Call OnDestroy with the stored state pointer.
//     sqlite::ModuleStateManager<MyModule>::OnDestroy(tab->state);
//     ...
//   }
//   // Do the same in OnConnect and OnDisconnect as in OnCreate and OnDestroy
//   // respectively.
//   static void OnConnect(...)
//   static void OnDisconnect(...)
// }
template <typename Module>
class ModuleStateManager : public ModuleStateManagerBase {
 public:
  // Lifecycle method to be called from Module::Create.
  [[nodiscard]] PerVtabState* OnCreate(
      int argc,
      const char* const* argv,
      std::unique_ptr<typename Module::State> state) {
    std::unique_ptr<void, void (*)(void*)> erased_state =
        std::unique_ptr<void, void (*)(void*)>(state.release(), [](void* ptr) {
          delete static_cast<typename Module::State*>(ptr);
        });
    return ModuleStateManagerBase::OnCreate(argc, argv,
                                            std::move(erased_state));
  }

  // Lifecycle method to be called from Module::Connect.
  [[nodiscard]] PerVtabState* OnConnect(int argc, const char* const* argv) {
    return ModuleStateManagerBase::OnConnect(argc, argv);
  }

  // Lifecycle method to be called from Module::Disconnect.
  static void OnDisconnect(PerVtabState* state) {
    ModuleStateManagerBase::OnDisconnect(state);
  }

  // Lifecycle method to be called from Module::Destroy.
  static void OnDestroy(PerVtabState* state) {
    ModuleStateManagerBase::OnDestroy(state);
  }

  // Method to be called from module callbacks to extract the module state
  // from the manager state.
  static typename Module::State* GetState(PerVtabState* s) {
    return static_cast<typename Module::State*>(
        ModuleStateManagerBase::GetState(s));
  }

  // Looks up the state of a module by name in O(n) time. This function should
  // not be called in the performance sensitive contexts. It must also be called
  // in a case where there are not multiple vtabs with the same name. This can
  // happen inside a transaction context where we are executing a "CREATE OR
  // REPLACE" operation.
  //
  // This function should only be called for speculative lookups from outside
  // the module implementation: use |GetState| inside the sqlite::Module
  // implementation.
  typename Module::State* FindStateByNameSlow(std::string_view name) {
    return static_cast<typename Module::State*>(
        ModuleStateManagerBase::FindStateByNameSlow(name));
  }

 protected:
  // Enforce that anyone who wants to use this class inherits from it.
  ModuleStateManager() = default;
};

}  // namespace perfetto::trace_processor::sqlite

#endif  // SRC_TRACE_PROCESSOR_SQLITE_MODULE_STATE_MANAGER_H_
