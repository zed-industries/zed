/*
 * Copyright (C) 2002-2014 Free Software Foundation, Inc.
 *
 * This file is part of LIBTASN1.
 *
 * LIBTASN1 is free software; you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * LIBTASN1 is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with LIBTASN1; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA
 *
 */

/**
 * libtasn1:Short_Description:
 *
 * GNU ASN.1 library
 */
/**
 * libtasn1:Long_Description:
 *
 * The Libtasn1 library provides Abstract Syntax Notation One (ASN.1, as
 * specified by the X.680 ITU-T recommendation) parsing and structures
 * management, and Distinguished Encoding Rules (DER, as per X.690)
 * encoding and decoding functions.
 */


#ifndef LIBTASN1_H
#define LIBTASN1_H

#ifndef ASN1_API
#if defined ASN1_BUILDING && defined HAVE_VISIBILITY && HAVE_VISIBILITY
#define ASN1_API __attribute__((__visibility__("default")))
#elif defined ASN1_BUILDING && defined _MSC_VER && ! defined ASN1_STATIC
#define ASN1_API __declspec(dllexport)
#elif defined _MSC_VER && ! defined ASN1_STATIC
#define ASN1_API __declspec(dllimport)
#else
#define ASN1_API
#endif
#endif

#ifdef __GNUC__
# define __LIBTASN1_CONST__  __attribute__((const))
# define __LIBTASN1_PURE__  __attribute__((pure))
#else
# define __LIBTASN1_CONST__
# define __LIBTASN1_PURE__
#endif

#include <sys/types.h>
#include <time.h>
#include <stdio.h>		/* for FILE* */

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * ASN1_VERSION:
 *
 * Version of the library as a string.
 */
#define ASN1_VERSION "4.16.0"

/**
 * ASN1_VERSION_MAJOR:
 *
 * Major version number of the library.
 */
#define ASN1_VERSION_MAJOR 4

/**
 * ASN1_VERSION_MINOR:
 *
 * Minor version number of the library.
 */
#define ASN1_VERSION_MINOR 16

/**
 * ASN1_VERSION_PATCH:
 *
 * Patch version number of the library.
 */
#define ASN1_VERSION_PATCH 0

/**
 * ASN1_VERSION_NUMBER:
 *
 * Version number of the library as a number.
 */
#define ASN1_VERSION_NUMBER 0x041000


#if defined __GNUC__ && !defined ASN1_INTERNAL_BUILD
# define _ASN1_GCC_VERSION (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__)
# if _ASN1_GCC_VERSION >= 30100
#  define _ASN1_GCC_ATTR_DEPRECATED __attribute__ ((__deprecated__))
# endif
#endif

#ifndef _ASN1_GCC_ATTR_DEPRECATED
#define _ASN1_GCC_ATTR_DEPRECATED
#endif

/*****************************************/
/* Errors returned by libtasn1 functions */
/*****************************************/
#define ASN1_SUCCESS			0
#define ASN1_FILE_NOT_FOUND		1
#define ASN1_ELEMENT_NOT_FOUND		2
#define ASN1_IDENTIFIER_NOT_FOUND	3
#define ASN1_DER_ERROR			4
#define ASN1_VALUE_NOT_FOUND		5
#define ASN1_GENERIC_ERROR		6
#define ASN1_VALUE_NOT_VALID		7
#define ASN1_TAG_ERROR			8
#define ASN1_TAG_IMPLICIT		9
#define ASN1_ERROR_TYPE_ANY		10
#define ASN1_SYNTAX_ERROR		11
#define ASN1_MEM_ERROR			12
#define ASN1_MEM_ALLOC_ERROR		13
#define ASN1_DER_OVERFLOW		14
#define ASN1_NAME_TOO_LONG		15
#define ASN1_ARRAY_ERROR		16
#define ASN1_ELEMENT_NOT_EMPTY		17
#define ASN1_TIME_ENCODING_ERROR	18
#define ASN1_RECURSION			19

/*************************************/
/* Constants used in asn1_visit_tree */
/*************************************/
#define ASN1_PRINT_NAME			1
#define ASN1_PRINT_NAME_TYPE		2
#define ASN1_PRINT_NAME_TYPE_VALUE	3
#define ASN1_PRINT_ALL			4

/*****************************************/
/* Constants returned by asn1_read_tag   */
/*****************************************/
#define ASN1_CLASS_UNIVERSAL		0x00	/* old: 1 */
#define ASN1_CLASS_APPLICATION		0x40	/* old: 2 */
#define ASN1_CLASS_CONTEXT_SPECIFIC	0x80	/* old: 3 */
#define ASN1_CLASS_PRIVATE		0xC0	/* old: 4 */
#define ASN1_CLASS_STRUCTURED		0x20

/*****************************************/
/* Constants returned by asn1_read_tag   */
/*****************************************/
#define ASN1_TAG_BOOLEAN		0x01
#define ASN1_TAG_INTEGER		0x02
#define ASN1_TAG_SEQUENCE		0x10
#define ASN1_TAG_SET			0x11
#define ASN1_TAG_OCTET_STRING		0x04
#define ASN1_TAG_BIT_STRING		0x03
#define ASN1_TAG_UTCTime		0x17
#define ASN1_TAG_GENERALIZEDTime	0x18
#define ASN1_TAG_OBJECT_ID		0x06
#define ASN1_TAG_ENUMERATED		0x0A
#define ASN1_TAG_NULL			0x05
#define ASN1_TAG_GENERALSTRING		0x1B
#define ASN1_TAG_NUMERIC_STRING		0x12
#define ASN1_TAG_IA5_STRING		0x16
#define ASN1_TAG_TELETEX_STRING		0x14
#define ASN1_TAG_PRINTABLE_STRING	0x13
#define ASN1_TAG_UNIVERSAL_STRING	0x1C
#define ASN1_TAG_BMP_STRING		0x1E
#define ASN1_TAG_UTF8_STRING		0x0C
#define ASN1_TAG_VISIBLE_STRING		0x1A

/**
 * asn1_node:
 *
 * Structure definition used for the node of the tree
 * that represents an ASN.1 DEFINITION.
 */
typedef struct asn1_node_st asn1_node_st;

typedef asn1_node_st *asn1_node;
typedef const asn1_node_st *asn1_node_const;

/**
 * ASN1_MAX_NAME_SIZE:
 *
 * Maximum number of characters of a name
 * inside a file with ASN1 definitions.
 */
#define ASN1_MAX_NAME_SIZE 64


/**
 * asn1_static_node:
 * @name: Node name
 * @type: Node typ
 * @value: Node value
 *
 * For the on-disk format of ASN.1 trees, created by asn1_parser2array().
 */
struct asn1_static_node_st
{
  const char *name;		/* Node name */
  unsigned int type;		/* Node type */
  const void *value;		/* Node value */
};
typedef struct asn1_static_node_st asn1_static_node;

/* List of constants for field type of node_asn  */
#define ASN1_ETYPE_INVALID        0
#define ASN1_ETYPE_CONSTANT       1
#define ASN1_ETYPE_IDENTIFIER     2
#define ASN1_ETYPE_INTEGER        3
#define ASN1_ETYPE_BOOLEAN        4
#define ASN1_ETYPE_SEQUENCE       5
#define ASN1_ETYPE_BIT_STRING     6
#define ASN1_ETYPE_OCTET_STRING   7
#define ASN1_ETYPE_TAG            8
#define ASN1_ETYPE_DEFAULT        9
#define ASN1_ETYPE_SIZE          10
#define ASN1_ETYPE_SEQUENCE_OF   11
#define ASN1_ETYPE_OBJECT_ID     12
#define ASN1_ETYPE_ANY           13
#define ASN1_ETYPE_SET           14
#define ASN1_ETYPE_SET_OF        15
#define ASN1_ETYPE_DEFINITIONS   16
#define ASN1_ETYPE_CHOICE        18
#define ASN1_ETYPE_IMPORTS       19
#define ASN1_ETYPE_NULL          20
#define ASN1_ETYPE_ENUMERATED    21
#define ASN1_ETYPE_GENERALSTRING 27
#define ASN1_ETYPE_NUMERIC_STRING 28
#define ASN1_ETYPE_IA5_STRING     29
#define ASN1_ETYPE_TELETEX_STRING 30
#define ASN1_ETYPE_PRINTABLE_STRING 31
#define ASN1_ETYPE_UNIVERSAL_STRING 32
#define ASN1_ETYPE_BMP_STRING     33
#define ASN1_ETYPE_UTF8_STRING    34
#define ASN1_ETYPE_VISIBLE_STRING 35
#define ASN1_ETYPE_UTC_TIME       36
#define ASN1_ETYPE_GENERALIZED_TIME 37

/**
 * ASN1_DELETE_FLAG_ZEROIZE:
 *
 * Used by: asn1_delete_structure2()
 *
 * Zeroize values prior to deinitialization.
 */
#define ASN1_DELETE_FLAG_ZEROIZE 1

/**
 * ASN1_DECODE_FLAG_ALLOW_PADDING:
 *
 * Used by: asn1_der_decoding2()
 *
 * This flag would allow arbitrary data past the DER data.
 */
#define ASN1_DECODE_FLAG_ALLOW_PADDING 1
/**
 * ASN1_DECODE_FLAG_STRICT_DER:
 *
 * Used by: asn1_der_decoding2()
 *
 * This flag would ensure that no BER decoding takes place.
 */
#define ASN1_DECODE_FLAG_STRICT_DER (1<<1)
/**
 * ASN1_DECODE_FLAG_ALLOW_INCORRECT_TIME:
 *
 * Used by: asn1_der_decoding2()
 *
 * This flag will tolerate Time encoding errors when in strict DER.
 */
#define ASN1_DECODE_FLAG_ALLOW_INCORRECT_TIME (1<<2)


/**
 * asn1_data_node_st:
 * @name: Node name
 * @value: Node value
 * @value_len: Node value size
 * @type: Node value type (ASN1_ETYPE_*)
 *
 * Data node inside a #asn1_node structure.
 */
struct asn1_data_node_st
{
  const char *name;		/* Node name */
  const void *value;		/* Node value */
  unsigned int value_len;	/* Node value size */
  unsigned int type;		/* Node value type (ASN1_ETYPE_*) */
};
typedef struct asn1_data_node_st asn1_data_node_st;

/***********************************/
/*  Fixed constants                */
/***********************************/

/**
 * ASN1_MAX_ERROR_DESCRIPTION_SIZE:
 *
 * Maximum number of characters
 * of a description message
 * (null character included).
 */
#define ASN1_MAX_ERROR_DESCRIPTION_SIZE 128

/***********************************/
/*  Functions definitions          */
/***********************************/

extern ASN1_API int
  asn1_parser2tree (const char *file,
		      asn1_node * definitions, char *error_desc);

extern ASN1_API int
  asn1_parser2array (const char *inputFileName,
		       const char *outputFileName,
		       const char *vectorName, char *error_desc);

extern ASN1_API int
  asn1_array2tree (const asn1_static_node * array,
		     asn1_node * definitions, char *errorDescription);

extern ASN1_API void
  asn1_print_structure (FILE * out, asn1_node_const structure,
			  const char *name, int mode);

extern ASN1_API int
  asn1_create_element (asn1_node_const definitions,
			 const char *source_name, asn1_node * element);

extern ASN1_API int asn1_delete_structure (asn1_node * structure);

extern ASN1_API int asn1_delete_structure2 (asn1_node * structure, unsigned int flags);

extern ASN1_API int
  asn1_delete_element (asn1_node structure, const char *element_name);

extern ASN1_API int
  asn1_write_value (asn1_node node_root, const char *name,
		      const void *ivalue, int len);

extern ASN1_API int
  asn1_read_value (asn1_node_const root, const char *name,
		     void *ivalue, int *len);

extern ASN1_API int
  asn1_read_value_type (asn1_node_const root, const char *name,
			  void *ivalue, int *len, unsigned int *etype);

extern ASN1_API int
  asn1_read_node_value (asn1_node_const node, asn1_data_node_st * data);

extern ASN1_API int
  asn1_number_of_elements (asn1_node_const element, const char *name, int *num);

extern ASN1_API int
  asn1_der_coding (asn1_node_const element, const char *name,
		     void *ider, int *len, char *ErrorDescription);

extern ASN1_API int
  asn1_der_decoding2 (asn1_node *element, const void *ider,
			int *max_ider_len, unsigned int flags,
			char *errorDescription);

extern ASN1_API int
  asn1_der_decoding (asn1_node * element, const void *ider,
		       int ider_len, char *errorDescription);

/* Do not use. Use asn1_der_decoding() instead. */
extern ASN1_API int
  asn1_der_decoding_element (asn1_node * structure,
			       const char *elementName,
			       const void *ider, int len,
			       char *errorDescription) _ASN1_GCC_ATTR_DEPRECATED;

extern ASN1_API int
  asn1_der_decoding_startEnd (asn1_node element,
				const void *ider, int ider_len,
				const char *name_element,
				int *start, int *end);

extern ASN1_API int
  asn1_expand_any_defined_by (asn1_node_const definitions, asn1_node * element);

extern ASN1_API int
  asn1_expand_octet_string (asn1_node_const definitions,
			      asn1_node * element,
			      const char *octetName, const char *objectName);

extern ASN1_API int
  asn1_read_tag (asn1_node_const root, const char *name,
		   int *tagValue, int *classValue);

extern ASN1_API const char *asn1_find_structure_from_oid (asn1_node_const
							    definitions,
							    const char
							    *oidValue);

__LIBTASN1_PURE__
extern ASN1_API const char *asn1_check_version (const char *req_version);

__LIBTASN1_PURE__
extern ASN1_API const char *asn1_strerror (int error);

extern ASN1_API void asn1_perror (int error);

#define ASN1_MAX_TAG_SIZE 4
#define ASN1_MAX_LENGTH_SIZE 9
#define ASN1_MAX_TL_SIZE (ASN1_MAX_TAG_SIZE+ASN1_MAX_LENGTH_SIZE)
extern ASN1_API long
  asn1_get_length_der (const unsigned char *der, int der_len, int *len);

extern ASN1_API long
  asn1_get_length_ber (const unsigned char *ber, int ber_len, int *len);

extern ASN1_API void
  asn1_length_der (unsigned long int len, unsigned char *der, int *der_len);

/* Other utility functions. */

extern ASN1_API
  int asn1_decode_simple_der (unsigned int etype, const unsigned char *der,
				unsigned int _der_len,
				const unsigned char **str,
				unsigned int *str_len);

extern ASN1_API
  int asn1_decode_simple_ber (unsigned int etype, const unsigned char *der,
				unsigned int _der_len,
				unsigned char **str,
				unsigned int *str_len,
				unsigned int *ber_len);

extern ASN1_API int
  asn1_encode_simple_der (unsigned int etype, const unsigned char *str,
			    unsigned int str_len, unsigned char *tl,
			    unsigned int *tl_len);

extern ASN1_API asn1_node
  asn1_find_node (asn1_node_const pointer, const char *name);

extern ASN1_API int
  asn1_copy_node (asn1_node dst, const char *dst_name,
		    asn1_node_const src, const char *src_name);
extern ASN1_API asn1_node
  asn1_dup_node (asn1_node_const src, const char *src_name);

/* Internal and low-level DER utility functions. */

extern ASN1_API int
  asn1_get_tag_der (const unsigned char *der, int der_len,
		      unsigned char *cls, int *len, unsigned long *tag);

extern ASN1_API void
  asn1_octet_der (const unsigned char *str, int str_len,
		    unsigned char *der, int *der_len);

extern ASN1_API int
  asn1_get_octet_der (const unsigned char *der, int der_len,
			int *ret_len, unsigned char *str,
			int str_size, int *str_len);

extern ASN1_API void asn1_bit_der (const unsigned char *str, int bit_len,
				     unsigned char *der, int *der_len);

extern ASN1_API int
  asn1_get_bit_der (const unsigned char *der, int der_len,
		      int *ret_len, unsigned char *str,
		      int str_size, int *bit_len);

extern ASN1_API int
  asn1_get_object_id_der (const unsigned char *der,
                          int der_len, int *ret_len,
                          char *str, int str_size);

extern ASN1_API int
  asn1_object_id_der (const char *str, unsigned char *der, int *der_len,
                      unsigned flags);

/* Compatibility types */

/**
 * asn1_retCode:
 *
 * Type formerly returned by libtasn1 functions.
 *
 * Deprecated: 3.0: Use int instead.
 */
typedef int asn1_retCode;

/**
 * node_asn_struct:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use #asn1_node instead.
 */
#define node_asn_struct asn1_node_st

/**
 * node_asn:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use #asn1_node instead.
 */
#define node_asn asn1_node_st

/**
 * ASN1_TYPE:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use #asn1_node instead.
 */
#define ASN1_TYPE asn1_node

/**
 * ASN1_TYPE_EMPTY:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use NULL instead.
 */
#define ASN1_TYPE_EMPTY NULL

/**
 * static_struct_asn:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use #asn1_static_node instead.
 */
#define static_struct_asn asn1_static_node_st

/**
 * ASN1_ARRAY_TYPE:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use #asn1_static_node instead.
 */
#define ASN1_ARRAY_TYPE asn1_static_node

/**
 * asn1_static_node_t:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use #asn1_static_node instead.
 */
#define asn1_static_node_t asn1_static_node

/**
 * node_data_struct:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use #asn1_data_node_st instead.
 */
#define node_data_struct asn1_data_node_st

/**
 * ASN1_DATA_NODE:
 *
 * Compat #define.
 *
 * Deprecated: 3.0: Use #asn1_data_node_st instead.
 */
#define ASN1_DATA_NODE asn1_data_node_st

#ifdef __cplusplus
}
#endif

#endif				/* LIBTASN1_H */
