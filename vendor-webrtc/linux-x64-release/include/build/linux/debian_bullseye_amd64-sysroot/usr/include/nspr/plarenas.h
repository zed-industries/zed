/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** PLArena-related declarations used to be split between plarenas.h and
** plarena.h. That split wasn't useful, so now all the declarations are in
** plarena.h. However, this file still exists so that any old code that
** includes it will still work.
**/
#include "plarena.h"
