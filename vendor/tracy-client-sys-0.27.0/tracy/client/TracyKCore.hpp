#ifndef __TRACYKCORE_HPP__
#define __TRACYKCORE_HPP__

#ifdef __linux__

#include <stdint.h>

#include "TracyFastVector.hpp"

namespace tracy
{

class KCore
{
    struct Offset
    {
        uint64_t start;
        uint64_t size;
        uint64_t offset;
    };

public:
    KCore();
    ~KCore();

    void* Retrieve( uint64_t addr, uint64_t size ) const;

private:
    int m_fd;
    FastVector<Offset> m_offsets;
};

}

#endif

#endif
