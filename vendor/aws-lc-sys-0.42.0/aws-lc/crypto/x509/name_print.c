// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/x509.h>

#include <inttypes.h>
#include <string.h>

#include <openssl/asn1.h>
#include <openssl/bio.h>
#include <openssl/obj.h>


static int maybe_write(BIO *out, const void *buf, int len) {
  // If |out| is NULL, ignore the output but report the length.
  return out == NULL || BIO_write(out, buf, len) == len;
}

// do_indent prints |indent| spaces to |out|.
static int do_indent(BIO *out, int indent) {
  for (int i = 0; i < indent; i++) {
    if (!maybe_write(out, " ", 1)) {
      return 0;
    }
  }
  return 1;
}

#define FN_WIDTH_LN 25
#define FN_WIDTH_SN 10

static int do_name_ex(BIO *out, const X509_NAME *n, int indent,
                      unsigned long flags) {
  int i, prev = -1, orflags, cnt;
  int fn_opt, fn_nid;
  char objtmp[80];
  const char *objbuf;
  int outlen, len;
  const char *sep_dn, *sep_mv, *sep_eq;
  int sep_dn_len, sep_mv_len, sep_eq_len;
  if (indent < 0) {
    indent = 0;
  }
  outlen = indent;
  if (!do_indent(out, indent)) {
    return -1;
  }
  switch (flags & XN_FLAG_SEP_MASK) {
    case XN_FLAG_SEP_MULTILINE:
      sep_dn = "\n";
      sep_dn_len = 1;
      sep_mv = " + ";
      sep_mv_len = 3;
      break;

    case XN_FLAG_SEP_COMMA_PLUS:
      sep_dn = ",";
      sep_dn_len = 1;
      sep_mv = "+";
      sep_mv_len = 1;
      indent = 0;
      break;

    case XN_FLAG_SEP_CPLUS_SPC:
      sep_dn = ", ";
      sep_dn_len = 2;
      sep_mv = " + ";
      sep_mv_len = 3;
      indent = 0;
      break;

    case XN_FLAG_SEP_SPLUS_SPC:
      sep_dn = "; ";
      sep_dn_len = 2;
      sep_mv = " + ";
      sep_mv_len = 3;
      indent = 0;
      break;

    default:
      return -1;
  }

  if (flags & XN_FLAG_SPC_EQ) {
    sep_eq = " = ";
    sep_eq_len = 3;
  } else {
    sep_eq = "=";
    sep_eq_len = 1;
  }

  fn_opt = flags & XN_FLAG_FN_MASK;

  cnt = X509_NAME_entry_count(n);
  for (i = 0; i < cnt; i++) {
    const X509_NAME_ENTRY *ent;
    if (flags & XN_FLAG_DN_REV) {
      ent = X509_NAME_get_entry(n, cnt - i - 1);
    } else {
      ent = X509_NAME_get_entry(n, i);
    }
    if (prev != -1) {
      if (prev == X509_NAME_ENTRY_set(ent)) {
        if (!maybe_write(out, sep_mv, sep_mv_len)) {
          return -1;
        }
        outlen += sep_mv_len;
      } else {
        if (!maybe_write(out, sep_dn, sep_dn_len)) {
          return -1;
        }
        outlen += sep_dn_len;
        if (!do_indent(out, indent)) {
          return -1;
        }
        outlen += indent;
      }
    }
    prev = X509_NAME_ENTRY_set(ent);
    const ASN1_OBJECT *fn = X509_NAME_ENTRY_get_object(ent);
    const ASN1_STRING *val = X509_NAME_ENTRY_get_data(ent);
    fn_nid = OBJ_obj2nid(fn);
    if (fn_opt != XN_FLAG_FN_NONE) {
      int objlen, fld_len;
      if ((fn_opt == XN_FLAG_FN_OID) || (fn_nid == NID_undef)) {
        OBJ_obj2txt(objtmp, sizeof objtmp, fn, 1);
        fld_len = 0;  // XXX: what should this be?
        objbuf = objtmp;
      } else {
        if (fn_opt == XN_FLAG_FN_SN) {
          fld_len = FN_WIDTH_SN;
          objbuf = OBJ_nid2sn(fn_nid);
        } else if (fn_opt == XN_FLAG_FN_LN) {
          fld_len = FN_WIDTH_LN;
          objbuf = OBJ_nid2ln(fn_nid);
        } else {
          fld_len = 0;  // XXX: what should this be?
          objbuf = "";
        }
      }
      objlen = strlen(objbuf);
      if (!maybe_write(out, objbuf, objlen)) {
        return -1;
      }
      if ((objlen < fld_len) && (flags & XN_FLAG_FN_ALIGN)) {
        if (!do_indent(out, fld_len - objlen)) {
          return -1;
        }
        outlen += fld_len - objlen;
      }
      if (!maybe_write(out, sep_eq, sep_eq_len)) {
        return -1;
      }
      outlen += objlen + sep_eq_len;
    }
    // If the field name is unknown then fix up the DER dump flag. We
    // might want to limit this further so it will DER dump on anything
    // other than a few 'standard' fields.
    if ((fn_nid == NID_undef) && (flags & XN_FLAG_DUMP_UNKNOWN_FIELDS)) {
      orflags = ASN1_STRFLGS_DUMP_ALL;
    } else {
      orflags = 0;
    }

    len = ASN1_STRING_print_ex(out, val, flags | orflags);
    if (len < 0) {
      return -1;
    }
    outlen += len;
  }
  return outlen;
}

int X509_NAME_print_ex(BIO *out, const X509_NAME *nm, int indent,
                       unsigned long flags) {
  if (flags == XN_FLAG_COMPAT) {
    return X509_NAME_print(out, nm, indent);
  }
  return do_name_ex(out, nm, indent, flags);
}

int X509_NAME_print_ex_fp(FILE *fp, const X509_NAME *nm, int indent,
                          unsigned long flags) {
  BIO *bio = NULL;
  if (fp != NULL) {
    // If |fp| is NULL, this function returns the number of bytes without
    // writing.
    bio = BIO_new_fp(fp, BIO_NOCLOSE);
    if (bio == NULL) {
      return -1;
    }
  }
  int ret = X509_NAME_print_ex(bio, nm, indent, flags);
  BIO_free(bio);
  return ret;
}
