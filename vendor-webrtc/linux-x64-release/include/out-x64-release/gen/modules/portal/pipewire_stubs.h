// This is generated file. Do not modify directly.

#ifndef GEN_MODULES_PORTAL_PIPEWIRE_STUBS_H_
#define GEN_MODULES_PORTAL_PIPEWIRE_STUBS_H_

#include <map>
#include <string>
#include <vector>

namespace modules_portal {
// Individual module initializer functions.
bool IsPipewireInitialized();
void InitializePipewire(void* module);
void UninitializePipewire();

// Enum and typedef for umbrella initializer.
enum StubModules {
  kModulePipewire = 0,
  kNumStubModules
};

typedef std::map<StubModules, std::vector<std::string>> StubPathMap;

// Umbrella initializer for all the modules in this stub file.
bool InitializeStubs(const StubPathMap& path_map);
}  // namespace modules_portal

#endif  // GEN_MODULES_PORTAL_PIPEWIRE_STUBS_H_
