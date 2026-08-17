#include <windows.h>

PVOID __stacker_get_current_fiber() {
    return GetCurrentFiber();
}
