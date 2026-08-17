/****************************************************************************
**
** Copyright (C) 2013 Klaralvdalens Datakonsult AB (KDAB).
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QOPENGLABSTRACTTEXTURE_H
#define QOPENGLABSTRACTTEXTURE_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#include <QtGui/qopengl.h>
#include <QtGui/qimage.h>
#include <QtCore/QScopedPointer>

QT_BEGIN_NAMESPACE

class QDebug;
class QOpenGLTexturePrivate;
class QOpenGLPixelTransferOptions;

class Q_GUI_EXPORT QOpenGLTexture
{
    Q_GADGET
public:
    enum Target {
        Target1D                   = 0x0DE0,    // GL_TEXTURE_1D
        Target1DArray              = 0x8C18,    // GL_TEXTURE_1D_ARRAY
        Target2D                   = 0x0DE1,    // GL_TEXTURE_2D
        Target2DArray              = 0x8C1A,    // GL_TEXTURE_2D_ARRAY
        Target3D                   = 0x806F,    // GL_TEXTURE_3D
        TargetCubeMap              = 0x8513,    // GL_TEXTURE_CUBE_MAP
        TargetCubeMapArray         = 0x9009,    // GL_TEXTURE_CUBE_MAP_ARRAY
        Target2DMultisample        = 0x9100,    // GL_TEXTURE_2D_MULTISAMPLE
        Target2DMultisampleArray   = 0x9102,    // GL_TEXTURE_2D_MULTISAMPLE_ARRAY
        TargetRectangle            = 0x84F5,    // GL_TEXTURE_RECTANGLE
        TargetBuffer               = 0x8C2A     // GL_TEXTURE_BUFFER
    };
    Q_ENUM(Target)

    enum BindingTarget {
        BindingTarget1D                    = 0x8068,   // GL_TEXTURE_BINDING_1D
        BindingTarget1DArray               = 0x8C1C,   // GL_TEXTURE_BINDING_1D_ARRAY
        BindingTarget2D                    = 0x8069,   // GL_TEXTURE_BINDING_2D
        BindingTarget2DArray               = 0x8C1D,   // GL_TEXTURE_BINDING_2D_ARRAY
        BindingTarget3D                    = 0x806A,   // GL_TEXTURE_BINDING_3D
        BindingTargetCubeMap               = 0x8514,   // GL_TEXTURE_BINDING_CUBE_MAP
        BindingTargetCubeMapArray          = 0x900A,   // GL_TEXTURE_BINDING_CUBE_MAP_ARRAY
        BindingTarget2DMultisample         = 0x9104,   // GL_TEXTURE_BINDING_2D_MULTISAMPLE
        BindingTarget2DMultisampleArray    = 0x9105,   // GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY
        BindingTargetRectangle             = 0x84F6,   // GL_TEXTURE_BINDING_RECTANGLE
        BindingTargetBuffer                = 0x8C2C    // GL_TEXTURE_BINDING_BUFFER
    };
    Q_ENUM(BindingTarget)

    enum MipMapGeneration {
        GenerateMipMaps,
        DontGenerateMipMaps
    };
    Q_ENUM(MipMapGeneration)

    enum TextureUnitReset {
        ResetTextureUnit,
        DontResetTextureUnit
    };
    Q_ENUM(TextureUnitReset)

    enum TextureFormat {
        NoFormat               = 0,         // GL_NONE

        // Unsigned normalized formats
        R8_UNorm               = 0x8229,    // GL_R8
        RG8_UNorm              = 0x822B,    // GL_RG8
        RGB8_UNorm             = 0x8051,    // GL_RGB8
        RGBA8_UNorm            = 0x8058,    // GL_RGBA8

        R16_UNorm              = 0x822A,    // GL_R16
        RG16_UNorm             = 0x822C,    // GL_RG16
        RGB16_UNorm            = 0x8054,    // GL_RGB16
        RGBA16_UNorm           = 0x805B,    // GL_RGBA16

        // Signed normalized formats
        R8_SNorm               = 0x8F94,    // GL_R8_SNORM
        RG8_SNorm              = 0x8F95,    // GL_RG8_SNORM
        RGB8_SNorm             = 0x8F96,    // GL_RGB8_SNORM
        RGBA8_SNorm            = 0x8F97,    // GL_RGBA8_SNORM

        R16_SNorm              = 0x8F98,    // GL_R16_SNORM
        RG16_SNorm             = 0x8F99,    // GL_RG16_SNORM
        RGB16_SNorm            = 0x8F9A,    // GL_RGB16_SNORM
        RGBA16_SNorm           = 0x8F9B,    // GL_RGBA16_SNORM

        // Unsigned integer formats
        R8U                    = 0x8232,    // GL_R8UI
        RG8U                   = 0x8238,    // GL_RG8UI
        RGB8U                  = 0x8D7D,    // GL_RGB8UI
        RGBA8U                 = 0x8D7C,    // GL_RGBA8UI

        R16U                   = 0x8234,    // GL_R16UI
        RG16U                  = 0x823A,    // GL_RG16UI
        RGB16U                 = 0x8D77,    // GL_RGB16UI
        RGBA16U                = 0x8D76,    // GL_RGBA16UI

        R32U                   = 0x8236,    // GL_R32UI
        RG32U                  = 0x823C,    // GL_RG32UI
        RGB32U                 = 0x8D71,    // GL_RGB32UI
        RGBA32U                = 0x8D70,    // GL_RGBA32UI

        // Signed integer formats
        R8I                    = 0x8231,    // GL_R8I
        RG8I                   = 0x8237,    // GL_RG8I
        RGB8I                  = 0x8D8F,    // GL_RGB8I
        RGBA8I                 = 0x8D8E,    // GL_RGBA8I

        R16I                   = 0x8233,    // GL_R16I
        RG16I                  = 0x8239,    // GL_RG16I
        RGB16I                 = 0x8D89,    // GL_RGB16I
        RGBA16I                = 0x8D88,    // GL_RGBA16I

        R32I                   = 0x8235,    // GL_R32I
        RG32I                  = 0x823B,    // GL_RG32I
        RGB32I                 = 0x8D83,    // GL_RGB32I
        RGBA32I                = 0x8D82,    // GL_RGBA32I

        // Floating point formats
        R16F                   = 0x822D,    // GL_R16F
        RG16F                  = 0x822F,    // GL_RG16F
        RGB16F                 = 0x881B,    // GL_RGB16F
        RGBA16F                = 0x881A,    // GL_RGBA16F

        R32F                   = 0x822E,    // GL_R32F
        RG32F                  = 0x8230,    // GL_RG32F
        RGB32F                 = 0x8815,    // GL_RGB32F
        RGBA32F                = 0x8814,    // GL_RGBA32F

        // Packed formats
        RGB9E5                 = 0x8C3D,    // GL_RGB9_E5
        RG11B10F               = 0x8C3A,    // GL_R11F_G11F_B10F
        RG3B2                  = 0x2A10,    // GL_R3_G3_B2
        R5G6B5                 = 0x8D62,    // GL_RGB565
        RGB5A1                 = 0x8057,    // GL_RGB5_A1
        RGBA4                  = 0x8056,    // GL_RGBA4
        RGB10A2                = 0x906F,    // GL_RGB10_A2UI

        // Depth formats
        D16                    = 0x81A5,    // GL_DEPTH_COMPONENT16
        D24                    = 0x81A6,    // GL_DEPTH_COMPONENT24
        D24S8                  = 0x88F0,    // GL_DEPTH24_STENCIL8
        D32                    = 0x81A7,    // GL_DEPTH_COMPONENT32
        D32F                   = 0x8CAC,    // GL_DEPTH_COMPONENT32F
        D32FS8X24              = 0x8CAD,    // GL_DEPTH32F_STENCIL8
        S8                     = 0x8D48,    // GL_STENCIL_INDEX8

        // Compressed formats
        RGB_DXT1               = 0x83F0,    // GL_COMPRESSED_RGB_S3TC_DXT1_EXT
        RGBA_DXT1              = 0x83F1,    // GL_COMPRESSED_RGBA_S3TC_DXT1_EXT
        RGBA_DXT3              = 0x83F2,    // GL_COMPRESSED_RGBA_S3TC_DXT3_EXT
        RGBA_DXT5              = 0x83F3,    // GL_COMPRESSED_RGBA_S3TC_DXT5_EXT
        R_ATI1N_UNorm          = 0x8DBB,    // GL_COMPRESSED_RED_RGTC1
        R_ATI1N_SNorm          = 0x8DBC,    // GL_COMPRESSED_SIGNED_RED_RGTC1
        RG_ATI2N_UNorm         = 0x8DBD,    // GL_COMPRESSED_RG_RGTC2
        RG_ATI2N_SNorm         = 0x8DBE,    // GL_COMPRESSED_SIGNED_RG_RGTC2
        RGB_BP_UNSIGNED_FLOAT  = 0x8E8F,    // GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB
        RGB_BP_SIGNED_FLOAT    = 0x8E8E,    // GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB
        RGB_BP_UNorm           = 0x8E8C,    // GL_COMPRESSED_RGBA_BPTC_UNORM_ARB
        R11_EAC_UNorm          = 0x9270,    // GL_COMPRESSED_R11_EAC
        R11_EAC_SNorm          = 0x9271,    // GL_COMPRESSED_SIGNED_R11_EAC
        RG11_EAC_UNorm         = 0x9272,    // GL_COMPRESSED_RG11_EAC
        RG11_EAC_SNorm         = 0x9273,    // GL_COMPRESSED_SIGNED_RG11_EAC
        RGB8_ETC2              = 0x9274,    // GL_COMPRESSED_RGB8_ETC2
        SRGB8_ETC2             = 0x9275,    // GL_COMPRESSED_SRGB8_ETC2
        RGB8_PunchThrough_Alpha1_ETC2 = 0x9276, // GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2
        SRGB8_PunchThrough_Alpha1_ETC2 = 0x9277, // GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2
        RGBA8_ETC2_EAC         = 0x9278,    // GL_COMPRESSED_RGBA8_ETC2_EAC
        SRGB8_Alpha8_ETC2_EAC  = 0x9279,    // GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC
        RGB8_ETC1              = 0x8D64,    // GL_ETC1_RGB8_OES
        RGBA_ASTC_4x4          = 0x93B0,    // GL_COMPRESSED_RGBA_ASTC_4x4_KHR
        RGBA_ASTC_5x4          = 0x93B1,    // GL_COMPRESSED_RGBA_ASTC_5x4_KHR
        RGBA_ASTC_5x5          = 0x93B2,    // GL_COMPRESSED_RGBA_ASTC_5x5_KHR
        RGBA_ASTC_6x5          = 0x93B3,    // GL_COMPRESSED_RGBA_ASTC_6x5_KHR
        RGBA_ASTC_6x6          = 0x93B4,    // GL_COMPRESSED_RGBA_ASTC_6x6_KHR
        RGBA_ASTC_8x5          = 0x93B5,    // GL_COMPRESSED_RGBA_ASTC_8x5_KHR
        RGBA_ASTC_8x6          = 0x93B6,    // GL_COMPRESSED_RGBA_ASTC_8x6_KHR
        RGBA_ASTC_8x8          = 0x93B7,    // GL_COMPRESSED_RGBA_ASTC_8x8_KHR
        RGBA_ASTC_10x5         = 0x93B8,    // GL_COMPRESSED_RGBA_ASTC_10x5_KHR
        RGBA_ASTC_10x6         = 0x93B9,    // GL_COMPRESSED_RGBA_ASTC_10x6_KHR
        RGBA_ASTC_10x8         = 0x93BA,    // GL_COMPRESSED_RGBA_ASTC_10x8_KHR
        RGBA_ASTC_10x10        = 0x93BB,    // GL_COMPRESSED_RGBA_ASTC_10x10_KHR
        RGBA_ASTC_12x10        = 0x93BC,    // GL_COMPRESSED_RGBA_ASTC_12x10_KHR
        RGBA_ASTC_12x12        = 0x93BD,    // GL_COMPRESSED_RGBA_ASTC_12x12_KHR
        SRGB8_Alpha8_ASTC_4x4  = 0x93D0,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR
        SRGB8_Alpha8_ASTC_5x4  = 0x93D1,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR
        SRGB8_Alpha8_ASTC_5x5  = 0x93D2,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR
        SRGB8_Alpha8_ASTC_6x5  = 0x93D3,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR
        SRGB8_Alpha8_ASTC_6x6  = 0x93D4,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR
        SRGB8_Alpha8_ASTC_8x5  = 0x93D5,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR
        SRGB8_Alpha8_ASTC_8x6  = 0x93D6,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR
        SRGB8_Alpha8_ASTC_8x8  = 0x93D7,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR
        SRGB8_Alpha8_ASTC_10x5 = 0x93D8,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR
        SRGB8_Alpha8_ASTC_10x6 = 0x93D9,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR
        SRGB8_Alpha8_ASTC_10x8 = 0x93DA,    // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR
        SRGB8_Alpha8_ASTC_10x10 = 0x93DB,   // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR
        SRGB8_Alpha8_ASTC_12x10 = 0x93DC,   // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR
        SRGB8_Alpha8_ASTC_12x12 = 0x93DD,   // GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR

        // sRGB formats
        SRGB8                  = 0x8C41,    // GL_SRGB8
        SRGB8_Alpha8           = 0x8C43,    // GL_SRGB8_ALPHA8
        SRGB_DXT1              = 0x8C4C,    // GL_COMPRESSED_SRGB_S3TC_DXT1_EXT
        SRGB_Alpha_DXT1        = 0x8C4D,    // GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT
        SRGB_Alpha_DXT3        = 0x8C4E,    // GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT
        SRGB_Alpha_DXT5        = 0x8C4F,    // GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT
        SRGB_BP_UNorm          = 0x8E8D,    // GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB

        // ES 2 formats
        DepthFormat            = 0x1902,    // GL_DEPTH_COMPONENT
        AlphaFormat            = 0x1906,    // GL_ALPHA
        RGBFormat              = 0x1907,    // GL_RGB
        RGBAFormat             = 0x1908,    // GL_RGBA
        LuminanceFormat        = 0x1909,    // GL_LUMINANCE
        LuminanceAlphaFormat   = 0x190A

    };
    Q_ENUM(TextureFormat)

    // This is not used externally yet but is reserved to allow checking of
    // compatibility between texture formats
#ifndef Q_QDOC
    enum TextureFormatClass {
        NoFormatClass,
        FormatClass_128Bit,
        FormatClass_96Bit,
        FormatClass_64Bit,
        FormatClass_48Bit,
        FormatClass_32Bit,
        FormatClass_24Bit,
        FormatClass_16Bit,
        FormatClass_8Bit,
        FormatClass_RGTC1_R,
        FormatClass_RGTC2_RG,
        FormatClass_BPTC_Unorm,
        FormatClass_BPTC_Float,
        FormatClass_S3TC_DXT1_RGB,
        FormatClass_S3TC_DXT1_RGBA,
        FormatClass_S3TC_DXT3_RGBA,
        FormatClass_S3TC_DXT5_RGBA,
        FormatClass_Unique
    };
#endif

    enum CubeMapFace {
        CubeMapPositiveX = 0x8515,  // GL_TEXTURE_CUBE_MAP_POSITIVE_X
        CubeMapNegativeX = 0x8516,  // GL_TEXTURE_CUBE_MAP_NEGATIVE_X
        CubeMapPositiveY = 0x8517,  // GL_TEXTURE_CUBE_MAP_POSITIVE_Y
        CubeMapNegativeY = 0x8518,  // GL_TEXTURE_CUBE_MAP_NEGATIVE_Y
        CubeMapPositiveZ = 0x8519,  // GL_TEXTURE_CUBE_MAP_POSITIVE_Z
        CubeMapNegativeZ = 0x851A   // GL_TEXTURE_CUBE_MAP_NEGATIVE_Z
    };
    Q_ENUM(CubeMapFace)

    enum PixelFormat {
        NoSourceFormat = 0,         // GL_NONE
        Red            = 0x1903,    // GL_RED
        RG             = 0x8227,    // GL_RG
        RGB            = 0x1907,    // GL_RGB
        BGR            = 0x80E0,    // GL_BGR
        RGBA           = 0x1908,    // GL_RGBA
        BGRA           = 0x80E1,    // GL_BGRA
        Red_Integer    = 0x8D94,    // GL_RED_INTEGER
        RG_Integer     = 0x8228,    // GL_RG_INTEGER
        RGB_Integer    = 0x8D98,    // GL_RGB_INTEGER
        BGR_Integer    = 0x8D9A,    // GL_BGR_INTEGER
        RGBA_Integer   = 0x8D99,    // GL_RGBA_INTEGER
        BGRA_Integer   = 0x8D9B,    // GL_BGRA_INTEGER
        Stencil        = 0x1901,    // GL_STENCIL_INDEX
        Depth          = 0x1902,    // GL_DEPTH_COMPONENT
        DepthStencil   = 0x84F9,    // GL_DEPTH_STENCIL
        Alpha          = 0x1906,    // GL_ALPHA
        Luminance      = 0x1909,    // GL_LUMINANCE
        LuminanceAlpha = 0x190A     // GL_LUMINANCE_ALPHA
    };
    Q_ENUM(PixelFormat)

    enum PixelType {
        NoPixelType        = 0,         // GL_NONE
        Int8               = 0x1400,    // GL_BYTE
        UInt8              = 0x1401,    // GL_UNSIGNED_BYTE
        Int16              = 0x1402,    // GL_SHORT
        UInt16             = 0x1403,    // GL_UNSIGNED_SHORT
        Int32              = 0x1404,    // GL_INT
        UInt32             = 0x1405,    // GL_UNSIGNED_INT
        Float16            = 0x140B,    // GL_HALF_FLOAT
        Float16OES         = 0x8D61,    // GL_HALF_FLOAT_OES
        Float32            = 0x1406,    // GL_FLOAT
        UInt32_RGB9_E5     = 0x8C3E,    // GL_UNSIGNED_INT_5_9_9_9_REV
        UInt32_RG11B10F    = 0x8C3B,    // GL_UNSIGNED_INT_10F_11F_11F_REV
        UInt8_RG3B2        = 0x8032,    // GL_UNSIGNED_BYTE_3_3_2
        UInt8_RG3B2_Rev    = 0x8362,    // GL_UNSIGNED_BYTE_2_3_3_REV
        UInt16_RGB5A1      = 0x8034,    // GL_UNSIGNED_SHORT_5_5_5_1
        UInt16_RGB5A1_Rev  = 0x8366,    // GL_UNSIGNED_SHORT_1_5_5_5_REV
        UInt16_R5G6B5      = 0x8363,    // GL_UNSIGNED_SHORT_5_6_5
        UInt16_R5G6B5_Rev  = 0x8364,    // GL_UNSIGNED_SHORT_5_6_5_REV
        UInt16_RGBA4       = 0x8033,    // GL_UNSIGNED_SHORT_4_4_4_4
        UInt16_RGBA4_Rev   = 0x8365,    // GL_UNSIGNED_SHORT_4_4_4_4_REV
        UInt32_RGBA8       = 0x8035,    // GL_UNSIGNED_INT_8_8_8_8
        UInt32_RGBA8_Rev   = 0x8367,    // GL_UNSIGNED_INT_8_8_8_8_REV
        UInt32_RGB10A2     = 0x8036,    // GL_UNSIGNED_INT_10_10_10_2
        UInt32_RGB10A2_Rev = 0x8368,    // GL_UNSIGNED_INT_2_10_10_10_REV
        UInt32_D24S8       = 0x84FA,    // GL_UNSIGNED_INT_24_8
        Float32_D32_UInt32_S8_X24 = 0x8DAD // GL_FLOAT_32_UNSIGNED_INT_24_8_REV
    };
    Q_ENUM(PixelType)

    enum SwizzleComponent {
        SwizzleRed   = 0x8E42,  // GL_TEXTURE_SWIZZLE_R
        SwizzleGreen = 0x8E43,  // GL_TEXTURE_SWIZZLE_G
        SwizzleBlue  = 0x8E44,  // GL_TEXTURE_SWIZZLE_B
        SwizzleAlpha = 0x8E45   // GL_TEXTURE_SWIZZLE_A
    };
    Q_ENUM(SwizzleComponent)

    enum SwizzleValue {
        RedValue   = 0x1903, // GL_RED
        GreenValue = 0x1904, // GL_GREEN
        BlueValue  = 0x1905, // GL_BLUE
        AlphaValue = 0x1906, // GL_ALPHA
        ZeroValue  = 0,      // GL_ZERO
        OneValue   = 1       // GL_ONE
    };
    Q_ENUM(SwizzleValue)

    enum WrapMode {
        Repeat         = 0x2901, // GL_REPEAT
        MirroredRepeat = 0x8370, // GL_MIRRORED_REPEAT
        ClampToEdge    = 0x812F, // GL_CLAMP_TO_EDGE
        ClampToBorder  = 0x812D  // GL_CLAMP_TO_BORDER
    };
    Q_ENUM(WrapMode)

    enum CoordinateDirection {
        DirectionS = 0x2802, // GL_TEXTURE_WRAP_S
        DirectionT = 0x2803, // GL_TEXTURE_WRAP_T
        DirectionR = 0x8072  // GL_TEXTURE_WRAP_R
    };
    Q_ENUM(CoordinateDirection)

    // Features
    enum Feature {
        ImmutableStorage            = 0x00000001,
        ImmutableMultisampleStorage = 0x00000002,
        TextureRectangle            = 0x00000004,
        TextureArrays               = 0x00000008,
        Texture3D                   = 0x00000010,
        TextureMultisample          = 0x00000020,
        TextureBuffer               = 0x00000040,
        TextureCubeMapArrays        = 0x00000080,
        Swizzle                     = 0x00000100,
        StencilTexturing            = 0x00000200,
        AnisotropicFiltering        = 0x00000400,
        NPOTTextures                = 0x00000800,
        NPOTTextureRepeat           = 0x00001000,
        Texture1D                   = 0x00002000,
        TextureComparisonOperators  = 0x00004000,
        TextureMipMapLevel          = 0x00008000,
#ifndef Q_QDOC
        MaxFeatureFlag              = 0x00010000
#endif
    };
    Q_DECLARE_FLAGS(Features, Feature)
    Q_ENUM(Feature)

    explicit QOpenGLTexture(Target target);
    explicit QOpenGLTexture(const QImage& image, MipMapGeneration genMipMaps = GenerateMipMaps);
    ~QOpenGLTexture();

    Target target() const;

    // Creation and destruction
    bool create();
    void destroy();
    bool isCreated() const;
    GLuint textureId() const;

    // Binding and releasing
    void bind();
    void bind(uint unit, TextureUnitReset reset = DontResetTextureUnit);
    void release();
    void release(uint unit, TextureUnitReset reset = DontResetTextureUnit);

    bool isBound() const;
    bool isBound(uint unit);
    static GLuint boundTextureId(BindingTarget target);
    static GLuint boundTextureId(uint unit, BindingTarget target);

    // Storage allocation
    void setFormat(TextureFormat format);
    TextureFormat format() const;
    void setSize(int width, int height = 1, int depth = 1);
    int width() const;
    int height() const;
    int depth() const;
    void setMipLevels(int levels);
    int mipLevels() const;
    int maximumMipLevels() const;
    void setLayers(int layers);
    int layers() const;
    int faces() const;
    void setSamples(int samples);
    int samples() const;
    void setFixedSamplePositions(bool fixed);
    bool isFixedSamplePositions() const;
    void allocateStorage();
    void allocateStorage(PixelFormat pixelFormat, PixelType pixelType);
    bool isStorageAllocated() const;

    QOpenGLTexture *createTextureView(Target target,
                                      TextureFormat viewFormat,
                                      int minimumMipmapLevel, int maximumMipmapLevel,
                                      int minimumLayer, int maximumLayer) const;
    bool isTextureView() const;

    // Pixel transfer
    // ### Qt 6: remove the non-const void * overloads
#if QT_DEPRECATED_SINCE(5, 3)
    QT_DEPRECATED void setData(int mipLevel, int layer, CubeMapFace cubeFace,
                               PixelFormat sourceFormat, PixelType sourceType,
                               void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    QT_DEPRECATED void setData(int mipLevel, int layer,
                               PixelFormat sourceFormat, PixelType sourceType,
                               void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    QT_DEPRECATED void setData(int mipLevel,
                               PixelFormat sourceFormat, PixelType sourceType,
                               void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    QT_DEPRECATED void setData(PixelFormat sourceFormat, PixelType sourceType,
                               void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
#endif // QT_DEPRECATED_SINCE(5, 3)

    void setData(int mipLevel, int layer, CubeMapFace cubeFace,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    void setData(int mipLevel, int layer, int layerCount, CubeMapFace cubeFace,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    void setData(int mipLevel, int layer,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    void setData(int mipLevel,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    void setData(PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);

    void setData(int xOffset, int yOffset, int zOffset,
                 int width, int height, int depth,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    void setData(int xOffset, int yOffset, int zOffset,
                 int width, int height, int depth, int mipLevel,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    void setData(int xOffset, int yOffset, int zOffset,
                 int width, int height, int depth,
                 int mipLevel, int layer,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    void setData(int xOffset, int yOffset, int zOffset,
                 int width, int height, int depth,
                 int mipLevel, int layer,
                 CubeMapFace cubeFace,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);
    void setData(int xOffset, int yOffset, int zOffset,
                 int width, int height, int depth,
                 int mipLevel, int layer,
                 CubeMapFace cubeFace, int layerCount,
                 PixelFormat sourceFormat, PixelType sourceType,
                 const void *data, const QOpenGLPixelTransferOptions * const options = nullptr);

    // Compressed data upload
    // ### Qt 6: remove the non-const void * overloads
#if QT_DEPRECATED_SINCE(5, 3)
    QT_DEPRECATED void setCompressedData(int mipLevel, int layer, CubeMapFace cubeFace,
                                         int dataSize, void *data,
                                         const QOpenGLPixelTransferOptions * const options = nullptr);
    QT_DEPRECATED void setCompressedData(int mipLevel, int layer,
                                         int dataSize, void *data,
                                         const QOpenGLPixelTransferOptions * const options = nullptr);
    QT_DEPRECATED void setCompressedData(int mipLevel, int dataSize, void *data,
                                         const QOpenGLPixelTransferOptions * const options = nullptr);
    QT_DEPRECATED void setCompressedData(int dataSize, void *data,
                                         const QOpenGLPixelTransferOptions * const options = nullptr);
#endif // QT_DEPRECATED_SINCE(5, 3)

    void setCompressedData(int mipLevel, int layer, CubeMapFace cubeFace,
                           int dataSize, const void *data,
                           const QOpenGLPixelTransferOptions * const options = nullptr);
    void setCompressedData(int mipLevel, int layer, int layerCount, CubeMapFace cubeFace,
                           int dataSize, const void *data,
                           const QOpenGLPixelTransferOptions * const options = nullptr);
    void setCompressedData(int mipLevel, int layer,
                           int dataSize, const void *data,
                           const QOpenGLPixelTransferOptions * const options = nullptr);
    void setCompressedData(int mipLevel, int dataSize, const void *data,
                           const QOpenGLPixelTransferOptions * const options = nullptr);
    void setCompressedData(int dataSize, const void *data,
                           const QOpenGLPixelTransferOptions * const options = nullptr);

    // Helpful overloads for setData
    void setData(const QImage& image, MipMapGeneration genMipMaps = GenerateMipMaps);

    static bool hasFeature(Feature feature);

    // Texture Parameters
    void setMipBaseLevel(int baseLevel);
    int mipBaseLevel() const;
    void setMipMaxLevel(int maxLevel);
    int mipMaxLevel() const;
    void setMipLevelRange(int baseLevel, int maxLevel);
    QPair<int, int> mipLevelRange() const;

    void setAutoMipMapGenerationEnabled(bool enabled);
    bool isAutoMipMapGenerationEnabled() const;

    void generateMipMaps();
    void generateMipMaps(int baseLevel, bool resetBaseLevel = true);

    void setSwizzleMask(SwizzleComponent component, SwizzleValue value);
    void setSwizzleMask(SwizzleValue r, SwizzleValue g,
                        SwizzleValue b, SwizzleValue a);
    SwizzleValue swizzleMask(SwizzleComponent component) const;

    enum DepthStencilMode {
        DepthMode   = 0x1902,   // GL_DEPTH_COMPONENT
        StencilMode = 0x1901    // GL_STENCIL_INDEX
    };
    Q_ENUM(DepthStencilMode)

    void setDepthStencilMode(DepthStencilMode mode);
    DepthStencilMode depthStencilMode() const;

    enum ComparisonFunction {
        CompareLessEqual    = 0x0203,   // GL_LEQUAL
        CompareGreaterEqual = 0x0206,   // GL_GEQUAL
        CompareLess         = 0x0201,   // GL_LESS
        CompareGreater      = 0x0204,   // GL_GREATER
        CompareEqual        = 0x0202,   // GL_EQUAL
        CommpareNotEqual    = 0x0205,   // GL_NOTEQUAL
        CompareAlways       = 0x0207,   // GL_ALWAYS
        CompareNever        = 0x0200    // GL_NEVER
    };
    Q_ENUM(ComparisonFunction)

    void setComparisonFunction(ComparisonFunction function);
    ComparisonFunction comparisonFunction() const;

    enum ComparisonMode {
        CompareRefToTexture = 0x884E,   // GL_COMPARE_REF_TO_TEXTURE
        CompareNone         = 0x0000    // GL_NONE
    };

    void setComparisonMode(ComparisonMode mode);
    ComparisonMode comparisonMode() const;

    // Sampling Parameters
    enum Filter {
        Nearest                 = 0x2600,   // GL_NEAREST
        Linear                  = 0x2601,   // GL_LINEAR
        NearestMipMapNearest    = 0x2700,   // GL_NEAREST_MIPMAP_NEAREST
        NearestMipMapLinear     = 0x2702,   // GL_NEAREST_MIPMAP_LINEAR
        LinearMipMapNearest     = 0x2701,   // GL_LINEAR_MIPMAP_NEAREST
        LinearMipMapLinear      = 0x2703    // GL_LINEAR_MIPMAP_LINEAR
    };
    Q_ENUM(Filter)

    void setMinificationFilter(Filter filter);
    Filter minificationFilter() const;
    void setMagnificationFilter(Filter filter);
    Filter magnificationFilter() const;
    void setMinMagFilters(Filter minificationFilter,
                          Filter magnificationFilter);
    QPair<Filter, Filter> minMagFilters() const;
    void setMaximumAnisotropy(float anisotropy);
    float maximumAnisotropy() const;

    void setWrapMode(WrapMode mode);
    void setWrapMode(CoordinateDirection direction, WrapMode mode);
    WrapMode wrapMode(CoordinateDirection direction) const;

    void setBorderColor(QColor color);
    void setBorderColor(float r, float g, float b, float a);
    void setBorderColor(int r, int g, int b, int a);
    void setBorderColor(uint r, uint g, uint b, uint a);

    QColor borderColor() const;
    void borderColor(float *border) const;
    void borderColor(int *border) const;
    void borderColor(unsigned int *border) const;

    void setMinimumLevelOfDetail(float value);
    float minimumLevelOfDetail() const;
    void setMaximumLevelOfDetail(float value);
    float maximumLevelOfDetail() const;
    void setLevelOfDetailRange(float min, float max);
    QPair<float, float> levelOfDetailRange() const;
    void setLevelofDetailBias(float bias);
    float levelofDetailBias() const;

#ifndef QT_NO_DEBUG_STREAM
    friend Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QOpenGLTexture *t);
#endif

private:
    Q_DISABLE_COPY(QOpenGLTexture)
    Q_DECLARE_PRIVATE(QOpenGLTexture)
    QScopedPointer<QOpenGLTexturePrivate> d_ptr;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QOpenGLTexture::Features)

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug debug, const QOpenGLTexture *t);
#endif

QT_END_NAMESPACE

#endif // QT_NO_OPENGL

#endif // QOPENGLABSTRACTTEXTURE_H
