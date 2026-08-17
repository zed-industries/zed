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
 * thwinp.h - Thai wide-char string input sequence filtering
 * Created: 2001-05-17
 */

#ifndef THAI_THWCINP_H
#define THAI_THWCINP_H

#include <thai/thailib.h>
#include <thai/thinp.h>
#include <thai/thwchar.h>

BEGIN_CDECL

extern int th_wcisaccept(thwchar_t c1, thwchar_t c2, thstrict_t s);
extern int th_wcvalidate(thwchar_t *c1, thwchar_t *c2, thstrict_t s);

END_CDECL

#endif  /* THAI_THWCINP_H */

