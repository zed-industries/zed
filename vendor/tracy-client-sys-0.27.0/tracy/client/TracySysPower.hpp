#ifndef __TRACYSYSPOWER_HPP__
#define __TRACYSYSPOWER_HPP__

#if defined __linux__
#  define TRACY_HAS_SYSPOWER
#endif

#ifdef TRACY_HAS_SYSPOWER

#include <stdint.h>
#include <stdio.h>

#include "TracyFastVector.hpp"

namespace tracy
{

class SysPower
{
    struct Domain
    {
        uint64_t value;
        uint64_t overflow;
        FILE* handle;
        const char* name;
    };

public:
    SysPower();
    ~SysPower();

    void Tick();

private:
    void ScanDirectory( const char* path, int parent );

    FastVector<Domain> m_domains;
    uint64_t m_lastTime;
};

}
#endif

#endif
