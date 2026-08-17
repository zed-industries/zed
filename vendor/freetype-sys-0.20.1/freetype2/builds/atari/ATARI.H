#if defined( GXVALID_H_ )
#pragma warn -aus /* too many unevaluated variables in gxvalid */
#endif

#ifndef ATARI_H
#define ATARI_H

#pragma warn -stu

/* PureC doesn't like 32bit enumerations */

#ifndef FT_IMAGE_TAG
#define FT_IMAGE_TAG( value, _x1, _x2, _x3, _x4 ) value
#endif /* FT_IMAGE_TAG */

#ifndef FT_ENC_TAG
#define FT_ENC_TAG( value, a, b, c, d ) value
#endif /* FT_ENC_TAG */

#endif /* ATARI_H */
