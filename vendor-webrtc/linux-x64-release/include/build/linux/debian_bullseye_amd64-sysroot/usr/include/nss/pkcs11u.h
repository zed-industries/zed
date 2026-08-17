/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
 * Copyright (C) 1994-1999 RSA Security Inc. Licence to copy this document
 * is granted provided that it is identified as "RSA Security Inc. Public-Key
 * Cryptography Standards (PKCS)" in all material mentioning or referencing
 * this document.
 */
/*
 * reset any packing set by pkcs11p.h
 */

#if defined(_WIN32) || defined(_WINDOWS)
#ifdef __clang__
#pragma clang diagnostic ignored "-Wpragma-pack"
#endif
#ifdef _MSC_VER
#pragma warning(disable : 4103)
#endif
#pragma pack(pop, cryptoki)
#endif
