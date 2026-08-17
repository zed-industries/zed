/***************************************************************************/
/*                                                                         */
/*  ftmac.c                                                                */
/*                                                                         */
/*    Mac FOND support.  Written by just@letterror.com.                    */
/*  Heavily Fixed by mpsuzuki, George Williams and Sean McBride            */
/*                                                                         */
/*  Copyright (C) 1996-2023 by                                             */
/*  Just van Rossum, David Turner, Robert Wilhelm, and Werner Lemberg.     */
/*                                                                         */
/*  This file is part of the FreeType project, and may only be used,       */
/*  modified, and distributed under the terms of the FreeType project      */
/*  license, LICENSE.TXT.  By continuing to use, modify, or distribute     */
/*  this file you indicate that you have read the license and              */
/*  understand and accept it fully.                                        */
/*                                                                         */
/***************************************************************************/


  /*
    Notes

    Mac suitcase files can (and often do!) contain multiple fonts.  To
    support this I use the face_index argument of FT_(Open|New)_Face()
    functions, and pretend the suitcase file is a collection.

    Warning: fbit and NFNT bitmap resources are not supported yet.  In old
    sfnt fonts, bitmap glyph data for each size is stored in each `NFNT'
    resources instead of the `bdat' table in the sfnt resource.  Therefore,
    face->num_fixed_sizes is set to 0, because bitmap data in `NFNT'
    resource is unavailable at present.

    The Mac FOND support works roughly like this:

    - Check whether the offered stream points to a Mac suitcase file.  This
      is done by checking the file type: it has to be 'FFIL' or 'tfil'.  The
      stream that gets passed to our init_face() routine is a stdio stream,
      which isn't usable for us, since the FOND resources live in the
      resource fork.  So we just grab the stream->pathname field.

    - Read the FOND resource into memory, then check whether there is a
      TrueType font and/or(!) a Type 1 font available.

    - If there is a Type 1 font available (as a separate `LWFN' file), read
      its data into memory, massage it slightly so it becomes PFB data, wrap
      it into a memory stream, load the Type 1 driver and delegate the rest
      of the work to it by calling FT_Open_Face().  (XXX TODO: after this
      has been done, the kerning data from the FOND resource should be
      appended to the face: On the Mac there are usually no AFM files
      available.  However, this is tricky since we need to map Mac char
      codes to ps glyph names to glyph ID's...)

    - If there is a TrueType font (an `sfnt' resource), read it into memory,
      wrap it into a memory stream, load the TrueType driver and delegate
      the rest of the work to it, by calling FT_Open_Face().

    - Some suitcase fonts (notably Onyx) might point the `LWFN' file to
      itself, even though it doesn't contains `POST' resources.  To handle
      this special case without opening the file an extra time, we just
      ignore errors from the `LWFN' and fallback to the `sfnt' if both are
      available.
  */


#include <freetype/freetype.h>
#include <freetype/tttags.h>
#include <freetype/internal/ftstream.h>
#include "ftbase.h"

#if defined( __GNUC__ ) || defined( __IBMC__ )
  /* This is for Mac OS X.  Without redefinition, OS_INLINE */
  /* expands to `static inline' which doesn't survive the   */
  /* -ansi compilation flag of GCC.                         */
#if !HAVE_ANSI_OS_INLINE
#undef  OS_INLINE
#define OS_INLINE   static __inline__
#endif
#include <CoreServices/CoreServices.h>
#include <ApplicationServices/ApplicationServices.h>
#include <sys/syslimits.h> /* PATH_MAX */
#else
#include <Resources.h>
#include <Fonts.h>
#include <Endian.h>
#include <Errors.h>
#include <Files.h>
#include <TextUtils.h>
#endif

#ifndef PATH_MAX
#define PATH_MAX 1024 /* same with Mac OS X's syslimits.h */
#endif

#if defined( __MWERKS__ ) && !TARGET_RT_MAC_MACHO
#include <FSp_fopen.h>
#endif

#define FT_DEPRECATED_ATTRIBUTE

#include <freetype/ftmac.h>

  /* undefine blocking-macros in ftmac.h */
#undef FT_GetFile_From_Mac_Name
#undef FT_GetFile_From_Mac_ATS_Name
#undef FT_New_Face_From_FOND
#undef FT_New_Face_From_FSSpec
#undef FT_New_Face_From_FSRef


  /* FSSpec functions are deprecated since Mac OS X 10.4 */
#ifndef HAVE_FSSPEC
#if TARGET_API_MAC_OS8 || TARGET_API_MAC_CARBON
#define HAVE_FSSPEC  1
#else
#define HAVE_FSSPEC  0
#endif
#endif

  /* most FSRef functions were introduced since Mac OS 9 */
#ifndef HAVE_FSREF
#if TARGET_API_MAC_OSX
#define HAVE_FSREF  1
#else
#define HAVE_FSREF  0
#endif
#endif

  /* QuickDraw is deprecated since Mac OS X 10.4 */
#ifndef HAVE_QUICKDRAW_CARBON
#if TARGET_API_MAC_OS8 || TARGET_API_MAC_CARBON
#define HAVE_QUICKDRAW_CARBON  1
#else
#define HAVE_QUICKDRAW_CARBON  0
#endif
#endif

  /* AppleTypeService is available since Mac OS X */
#ifndef HAVE_ATS
#if TARGET_API_MAC_OSX
#define HAVE_ATS  1
#ifndef kATSOptionFlagsUnRestrictedScope /* since Mac OS X 10.1 */
#define kATSOptionFlagsUnRestrictedScope kATSOptionFlagsDefault
#endif
#else
#define HAVE_ATS  0
#endif
#endif

  /* `configure' checks the availability of `ResourceIndex' strictly */
  /* and sets HAVE_TYPE_RESOURCE_INDEX to 1 or 0 always.  If it is   */
  /* not set (e.g., a build without `configure'), the availability   */
  /* is guessed from the SDK version.                                */
#ifndef HAVE_TYPE_RESOURCE_INDEX
#if !defined( MAC_OS_X_VERSION_10_5 ) || \
    ( MAC_OS_X_VERSION_MAX_ALLOWED < MAC_OS_X_VERSION_10_5 )
#define HAVE_TYPE_RESOURCE_INDEX 0
#else
#define HAVE_TYPE_RESOURCE_INDEX 1
#endif
#endif /* !HAVE_TYPE_RESOURCE_INDEX */

#if ( HAVE_TYPE_RESOURCE_INDEX == 0 )
typedef short ResourceIndex;
#endif

  /* Set PREFER_LWFN to 1 if LWFN (Type 1) is preferred over
     TrueType in case *both* are available (this is not common,
     but it *is* possible). */
#ifndef PREFER_LWFN
#define PREFER_LWFN  1
#endif

#ifdef FT_MACINTOSH

#if !HAVE_QUICKDRAW_CARBON  /* QuickDraw is deprecated since Mac OS X 10.4 */

  FT_EXPORT_DEF( FT_Error )
  FT_GetFile_From_Mac_Name( const char*  fontName,
                            FSSpec*      pathSpec,
                            FT_Long*     face_index )
  {
    FT_UNUSED( fontName );
    FT_UNUSED( pathSpec );
    FT_UNUSED( face_index );

    return FT_THROW( Unimplemented_Feature );
  }

#else

  FT_EXPORT_DEF( FT_Error )
  FT_GetFile_From_Mac_Name( const char*  fontName,
                            FSSpec*      pathSpec,
                            FT_Long*     face_index )
  {
    OptionBits            options = kFMUseGlobalScopeOption;

    FMFontFamilyIterator  famIter;
    OSStatus              status = FMCreateFontFamilyIterator( NULL, NULL,
                                                               options,
                                                               &famIter );
    FMFont                the_font = 0;
    FMFontFamily          family   = 0;


    if ( !fontName || !face_index )
      return FT_THROW( Invalid_Argument );

    *face_index = 0;
    while ( status == 0 && !the_font )
    {
      status = FMGetNextFontFamily( &famIter, &family );
      if ( status == 0 )
      {
        int                           stat2;
        FMFontFamilyInstanceIterator  instIter;
        Str255                        famNameStr;
        char                          famName[256];


        /* get the family name */
        FMGetFontFamilyName( family, famNameStr );
        CopyPascalStringToC( famNameStr, famName );

        /* iterate through the styles */
        FMCreateFontFamilyInstanceIterator( family, &instIter );

        *face_index = 0;
        stat2       = 0;

        while ( stat2 == 0 && !the_font )
        {
          FMFontStyle  style;
          FMFontSize   size;
          FMFont       font;


          stat2 = FMGetNextFontFamilyInstance( &instIter, &font,
                                               &style, &size );
          if ( stat2 == 0 && size == 0 )
          {
            char  fullName[256];


            /* build up a complete face name */
            ft_strcpy( fullName, famName );
            if ( style & bold )
              ft_strcat( fullName, " Bold" );
            if ( style & italic )
              ft_strcat( fullName, " Italic" );

            /* compare with the name we are looking for */
            if ( ft_strcmp( fullName, fontName ) == 0 )
            {
              /* found it! */
              the_font = font;
            }
            else
              ++(*face_index);
          }
        }

        FMDisposeFontFamilyInstanceIterator( &instIter );
      }
    }

    FMDisposeFontFamilyIterator( &famIter );

    if ( the_font )
    {
      FMGetFontContainer( the_font, pathSpec );
      return FT_Err_Ok;
    }
    else
      return FT_THROW( Unknown_File_Format );
  }

#endif /* HAVE_QUICKDRAW_CARBON */


#if HAVE_ATS

  /* Private function.                                         */
  /* The FSSpec type has been discouraged for a long time,     */
  /* unfortunately an FSRef replacement API for                */
  /* ATSFontGetFileSpecification() is only available in        */
  /* Mac OS X 10.5 and later.                                  */
  static OSStatus
  FT_ATSFontGetFileReference( ATSFontRef  ats_font_id,
                              FSRef*      ats_font_ref )
  {
    OSStatus  err;

#if !defined( MAC_OS_X_VERSION_10_5 ) || \
    MAC_OS_X_VERSION_MIN_REQUIRED < MAC_OS_X_VERSION_10_5
    FSSpec    spec;


    err = ATSFontGetFileSpecification( ats_font_id, &spec );
    if ( noErr == err )
      err = FSpMakeFSRef( &spec, ats_font_ref );
#else
    err = ATSFontGetFileReference( ats_font_id, ats_font_ref );
#endif

    return err;
  }


  static FT_Error
  FT_GetFileRef_From_Mac_ATS_Name( const char*  fontName,
                                   FSRef*       ats_font_ref,
                                   FT_Long*     face_index )
  {
    CFStringRef  cf_fontName;
    ATSFontRef   ats_font_id;


    *face_index = 0;

    cf_fontName = CFStringCreateWithCString( NULL, fontName,
                                             kCFStringEncodingMacRoman );
    ats_font_id = ATSFontFindFromName( cf_fontName,
                                       kATSOptionFlagsUnRestrictedScope );
    CFRelease( cf_fontName );

    if ( ats_font_id == 0 || ats_font_id == 0xFFFFFFFFUL )
      return FT_THROW( Unknown_File_Format );

    if ( noErr != FT_ATSFontGetFileReference( ats_font_id, ats_font_ref ) )
      return FT_THROW( Unknown_File_Format );

    /* face_index calculation by searching preceding fontIDs */
    /* with same FSRef                                       */
    {
      ATSFontRef  id2 = ats_font_id - 1;
      FSRef       ref2;


      while ( id2 > 0 )
      {
        if ( noErr != FT_ATSFontGetFileReference( id2, &ref2 ) )
          break;
        if ( noErr != FSCompareFSRefs( ats_font_ref, &ref2 ) )
          break;

        id2--;
      }
      *face_index = ats_font_id - ( id2 + 1 );
    }

    return FT_Err_Ok;
  }

#endif

#if !HAVE_ATS

  FT_EXPORT_DEF( FT_Error )
  FT_GetFilePath_From_Mac_ATS_Name( const char*  fontName,
                                    UInt8*       path,
                                    UInt32       maxPathSize,
                                    FT_Long*     face_index )
  {
    FT_UNUSED( fontName );
    FT_UNUSED( path );
    FT_UNUSED( maxPathSize );
    FT_UNUSED( face_index );

    return FT_THROW( Unimplemented_Feature );
  }

#else

  FT_EXPORT_DEF( FT_Error )
  FT_GetFilePath_From_Mac_ATS_Name( const char*  fontName,
                                    UInt8*       path,
                                    UInt32       maxPathSize,
                                    FT_Long*     face_index )
  {
    FSRef     ref;
    FT_Error  err;


    err = FT_GetFileRef_From_Mac_ATS_Name( fontName, &ref, face_index );
    if ( err )
      return err;

    if ( noErr != FSRefMakePath( &ref, path, maxPathSize ) )
      return FT_THROW( Unknown_File_Format );

    return FT_Err_Ok;
  }

#endif /* HAVE_ATS */


#if !HAVE_FSSPEC || !HAVE_ATS

  FT_EXPORT_DEF( FT_Error )
  FT_GetFile_From_Mac_ATS_Name( const char*  fontName,
                                FSSpec*      pathSpec,
                                FT_Long*     face_index )
  {
    FT_UNUSED( fontName );
    FT_UNUSED( pathSpec );
    FT_UNUSED( face_index );

    return FT_THROW( Unimplemented_Feature );
  }

#else

  /* This function is deprecated because FSSpec is deprecated in Mac OS X. */
  FT_EXPORT_DEF( FT_Error )
  FT_GetFile_From_Mac_ATS_Name( const char*  fontName,
                                FSSpec*      pathSpec,
                                FT_Long*     face_index )
  {
    FSRef     ref;
    FT_Error  err;


    err = FT_GetFileRef_From_Mac_ATS_Name( fontName, &ref, face_index );
    if ( err )
      return err;

    if ( noErr != FSGetCatalogInfo( &ref, kFSCatInfoNone, NULL, NULL,
                                    pathSpec, NULL ) )
      return FT_THROW( Unknown_File_Format );

    return FT_Err_Ok;
  }

#endif


#if defined( __MWERKS__ ) && !TARGET_RT_MAC_MACHO

#define STREAM_FILE( stream )  ( (FT_FILE*)stream->descriptor.pointer )


  FT_CALLBACK_DEF( void )
  ft_FSp_stream_close( FT_Stream  stream )
  {
    ft_fclose( STREAM_FILE( stream ) );

    stream->descriptor.pointer = NULL;
    stream->size               = 0;
    stream->base               = NULL;
  }


  FT_CALLBACK_DEF( unsigned long )
  ft_FSp_stream_io( FT_Stream       stream,
                    unsigned long   offset,
                    unsigned char*  buffer,
                    unsigned long   count )
  {
    FT_FILE*  file;


    file = STREAM_FILE( stream );

    ft_fseek( file, offset, SEEK_SET );

    return (unsigned long)ft_fread( buffer, 1, count, file );
  }

#endif  /* __MWERKS__ && !TARGET_RT_MAC_MACHO */


#if HAVE_FSSPEC && !HAVE_FSREF

  /* isDirectory is a dummy to synchronize API with FSPathMakeRef() */
  static OSErr
  FT_FSPathMakeSpec( const UInt8*  pathname,
                     FSSpec*       spec_p,
                     Boolean       isDirectory )
  {
    const char  *p, *q;
    short       vRefNum;
    long        dirID;
    Str255      nodeName;
    OSErr       err;
    FT_UNUSED( isDirectory );


    p = q = (const char *)pathname;
    dirID   = 0;
    vRefNum = 0;

    while ( 1 )
    {
      int  len = ft_strlen( p );


      if ( len > 255 )
        len = 255;

      q = p + len;

      if ( q == p )
        return 0;

      if ( 255 < ft_strlen( (char *)pathname ) )
      {
        while ( p < q && *q != ':' )
          q--;
      }

      if ( p < q )
        *(char *)nodeName = q - p;
      else if ( ft_strlen( p ) < 256 )
        *(char *)nodeName = ft_strlen( p );
      else
        return errFSNameTooLong;

      ft_strncpy( (char *)nodeName + 1, (char *)p, *(char *)nodeName );
      err = FSMakeFSSpec( vRefNum, dirID, nodeName, spec_p );
      if ( err || '\0' == *q )
        return err;

      vRefNum = spec_p->vRefNum;
      dirID   = spec_p->parID;

      p = q;
    }
  }


  static OSErr
  FT_FSpMakePath( const FSSpec*  spec_p,
                  UInt8*         path,
                  UInt32         maxPathSize )
  {
    OSErr   err;
    FSSpec  spec = *spec_p;
    short   vRefNum;
    long    dirID;
    Str255  parDir_name;


    FT_MEM_SET( path, 0, maxPathSize );
    while ( 1 )
    {
      int             child_namelen = ft_strlen( (char *)path );
      unsigned char   node_namelen  = spec.name[0];
      unsigned char*  node_name     = spec.name + 1;


      if ( node_namelen + child_namelen > maxPathSize )
        return errFSNameTooLong;

      FT_MEM_MOVE( path + node_namelen + 1, path, child_namelen );
      FT_MEM_COPY( path, node_name, node_namelen );
      if ( child_namelen > 0 )
        path[node_namelen] = ':';

      vRefNum        = spec.vRefNum;
      dirID          = spec.parID;
      parDir_name[0] = '\0';
      err = FSMakeFSSpec( vRefNum, dirID, parDir_name, &spec );
      if ( noErr != err || dirID == spec.parID )
        break;
    }
    return noErr;
  }

#endif /* HAVE_FSSPEC && !HAVE_FSREF */


  static OSErr
  FT_FSPathMakeRes( const UInt8*    pathname,
                    ResFileRefNum*  res )
  {

#if HAVE_FSREF

    OSErr  err;
    FSRef  ref;


    if ( noErr != FSPathMakeRef( pathname, &ref, FALSE ) )
      return FT_THROW( Cannot_Open_Resource );

    /* at present, no support for dfont format */
    err = FSOpenResourceFile( &ref, 0, NULL, fsRdPerm, res );
    if ( noErr == err )
      return err;

    /* fallback to original resource-fork font */
    *res = FSOpenResFile( &ref, fsRdPerm );
    err  = ResError();

#else

    OSErr   err;
    FSSpec  spec;


    if ( noErr != FT_FSPathMakeSpec( pathname, &spec, FALSE ) )
      return FT_THROW( Cannot_Open_Resource );

    /* at present, no support for dfont format without FSRef */
    /* (see above), try original resource-fork font          */
    *res = FSpOpenResFile( &spec, fsRdPerm );
    err  = ResError();

#endif /* HAVE_FSREF */

    return err;
  }


  /* Return the file type for given pathname */
  static OSType
  get_file_type_from_path( const UInt8*  pathname )
  {

#if HAVE_FSREF

    FSRef          ref;
    FSCatalogInfo  info;


    if ( noErr != FSPathMakeRef( pathname, &ref, FALSE ) )
      return ( OSType ) 0;

    if ( noErr != FSGetCatalogInfo( &ref, kFSCatInfoFinderInfo, &info,
                                    NULL, NULL, NULL ) )
      return ( OSType ) 0;

    return ((FInfo *)(info.finderInfo))->fdType;

#else

    FSSpec  spec;
    FInfo   finfo;


    if ( noErr != FT_FSPathMakeSpec( pathname, &spec, FALSE ) )
      return ( OSType ) 0;

    if ( noErr != FSpGetFInfo( &spec, &finfo ) )
      return ( OSType ) 0;

    return finfo.fdType;

#endif /* HAVE_FSREF */

  }


  /* Given a PostScript font name, create the Macintosh LWFN file name. */
  static void
  create_lwfn_name( char*   ps_name,
                    Str255  lwfn_file_name )
  {
    int       max = 5, count = 0;
    FT_Byte*  p = lwfn_file_name;
    FT_Byte*  q = (FT_Byte*)ps_name;


    lwfn_file_name[0] = 0;

    while ( *q )
    {
      if ( ft_isupper( *q ) )
      {
        if ( count )
          max = 3;
        count = 0;
      }
      if ( count < max && ( ft_isalnum( *q ) || *q == '_' ) )
      {
        *++p = *q;
        lwfn_file_name[0]++;
        count++;
      }
      q++;
    }
  }


  static short
  count_faces_sfnt( char*  fond_data )
  {
    /* The count is 1 greater than the value in the FOND.  */
    /* Isn't that cute? :-)                                */

    return EndianS16_BtoN( *( (short*)( fond_data +
                                        sizeof ( FamRec ) ) ) ) + 1;
  }


  static short
  count_faces_scalable( char*  fond_data )
  {
    AsscEntry*  assoc;
    short       i, face, face_all;


    face_all = EndianS16_BtoN( *( (short *)( fond_data +
                                             sizeof ( FamRec ) ) ) ) + 1;
    assoc    = (AsscEntry*)( fond_data + sizeof ( FamRec ) + 2 );
    face     = 0;

    for ( i = 0; i < face_all; i++ )
    {
      if ( 0 == EndianS16_BtoN( assoc[i].fontSize ) )
        face++;
    }
    return face;
  }


  /* Look inside the FOND data, answer whether there should be an SFNT
     resource, and answer the name of a possible LWFN Type 1 file.

     Thanks to Paul Miller (paulm@profoundeffects.com) for the fix
     to load a face OTHER than the first one in the FOND!
  */

  static void
  parse_fond( char*   fond_data,
              short*  have_sfnt,
              ResID*  sfnt_id,
              Str255  lwfn_file_name,
              short   face_index )
  {
    AsscEntry*  assoc;
    AsscEntry*  base_assoc;
    FamRec*     fond;


    *sfnt_id          = 0;
    *have_sfnt        = 0;
    lwfn_file_name[0] = 0;

    fond       = (FamRec*)fond_data;
    assoc      = (AsscEntry*)( fond_data + sizeof ( FamRec ) + 2 );
    base_assoc = assoc;

    /* the maximum faces in a FOND is 48, size of StyleTable.indexes[] */
    if ( 47 < face_index )
      return;

    /* Let's do a little range checking before we get too excited here */
    if ( face_index < count_faces_sfnt( fond_data ) )
    {
      assoc += face_index;        /* add on the face_index! */

      /* if the face at this index is not scalable,
         fall back to the first one (old behavior) */
      if ( EndianS16_BtoN( assoc->fontSize ) == 0 )
      {
        *have_sfnt = 1;
        *sfnt_id   = EndianS16_BtoN( assoc->fontID );
      }
      else if ( base_assoc->fontSize == 0 )
      {
        *have_sfnt = 1;
        *sfnt_id   = EndianS16_BtoN( base_assoc->fontID );
      }
    }

    if ( EndianS32_BtoN( fond->ffStylOff ) )
    {
      unsigned char*  p = (unsigned char*)fond_data;
      StyleTable*     style;
      unsigned short  string_count;
      char            ps_name[256];
      unsigned char*  names[64];
      int             i;


      p += EndianS32_BtoN( fond->ffStylOff );
      style = (StyleTable*)p;
      p += sizeof ( StyleTable );
      string_count = EndianS16_BtoN( *(short*)(p) );
      string_count = FT_MIN( 64, string_count );
      p += sizeof ( short );

      for ( i = 0; i < string_count; i++ )
      {
        names[i] = p;
        p       += names[i][0];
        p++;
      }

      {
        size_t  ps_name_len = (size_t)names[0][0];


        if ( ps_name_len != 0 )
        {
          ft_memcpy(ps_name, names[0] + 1, ps_name_len);
          ps_name[ps_name_len] = 0;
        }
        if ( style->indexes[face_index] > 1 &&
             style->indexes[face_index] <= string_count )
        {
          unsigned char*  suffixes = names[style->indexes[face_index] - 1];


          for ( i = 1; i <= suffixes[0]; i++ )
          {
            unsigned char*  s;
            size_t          j = suffixes[i] - 1;


            if ( j < string_count && ( s = names[j] ) != NULL )
            {
              size_t  s_len = (size_t)s[0];


              if ( s_len != 0 && ps_name_len + s_len < sizeof ( ps_name ) )
              {
                ft_memcpy( ps_name + ps_name_len, s + 1, s_len );
                ps_name_len += s_len;
                ps_name[ps_name_len] = 0;
              }
            }
          }
        }
      }

      create_lwfn_name( ps_name, lwfn_file_name );
    }
  }


  static  FT_Error
  lookup_lwfn_by_fond( const UInt8*      path_fond,
                       ConstStr255Param  base_lwfn,
                       UInt8*            path_lwfn,
                       int               path_size )
  {

#if HAVE_FSREF

    FSRef  ref, par_ref;
    int    dirname_len;


    /* Pathname for FSRef can be in various formats: HFS, HFS+, and POSIX. */
    /* We should not extract parent directory by string manipulation.      */

    if ( noErr != FSPathMakeRef( path_fond, &ref, FALSE ) )
      return FT_THROW( Invalid_Argument );

    if ( noErr != FSGetCatalogInfo( &ref, kFSCatInfoNone,
                                    NULL, NULL, NULL, &par_ref ) )
      return FT_THROW( Invalid_Argument );

    if ( noErr != FSRefMakePath( &par_ref, path_lwfn, path_size ) )
      return FT_THROW( Invalid_Argument );

    if ( ft_strlen( (char *)path_lwfn ) + 1 + base_lwfn[0] > path_size )
      return FT_THROW( Invalid_Argument );

    /* now we have absolute dirname in path_lwfn */
    if ( path_lwfn[0] == '/' )
      ft_strcat( (char *)path_lwfn, "/" );
    else
      ft_strcat( (char *)path_lwfn, ":" );

    dirname_len = ft_strlen( (char *)path_lwfn );
    ft_strcat( (char *)path_lwfn, (char *)base_lwfn + 1 );
    path_lwfn[dirname_len + base_lwfn[0]] = '\0';

    if ( noErr != FSPathMakeRef( path_lwfn, &ref, FALSE ) )
      return FT_THROW( Cannot_Open_Resource );

    if ( noErr != FSGetCatalogInfo( &ref, kFSCatInfoNone,
                                    NULL, NULL, NULL, NULL ) )
      return FT_THROW( Cannot_Open_Resource );

    return FT_Err_Ok;

#else

    int     i;
    FSSpec  spec;


    /* pathname for FSSpec is always HFS format */
    if ( ft_strlen( (char *)path_fond ) > path_size )
      return FT_THROW( Invalid_Argument );

    ft_strcpy( (char *)path_lwfn, (char *)path_fond );

    i = ft_strlen( (char *)path_lwfn ) - 1;
    while ( i > 0 && ':' != path_lwfn[i] )
      i--;

    if ( i + 1 + base_lwfn[0] > path_size )
      return FT_THROW( Invalid_Argument );

    if ( ':' == path_lwfn[i] )
    {
      ft_strcpy( (char *)path_lwfn + i + 1, (char *)base_lwfn + 1 );
      path_lwfn[i + 1 + base_lwfn[0]] = '\0';
    }
    else
    {
      ft_strcpy( (char *)path_lwfn, (char *)base_lwfn + 1 );
      path_lwfn[base_lwfn[0]] = '\0';
    }

    if ( noErr != FT_FSPathMakeSpec( path_lwfn, &spec, FALSE ) )
      return FT_THROW( Cannot_Open_Resource );

    return FT_Err_Ok;

#endif /* HAVE_FSREF */

  }


  static short
  count_faces( Handle        fond,
               const UInt8*  pathname )
  {
    ResID     sfnt_id;
    short     have_sfnt, have_lwfn;
    Str255    lwfn_file_name;
    UInt8     buff[PATH_MAX];
    FT_Error  err;
    short     num_faces;


    have_sfnt = have_lwfn = 0;

    HLock( fond );
    parse_fond( *fond, &have_sfnt, &sfnt_id, lwfn_file_name, 0 );

    if ( lwfn_file_name[0] )
    {
      err = lookup_lwfn_by_fond( pathname, lwfn_file_name,
                                 buff, sizeof ( buff ) );
      if ( !err )
        have_lwfn = 1;
    }

    if ( have_lwfn && ( !have_sfnt || PREFER_LWFN ) )
      num_faces = 1;
    else
      num_faces = count_faces_scalable( *fond );

    HUnlock( fond );
    return num_faces;
  }


  /* Read Type 1 data from the POST resources inside the LWFN file,
     return a PFB buffer.  This is somewhat convoluted because the FT2
     PFB parser wants the ASCII header as one chunk, and the LWFN
     chunks are often not organized that way, so we glue chunks
     of the same type together. */
  static FT_Error
  read_lwfn( FT_Memory      memory,
             ResFileRefNum  res,
             FT_Byte**      pfb_data,
             FT_ULong*      size )
  {
    FT_Error       error = FT_Err_Ok;
    ResID          res_id;
    unsigned char  *buffer, *p, *size_p = NULL;
    FT_ULong       total_size = 0;
    FT_ULong       old_total_size = 0;
    FT_ULong       post_size, pfb_chunk_size;
    Handle         post_data;
    char           code, last_code;


    UseResFile( res );

    /* First pass: load all POST resources, and determine the size of */
    /* the output buffer.                                             */
    res_id    = 501;
    last_code = -1;

    for (;;)
    {
      post_data = Get1Resource( TTAG_POST, res_id++ );
      if ( post_data == NULL )
        break;  /* we are done */

      code = (*post_data)[0];

      if ( code != last_code )
      {
        if ( code == 5 )
          total_size += 2; /* just the end code */
        else
          total_size += 6; /* code + 4 bytes chunk length */
      }

      total_size += GetHandleSize( post_data ) - 2;
      last_code = code;

      /* detect integer overflows */
      if ( total_size < old_total_size )
      {
        error = FT_ERR( Array_Too_Large );
        goto Error;
      }

      old_total_size = total_size;
    }

    if ( FT_QALLOC( buffer, (FT_Long)total_size ) )
      goto Error;

    /* Second pass: append all POST data to the buffer, add PFB fields. */
    /* Glue all consecutive chunks of the same type together.           */
    p              = buffer;
    res_id         = 501;
    last_code      = -1;
    pfb_chunk_size = 0;

    for (;;)
    {
      post_data = Get1Resource( TTAG_POST, res_id++ );
      if ( post_data == NULL )
        break;  /* we are done */

      post_size = (FT_ULong)GetHandleSize( post_data ) - 2;
      code = (*post_data)[0];

      if ( code != last_code )
      {
        if ( last_code != -1 )
        {
          /* we are done adding a chunk, fill in the size field */
          if ( size_p != NULL )
          {
            *size_p++ = (FT_Byte)(   pfb_chunk_size         & 0xFF );
            *size_p++ = (FT_Byte)( ( pfb_chunk_size >> 8  ) & 0xFF );
            *size_p++ = (FT_Byte)( ( pfb_chunk_size >> 16 ) & 0xFF );
            *size_p++ = (FT_Byte)( ( pfb_chunk_size >> 24 ) & 0xFF );
          }
          pfb_chunk_size = 0;
        }

        *p++ = 0x80;
        if ( code == 5 )
          *p++ = 0x03;  /* the end */
        else if ( code == 2 )
          *p++ = 0x02;  /* binary segment */
        else
          *p++ = 0x01;  /* ASCII segment */

        if ( code != 5 )
        {
          size_p = p;   /* save for later */
          p += 4;       /* make space for size field */
        }
      }

      ft_memcpy( p, *post_data + 2, post_size );
      pfb_chunk_size += post_size;
      p += post_size;
      last_code = code;
    }

    *pfb_data = buffer;
    *size = total_size;

  Error:
    CloseResFile( res );
    return error;
  }


  /* Create a new FT_Face from a file spec to an LWFN file. */
  static FT_Error
  FT_New_Face_From_LWFN( FT_Library    library,
                         const UInt8*  pathname,
                         FT_Long       face_index,
                         FT_Face*      aface )
  {
    FT_Byte*       pfb_data;
    FT_ULong       pfb_size;
    FT_Error       error;
    ResFileRefNum  res;


    if ( noErr != FT_FSPathMakeRes( pathname, &res ) )
      return FT_THROW( Cannot_Open_Resource );

    pfb_data = NULL;
    pfb_size = 0;
    error = read_lwfn( library->memory, res, &pfb_data, &pfb_size );
    CloseResFile( res ); /* PFB is already loaded, useless anymore */
    if ( error )
      return error;

    return open_face_from_buffer( library,
                                  pfb_data,
                                  pfb_size,
                                  face_index,
                                  "type1",
                                  aface );
  }


  /* Create a new FT_Face from an SFNT resource, specified by res ID. */
  static FT_Error
  FT_New_Face_From_SFNT( FT_Library  library,
                         ResID       sfnt_id,
                         FT_Long     face_index,
                         FT_Face*    aface )
  {
    Handle     sfnt = NULL;
    FT_Byte*   sfnt_data;
    size_t     sfnt_size;
    FT_Error   error  = FT_Err_Ok;
    FT_Memory  memory = library->memory;
    int        is_cff, is_sfnt_ps;


    sfnt = GetResource( TTAG_sfnt, sfnt_id );
    if ( sfnt == NULL )
      return FT_THROW( Invalid_Handle );

    sfnt_size = (FT_ULong)GetHandleSize( sfnt );
    if ( FT_QALLOC( sfnt_data, (FT_Long)sfnt_size ) )
    {
      ReleaseResource( sfnt );
      return error;
    }

    HLock( sfnt );
    ft_memcpy( sfnt_data, *sfnt, sfnt_size );
    HUnlock( sfnt );
    ReleaseResource( sfnt );

    is_cff     = sfnt_size > 4 && !ft_memcmp( sfnt_data, "OTTO", 4 );
    is_sfnt_ps = sfnt_size > 4 && !ft_memcmp( sfnt_data, "typ1", 4 );

    if ( is_sfnt_ps )
    {
      FT_Stream  stream;


      if ( FT_NEW( stream ) )
        goto Try_OpenType;

      FT_Stream_OpenMemory( stream, sfnt_data, sfnt_size );
      if ( !open_face_PS_from_sfnt_stream( library,
                                           stream,
                                           face_index,
                                           0, NULL,
                                           aface ) )
      {
        FT_Stream_Close( stream );
        FT_FREE( stream );
        FT_FREE( sfnt_data );
        goto Exit;
      }

      FT_FREE( stream );
    }
  Try_OpenType:
    error = open_face_from_buffer( library,
                                   sfnt_data,
                                   sfnt_size,
                                   face_index,
                                   is_cff ? "cff" : "truetype",
                                   aface );
  Exit:
    return error;
  }


  /* Create a new FT_Face from a file spec to a suitcase file. */
  static FT_Error
  FT_New_Face_From_Suitcase( FT_Library    library,
                             const UInt8*  pathname,
                             FT_Long       face_index,
                             FT_Face*      aface )
  {
    FT_Error       error = FT_ERR( Cannot_Open_Resource );
    ResFileRefNum  res_ref;
    ResourceIndex  res_index;
    Handle         fond;
    short          num_faces_in_res;


    if ( noErr != FT_FSPathMakeRes( pathname, &res_ref ) )
      return FT_THROW( Cannot_Open_Resource );

    UseResFile( res_ref );
    if ( ResError() )
      return FT_THROW( Cannot_Open_Resource );

    num_faces_in_res = 0;
    for ( res_index = 1; ; ++res_index )
    {
      short  num_faces_in_fond;


      fond = Get1IndResource( TTAG_FOND, res_index );
      if ( ResError() )
        break;

      num_faces_in_fond  = count_faces( fond, pathname );
      num_faces_in_res  += num_faces_in_fond;

      if ( 0 <= face_index && face_index < num_faces_in_fond && error )
        error = FT_New_Face_From_FOND( library, fond, face_index, aface );

      face_index -= num_faces_in_fond;
    }

    CloseResFile( res_ref );
    if ( !error && aface )
      (*aface)->num_faces = num_faces_in_res;
    return error;
  }


  /* documentation is in ftmac.h */

  FT_EXPORT_DEF( FT_Error )
  FT_New_Face_From_FOND( FT_Library  library,
                         Handle      fond,
                         FT_Long     face_index,
                         FT_Face*    aface )
  {
    short     have_sfnt, have_lwfn = 0;
    ResID     sfnt_id, fond_id;
    OSType    fond_type;
    Str255    fond_name;
    Str255    lwfn_file_name;
    UInt8     path_lwfn[PATH_MAX];
    OSErr     err;
    FT_Error  error = FT_Err_Ok;


    /* test for valid `aface' and `library' delayed to */
    /* `FT_New_Face_From_XXX'                          */

    GetResInfo( fond, &fond_id, &fond_type, fond_name );
    if ( ResError() != noErr || fond_type != TTAG_FOND )
      return FT_THROW( Invalid_File_Format );

    HLock( fond );
    parse_fond( *fond, &have_sfnt, &sfnt_id, lwfn_file_name, face_index );
    HUnlock( fond );

    if ( lwfn_file_name[0] )
    {
      ResFileRefNum  res;


      res = HomeResFile( fond );
      if ( noErr != ResError() )
        goto found_no_lwfn_file;

#if HAVE_FSREF

      {
        UInt8  path_fond[PATH_MAX];
        FSRef  ref;


        err = FSGetForkCBInfo( res, kFSInvalidVolumeRefNum,
                               NULL, NULL, NULL, &ref, NULL );
        if ( noErr != err )
          goto found_no_lwfn_file;

        err = FSRefMakePath( &ref, path_fond, sizeof ( path_fond ) );
        if ( noErr != err )
          goto found_no_lwfn_file;

        error = lookup_lwfn_by_fond( path_fond, lwfn_file_name,
                                     path_lwfn, sizeof ( path_lwfn ) );
        if ( !error )
          have_lwfn = 1;
      }

#elif HAVE_FSSPEC

      {
        UInt8     path_fond[PATH_MAX];
        FCBPBRec  pb;
        Str255    fond_file_name;
        FSSpec    spec;


        FT_MEM_SET( &spec, 0, sizeof ( FSSpec ) );
        FT_MEM_SET( &pb,   0, sizeof ( FCBPBRec ) );

        pb.ioNamePtr = fond_file_name;
        pb.ioVRefNum = 0;
        pb.ioRefNum  = res;
        pb.ioFCBIndx = 0;

        err = PBGetFCBInfoSync( &pb );
        if ( noErr != err )
          goto found_no_lwfn_file;

        err = FSMakeFSSpec( pb.ioFCBVRefNum, pb.ioFCBParID,
                            fond_file_name, &spec );
        if ( noErr != err )
          goto found_no_lwfn_file;

        err = FT_FSpMakePath( &spec, path_fond, sizeof ( path_fond ) );
        if ( noErr != err )
          goto found_no_lwfn_file;

        error = lookup_lwfn_by_fond( path_fond, lwfn_file_name,
                                     path_lwfn, sizeof ( path_lwfn ) );
        if ( !error )
          have_lwfn = 1;
      }

#endif /* HAVE_FSREF, HAVE_FSSPEC */

    }

    if ( have_lwfn && ( !have_sfnt || PREFER_LWFN ) )
      error = FT_New_Face_From_LWFN( library,
                                     path_lwfn,
                                     face_index,
                                     aface );
    else
      error = FT_ERR( Unknown_File_Format );

  found_no_lwfn_file:
    if ( have_sfnt && error )
      error = FT_New_Face_From_SFNT( library,
                                     sfnt_id,
                                     face_index,
                                     aface );

    return error;
  }


  /* Common function to load a new FT_Face from a resource file. */
  static FT_Error
  FT_New_Face_From_Resource( FT_Library    library,
                             const UInt8*  pathname,
                             FT_Long       face_index,
                             FT_Face*      aface )
  {
    OSType    file_type;
    FT_Error  error;


    /* LWFN is a (very) specific file format, check for it explicitly */
    file_type = get_file_type_from_path( pathname );
    if ( file_type == TTAG_LWFN )
      return FT_New_Face_From_LWFN( library, pathname, face_index, aface );

    /* Otherwise the file type doesn't matter (there are more than  */
    /* `FFIL' and `tfil').  Just try opening it as a font suitcase; */
    /* if it works, fine.                                           */

    error = FT_New_Face_From_Suitcase( library, pathname, face_index, aface );
    if ( !error )
      return error;

    /* let it fall through to normal loader (.ttf, .otf, etc.); */
    /* we signal this by returning no error and no FT_Face      */
    *aface = NULL;
    return 0;
  }


  /*************************************************************************/
  /*                                                                       */
  /* <Function>                                                            */
  /*    FT_New_Face                                                        */
  /*                                                                       */
  /* <Description>                                                         */
  /*    This is the Mac-specific implementation of FT_New_Face.  In        */
  /*    addition to the standard FT_New_Face() functionality, it also      */
  /*    accepts pathnames to Mac suitcase files.  For further              */
  /*    documentation see the original FT_New_Face() in freetype.h.        */
  /*                                                                       */
  FT_EXPORT_DEF( FT_Error )
  FT_New_Face( FT_Library   library,
               const char*  pathname,
               FT_Long      face_index,
               FT_Face*     aface )
  {
    FT_Open_Args  args;
    FT_Error      error;


    /* test for valid `library' and `aface' delayed to FT_Open_Face() */
    if ( !pathname )
      return FT_THROW( Invalid_Argument );

    *aface = NULL;

    /* try resourcefork based font: LWFN, FFIL */
    error = FT_New_Face_From_Resource( library, (UInt8 *)pathname,
                                       face_index, aface );
    if ( error || *aface )
      return error;

    /* let it fall through to normal loader (.ttf, .otf, etc.) */
    args.flags    = FT_OPEN_PATHNAME;
    args.pathname = (char*)pathname;
    return FT_Open_Face( library, &args, face_index, aface );
  }


  /*************************************************************************/
  /*                                                                       */
  /* <Function>                                                            */
  /*    FT_New_Face_From_FSRef                                             */
  /*                                                                       */
  /* <Description>                                                         */
  /*    FT_New_Face_From_FSRef is identical to FT_New_Face except it       */
  /*    accepts an FSRef instead of a path.                                */
  /*                                                                       */
  /* This function is deprecated because Carbon data types (FSRef)         */
  /* are not cross-platform, and thus not suitable for the FreeType API.   */
  FT_EXPORT_DEF( FT_Error )
  FT_New_Face_From_FSRef( FT_Library    library,
                          const FSRef*  ref,
                          FT_Long       face_index,
                          FT_Face*      aface )
  {

#if !HAVE_FSREF

    FT_UNUSED( library );
    FT_UNUSED( ref );
    FT_UNUSED( face_index );
    FT_UNUSED( aface );

    return FT_THROW( Unimplemented_Feature );

#else

    FT_Error      error;
    FT_Open_Args  args;
    OSErr   err;
    UInt8   pathname[PATH_MAX];


    /* test for valid `library' and `aface' delayed to `FT_Open_Face' */

    if ( !ref )
      return FT_THROW( Invalid_Argument );

    err = FSRefMakePath( ref, pathname, sizeof ( pathname ) );
    if ( err )
      error = FT_ERR( Cannot_Open_Resource );

    error = FT_New_Face_From_Resource( library, pathname, face_index, aface );
    if ( error || *aface )
      return error;

    /* fallback to datafork font */
    args.flags    = FT_OPEN_PATHNAME;
    args.pathname = (char*)pathname;
    return FT_Open_Face( library, &args, face_index, aface );

#endif /* HAVE_FSREF */

  }


  /*************************************************************************/
  /*                                                                       */
  /* <Function>                                                            */
  /*    FT_New_Face_From_FSSpec                                            */
  /*                                                                       */
  /* <Description>                                                         */
  /*    FT_New_Face_From_FSSpec is identical to FT_New_Face except it      */
  /*    accepts an FSSpec instead of a path.                               */
  /*                                                                       */
  /* This function is deprecated because Carbon data types (FSSpec)        */
  /* are not cross-platform, and thus not suitable for the FreeType API.   */
  FT_EXPORT_DEF( FT_Error )
  FT_New_Face_From_FSSpec( FT_Library     library,
                           const FSSpec*  spec,
                           FT_Long        face_index,
                           FT_Face*       aface )
  {

#if HAVE_FSREF

    FSRef  ref;


    if ( !spec || FSpMakeFSRef( spec, &ref ) != noErr )
      return FT_THROW( Invalid_Argument );
    else
      return FT_New_Face_From_FSRef( library, &ref, face_index, aface );

#elif HAVE_FSSPEC

    FT_Error      error;
    FT_Open_Args  args;
    OSErr         err;
    UInt8         pathname[PATH_MAX];


    if ( !spec )
      return FT_THROW( Invalid_Argument );

    err = FT_FSpMakePath( spec, pathname, sizeof ( pathname ) );
    if ( err )
      error = FT_ERR( Cannot_Open_Resource );

    error = FT_New_Face_From_Resource( library, pathname, face_index, aface );
    if ( error || *aface )
      return error;

    /* fallback to datafork font */
    args.flags    = FT_OPEN_PATHNAME;
    args.pathname = (char*)pathname;
    return FT_Open_Face( library, &args, face_index, aface );

#else

    FT_UNUSED( library );
    FT_UNUSED( spec );
    FT_UNUSED( face_index );
    FT_UNUSED( aface );

    return FT_THROW( Unimplemented_Feature );

#endif /* HAVE_FSREF, HAVE_FSSPEC */

  }

#endif /* FT_MACINTOSH */


/* END */
