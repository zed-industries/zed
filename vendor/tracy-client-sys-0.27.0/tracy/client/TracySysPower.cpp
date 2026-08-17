#include "TracySysPower.hpp"

#ifdef TRACY_HAS_SYSPOWER

#include <sys/types.h>
#include <dirent.h>
#include <chrono>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>

#include "TracyDebug.hpp"
#include "TracyProfiler.hpp"
#include "../common/TracyAlloc.hpp"

namespace tracy
{

SysPower::SysPower()
    : m_domains( 4 )
    , m_lastTime( 0 )
{
    ScanDirectory( "/sys/devices/virtual/powercap/intel-rapl", -1 );
}

SysPower::~SysPower()
{
    for( auto& v : m_domains )
    {
        fclose( v.handle );
        // Do not release v.name, as it may be still needed
    }
}

void SysPower::Tick()
{
    auto t = std::chrono::high_resolution_clock::now().time_since_epoch().count();
    if( t - m_lastTime > 10000000 )    // 10 ms
    {
        m_lastTime = t;
        for( auto& v : m_domains )
        {
            char tmp[32];
            if( fread( tmp, 1, 32, v.handle ) > 0 )
            {
                rewind( v.handle );
                auto p = (uint64_t)atoll( tmp );
                uint64_t delta;
                if( p >= v.value )
                {
                    delta = p - v.value;
                }
                else
                {
                    delta = v.overflow - v.value + p;
                }
                v.value = p;

                TracyLfqPrepare( QueueType::SysPowerReport );
                MemWrite( &item->sysPower.time, Profiler::GetTime() );
                MemWrite( &item->sysPower.delta, delta );
                MemWrite( &item->sysPower.name, (uint64_t)v.name );
                TracyLfqCommit;
            }
        }
    }
}

void SysPower::ScanDirectory( const char* path, int parent )
{
    DIR* dir = opendir( path );
    if( !dir ) return;
    struct dirent* ent;
    uint64_t maxRange = 0;
    char* name = nullptr;
    FILE* handle = nullptr;
    while( ( ent = readdir( dir ) ) )
    {
        if( ent->d_type == DT_REG )
        {
            if( strcmp( ent->d_name, "max_energy_range_uj" ) == 0 )
            {
                char tmp[PATH_MAX];
                snprintf( tmp, PATH_MAX, "%s/max_energy_range_uj", path );
                FILE* f = fopen( tmp, "r" );
                if( f )
                {
                    (void)fscanf( f, "%" PRIu64, &maxRange );
                    fclose( f );
                }
            }
            else if( strcmp( ent->d_name, "name" ) == 0 )
            {
                char tmp[PATH_MAX];
                snprintf( tmp, PATH_MAX, "%s/name", path );
                FILE* f = fopen( tmp, "r" );
                if( f )
                {
                    char ntmp[128];
                    if( fgets( ntmp, 128, f ) )
                    {
                        // Last character is newline, skip it
                        const auto sz = strlen( ntmp ) - 1;
                        if( parent < 0 )
                        {
                            name = (char*)tracy_malloc( sz + 1 );
                            memcpy( name, ntmp, sz );
                            name[sz] = '\0';
                        }
                        else
                        {
                            const auto p = m_domains[parent];
                            const auto psz = strlen( p.name );
                            name = (char*)tracy_malloc( psz + sz + 2 );
                            memcpy( name, p.name, psz );
                            name[psz] = ':';
                            memcpy( name+psz+1, ntmp, sz );
                            name[psz+sz+1] = '\0';
                        }
                    }
                    fclose( f );
                }
            }
            else if( strcmp( ent->d_name, "energy_uj" ) == 0 )
            {
                char tmp[PATH_MAX];
                snprintf( tmp, PATH_MAX, "%s/energy_uj", path );
                handle = fopen( tmp, "r" );
            }
        }
        if( name && handle && maxRange > 0 ) break;
    }
    if( name && handle && maxRange > 0 )
    {
        parent = (int)m_domains.size();
        Domain* domain = m_domains.push_next();
        domain->value = 0;
        domain->overflow = maxRange;
        domain->handle = handle;
        domain->name = name;
        TracyDebug( "Power domain id %i, %s found at %s\n", parent, name, path );
    }
    else
    {
        if( name ) tracy_free( name );
        if( handle ) fclose( handle );
    }

    rewinddir( dir );
    while( ( ent = readdir( dir ) ) )
    {
        if( ent->d_type == DT_DIR && strncmp( ent->d_name, "intel-rapl:", 11 ) == 0 )
        {
            char tmp[PATH_MAX];
            snprintf( tmp, PATH_MAX, "%s/%s", path, ent->d_name );
            ScanDirectory( tmp, parent );
        }
    }
    closedir( dir );
}

}

#endif
