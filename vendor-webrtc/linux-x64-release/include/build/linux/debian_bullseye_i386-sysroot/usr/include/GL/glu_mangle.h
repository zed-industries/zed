/*
 * Mesa 3-D graphics library
 * Version:  3.0
 * Copyright (C) 1995-1998  Brian Paul
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the Free
 * Software Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 */


#ifndef GLU_MANGLE_H
#define GLU_MANGLE_H


#define gluLookAt mgluLookAt
#define gluOrtho2D mgluOrtho2D
#define gluPerspective mgluPerspective
#define gluPickMatrix mgluPickMatrix
#define gluProject mgluProject
#define gluUnProject mgluUnProject
#define gluErrorString mgluErrorString
#define gluScaleImage mgluScaleImage
#define gluBuild1DMipmaps mgluBuild1DMipmaps
#define gluBuild2DMipmaps mgluBuild2DMipmaps
#define gluNewQuadric mgluNewQuadric
#define gluDeleteQuadric mgluDeleteQuadric
#define gluQuadricDrawStyle mgluQuadricDrawStyle
#define gluQuadricOrientation mgluQuadricOrientation
#define gluQuadricNormals mgluQuadricNormals
#define gluQuadricTexture mgluQuadricTexture
#define gluQuadricCallback mgluQuadricCallback
#define gluCylinder mgluCylinder
#define gluSphere mgluSphere
#define gluDisk mgluDisk
#define gluPartialDisk mgluPartialDisk
#define gluNewNurbsRenderer mgluNewNurbsRenderer
#define gluDeleteNurbsRenderer mgluDeleteNurbsRenderer
#define gluLoadSamplingMatrices mgluLoadSamplingMatrices
#define gluNurbsProperty mgluNurbsProperty
#define gluGetNurbsProperty mgluGetNurbsProperty
#define gluBeginCurve mgluBeginCurve
#define gluEndCurve mgluEndCurve
#define gluNurbsCurve mgluNurbsCurve
#define gluBeginSurface mgluBeginSurface
#define gluEndSurface mgluEndSurface
#define gluNurbsSurface mgluNurbsSurface
#define gluBeginTrim mgluBeginTrim
#define gluEndTrim mgluEndTrim
#define gluPwlCurve mgluPwlCurve
#define gluNurbsCallback mgluNurbsCallback
#define gluNewTess mgluNewTess
#define gluDeleteTess mgluDeleteTess
#define gluTessBeginPolygon mgluTessBeginPolygon
#define gluTessBeginContour mgluTessBeginContour
#define gluTessVertex mgluTessVertex
#define gluTessEndPolygon mgluTessEndPolygon
#define gluTessEndContour mgluTessEndContour
#define gluTessProperty mgluTessProperty
#define gluTessNormal mgluTessNormal
#define gluTessCallback mgluTessCallback
#define gluGetTessProperty mgluGetTessProperty
#define gluBeginPolygon mgluBeginPolygon
#define gluNextContour mgluNextContour
#define gluEndPolygon mgluEndPolygon
#define gluGetString mgluGetString
#define gluBuild1DMipmapLevels mgluBuild1DMipmapLevels
#define gluBuild2DMipmapLevels mgluBuild2DMipmapLevels
#define gluBuild3DMipmapLevels mgluBuild3DMipmapLevels
#define gluBuild3DMipmaps mgluBuild3DMipmaps
#define gluCheckExtension mgluCheckExtension
#define gluUnProject4 mgluUnProject4
#define gluNurbsCallbackData mgluNurbsCallbackData
#define gluNurbsCallbackDataEXT mgluNurbsCallbackDataEXT

#endif
