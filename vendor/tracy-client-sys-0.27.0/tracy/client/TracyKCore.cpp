#ifdef __linux__

#include <algorithm>
#include <assert.h>
#include <fcntl.h>
#include <limits.h>
#include <unistd.h>

#include "TracyDebug.hpp"
#include "TracyKCore.hpp"
#include "../common/TracyAlloc.hpp"

#if !defined(__GLIBC__) && !defined(__WORDSIZE)
// include __WORDSIZE headers for musl
#  include <bits/reg.h>
#endif

namespace tracy
{

using elf_half = uint16_t;
using elf_word = uint32_t;
using elf_sword = int32_t;

#if __WORDSIZE == 32
    using elf_addr = uint32_t;
    using elf_off = uint32_t;
    using elf_xword = uint32_t;
#else
    using elf_addr = uint64_t;
    using elf_off = uint64_t;
    using elf_xword = uint64_t;
#endif

struct elf_ehdr
{
    unsigned char e_ident[16];
    elf_half e_type;
    elf_half e_machine;
    elf_word e_version;
    elf_addr e_entry;
    elf_off e_phoff;
    elf_off e_shoff;
    elf_word e_flags;
    elf_half e_ehsize;
    elf_half e_phentsize;
    elf_half e_phnum;
    elf_half e_shentsize;
    elf_half e_shnum;
    elf_half e_shstrndx;
};

struct elf_phdr
{
    elf_word p_type;
    elf_word p_flags;
    elf_off p_offset;
    elf_addr p_vaddr;
    elf_addr p_paddr;
    elf_xword p_filesz;
    elf_xword p_memsz;
    uint64_t p_align;   // include 32-bit-only flags field for 32-bit compatibility
};

KCore::KCore()
    : m_offsets( 16 )
{
    m_fd = open( "/proc/kcore", O_RDONLY );
    if( m_fd == -1 ) return;

    elf_ehdr ehdr;
    if( read( m_fd, &ehdr, sizeof( ehdr ) ) != sizeof( ehdr ) ) goto err;

    assert( ehdr.e_phentsize == sizeof( elf_phdr ) );

    for( elf_half i=0; i<ehdr.e_phnum; i++ )
    {
        elf_phdr phdr;
        if( lseek( m_fd, ehdr.e_phoff + i * ehdr.e_phentsize, SEEK_SET ) == -1 ) goto err;
        if( read( m_fd, &phdr, sizeof( phdr ) ) != sizeof( phdr ) ) goto err;
        if( phdr.p_type != 1 ) continue;

        auto ptr = m_offsets.push_next();
        ptr->start = phdr.p_vaddr;
        ptr->size = phdr.p_memsz;
        ptr->offset = phdr.p_offset;
    }

    std::sort( m_offsets.begin(), m_offsets.end(), []( const Offset& lhs, const Offset& rhs ) { return lhs.start < rhs.start; } );
    TracyDebug( "KCore: %zu segments found\n", m_offsets.size() );
    return;

err:
    close( m_fd );
    m_fd = -1;
}

KCore::~KCore()
{
    if( m_fd != -1 ) close( m_fd );
}

void* KCore::Retrieve( uint64_t addr, uint64_t size ) const
{
    if( m_fd == -1 ) return nullptr;
    auto it = std::lower_bound( m_offsets.begin(), m_offsets.end(), addr, []( const Offset& lhs, uint64_t rhs ) { return lhs.start + lhs.size < rhs; } );
    if( it == m_offsets.end() ) return nullptr;
    if( addr + size > it->start + it->size ) return nullptr;
    if( lseek( m_fd, it->offset + addr - it->start, SEEK_SET ) == -1 ) return nullptr;
    auto ptr = tracy_malloc( size );
    if( read( m_fd, ptr, size ) != ssize_t( size ) )
    {
        tracy_free( ptr );
        return nullptr;
    }
    return ptr;
}

}

#endif