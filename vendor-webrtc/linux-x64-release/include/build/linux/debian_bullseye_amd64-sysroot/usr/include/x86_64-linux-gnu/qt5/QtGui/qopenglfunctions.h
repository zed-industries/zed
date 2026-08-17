/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
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

#ifndef QOPENGLFUNCTIONS_H
#define QOPENGLFUNCTIONS_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#ifdef __GLEW_H__
#if defined(Q_CC_GNU)
#warning qopenglfunctions.h is not compatible with GLEW, GLEW defines will be undefined
#warning To use GLEW with Qt, do not include <qopengl.h> or <QOpenGLFunctions> after glew.h
#endif
#endif

#include <QtGui/qopengl.h>
#include <QtGui/qopenglcontext.h>

//#define Q_ENABLE_OPENGL_FUNCTIONS_DEBUG

#ifdef QT_OPENGL_ES
typedef double GLdouble;
#endif

#ifdef Q_ENABLE_OPENGL_FUNCTIONS_DEBUG
#include <stdio.h>
#define Q_OPENGL_FUNCTIONS_DEBUG \
    GLenum error = glGetError(); \
    if (error != GL_NO_ERROR) { \
        unsigned clamped = qMin(unsigned(error - GL_INVALID_ENUM), 4U); \
        const char *errors[] = { "GL_INVALID_ENUM", "GL_INVALID_VALUE", "GL_INVALID_OPERATION", "Unknown" }; \
        printf("GL error at %s:%d: %s\n", __FILE__, __LINE__, errors[clamped]); \
        int *value = 0; \
        *value = 0; \
    }
#else
#define Q_OPENGL_FUNCTIONS_DEBUG
#endif

QT_BEGIN_NAMESPACE

struct QOpenGLFunctionsPrivate;

// Undefine any macros from GLEW, qopenglextensions_p.h, etc that
// may interfere with the definition of QOpenGLFunctions.
#undef glBindTexture
#undef glBlendFunc
#undef glClear
#undef glClearColor
#undef glClearStencil
#undef glColorMask
#undef glCopyTexImage2D
#undef glCopyTexSubImage2D
#undef glCullFace
#undef glDeleteTextures
#undef glDepthFunc
#undef glDepthMask
#undef glDisable
#undef glDrawArrays
#undef glDrawElements
#undef glEnable
#undef glFinish
#undef glFlush
#undef glFrontFace
#undef glGenTextures
#undef glGetBooleanv
#undef glGetError
#undef glGetFloatv
#undef glGetIntegerv
#undef glGetString
#undef glGetTexParameterfv
#undef glGetTexParameteriv
#undef glHint
#undef glIsEnabled
#undef glIsTexture
#undef glLineWidth
#undef glPixelStorei
#undef glPolygonOffset
#undef glReadPixels
#undef glScissor
#undef glStencilFunc
#undef glStencilMask
#undef glStencilOp
#undef glTexImage2D
#undef glTexParameterf
#undef glTexParameterfv
#undef glTexParameteri
#undef glTexParameteriv
#undef glTexSubImage2D
#undef glViewport

#undef glActiveTexture
#undef glAttachShader
#undef glBindAttribLocation
#undef glBindBuffer
#undef glBindFramebuffer
#undef glBindRenderbuffer
#undef glBlendColor
#undef glBlendEquation
#undef glBlendEquationSeparate
#undef glBlendFuncSeparate
#undef glBufferData
#undef glBufferSubData
#undef glCheckFramebufferStatus
#undef glClearDepthf
#undef glCompileShader
#undef glCompressedTexImage2D
#undef glCompressedTexSubImage2D
#undef glCreateProgram
#undef glCreateShader
#undef glDeleteBuffers
#undef glDeleteFramebuffers
#undef glDeleteProgram
#undef glDeleteRenderbuffers
#undef glDeleteShader
#undef glDepthRangef
#undef glDetachShader
#undef glDisableVertexAttribArray
#undef glEnableVertexAttribArray
#undef glFramebufferRenderbuffer
#undef glFramebufferTexture2D
#undef glGenBuffers
#undef glGenerateMipmap
#undef glGenFramebuffers
#undef glGenRenderbuffers
#undef glGetActiveAttrib
#undef glGetActiveUniform
#undef glGetAttachedShaders
#undef glGetAttribLocation
#undef glGetBufferParameteriv
#undef glGetFramebufferAttachmentParameteriv
#undef glGetProgramiv
#undef glGetProgramInfoLog
#undef glGetRenderbufferParameteriv
#undef glGetShaderiv
#undef glGetShaderInfoLog
#undef glGetShaderPrecisionFormat
#undef glGetShaderSource
#undef glGetUniformfv
#undef glGetUniformiv
#undef glGetUniformLocation
#undef glGetVertexAttribfv
#undef glGetVertexAttribiv
#undef glGetVertexAttribPointerv
#undef glIsBuffer
#undef glIsFramebuffer
#undef glIsProgram
#undef glIsRenderbuffer
#undef glIsShader
#undef glLinkProgram
#undef glReleaseShaderCompiler
#undef glRenderbufferStorage
#undef glSampleCoverage
#undef glShaderBinary
#undef glShaderSource
#undef glStencilFuncSeparate
#undef glStencilMaskSeparate
#undef glStencilOpSeparate
#undef glUniform1f
#undef glUniform1fv
#undef glUniform1i
#undef glUniform1iv
#undef glUniform2f
#undef glUniform2fv
#undef glUniform2i
#undef glUniform2iv
#undef glUniform3f
#undef glUniform3fv
#undef glUniform3i
#undef glUniform3iv
#undef glUniform4f
#undef glUniform4fv
#undef glUniform4i
#undef glUniform4iv
#undef glUniformMatrix2fv
#undef glUniformMatrix3fv
#undef glUniformMatrix4fv
#undef glUseProgram
#undef glValidateProgram
#undef glVertexAttrib1f
#undef glVertexAttrib1fv
#undef glVertexAttrib2f
#undef glVertexAttrib2fv
#undef glVertexAttrib3f
#undef glVertexAttrib3fv
#undef glVertexAttrib4f
#undef glVertexAttrib4fv
#undef glVertexAttribPointer

#undef glTexLevelParameteriv

#if defined(Q_CLANG_QDOC)
#undef GLbitfield
typedef unsigned int GLbitfield;
#undef GLchar
typedef char GLchar;
#endif

class Q_GUI_EXPORT QOpenGLFunctions
{
public:
    QOpenGLFunctions();
    explicit QOpenGLFunctions(QOpenGLContext *context);
    ~QOpenGLFunctions() {}

    enum OpenGLFeature
    {
        Multitexture          = 0x0001,
        Shaders               = 0x0002,
        Buffers               = 0x0004,
        Framebuffers          = 0x0008,
        BlendColor            = 0x0010,
        BlendEquation         = 0x0020,
        BlendEquationSeparate = 0x0040,
        BlendFuncSeparate     = 0x0080,
        BlendSubtract         = 0x0100,
        CompressedTextures    = 0x0200,
        Multisample           = 0x0400,
        StencilSeparate       = 0x0800,
        NPOTTextures          = 0x1000,
        NPOTTextureRepeat     = 0x2000,
        FixedFunctionPipeline = 0x4000,
        TextureRGFormats      = 0x8000,
        MultipleRenderTargets = 0x10000,
        BlendEquationAdvanced = 0x20000,
    };
    Q_DECLARE_FLAGS(OpenGLFeatures, OpenGLFeature)

    QOpenGLFunctions::OpenGLFeatures openGLFeatures() const;
    bool hasOpenGLFeature(QOpenGLFunctions::OpenGLFeature feature) const;

    void initializeOpenGLFunctions();

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED void initializeGLFunctions() { initializeOpenGLFunctions(); }
#endif

    // GLES2 + OpenGL1 common subset
    void glBindTexture(GLenum target, GLuint texture);
    void glBlendFunc(GLenum sfactor, GLenum dfactor);
    void glClear(GLbitfield mask);
    void glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
    void glClearStencil(GLint s);
    void glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
    void glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
    void glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
    void glCullFace(GLenum mode);
    void glDeleteTextures(GLsizei n, const GLuint* textures);
    void glDepthFunc(GLenum func);
    void glDepthMask(GLboolean flag);
    void glDisable(GLenum cap);
    void glDrawArrays(GLenum mode, GLint first, GLsizei count);
    void glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid* indices);
    void glEnable(GLenum cap);
    void glFinish();
    void glFlush();
    void glFrontFace(GLenum mode);
    void glGenTextures(GLsizei n, GLuint* textures);
    void glGetBooleanv(GLenum pname, GLboolean* params);
    GLenum glGetError();
    void glGetFloatv(GLenum pname, GLfloat* params);
    void glGetIntegerv(GLenum pname, GLint* params);
    const GLubyte *glGetString(GLenum name);
    void glGetTexParameterfv(GLenum target, GLenum pname, GLfloat* params);
    void glGetTexParameteriv(GLenum target, GLenum pname, GLint* params);
    void glHint(GLenum target, GLenum mode);
    GLboolean glIsEnabled(GLenum cap);
    GLboolean glIsTexture(GLuint texture);
    void glLineWidth(GLfloat width);
    void glPixelStorei(GLenum pname, GLint param);
    void glPolygonOffset(GLfloat factor, GLfloat units);
    void glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid* pixels);
    void glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
    void glStencilFunc(GLenum func, GLint ref, GLuint mask);
    void glStencilMask(GLuint mask);
    void glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
    void glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid* pixels);
    void glTexParameterf(GLenum target, GLenum pname, GLfloat param);
    void glTexParameterfv(GLenum target, GLenum pname, const GLfloat* params);
    void glTexParameteri(GLenum target, GLenum pname, GLint param);
    void glTexParameteriv(GLenum target, GLenum pname, const GLint* params);
    void glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid* pixels);
    void glViewport(GLint x, GLint y, GLsizei width, GLsizei height);

    // GL(ES)2
    void glActiveTexture(GLenum texture);
    void glAttachShader(GLuint program, GLuint shader);
    void glBindAttribLocation(GLuint program, GLuint index, const char* name);
    void glBindBuffer(GLenum target, GLuint buffer);
    void glBindFramebuffer(GLenum target, GLuint framebuffer);
    void glBindRenderbuffer(GLenum target, GLuint renderbuffer);
    void glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
    void glBlendEquation(GLenum mode);
    void glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
    void glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
    void glBufferData(GLenum target, qopengl_GLsizeiptr size, const void* data, GLenum usage);
    void glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void* data);
    GLenum glCheckFramebufferStatus(GLenum target);
    void glClearDepthf(GLclampf depth);
    void glCompileShader(GLuint shader);
    void glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void* data);
    void glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void* data);
    GLuint glCreateProgram();
    GLuint glCreateShader(GLenum type);
    void glDeleteBuffers(GLsizei n, const GLuint* buffers);
    void glDeleteFramebuffers(GLsizei n, const GLuint* framebuffers);
    void glDeleteProgram(GLuint program);
    void glDeleteRenderbuffers(GLsizei n, const GLuint* renderbuffers);
    void glDeleteShader(GLuint shader);
    void glDepthRangef(GLclampf zNear, GLclampf zFar);
    void glDetachShader(GLuint program, GLuint shader);
    void glDisableVertexAttribArray(GLuint index);
    void glEnableVertexAttribArray(GLuint index);
    void glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
    void glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
    void glGenBuffers(GLsizei n, GLuint* buffers);
    void glGenerateMipmap(GLenum target);
    void glGenFramebuffers(GLsizei n, GLuint* framebuffers);
    void glGenRenderbuffers(GLsizei n, GLuint* renderbuffers);
    void glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, char* name);
    void glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, char* name);
    void glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei* count, GLuint* shaders);
    GLint glGetAttribLocation(GLuint program, const char* name);
    void glGetBufferParameteriv(GLenum target, GLenum pname, GLint* params);
    void glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint* params);
    void glGetProgramiv(GLuint program, GLenum pname, GLint* params);
    void glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei* length, char* infolog);
    void glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint* params);
    void glGetShaderiv(GLuint shader, GLenum pname, GLint* params);
    void glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei* length, char* infolog);
    void glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint* range, GLint* precision);
    void glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei* length, char* source);
    void glGetUniformfv(GLuint program, GLint location, GLfloat* params);
    void glGetUniformiv(GLuint program, GLint location, GLint* params);
    GLint glGetUniformLocation(GLuint program, const char* name);
    void glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat* params);
    void glGetVertexAttribiv(GLuint index, GLenum pname, GLint* params);
    void glGetVertexAttribPointerv(GLuint index, GLenum pname, void** pointer);
    GLboolean glIsBuffer(GLuint buffer);
    GLboolean glIsFramebuffer(GLuint framebuffer);
    GLboolean glIsProgram(GLuint program);
    GLboolean glIsRenderbuffer(GLuint renderbuffer);
    GLboolean glIsShader(GLuint shader);
    void glLinkProgram(GLuint program);
    void glReleaseShaderCompiler();
    void glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
    void glSampleCoverage(GLclampf value, GLboolean invert);
    void glShaderBinary(GLint n, const GLuint* shaders, GLenum binaryformat, const void* binary, GLint length);
    void glShaderSource(GLuint shader, GLsizei count, const char** string, const GLint* length);
    void glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
    void glStencilMaskSeparate(GLenum face, GLuint mask);
    void glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
    void glUniform1f(GLint location, GLfloat x);
    void glUniform1fv(GLint location, GLsizei count, const GLfloat* v);
    void glUniform1i(GLint location, GLint x);
    void glUniform1iv(GLint location, GLsizei count, const GLint* v);
    void glUniform2f(GLint location, GLfloat x, GLfloat y);
    void glUniform2fv(GLint location, GLsizei count, const GLfloat* v);
    void glUniform2i(GLint location, GLint x, GLint y);
    void glUniform2iv(GLint location, GLsizei count, const GLint* v);
    void glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z);
    void glUniform3fv(GLint location, GLsizei count, const GLfloat* v);
    void glUniform3i(GLint location, GLint x, GLint y, GLint z);
    void glUniform3iv(GLint location, GLsizei count, const GLint* v);
    void glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
    void glUniform4fv(GLint location, GLsizei count, const GLfloat* v);
    void glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w);
    void glUniform4iv(GLint location, GLsizei count, const GLint* v);
    void glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value);
    void glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value);
    void glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value);
    void glUseProgram(GLuint program);
    void glValidateProgram(GLuint program);
    void glVertexAttrib1f(GLuint indx, GLfloat x);
    void glVertexAttrib1fv(GLuint indx, const GLfloat* values);
    void glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y);
    void glVertexAttrib2fv(GLuint indx, const GLfloat* values);
    void glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z);
    void glVertexAttrib3fv(GLuint indx, const GLfloat* values);
    void glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
    void glVertexAttrib4fv(GLuint indx, const GLfloat* values);
    void glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void* ptr);

protected:
    QOpenGLFunctionsPrivate *d_ptr;
    static bool isInitialized(const QOpenGLFunctionsPrivate *d) { return d != nullptr; }
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QOpenGLFunctions::OpenGLFeatures)

#define QT_OPENGL_DECLARE_FUNCTIONS(ret, name, args) \
    ret (QOPENGLF_APIENTRYP name)args;
#define QT_OPENGL_COUNT_FUNCTIONS(ret, name, args) +1

#define QT_OPENGL_DECLARE(FUNCTIONS) \
public: \
    struct Functions { \
        FUNCTIONS(QT_OPENGL_DECLARE_FUNCTIONS) \
    }; \
    union { \
        QFunctionPointer functions[FUNCTIONS(QT_OPENGL_COUNT_FUNCTIONS)]; \
        Functions f; \
    }; \
private: \
    void init(QOpenGLContext *context);

struct QOpenGLFunctionsPrivate
{
    QOpenGLFunctionsPrivate(QOpenGLContext *ctx);

#define QT_OPENGL_FUNCTIONS(F) \
    F(void, BindTexture, (GLenum target, GLuint texture)) \
    F(void, BlendFunc, (GLenum sfactor, GLenum dfactor)) \
    F(void, Clear, (GLbitfield mask)) \
    F(void, ClearColor, (GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha)) \
    F(void, ClearDepthf, (GLclampf depth)) \
    F(void, ClearStencil, (GLint s)) \
    F(void, ColorMask, (GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha)) \
    F(void, CopyTexImage2D, (GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border)) \
    F(void, CopyTexSubImage2D, (GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height)) \
    F(void, CullFace, (GLenum mode)) \
    F(void, DeleteTextures, (GLsizei n, const GLuint* textures)) \
    F(void, DepthFunc, (GLenum func)) \
    F(void, DepthMask, (GLboolean flag)) \
    F(void, DepthRangef, (GLclampf nearVal, GLclampf farVal)) \
    F(void, Disable, (GLenum cap)) \
    F(void, DrawArrays, (GLenum mode, GLint first, GLsizei count)) \
    F(void, DrawElements, (GLenum mode, GLsizei count, GLenum type, const GLvoid* indices)) \
    F(void, Enable, (GLenum cap)) \
    F(void, Finish, ()) \
    F(void, Flush, ()) \
    F(void, FrontFace, (GLenum mode)) \
    F(void, GenTextures, (GLsizei n, GLuint* textures)) \
    F(void, GetBooleanv, (GLenum pname, GLboolean* params)) \
    F(GLenum, GetError, ()) \
    F(void, GetFloatv, (GLenum pname, GLfloat* params)) \
    F(void, GetIntegerv, (GLenum pname, GLint* params)) \
    F(const GLubyte *, GetString, (GLenum name)) \
    F(void, GetTexParameterfv, (GLenum target, GLenum pname, GLfloat* params)) \
    F(void, GetTexParameteriv, (GLenum target, GLenum pname, GLint* params)) \
    F(void, Hint, (GLenum target, GLenum mode)) \
    F(GLboolean, IsEnabled, (GLenum cap)) \
    F(GLboolean, IsTexture, (GLuint texture)) \
    F(void, LineWidth, (GLfloat width)) \
    F(void, PixelStorei, (GLenum pname, GLint param)) \
    F(void, PolygonOffset, (GLfloat factor, GLfloat units)) \
    F(void, ReadPixels, (GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid* pixels)) \
    F(void, Scissor, (GLint x, GLint y, GLsizei width, GLsizei height)) \
    F(void, StencilFunc, (GLenum func, GLint ref, GLuint mask)) \
    F(void, StencilMask, (GLuint mask)) \
    F(void, StencilOp, (GLenum fail, GLenum zfail, GLenum zpass)) \
    F(void, TexImage2D, (GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid* pixels)) \
    F(void, TexParameterf, (GLenum target, GLenum pname, GLfloat param)) \
    F(void, TexParameterfv, (GLenum target, GLenum pname, const GLfloat* params)) \
    F(void, TexParameteri, (GLenum target, GLenum pname, GLint param)) \
    F(void, TexParameteriv, (GLenum target, GLenum pname, const GLint* params)) \
    F(void, TexSubImage2D, (GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid* pixels)) \
    F(void, Viewport, (GLint x, GLint y, GLsizei width, GLsizei height)) \
    F(void, ActiveTexture, (GLenum texture)) \
    F(void, AttachShader, (GLuint program, GLuint shader)) \
    F(void, BindAttribLocation, (GLuint program, GLuint index, const char* name)) \
    F(void, BindBuffer, (GLenum target, GLuint buffer)) \
    F(void, BindFramebuffer, (GLenum target, GLuint framebuffer)) \
    F(void, BindRenderbuffer, (GLenum target, GLuint renderbuffer)) \
    F(void, BlendColor, (GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha)) \
    F(void, BlendEquation, (GLenum mode)) \
    F(void, BlendEquationSeparate, (GLenum modeRGB, GLenum modeAlpha)) \
    F(void, BlendFuncSeparate, (GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha)) \
    F(void, BufferData, (GLenum target, qopengl_GLsizeiptr size, const void* data, GLenum usage)) \
    F(void, BufferSubData, (GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void* data)) \
    F(GLenum, CheckFramebufferStatus, (GLenum target)) \
    F(void, CompileShader, (GLuint shader)) \
    F(void, CompressedTexImage2D, (GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void* data)) \
    F(void, CompressedTexSubImage2D, (GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void* data)) \
    F(GLuint, CreateProgram, ()) \
    F(GLuint, CreateShader, (GLenum type)) \
    F(void, DeleteBuffers, (GLsizei n, const GLuint* buffers)) \
    F(void, DeleteFramebuffers, (GLsizei n, const GLuint* framebuffers)) \
    F(void, DeleteProgram, (GLuint program)) \
    F(void, DeleteRenderbuffers, (GLsizei n, const GLuint* renderbuffers)) \
    F(void, DeleteShader, (GLuint shader)) \
    F(void, DetachShader, (GLuint program, GLuint shader)) \
    F(void, DisableVertexAttribArray, (GLuint index)) \
    F(void, EnableVertexAttribArray, (GLuint index)) \
    F(void, FramebufferRenderbuffer, (GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer)) \
    F(void, FramebufferTexture2D, (GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level)) \
    F(void, GenBuffers, (GLsizei n, GLuint* buffers)) \
    F(void, GenerateMipmap, (GLenum target)) \
    F(void, GenFramebuffers, (GLsizei n, GLuint* framebuffers)) \
    F(void, GenRenderbuffers, (GLsizei n, GLuint* renderbuffers)) \
    F(void, GetActiveAttrib, (GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, char* name)) \
    F(void, GetActiveUniform, (GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, char* name)) \
    F(void, GetAttachedShaders, (GLuint program, GLsizei maxcount, GLsizei* count, GLuint* shaders)) \
    F(GLint, GetAttribLocation, (GLuint program, const char* name)) \
    F(void, GetBufferParameteriv, (GLenum target, GLenum pname, GLint* params)) \
    F(void, GetFramebufferAttachmentParameteriv, (GLenum target, GLenum attachment, GLenum pname, GLint* params)) \
    F(void, GetProgramiv, (GLuint program, GLenum pname, GLint* params)) \
    F(void, GetProgramInfoLog, (GLuint program, GLsizei bufsize, GLsizei* length, char* infolog)) \
    F(void, GetRenderbufferParameteriv, (GLenum target, GLenum pname, GLint* params)) \
    F(void, GetShaderiv, (GLuint shader, GLenum pname, GLint* params)) \
    F(void, GetShaderInfoLog, (GLuint shader, GLsizei bufsize, GLsizei* length, char* infolog)) \
    F(void, GetShaderPrecisionFormat, (GLenum shadertype, GLenum precisiontype, GLint* range, GLint* precision)) \
    F(void, GetShaderSource, (GLuint shader, GLsizei bufsize, GLsizei* length, char* source)) \
    F(void, GetUniformfv, (GLuint program, GLint location, GLfloat* params)) \
    F(void, GetUniformiv, (GLuint program, GLint location, GLint* params)) \
    F(GLint, GetUniformLocation, (GLuint program, const char* name)) \
    F(void, GetVertexAttribfv, (GLuint index, GLenum pname, GLfloat* params)) \
    F(void, GetVertexAttribiv, (GLuint index, GLenum pname, GLint* params)) \
    F(void, GetVertexAttribPointerv, (GLuint index, GLenum pname, void** pointer)) \
    F(GLboolean, IsBuffer, (GLuint buffer)) \
    F(GLboolean, IsFramebuffer, (GLuint framebuffer)) \
    F(GLboolean, IsProgram, (GLuint program)) \
    F(GLboolean, IsRenderbuffer, (GLuint renderbuffer)) \
    F(GLboolean, IsShader, (GLuint shader)) \
    F(void, LinkProgram, (GLuint program)) \
    F(void, ReleaseShaderCompiler, ()) \
    F(void, RenderbufferStorage, (GLenum target, GLenum internalformat, GLsizei width, GLsizei height)) \
    F(void, SampleCoverage, (GLclampf value, GLboolean invert)) \
    F(void, ShaderBinary, (GLint n, const GLuint* shaders, GLenum binaryformat, const void* binary, GLint length)) \
    F(void, ShaderSource, (GLuint shader, GLsizei count, const char** string, const GLint* length)) \
    F(void, StencilFuncSeparate, (GLenum face, GLenum func, GLint ref, GLuint mask)) \
    F(void, StencilMaskSeparate, (GLenum face, GLuint mask)) \
    F(void, StencilOpSeparate, (GLenum face, GLenum fail, GLenum zfail, GLenum zpass)) \
    F(void, Uniform1f, (GLint location, GLfloat x)) \
    F(void, Uniform1fv, (GLint location, GLsizei count, const GLfloat* v)) \
    F(void, Uniform1i, (GLint location, GLint x)) \
    F(void, Uniform1iv, (GLint location, GLsizei count, const GLint* v)) \
    F(void, Uniform2f, (GLint location, GLfloat x, GLfloat y)) \
    F(void, Uniform2fv, (GLint location, GLsizei count, const GLfloat* v)) \
    F(void, Uniform2i, (GLint location, GLint x, GLint y)) \
    F(void, Uniform2iv, (GLint location, GLsizei count, const GLint* v)) \
    F(void, Uniform3f, (GLint location, GLfloat x, GLfloat y, GLfloat z)) \
    F(void, Uniform3fv, (GLint location, GLsizei count, const GLfloat* v)) \
    F(void, Uniform3i, (GLint location, GLint x, GLint y, GLint z)) \
    F(void, Uniform3iv, (GLint location, GLsizei count, const GLint* v)) \
    F(void, Uniform4f, (GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w)) \
    F(void, Uniform4fv, (GLint location, GLsizei count, const GLfloat* v)) \
    F(void, Uniform4i, (GLint location, GLint x, GLint y, GLint z, GLint w)) \
    F(void, Uniform4iv, (GLint location, GLsizei count, const GLint* v)) \
    F(void, UniformMatrix2fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)) \
    F(void, UniformMatrix3fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)) \
    F(void, UniformMatrix4fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)) \
    F(void, UseProgram, (GLuint program)) \
    F(void, ValidateProgram, (GLuint program)) \
    F(void, VertexAttrib1f, (GLuint indx, GLfloat x)) \
    F(void, VertexAttrib1fv, (GLuint indx, const GLfloat* values)) \
    F(void, VertexAttrib2f, (GLuint indx, GLfloat x, GLfloat y)) \
    F(void, VertexAttrib2fv, (GLuint indx, const GLfloat* values)) \
    F(void, VertexAttrib3f, (GLuint indx, GLfloat x, GLfloat y, GLfloat z)) \
    F(void, VertexAttrib3fv, (GLuint indx, const GLfloat* values)) \
    F(void, VertexAttrib4f, (GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w)) \
    F(void, VertexAttrib4fv, (GLuint indx, const GLfloat* values)) \
    F(void, VertexAttribPointer, (GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void* ptr)) \
    F(void, ClearDepth, (GLdouble depth)) \
    F(void, DepthRange, (GLdouble zNear, GLdouble zFar)) \

    QT_OPENGL_DECLARE(QT_OPENGL_FUNCTIONS)
};

// GLES2 + OpenGL1 common subset

inline void QOpenGLFunctions::glBindTexture(GLenum target, GLuint texture)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBindTexture(target, texture);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BindTexture(target, texture);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBlendFunc(GLenum sfactor, GLenum dfactor)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBlendFunc(sfactor, dfactor);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BlendFunc(sfactor, dfactor);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glClear(GLbitfield mask)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glClear(mask);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Clear(mask);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glClearColor(red, green, blue, alpha);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ClearColor(red, green, blue, alpha);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glClearStencil(GLint s)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glClearStencil(s);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ClearStencil(s);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glColorMask(red, green, blue, alpha);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ColorMask(red, green, blue, alpha);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glCopyTexImage2D(target, level, internalformat, x, y, width,height, border);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.CopyTexImage2D(target, level, internalformat, x, y, width,height, border);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glCullFace(GLenum mode)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glCullFace(mode);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.CullFace(mode);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDeleteTextures(GLsizei n, const GLuint* textures)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDeleteTextures(n, textures);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DeleteTextures(n, textures);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDepthFunc(GLenum func)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDepthFunc(func);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DepthFunc(func);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDepthMask(GLboolean flag)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDepthMask(flag);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DepthMask(flag);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDisable(GLenum cap)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDisable(cap);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Disable(cap);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDrawArrays(GLenum mode, GLint first, GLsizei count)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDrawArrays(mode, first, count);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DrawArrays(mode, first, count);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid* indices)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDrawElements(mode, count, type, indices);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DrawElements(mode, count, type, indices);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glEnable(GLenum cap)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glEnable(cap);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Enable(cap);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glFinish()
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glFinish();
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Finish();
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glFlush()
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glFlush();
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Flush();
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glFrontFace(GLenum mode)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glFrontFace(mode);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.FrontFace(mode);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGenTextures(GLsizei n, GLuint* textures)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGenTextures(n, textures);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GenTextures(n, textures);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetBooleanv(GLenum pname, GLboolean* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetBooleanv(pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetBooleanv(pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLenum QOpenGLFunctions::glGetError()
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLenum result = ::glGetError();
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLenum result = d_ptr->f.GetError();
#endif
    return result;
}

inline void QOpenGLFunctions::glGetFloatv(GLenum pname, GLfloat* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetFloatv(pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetFloatv(pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetIntegerv(GLenum pname, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetIntegerv(pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetIntegerv(pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline const GLubyte *QOpenGLFunctions::glGetString(GLenum name)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    const GLubyte *result = ::glGetString(name);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    const GLubyte *result = d_ptr->f.GetString(name);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLFunctions::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetTexParameterfv(target, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetTexParameterfv(target, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetTexParameteriv(GLenum target, GLenum pname, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetTexParameteriv(target, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetTexParameteriv(target, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glHint(GLenum target, GLenum mode)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glHint(target, mode);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Hint(target, mode);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLboolean QOpenGLFunctions::glIsEnabled(GLenum cap)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLboolean result = ::glIsEnabled(cap);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLboolean result = d_ptr->f.IsEnabled(cap);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLFunctions::glIsTexture(GLuint texture)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLboolean result = ::glIsTexture(texture);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLboolean result = d_ptr->f.IsTexture(texture);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLFunctions::glLineWidth(GLfloat width)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glLineWidth(width);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.LineWidth(width);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glPixelStorei(GLenum pname, GLint param)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glPixelStorei(pname, param);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.PixelStorei(pname, param);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glPolygonOffset(GLfloat factor, GLfloat units)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glPolygonOffset(factor, units);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.PolygonOffset(factor, units);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid* pixels)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glReadPixels(x, y, width, height, format, type, pixels);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ReadPixels(x, y, width, height, format, type, pixels);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glScissor(GLint x, GLint y, GLsizei width, GLsizei height)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glScissor(x, y, width, height);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Scissor(x, y, width, height);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glStencilFunc(GLenum func, GLint ref, GLuint mask)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glStencilFunc(func, ref, mask);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.StencilFunc(func, ref, mask);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glStencilMask(GLuint mask)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glStencilMask(mask);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.StencilMask(mask);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glStencilOp(fail, zfail, zpass);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.StencilOp(fail, zfail, zpass);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid* pixels)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glTexImage2D(target, level, internalformat, width,height, border, format, type, pixels);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.TexImage2D(target, level, internalformat, width,height, border, format, type, pixels);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glTexParameterf(GLenum target, GLenum pname, GLfloat param)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glTexParameterf(target, pname, param);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.TexParameterf(target, pname, param);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glTexParameterfv(GLenum target, GLenum pname, const GLfloat* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glTexParameterfv(target, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.TexParameterfv(target, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glTexParameteri(GLenum target, GLenum pname, GLint param)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glTexParameteri(target, pname, param);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.TexParameteri(target, pname, param);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glTexParameteriv(GLenum target, GLenum pname, const GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glTexParameteriv(target, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.TexParameteriv(target, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid* pixels)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.TexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glViewport(GLint x, GLint y, GLsizei width, GLsizei height)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glViewport(x, y, width, height);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Viewport(x, y, width, height);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

// GL(ES)2

inline void QOpenGLFunctions::glActiveTexture(GLenum texture)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glActiveTexture(texture);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ActiveTexture(texture);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glAttachShader(program, shader);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.AttachShader(program, shader);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char* name)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBindAttribLocation(program, index, name);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BindAttribLocation(program, index, name);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBindBuffer(GLenum target, GLuint buffer)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBindBuffer(target, buffer);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BindBuffer(target, buffer);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBindFramebuffer(GLenum target, GLuint framebuffer)
{
    if (framebuffer == 0)
        framebuffer = QOpenGLContext::currentContext()->defaultFramebufferObject();
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBindFramebuffer(target, framebuffer);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BindFramebuffer(target, framebuffer);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBindRenderbuffer(GLenum target, GLuint renderbuffer)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBindRenderbuffer(target, renderbuffer);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BindRenderbuffer(target, renderbuffer);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBlendColor(red, green, blue, alpha);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BlendColor(red, green, blue, alpha);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBlendEquation(GLenum mode)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBlendEquation(mode);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BlendEquation(mode);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBlendEquationSeparate(modeRGB, modeAlpha);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BlendEquationSeparate(modeRGB, modeAlpha);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBufferData(GLenum target, qopengl_GLsizeiptr size, const void* data, GLenum usage)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBufferData(target, size, data, usage);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BufferData(target, size, data, usage);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void* data)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glBufferSubData(target, offset, size, data);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.BufferSubData(target, offset, size, data);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLenum QOpenGLFunctions::glCheckFramebufferStatus(GLenum target)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLenum result = ::glCheckFramebufferStatus(target);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLenum result = d_ptr->f.CheckFramebufferStatus(target);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLFunctions::glClearDepthf(GLclampf depth)
{
#if defined(QT_OPENGL_ES) && defined(Q_OS_ANDROID)
    ::glClearDepthf(depth);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ClearDepthf(depth);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glCompileShader(GLuint shader)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glCompileShader(shader);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.CompileShader(shader);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void* data)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glCompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.CompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void* data)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.CompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLuint QOpenGLFunctions::glCreateProgram()
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLuint result = ::glCreateProgram();
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLuint result = d_ptr->f.CreateProgram();
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLuint QOpenGLFunctions::glCreateShader(GLenum type)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLuint result = ::glCreateShader(type);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLuint result = d_ptr->f.CreateShader(type);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLFunctions::glDeleteBuffers(GLsizei n, const GLuint* buffers)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDeleteBuffers(n, buffers);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DeleteBuffers(n, buffers);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDeleteFramebuffers(GLsizei n, const GLuint* framebuffers)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDeleteFramebuffers(n, framebuffers);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DeleteFramebuffers(n, framebuffers);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDeleteProgram(GLuint program)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDeleteProgram(program);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DeleteProgram(program);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDeleteRenderbuffers(GLsizei n, const GLuint* renderbuffers)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDeleteRenderbuffers(n, renderbuffers);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DeleteRenderbuffers(n, renderbuffers);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDeleteShader(GLuint shader)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDeleteShader(shader);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DeleteShader(shader);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDepthRangef(GLclampf zNear, GLclampf zFar)
{
#if defined(QT_OPENGL_ES) && defined(Q_OS_ANDROID)
    ::glDepthRangef(zNear, zFar);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DepthRangef(zNear, zFar);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDetachShader(program, shader);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DetachShader(program, shader);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glDisableVertexAttribArray(index);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.DisableVertexAttribArray(index);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glEnableVertexAttribArray(index);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.EnableVertexAttribArray(index);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glFramebufferTexture2D(target, attachment, textarget, texture, level);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.FramebufferTexture2D(target, attachment, textarget, texture, level);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGenBuffers(GLsizei n, GLuint* buffers)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGenBuffers(n, buffers);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GenBuffers(n, buffers);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGenerateMipmap(GLenum target)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGenerateMipmap(target);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GenerateMipmap(target);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGenFramebuffers(GLsizei n, GLuint* framebuffers)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGenFramebuffers(n, framebuffers);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GenFramebuffers(n, framebuffers);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGenRenderbuffers(GLsizei n, GLuint* renderbuffers)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGenRenderbuffers(n, renderbuffers);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GenRenderbuffers(n, renderbuffers);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, char* name)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetActiveAttrib(program, index, bufsize, length, size, type, name);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetActiveAttrib(program, index, bufsize, length, size, type, name);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, char* name)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetActiveUniform(program, index, bufsize, length, size, type, name);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetActiveUniform(program, index, bufsize, length, size, type, name);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei* count, GLuint* shaders)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetAttachedShaders(program, maxcount, count, shaders);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetAttachedShaders(program, maxcount, count, shaders);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char* name)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLint result = ::glGetAttribLocation(program, name);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLint result = d_ptr->f.GetAttribLocation(program, name);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLFunctions::glGetBufferParameteriv(GLenum target, GLenum pname, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetBufferParameteriv(target, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetBufferParameteriv(target, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetFramebufferAttachmentParameteriv(target, attachment, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetFramebufferAttachmentParameteriv(target, attachment, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetProgramiv(GLuint program, GLenum pname, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetProgramiv(program, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetProgramiv(program, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei* length, char* infolog)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetProgramInfoLog(program, bufsize, length, infolog);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetProgramInfoLog(program, bufsize, length, infolog);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetRenderbufferParameteriv(target, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetRenderbufferParameteriv(target, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetShaderiv(GLuint shader, GLenum pname, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetShaderiv(shader, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetShaderiv(shader, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei* length, char* infolog)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetShaderInfoLog(shader, bufsize, length, infolog);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetShaderInfoLog(shader, bufsize, length, infolog);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint* range, GLint* precision)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetShaderPrecisionFormat(shadertype, precisiontype, range, precision);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetShaderPrecisionFormat(shadertype, precisiontype, range, precision);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei* length, char* source)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetShaderSource(shader, bufsize, length, source);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetShaderSource(shader, bufsize, length, source);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, GLfloat* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetUniformfv(program, location, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetUniformfv(program, location, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetUniformiv(program, location, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetUniformiv(program, location, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char* name)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLint result = ::glGetUniformLocation(program, name);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLint result = d_ptr->f.GetUniformLocation(program, name);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetVertexAttribfv(index, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetVertexAttribfv(index, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, GLenum pname, GLint* params)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetVertexAttribiv(index, pname, params);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetVertexAttribiv(index, pname, params);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, GLenum pname, void** pointer)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glGetVertexAttribPointerv(index, pname, pointer);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.GetVertexAttribPointerv(index, pname, pointer);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLboolean QOpenGLFunctions::glIsBuffer(GLuint buffer)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLboolean result = ::glIsBuffer(buffer);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLboolean result = d_ptr->f.IsBuffer(buffer);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLboolean result = ::glIsFramebuffer(framebuffer);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLboolean result = d_ptr->f.IsFramebuffer(framebuffer);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLFunctions::glIsProgram(GLuint program)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLboolean result = ::glIsProgram(program);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLboolean result = d_ptr->f.IsProgram(program);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLboolean result = ::glIsRenderbuffer(renderbuffer);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLboolean result = d_ptr->f.IsRenderbuffer(renderbuffer);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLFunctions::glIsShader(GLuint shader)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    GLboolean result = ::glIsShader(shader);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    GLboolean result = d_ptr->f.IsShader(shader);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLFunctions::glLinkProgram(GLuint program)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glLinkProgram(program);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.LinkProgram(program);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glReleaseShaderCompiler()
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glReleaseShaderCompiler();
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ReleaseShaderCompiler();
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glRenderbufferStorage(target, internalformat, width, height);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.RenderbufferStorage(target, internalformat, width, height);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glSampleCoverage(GLclampf value, GLboolean invert)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glSampleCoverage(value, invert);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.SampleCoverage(value, invert);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint* shaders, GLenum binaryformat, const void* binary, GLint length)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glShaderBinary(n, shaders, binaryformat, binary, length);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ShaderBinary(n, shaders, binaryformat, binary, length);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glShaderSource(GLuint shader, GLsizei count, const char** string, const GLint* length)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glShaderSource(shader, count, string, length);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ShaderSource(shader, count, string, length);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glStencilFuncSeparate(face, func, ref, mask);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.StencilFuncSeparate(face, func, ref, mask);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glStencilMaskSeparate(GLenum face, GLuint mask)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glStencilMaskSeparate(face, mask);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.StencilMaskSeparate(face, mask);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glStencilOpSeparate(face, fail, zfail, zpass);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.StencilOpSeparate(face, fail, zfail, zpass);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform1f(GLint location, GLfloat x)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform1f(location, x);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform1f(location, x);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform1fv(GLint location, GLsizei count, const GLfloat* v)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform1fv(location, count, v);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform1fv(location, count, v);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform1i(GLint location, GLint x)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform1i(location, x);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform1i(location, x);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform1iv(GLint location, GLsizei count, const GLint* v)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform1iv(location, count, v);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform1iv(location, count, v);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform2f(GLint location, GLfloat x, GLfloat y)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform2f(location, x, y);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform2f(location, x, y);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform2fv(GLint location, GLsizei count, const GLfloat* v)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform2fv(location, count, v);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform2fv(location, count, v);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform2i(location, x, y);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform2i(location, x, y);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform2iv(GLint location, GLsizei count, const GLint* v)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform2iv(location, count, v);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform2iv(location, count, v);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform3f(location, x, y, z);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform3f(location, x, y, z);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform3fv(GLint location, GLsizei count, const GLfloat* v)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform3fv(location, count, v);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform3fv(location, count, v);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform3i(location, x, y, z);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform3i(location, x, y, z);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform3iv(GLint location, GLsizei count, const GLint* v)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform3iv(location, count, v);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform3iv(location, count, v);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform4f(location, x, y, z, w);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform4f(location, x, y, z, w);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform4fv(GLint location, GLsizei count, const GLfloat* v)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform4fv(location, count, v);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform4fv(location, count, v);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform4i(location, x, y, z, w);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform4i(location, x, y, z, w);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniform4iv(GLint location, GLsizei count, const GLint* v)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniform4iv(location, count, v);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.Uniform4iv(location, count, v);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniformMatrix2fv(location, count, transpose, value);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.UniformMatrix2fv(location, count, transpose, value);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniformMatrix3fv(location, count, transpose, value);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.UniformMatrix3fv(location, count, transpose, value);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUniformMatrix4fv(location, count, transpose, value);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.UniformMatrix4fv(location, count, transpose, value);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glUseProgram(GLuint program)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glUseProgram(program);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.UseProgram(program);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glValidateProgram(GLuint program)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glValidateProgram(program);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.ValidateProgram(program);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, GLfloat x)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttrib1f(indx, x);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttrib1f(indx, x);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const GLfloat* values)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttrib1fv(indx, values);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttrib1fv(indx, values);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttrib2f(indx, x, y);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttrib2f(indx, x, y);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const GLfloat* values)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttrib2fv(indx, values);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttrib2fv(indx, values);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttrib3f(indx, x, y, z);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttrib3f(indx, x, y, z);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const GLfloat* values)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttrib3fv(indx, values);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttrib3fv(indx, values);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttrib4f(indx, x, y, z, w);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttrib4f(indx, x, y, z, w);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const GLfloat* values)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttrib4fv(indx, values);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttrib4fv(indx, values);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void* ptr)
{
#if defined(QT_OPENGL_ES_2) && defined(Q_OS_ANDROID)
    ::glVertexAttribPointer(indx, size, type, normalized, stride, ptr);
#else
    Q_ASSERT(QOpenGLFunctions::isInitialized(d_ptr));
    d_ptr->f.VertexAttribPointer(indx, size, type, normalized, stride, ptr);
#endif
    Q_OPENGL_FUNCTIONS_DEBUG
}

#undef QT_OPENGL_DECLARE_FUNCTIONS
#undef QT_OPENGL_COUNT_FUNCTIONS
#undef QT_OPENGL_DECLARE

QT_END_NAMESPACE

#endif // QT_NO_OPENGL

#endif
