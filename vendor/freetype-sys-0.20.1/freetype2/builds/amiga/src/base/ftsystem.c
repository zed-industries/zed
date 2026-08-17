/***************************************************************************/
/*                                                                         */
/*  ftsystem.c                                                             */
/*                                                                         */
/*    Amiga-specific FreeType low-level system interface (body).           */
/*                                                                         */
/*  Copyright (C) 1996-2023 by                                             */
/*  David Turner, Robert Wilhelm, Werner Lemberg and Detlef Würkner.       */
/*                                                                         */
/*  This file is part of the FreeType project, and may only be used,       */
/*  modified, and distributed under the terms of the FreeType project      */
/*  license, LICENSE.TXT.  By continuing to use, modify, or distribute     */
/*  this file you indicate that you have read the license and              */
/*  understand and accept it fully.                                        */
/*                                                                         */
/***************************************************************************/

  /*************************************************************************/
  /*                                                                       */
  /* This file contains the Amiga interface used by FreeType to access     */
  /* low-level, i.e. memory management, i/o access as well as thread       */
  /* synchronisation.                                                      */
  /*                                                                       */
  /*************************************************************************/


  /*************************************************************************/
  /*                                                                       */
  /* Maintained by Detlef Würkner <TetiSoft@apg.lahn.de>                   */
  /*                                                                       */
  /* Based on the original ftsystem.c,                                     */
  /* modified to avoid fopen(), fclose(), fread(), fseek(), ftell(),       */
  /* malloc(), realloc(), and free().                                      */
  /*                                                                       */
  /* Those C library functions are often not thread-safe or cant be        */
  /* used in a shared Amiga library. If that's not a problem for you,       */
  /* you can of course use the default ftsystem.c with C library calls     */
  /* instead.                                                              */
  /*                                                                       */
  /* This implementation needs exec V39+ because it uses AllocPooled() etc */
  /*                                                                       */
  /*************************************************************************/

#define __NOLIBBASE__
#define __NOGLOBALIFACE__
#define __USE_INLINE__
#include <proto/exec.h>
#include <dos/stdio.h>
#include <proto/dos.h>
#ifdef __amigaos4__
extern struct ExecIFace *IExec;
extern struct DOSIFace  *IDOS;
#else
extern struct Library   *SysBase;
extern struct Library   *DOSBase;
#endif

#define IOBUF_SIZE 512

/* structure that helps us to avoid
 * useless calls of Seek() and Read()
 */
struct SysFile
{
  BPTR  file;
  ULONG iobuf_start;
  ULONG iobuf_end;
  UBYTE iobuf[IOBUF_SIZE];
};

#ifndef __amigaos4__
/* C implementation of AllocVecPooled (see autodoc exec/AllocPooled) */
APTR
Alloc_VecPooled( APTR   poolHeader,
                 ULONG  memSize )
{
  ULONG  newSize = memSize + sizeof ( ULONG );
  ULONG  *mem = AllocPooled( poolHeader, newSize );

  if ( !mem )
    return NULL;
  *mem = newSize;
  return mem + 1;
}

/* C implementation of FreeVecPooled (see autodoc exec/AllocPooled) */
void
Free_VecPooled( APTR  poolHeader,
                APTR  memory )
{
  ULONG  *realmem = (ULONG *)memory - 1;

  FreePooled( poolHeader, realmem, *realmem );
}
#endif

#include <ft2build.h>
#include FT_CONFIG_CONFIG_H
#include <freetype/internal/ftdebug.h>
#include <freetype/ftsystem.h>
#include <freetype/fterrors.h>
#include <freetype/fttypes.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>


  /*************************************************************************/
  /*                                                                       */
  /*                       MEMORY MANAGEMENT INTERFACE                     */
  /*                                                                       */
  /*************************************************************************/

  /*************************************************************************/
  /*                                                                       */
  /* It is not necessary to do any error checking for the                  */
  /* allocation-related functions.  This is done by the higher level       */
  /* routines like ft_mem_alloc() or ft_mem_realloc().                     */
  /*                                                                       */
  /*************************************************************************/


  /*************************************************************************/
  /*                                                                       */
  /* <Function>                                                            */
  /*    ft_alloc                                                           */
  /*                                                                       */
  /* <Description>                                                         */
  /*    The memory allocation function.                                    */
  /*                                                                       */
  /* <Input>                                                               */
  /*    memory :: A pointer to the memory object.                          */
  /*                                                                       */
  /*    size   :: The requested size in bytes.                             */
  /*                                                                       */
  /* <Return>                                                              */
  /*    The address of newly allocated block.                              */
  /*                                                                       */
  FT_CALLBACK_DEF( void* )
  ft_alloc( FT_Memory  memory,
            long       size )
  {
#ifdef __amigaos4__
    return AllocVecPooled( memory->user, size );
#else
    return Alloc_VecPooled( memory->user, size );
#endif
  }


  /*************************************************************************/
  /*                                                                       */
  /* <Function>                                                            */
  /*    ft_realloc                                                         */
  /*                                                                       */
  /* <Description>                                                         */
  /*    The memory reallocation function.                                  */
  /*                                                                       */
  /* <Input>                                                               */
  /*    memory   :: A pointer to the memory object.                        */
  /*                                                                       */
  /*    cur_size :: The current size of the allocated memory block.        */
  /*                                                                       */
  /*    new_size :: The newly requested size in bytes.                     */
  /*                                                                       */
  /*    block    :: The current address of the block in memory.            */
  /*                                                                       */
  /* <Return>                                                              */
  /*    The address of the reallocated memory block.                       */
  /*                                                                       */
  FT_CALLBACK_DEF( void* )
  ft_realloc( FT_Memory  memory,
              long       cur_size,
              long       new_size,
              void*      block )
  {
    void* new_block;

#ifdef __amigaos4__
    new_block = AllocVecPooled ( memory->user, new_size );
#else
    new_block = Alloc_VecPooled ( memory->user, new_size );
#endif
    if ( new_block != NULL )
    {
      CopyMem ( block, new_block,
                ( new_size > cur_size ) ? cur_size : new_size );
#ifdef __amigaos4__
      FreeVecPooled ( memory->user, block );
#else
      Free_VecPooled ( memory->user, block );
#endif
    }
    return new_block;
  }


  /*************************************************************************/
  /*                                                                       */
  /* <Function>                                                            */
  /*    ft_free                                                            */
  /*                                                                       */
  /* <Description>                                                         */
  /*    The memory release function.                                       */
  /*                                                                       */
  /* <Input>                                                               */
  /*    memory  :: A pointer to the memory object.                         */
  /*                                                                       */
  /*    block   :: The address of block in memory to be freed.             */
  /*                                                                       */
  FT_CALLBACK_DEF( void )
  ft_free( FT_Memory  memory,
           void*      block )
  {
#ifdef __amigaos4__
    FreeVecPooled( memory->user, block );
#else
    Free_VecPooled( memory->user, block );
#endif
  }


  /*************************************************************************/
  /*                                                                       */
  /*                     RESOURCE MANAGEMENT INTERFACE                     */
  /*                                                                       */
  /*************************************************************************/


  /*************************************************************************/
  /*                                                                       */
  /* The macro FT_COMPONENT is used in trace mode.  It is an implicit      */
  /* parameter of the FT_TRACE() and FT_ERROR() macros, used to print/log  */
  /* messages during execution.                                            */
  /*                                                                       */
#undef  FT_COMPONENT
#define FT_COMPONENT  io

  /* We use the macro STREAM_FILE for convenience to extract the       */
  /* system-specific stream handle from a given FreeType stream object */
#define STREAM_FILE( stream )  ( (struct SysFile *)stream->descriptor.pointer )


  /*************************************************************************/
  /*                                                                       */
  /* <Function>                                                            */
  /*    ft_amiga_stream_close                                              */
  /*                                                                       */
  /* <Description>                                                         */
  /*    The function to close a stream.                                    */
  /*                                                                       */
  /* <Input>                                                               */
  /*    stream :: A pointer to the stream object.                          */
  /*                                                                       */
  FT_CALLBACK_DEF( void )
  ft_amiga_stream_close( FT_Stream  stream )
  {
    struct SysFile* sysfile;

    sysfile = STREAM_FILE( stream );
    Close ( sysfile->file );
    FreeMem ( sysfile, sizeof ( struct SysFile ));

    stream->descriptor.pointer = NULL;
    stream->size               = 0;
    stream->base               = NULL;
  }


  /*************************************************************************/
  /*                                                                       */
  /* <Function>                                                            */
  /*    ft_amiga_stream_io                                                 */
  /*                                                                       */
  /* <Description>                                                         */
  /*    The function to open a stream.                                     */
  /*                                                                       */
  /* <Input>                                                               */
  /*    stream :: A pointer to the stream object.                          */
  /*                                                                       */
  /*    offset :: The position in the data stream to start reading.        */
  /*                                                                       */
  /*    buffer :: The address of buffer to store the read data.            */
  /*                                                                       */
  /*    count  :: The number of bytes to read from the stream.             */
  /*                                                                       */
  /* <Return>                                                              */
  /*    The number of bytes actually read.                                 */
  /*                                                                       */
  FT_CALLBACK_DEF( unsigned long )
  ft_amiga_stream_io( FT_Stream       stream,
                      unsigned long   offset,
                      unsigned char*  buffer,
                      unsigned long   count )
  {
    struct SysFile* sysfile;
    unsigned long   read_bytes;

    if ( count != 0 )
    {
      sysfile = STREAM_FILE( stream );

      /* handle the seek */
      if ( (offset < sysfile->iobuf_start) || (offset + count > sysfile->iobuf_end) )
      {
        /* requested offset implies we need a buffer refill */
        if ( !sysfile->iobuf_end || offset != sysfile->iobuf_end )
        {
          /* a physical seek is necessary */
          Seek( sysfile->file, offset, OFFSET_BEGINNING );
        }
        sysfile->iobuf_start = offset;
        sysfile->iobuf_end = 0; /* trigger a buffer refill */
      }

      /* handle the read */
      if ( offset + count <= sysfile->iobuf_end )
      {
        /* we have buffer and requested bytes are all inside our buffer */
        CopyMem( &sysfile->iobuf[offset - sysfile->iobuf_start], buffer, count );
        read_bytes = count;
      }
      else
      {
        /* (re)fill buffer */
        if ( count <= IOBUF_SIZE )
        {
          /* requested bytes is a subset of the buffer */
          read_bytes = Read( sysfile->file, sysfile->iobuf, IOBUF_SIZE );
          if ( read_bytes == -1UL )
          {
            /* error */
            read_bytes = 0;
          }
          else
          {
            sysfile->iobuf_end = offset + read_bytes;
            CopyMem( sysfile->iobuf, buffer, count );
            if ( read_bytes > count )
            {
              read_bytes = count;
            }
          }
        }
        else
        {
          /* we actually need more than our buffer can hold, so we decide
          ** to do a single big read, and then copy the last IOBUF_SIZE
          ** bytes of that to our internal buffer for later use */
          read_bytes = Read( sysfile->file, buffer, count );
          if ( read_bytes == -1UL )
          {
            /* error */
            read_bytes = 0;
          }
          else
          {
            ULONG bufsize;

            bufsize = ( read_bytes > IOBUF_SIZE ) ? IOBUF_SIZE : read_bytes;
            sysfile->iobuf_end = offset + read_bytes;
            sysfile->iobuf_start = sysfile->iobuf_end - bufsize;
            CopyMem( &buffer[read_bytes - bufsize] , sysfile->iobuf, bufsize );
          }
        }
      }
    }
    else
    {
      read_bytes = 0;
    }

    return read_bytes;
  }


  /* documentation is in ftobjs.h */

  FT_BASE_DEF( FT_Error )
  FT_Stream_Open( FT_Stream    stream,
                  const char*  filepathname )
  {
    struct FileInfoBlock*  fib;
    struct SysFile*        sysfile;


    if ( !stream )
      return FT_THROW( Invalid_Stream_Handle );

#ifdef __amigaos4__
    sysfile = AllocMem ( sizeof (struct SysFile ), MEMF_SHARED );
#else
    sysfile = AllocMem ( sizeof (struct SysFile ), MEMF_PUBLIC );
#endif
    if ( !sysfile )
    {
      FT_ERROR(( "FT_Stream_Open:" ));
      FT_ERROR(( " could not open `%s'\n", filepathname ));

      return FT_THROW( Cannot_Open_Resource );
    }
    sysfile->file = Open( (STRPTR)filepathname, MODE_OLDFILE );
    if ( !sysfile->file )
    {
      FreeMem ( sysfile, sizeof ( struct SysFile ));
      FT_ERROR(( "FT_Stream_Open:" ));
      FT_ERROR(( " could not open `%s'\n", filepathname ));

      return FT_THROW( Cannot_Open_Resource );
    }

    fib = AllocDosObject( DOS_FIB, NULL );
    if ( !fib )
    {
      Close ( sysfile->file );
      FreeMem ( sysfile, sizeof ( struct SysFile ));
      FT_ERROR(( "FT_Stream_Open:" ));
      FT_ERROR(( " could not open `%s'\n", filepathname ));

      return FT_THROW( Cannot_Open_Resource );
    }
    if ( !( ExamineFH( sysfile->file, fib ) ) )
    {
      FreeDosObject( DOS_FIB, fib );
      Close ( sysfile->file );
      FreeMem ( sysfile, sizeof ( struct SysFile ));
      FT_ERROR(( "FT_Stream_Open:" ));
      FT_ERROR(( " could not open `%s'\n", filepathname ));

      return FT_THROW( Cannot_Open_Resource );
    }
    stream->size = fib->fib_Size;
    FreeDosObject( DOS_FIB, fib );

    stream->descriptor.pointer = (void *)sysfile;
    stream->pathname.pointer   = (char*)filepathname;
    sysfile->iobuf_start       = 0;
    sysfile->iobuf_end         = 0;
    stream->pos                = 0;

    stream->read  = ft_amiga_stream_io;
    stream->close = ft_amiga_stream_close;

    if ( !stream->size )
    {
      ft_amiga_stream_close( stream );
      FT_ERROR(( "FT_Stream_Open:" ));
      FT_ERROR(( " opened `%s' but zero-sized\n", filepathname ));
      return FT_THROW( Cannot_Open_Stream );
    }

    FT_TRACE1(( "FT_Stream_Open:" ));
    FT_TRACE1(( " opened `%s' (%ld bytes) successfully\n",
                filepathname, stream->size ));

    return FT_Err_Ok;
  }


#ifdef FT_DEBUG_MEMORY

  extern FT_Int
  ft_mem_debug_init( FT_Memory  memory );

  extern void
  ft_mem_debug_done( FT_Memory  memory );

#endif


  /* documentation is in ftobjs.h */

  FT_BASE_DEF( FT_Memory )
  FT_New_Memory( void )
  {
    FT_Memory  memory;


#ifdef __amigaos4__
    memory = (FT_Memory)AllocVec( sizeof ( *memory ), MEMF_SHARED );
#else
    memory = (FT_Memory)AllocVec( sizeof ( *memory ), MEMF_PUBLIC );
#endif
    if ( memory )
    {
#ifdef __amigaos4__
      memory->user = CreatePool( MEMF_SHARED, 16384, 16384 );
#else
      memory->user = CreatePool( MEMF_PUBLIC, 16384, 16384 );
#endif
      if ( memory->user == NULL )
      {
        FreeVec( memory );
        memory = NULL;
      }
      else
      {
        memory->alloc   = ft_alloc;
        memory->realloc = ft_realloc;
        memory->free    = ft_free;
#ifdef FT_DEBUG_MEMORY
        ft_mem_debug_init( memory );
#endif
      }
    }

    return memory;
  }


  /* documentation is in ftobjs.h */

  FT_BASE_DEF( void )
  FT_Done_Memory( FT_Memory  memory )
  {
#ifdef FT_DEBUG_MEMORY
    ft_mem_debug_done( memory );
#endif

    DeletePool( memory->user );
    FreeVec( memory );
  }

/*
Local Variables:
coding: latin-1
End:
*/
/* END */
