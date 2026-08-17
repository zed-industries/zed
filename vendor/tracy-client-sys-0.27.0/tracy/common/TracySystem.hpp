#ifndef __TRACYSYSTEM_HPP__
#define __TRACYSYSTEM_HPP__

#include <stdint.h>

#include "TracyApi.h"

namespace tracy
{

namespace detail
{
TRACY_API uint32_t GetThreadHandleImpl();
}

#ifdef TRACY_ENABLE
struct ThreadNameData
{
    uint32_t id;
    int32_t groupHint;
    const char* name;
    ThreadNameData* next;
};

ThreadNameData* GetThreadNameData( uint32_t id );

TRACY_API uint32_t GetThreadHandle();
#else
static inline uint32_t GetThreadHandle()
{
    return detail::GetThreadHandleImpl();
}
#endif

TRACY_API void SetThreadName( const char* name );
TRACY_API void SetThreadNameWithHint( const char* name, int32_t groupHint );
TRACY_API const char* GetThreadName( uint32_t id );

TRACY_API const char* GetEnvVar( const char* name );

}

#endif
