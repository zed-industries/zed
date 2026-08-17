/*
 * libthai - Thai Language Support Library
 * Copyright (C) 2001  Theppitak Karoonboonyanan <theppitak@gmail.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

/*
 * thstr.h - Thai string manipulators
 * Created: 2001-08-03
 */

#ifndef THAI_THSTR_H
#define THAI_THSTR_H

#include <thai/thailib.h>

BEGIN_CDECL

/**
 * @file   thstr.h
 * @brief  Thai string manipulators
 */

extern size_t th_normalize(thchar_t dest[], const thchar_t *src, size_t n);

END_CDECL

#endif  /* THAI_THSTR_H */

