/* graphene-version.h: Version info
 *
 * SPDX-License-Identifier: MIT
 *
 * Copyright 2014  Emmanuele Bassi
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

/**
 * SECTION:graphene-version
 * @title: Versioning information
 * @short_description: Detemining the version of Graphene in use
 *
 * Graphene provides symbols to know the version of the library at compile
 * time.
 */

#pragma once

#if !defined(GRAPHENE_H_INSIDE) && !defined(GRAPHENE_COMPILATION)
#error "Only graphene.h can be included directly."
#endif

/**
 * GRAPHENE_MAJOR_VERSION:
 *
 * Evaluates to the major version number of the library version,
 * e.g. 1 in 1.2.3.
 *
 * Since: 1.0
 */
#define GRAPHENE_MAJOR_VERSION          (1)

/**
 * GRAPHENE_MINOR_VERSION:
 *
 * Evaluates to the minor version number of the library version,
 * e.g. 2 in 1.2.3.
 *
 * Since: 1.0
 */
#define GRAPHENE_MINOR_VERSION          (10)

/**
 * GRAPHENE_MICRO_VERSION:
 *
 * Evaluates to the micro version number of the library version,
 * e.g. 3 in 1.2.3.
 *
 * Since: 1.0
 */
#define GRAPHENE_MICRO_VERSION          (4)
