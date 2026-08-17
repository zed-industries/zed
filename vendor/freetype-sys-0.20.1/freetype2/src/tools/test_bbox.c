#include <freetype/freetype.h>
#include <freetype/ftbbox.h>


#include <time.h>    /* for clock() */

/* SunOS 4.1.* does not define CLOCKS_PER_SEC, so include <sys/param.h> */
/* to get the HZ macro which is the equivalent.                         */
#if defined(__sun__) && !defined(SVR4) && !defined(__SVR4)
#include <sys/param.h>
#define CLOCKS_PER_SEC HZ
#endif

  static long
  get_time( void )
  {
    return clock() * 10000L / CLOCKS_PER_SEC;
  }




  /* test bbox computations */

#define  XSCALE    65536
#define  XX(x)     ((FT_Pos)(x*XSCALE))
#define  XVEC(x,y)  { XX(x), XX(y) }
#define  XVAL(x)   ((x)/(1.0*XSCALE))

  /* dummy outline #1 */
  static FT_Vector  dummy_vec_1[4] =
  {
#if 1
    XVEC( 408.9111, 535.3164 ),
    XVEC( 455.8887, 634.396  ),
    XVEC( -37.8765, 786.2207 ),
    XVEC( 164.6074, 535.3164 )
#else
    { (FT_Int32)0x0198E93DL , (FT_Int32)0x021750FFL },  /* 408.9111, 535.3164 */
    { (FT_Int32)0x01C7E312L , (FT_Int32)0x027A6560L },  /* 455.8887, 634.3960 */
    { (FT_Int32)0xFFDA1F9EL , (FT_Int32)0x0312387FL },  /* -37.8765, 786.2207 */
    { (FT_Int32)0x00A49B7EL , (FT_Int32)0x021750FFL }   /* 164.6074, 535.3164 */
#endif
   };

  static char  dummy_tag_1[4] =
  {
    FT_CURVE_TAG_ON,
    FT_CURVE_TAG_CUBIC,
    FT_CURVE_TAG_CUBIC,
    FT_CURVE_TAG_ON
  };

  static short  dummy_contour_1[1] =
  {
    3
  };

  static FT_Outline  dummy_outline_1 =
  {
    1,
    4,
    dummy_vec_1,
    dummy_tag_1,
    dummy_contour_1,
    0
  };


  /* dummy outline #2 */
  static FT_Vector  dummy_vec_2[4] =
  {
    XVEC( 100.0, 100.0 ),
    XVEC( 100.0, 200.0 ),
    XVEC( 200.0, 200.0 ),
    XVEC( 200.0, 133.0 )
  };

  static FT_Outline  dummy_outline_2 =
  {
    1,
    4,
    dummy_vec_2,
    dummy_tag_1,
    dummy_contour_1,
    0
  };


  /* dummy outline #3 with bbox of [0 100 128 128] precisely */
  static FT_Vector  dummy_vec_3[4] =
  {
    XVEC( 100.0, 127.0 ),
    XVEC( 200.0, 127.0 ),
    XVEC(   0.0, 136.0 ),
    XVEC(   0.0, 100.0 )
  };

  static FT_Outline  dummy_outline_3 =
  {
    1,
    4,
    dummy_vec_3,
    dummy_tag_1,
    dummy_contour_1,
    0
  };


  static void
  dump_outline( FT_Outline*  outline )
  {
    FT_BBox  bbox;

    /* compute and display cbox */
    FT_Outline_Get_CBox( outline, &bbox );
    printf( "cbox = [%.2f %.2f %.2f %.2f]\n",
             XVAL( bbox.xMin ),
             XVAL( bbox.yMin ),
             XVAL( bbox.xMax ),
             XVAL( bbox.yMax ) );

    /* compute and display bbox */
    FT_Outline_Get_BBox( outline, &bbox );
    printf( "bbox = [%.2f %.2f %.2f %.2f]\n",
             XVAL( bbox.xMin ),
             XVAL( bbox.yMin ),
             XVAL( bbox.xMax ),
             XVAL( bbox.yMax ) );
  }



  static void
  profile_outline( FT_Outline*   outline,
                   long          repeat )
  {
    FT_BBox  bbox;
    long     count;
    long     time0;

    time0 = get_time();
    for ( count = repeat; count > 0; count-- )
      FT_Outline_Get_CBox( outline, &bbox );

    time0 = get_time() - time0;
    printf( "time = %6.3f cbox = [%8.4f %8.4f %8.4f %8.4f]\n",
             ((double)time0/10000.0),
             XVAL( bbox.xMin ),
             XVAL( bbox.yMin ),
             XVAL( bbox.xMax ),
             XVAL( bbox.yMax ) );
    printf( "cbox_hex = [%08X %08X %08X %08X]\n",
             bbox.xMin, bbox.yMin, bbox.xMax, bbox.yMax );


    time0 = get_time();
    for ( count = repeat; count > 0; count-- )
      FT_Outline_Get_BBox( outline, &bbox );

    time0 = get_time() - time0;
    printf( "time = %6.3f bbox = [%8.4f %8.4f %8.4f %8.4f]\n",
             ((double)time0/10000.0),
             XVAL( bbox.xMin ),
             XVAL( bbox.yMin ),
             XVAL( bbox.xMax ),
             XVAL( bbox.yMax ) );
    printf( "bbox_hex = [%08X %08X %08X %08X]\n",
             bbox.xMin, bbox.yMin, bbox.xMax, bbox.yMax );
  }

#define REPEAT  1000000L

  int  main( int  argc, char**  argv )
  {
    printf( "outline #1\n" );
    profile_outline( &dummy_outline_1, REPEAT );

    printf( "outline #2\n" );
    profile_outline( &dummy_outline_2, REPEAT );

    printf( "outline #3\n" );
    profile_outline( &dummy_outline_3, REPEAT );

    return 0;
  }

