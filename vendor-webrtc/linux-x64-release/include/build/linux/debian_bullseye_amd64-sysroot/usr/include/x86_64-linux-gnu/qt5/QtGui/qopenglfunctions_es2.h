/****************************************************************************
**
** Copyright (C) 2013 Klaralvdalens Datakonsult AB (KDAB)
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

#ifndef QOPENGLVERSIONFUNCTIONS_ES2_H
#define QOPENGLVERSIONFUNCTIONS_ES2_H

#include <QtGui/qtguiglobal.h>

#if defined(QT_OPENGL_ES_2) || defined(Q_QDOC)

#include <QtGui/QOpenGLVersionFunctions>
#include <QtGui/qopenglcontext.h>

QT_BEGIN_NAMESPACE

class QOpenGLFunctions_ES2Private;

class Q_GUI_EXPORT QOpenGLFunctions_ES2 : public QAbstractOpenGLFunctions
{
public:
    QOpenGLFunctions_ES2();
    ~QOpenGLFunctions_ES2();

    bool initializeOpenGLFunctions() override;

    // OpenGL ES2 core functions
    void glActiveTexture(GLenum texture);
    void glAttachShader(GLuint program, GLuint shader);
    void glBindAttribLocation(GLuint program, GLuint index, const GLchar* name);
    void glBindBuffer(GLenum target, GLuint buffer);
    void glBindFramebuffer(GLenum target, GLuint framebuffer);
    void glBindRenderbuffer(GLenum target, GLuint renderbuffer);
    void glBindTexture(GLenum target, GLuint texture);
    void glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
    void glBlendEquation(GLenum mode);
    void glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
    void glBlendFunc(GLenum sfactor, GLenum dfactor);
    void glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
    void glBufferData(GLenum target, GLsizeiptr size, const GLvoid* data, GLenum usage);
    void glBufferSubData(GLenum target, GLintptr offset, GLsizeiptr size, const GLvoid* data);
    GLenum glCheckFramebufferStatus(GLenum target);
    void glClear(GLbitfield mask);
    void glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
    void glClearDepthf(GLclampf depth);
    void glClearStencil(GLint s);
    void glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
    void glCompileShader(GLuint shader);
    void glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const GLvoid* data);
    void glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const GLvoid* data);
    void glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
    void glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
    GLuint glCreateProgram(void);
    GLuint glCreateShader(GLenum type);
    void glCullFace(GLenum mode);
    void glDeleteBuffers(GLsizei n, const GLuint* buffers);
    void glDeleteFramebuffers(GLsizei n, const GLuint* framebuffers);
    void glDeleteProgram(GLuint program);
    void glDeleteRenderbuffers(GLsizei n, const GLuint* renderbuffers);
    void glDeleteShader(GLuint shader);
    void glDeleteTextures(GLsizei n, const GLuint* textures);
    void glDepthFunc(GLenum func);
    void glDepthMask(GLboolean flag);
    void glDepthRangef(GLclampf zNear, GLclampf zFar);
    void glDetachShader(GLuint program, GLuint shader);
    void glDisable(GLenum cap);
    void glDisableVertexAttribArray(GLuint index);
    void glDrawArrays(GLenum mode, GLint first, GLsizei count);
    void glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid* indices);
    void glEnable(GLenum cap);
    void glEnableVertexAttribArray(GLuint index);
    void glFinish(void);
    void glFlush(void);
    void glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
    void glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
    void glFrontFace(GLenum mode);
    void glGenBuffers(GLsizei n, GLuint* buffers);
    void glGenerateMipmap(GLenum target);
    void glGenFramebuffers(GLsizei n, GLuint* framebuffers);
    void glGenRenderbuffers(GLsizei n, GLuint* renderbuffers);
    void glGenTextures(GLsizei n, GLuint* textures);
    void glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, GLchar* name);
    void glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, GLchar* name);
    void glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei* count, GLuint* shaders);
    int glGetAttribLocation(GLuint program, const GLchar* name);
    void glGetBooleanv(GLenum pname, GLboolean* params);
    void glGetBufferParameteriv(GLenum target, GLenum pname, GLint* params);
    GLenum glGetError(void);
    void glGetFloatv(GLenum pname, GLfloat* params);
    void glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint* params);
    void glGetIntegerv(GLenum pname, GLint* params);
    void glGetProgramiv(GLuint program, GLenum pname, GLint* params);
    void glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei* length, GLchar* infolog);
    void glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint* params);
    void glGetShaderiv(GLuint shader, GLenum pname, GLint* params);
    void glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei* length, GLchar* infolog);
    void glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint* range, GLint* precision);
    void glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei* length, GLchar* source);
    const GLubyte* glGetString(GLenum name);
    void glGetTexParameterfv(GLenum target, GLenum pname, GLfloat* params);
    void glGetTexParameteriv(GLenum target, GLenum pname, GLint* params);
    void glGetUniformfv(GLuint program, GLint location, GLfloat* params);
    void glGetUniformiv(GLuint program, GLint location, GLint* params);
    int glGetUniformLocation(GLuint program, const GLchar* name);
    void glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat* params);
    void glGetVertexAttribiv(GLuint index, GLenum pname, GLint* params);
    void glGetVertexAttribPointerv(GLuint index, GLenum pname, GLvoid** pointer);
    void glHint(GLenum target, GLenum mode);
    GLboolean glIsBuffer(GLuint buffer);
    GLboolean glIsEnabled(GLenum cap);
    GLboolean glIsFramebuffer(GLuint framebuffer);
    GLboolean glIsProgram(GLuint program);
    GLboolean glIsRenderbuffer(GLuint renderbuffer);
    GLboolean glIsShader(GLuint shader);
    GLboolean glIsTexture(GLuint texture);
    void glLineWidth(GLfloat width);
    void glLinkProgram(GLuint program);
    void glPixelStorei(GLenum pname, GLint param);
    void glPolygonOffset(GLfloat factor, GLfloat units);
    void glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid* pixels);
    void glReleaseShaderCompiler(void);
    void glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
    void glSampleCoverage(GLclampf value, GLboolean invert);
    void glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
    void glShaderBinary(GLsizei n, const GLuint* shaders, GLenum binaryformat, const GLvoid* binary, GLsizei length);
    void glShaderSource(GLuint shader, GLsizei count, const GLchar* *string, const GLint* length);
    void glStencilFunc(GLenum func, GLint ref, GLuint mask);
    void glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
    void glStencilMask(GLuint mask);
    void glStencilMaskSeparate(GLenum face, GLuint mask);
    void glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
    void glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
    void glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid* pixels);
    void glTexParameterf(GLenum target, GLenum pname, GLfloat param);
    void glTexParameterfv(GLenum target, GLenum pname, const GLfloat* params);
    void glTexParameteri(GLenum target, GLenum pname, GLint param);
    void glTexParameteriv(GLenum target, GLenum pname, const GLint* params);
    void glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid* pixels);
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
    void glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const GLvoid* ptr);
    void glViewport(GLint x, GLint y, GLsizei width, GLsizei height);

private:
    friend class QOpenGLContext;

    static bool isContextCompatible(QOpenGLContext *context);
    static QOpenGLVersionProfile versionProfile();

    // For future expansion - not used
    QOpenGLFunctions_ES2Private* d_es2;
};

// OpenGL ES2 core functions
inline void QOpenGLFunctions_ES2::glActiveTexture(GLenum texture)
{
    ::glActiveTexture(texture);
}

inline void QOpenGLFunctions_ES2::glAttachShader(GLuint program, GLuint shader)
{
    ::glAttachShader(program, shader);
}

inline void QOpenGLFunctions_ES2::glBindAttribLocation(GLuint program, GLuint index, const GLchar* name)
{
    ::glBindAttribLocation(program, index, name);
}

inline void QOpenGLFunctions_ES2::glBindBuffer(GLenum target, GLuint buffer)
{
    ::glBindBuffer(target, buffer);
}

inline void QOpenGLFunctions_ES2::glBindFramebuffer(GLenum target, GLuint framebuffer)
{
    ::glBindFramebuffer(target, framebuffer);
}

inline void QOpenGLFunctions_ES2::glBindRenderbuffer(GLenum target, GLuint renderbuffer)
{
    ::glBindRenderbuffer(target, renderbuffer);
}

inline void QOpenGLFunctions_ES2::glBindTexture(GLenum target, GLuint texture)
{
    ::glBindTexture(target, texture);
}

inline void QOpenGLFunctions_ES2::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha)
{
    ::glBlendColor(red, green, blue, alpha);
}

inline void QOpenGLFunctions_ES2::glBlendEquation(GLenum mode)
{
    ::glBlendEquation(mode);
}

inline void QOpenGLFunctions_ES2::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha)
{
    ::glBlendEquationSeparate(modeRGB, modeAlpha);
}

inline void QOpenGLFunctions_ES2::glBlendFunc(GLenum sfactor, GLenum dfactor)
{
    ::glBlendFunc(sfactor, dfactor);
}

inline void QOpenGLFunctions_ES2::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha)
{
    ::glBlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
}

inline void QOpenGLFunctions_ES2::glBufferData(GLenum target, GLsizeiptr size, const GLvoid* data, GLenum usage)
{
    ::glBufferData(target, size, data, usage);
}

inline void QOpenGLFunctions_ES2::glBufferSubData(GLenum target, GLintptr offset, GLsizeiptr size, const GLvoid* data)
{
    ::glBufferSubData(target, offset, size, data);
}

inline GLenum QOpenGLFunctions_ES2::glCheckFramebufferStatus(GLenum target)
{
    return ::glCheckFramebufferStatus(target);
}

inline void QOpenGLFunctions_ES2::glClear(GLbitfield mask)
{
    ::glClear(mask);
}

inline void QOpenGLFunctions_ES2::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha)
{
    ::glClearColor(red, green, blue, alpha);
}

inline void QOpenGLFunctions_ES2::glClearDepthf(GLclampf depth)
{
    ::glClearDepthf(depth);
}

inline void QOpenGLFunctions_ES2::glClearStencil(GLint s)
{
    ::glClearStencil(s);
}

inline void QOpenGLFunctions_ES2::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha)
{
    ::glColorMask(red, green, blue, alpha);
}

inline void QOpenGLFunctions_ES2::glCompileShader(GLuint shader)
{
    ::glCompileShader(shader);
}

inline void QOpenGLFunctions_ES2::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const GLvoid* data)
{
    ::glCompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data);
}

inline void QOpenGLFunctions_ES2::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const GLvoid* data)
{
    ::glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data);
}

inline void QOpenGLFunctions_ES2::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border)
{
    ::glCopyTexImage2D(target, level, internalformat, x, y, width, height, border);
}

inline void QOpenGLFunctions_ES2::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height)
{
    ::glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
}

inline GLuint QOpenGLFunctions_ES2::glCreateProgram(void)
{
    return ::glCreateProgram();
}

inline GLuint QOpenGLFunctions_ES2::glCreateShader(GLenum type)
{
    return ::glCreateShader(type);
}

inline void QOpenGLFunctions_ES2::glCullFace(GLenum mode)
{
    ::glCullFace(mode);
}

inline void QOpenGLFunctions_ES2::glDeleteBuffers(GLsizei n, const GLuint* buffers)
{
    ::glDeleteBuffers(n, buffers);
}

inline void QOpenGLFunctions_ES2::glDeleteFramebuffers(GLsizei n, const GLuint* framebuffers)
{
    ::glDeleteFramebuffers(n, framebuffers);
}

inline void QOpenGLFunctions_ES2::glDeleteProgram(GLuint program)
{
    ::glDeleteProgram(program);
}

inline void QOpenGLFunctions_ES2::glDeleteRenderbuffers(GLsizei n, const GLuint* renderbuffers)
{
    ::glDeleteRenderbuffers(n, renderbuffers);
}

inline void QOpenGLFunctions_ES2::glDeleteShader(GLuint shader)
{
    ::glDeleteShader(shader);
}

inline void QOpenGLFunctions_ES2::glDeleteTextures(GLsizei n, const GLuint* textures)
{
    ::glDeleteTextures(n, textures);
}

inline void QOpenGLFunctions_ES2::glDepthFunc(GLenum func)
{
    ::glDepthFunc(func);
}

inline void QOpenGLFunctions_ES2::glDepthMask(GLboolean flag)
{
    ::glDepthMask(flag);
}

inline void QOpenGLFunctions_ES2::glDepthRangef(GLclampf zNear, GLclampf zFar)
{
    ::glDepthRangef(zNear, zFar);
}

inline void QOpenGLFunctions_ES2::glDetachShader(GLuint program, GLuint shader)
{
    ::glDetachShader(program, shader);
}

inline void QOpenGLFunctions_ES2::glDisable(GLenum cap)
{
    ::glDisable(cap);
}

inline void QOpenGLFunctions_ES2::glDisableVertexAttribArray(GLuint index)
{
    ::glDisableVertexAttribArray(index);
}

inline void QOpenGLFunctions_ES2::glDrawArrays(GLenum mode, GLint first, GLsizei count)
{
    ::glDrawArrays(mode, first, count);
}

inline void QOpenGLFunctions_ES2::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid* indices)
{
    ::glDrawElements(mode, count, type, indices);
}

inline void QOpenGLFunctions_ES2::glEnable(GLenum cap)
{
    ::glEnable(cap);
}

inline void QOpenGLFunctions_ES2::glEnableVertexAttribArray(GLuint index)
{
    ::glEnableVertexAttribArray(index);
}

inline void QOpenGLFunctions_ES2::glFinish(void)
{
    ::glFinish();
}

inline void QOpenGLFunctions_ES2::glFlush(void)
{
    ::glFlush();
}

inline void QOpenGLFunctions_ES2::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer)
{
    ::glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
}

inline void QOpenGLFunctions_ES2::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level)
{
    ::glFramebufferTexture2D(target, attachment, textarget, texture, level);
}

inline void QOpenGLFunctions_ES2::glFrontFace(GLenum mode)
{
    ::glFrontFace(mode);
}

inline void QOpenGLFunctions_ES2::glGenBuffers(GLsizei n, GLuint* buffers)
{
    ::glGenBuffers(n, buffers);
}

inline void QOpenGLFunctions_ES2::glGenerateMipmap(GLenum target)
{
    ::glGenerateMipmap(target);
}

inline void QOpenGLFunctions_ES2::glGenFramebuffers(GLsizei n, GLuint* framebuffers)
{
    ::glGenFramebuffers(n, framebuffers);
}

inline void QOpenGLFunctions_ES2::glGenRenderbuffers(GLsizei n, GLuint* renderbuffers)
{
    ::glGenRenderbuffers(n, renderbuffers);
}

inline void QOpenGLFunctions_ES2::glGenTextures(GLsizei n, GLuint* textures)
{
    ::glGenTextures(n, textures);
}

inline void QOpenGLFunctions_ES2::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, GLchar* name)
{
    ::glGetActiveAttrib(program, index, bufsize, length, size, type, name);
}

inline void QOpenGLFunctions_ES2::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei* length, GLint* size, GLenum* type, GLchar* name)
{
    ::glGetActiveUniform(program, index, bufsize, length, size, type, name);
}

inline void QOpenGLFunctions_ES2::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei* count, GLuint* shaders)
{
    ::glGetAttachedShaders(program, maxcount, count, shaders);
}

inline int QOpenGLFunctions_ES2::glGetAttribLocation(GLuint program, const GLchar* name)
{
    return ::glGetAttribLocation(program, name);
}

inline void QOpenGLFunctions_ES2::glGetBooleanv(GLenum pname, GLboolean* params)
{
    ::glGetBooleanv(pname, params);
}

inline void QOpenGLFunctions_ES2::glGetBufferParameteriv(GLenum target, GLenum pname, GLint* params)
{
    ::glGetBufferParameteriv(target, pname, params);
}

inline GLenum QOpenGLFunctions_ES2::glGetError(void)
{
    return ::glGetError();
}

inline void QOpenGLFunctions_ES2::glGetFloatv(GLenum pname, GLfloat* params)
{
    ::glGetFloatv(pname, params);
}

inline void QOpenGLFunctions_ES2::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint* params)
{
    ::glGetFramebufferAttachmentParameteriv(target, attachment, pname, params);
}

inline void QOpenGLFunctions_ES2::glGetIntegerv(GLenum pname, GLint* params)
{
    ::glGetIntegerv(pname, params);
}

inline void QOpenGLFunctions_ES2::glGetProgramiv(GLuint program, GLenum pname, GLint* params)
{
    ::glGetProgramiv(program, pname, params);
}

inline void QOpenGLFunctions_ES2::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei* length, GLchar* infolog)
{
    ::glGetProgramInfoLog(program, bufsize, length, infolog);
}

inline void QOpenGLFunctions_ES2::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint* params)
{
    ::glGetRenderbufferParameteriv(target, pname, params);
}

inline void QOpenGLFunctions_ES2::glGetShaderiv(GLuint shader, GLenum pname, GLint* params)
{
    ::glGetShaderiv(shader, pname, params);
}

inline void QOpenGLFunctions_ES2::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei* length, GLchar* infolog)
{
    ::glGetShaderInfoLog(shader, bufsize, length, infolog);
}

inline void QOpenGLFunctions_ES2::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint* range, GLint* precision)
{
    ::glGetShaderPrecisionFormat(shadertype, precisiontype, range, precision);
}

inline void QOpenGLFunctions_ES2::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei* length, GLchar* source)
{
    ::glGetShaderSource(shader, bufsize, length, source);
}

inline const GLubyte* QOpenGLFunctions_ES2::glGetString(GLenum name)
{
    return ::glGetString(name);
}

inline void QOpenGLFunctions_ES2::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat* params)
{
    ::glGetTexParameterfv(target, pname, params);
}

inline void QOpenGLFunctions_ES2::glGetTexParameteriv(GLenum target, GLenum pname, GLint* params)
{
    ::glGetTexParameteriv(target, pname, params);
}

inline void QOpenGLFunctions_ES2::glGetUniformfv(GLuint program, GLint location, GLfloat* params)
{
    ::glGetUniformfv(program, location, params);
}

inline void QOpenGLFunctions_ES2::glGetUniformiv(GLuint program, GLint location, GLint* params)
{
    ::glGetUniformiv(program, location, params);
}

inline int QOpenGLFunctions_ES2::glGetUniformLocation(GLuint program, const GLchar* name)
{
    return ::glGetUniformLocation(program, name);
}

inline void QOpenGLFunctions_ES2::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat* params)
{
    ::glGetVertexAttribfv(index, pname, params);
}

inline void QOpenGLFunctions_ES2::glGetVertexAttribiv(GLuint index, GLenum pname, GLint* params)
{
    ::glGetVertexAttribiv(index, pname, params);
}

inline void QOpenGLFunctions_ES2::glGetVertexAttribPointerv(GLuint index, GLenum pname, GLvoid** pointer)
{
    ::glGetVertexAttribPointerv(index, pname, pointer);
}

inline void QOpenGLFunctions_ES2::glHint(GLenum target, GLenum mode)
{
    ::glHint(target, mode);
}

inline GLboolean QOpenGLFunctions_ES2::glIsBuffer(GLuint buffer)
{
    return ::glIsBuffer(buffer);
}

inline GLboolean QOpenGLFunctions_ES2::glIsEnabled(GLenum cap)
{
    return ::glIsEnabled(cap);
}

inline GLboolean QOpenGLFunctions_ES2::glIsFramebuffer(GLuint framebuffer)
{
    return ::glIsFramebuffer(framebuffer);
}

inline GLboolean QOpenGLFunctions_ES2::glIsProgram(GLuint program)
{
    return ::glIsProgram(program);
}

inline GLboolean QOpenGLFunctions_ES2::glIsRenderbuffer(GLuint renderbuffer)
{
    return ::glIsRenderbuffer(renderbuffer);
}

inline GLboolean QOpenGLFunctions_ES2::glIsShader(GLuint shader)
{
    return ::glIsShader(shader);
}

inline GLboolean QOpenGLFunctions_ES2::glIsTexture(GLuint texture)
{
    return ::glIsTexture(texture);
}

inline void QOpenGLFunctions_ES2::glLineWidth(GLfloat width)
{
    ::glLineWidth(width);
}

inline void QOpenGLFunctions_ES2::glLinkProgram(GLuint program)
{
    ::glLinkProgram(program);
}

inline void QOpenGLFunctions_ES2::glPixelStorei(GLenum pname, GLint param)
{
    ::glPixelStorei(pname, param);
}

inline void QOpenGLFunctions_ES2::glPolygonOffset(GLfloat factor, GLfloat units)
{
    ::glPolygonOffset(factor, units);
}

inline void QOpenGLFunctions_ES2::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid* pixels)
{
    ::glReadPixels(x, y, width, height, format, type, pixels);
}

inline void QOpenGLFunctions_ES2::glReleaseShaderCompiler(void)
{
    ::glReleaseShaderCompiler();
}

inline void QOpenGLFunctions_ES2::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height)
{
    ::glRenderbufferStorage(target, internalformat, width, height);
}

inline void QOpenGLFunctions_ES2::glSampleCoverage(GLclampf value, GLboolean invert)
{
    ::glSampleCoverage(value, invert);
}

inline void QOpenGLFunctions_ES2::glScissor(GLint x, GLint y, GLsizei width, GLsizei height)
{
    ::glScissor(x, y, width, height);
}

inline void QOpenGLFunctions_ES2::glShaderBinary(GLsizei n, const GLuint* shaders, GLenum binaryformat, const GLvoid* binary, GLsizei length)
{
    ::glShaderBinary(n, shaders, binaryformat, binary, length);
}

inline void QOpenGLFunctions_ES2::glShaderSource(GLuint shader, GLsizei count, const GLchar* *string, const GLint* length)
{
    ::glShaderSource(shader, count, string, length);
}

inline void QOpenGLFunctions_ES2::glStencilFunc(GLenum func, GLint ref, GLuint mask)
{
    ::glStencilFunc(func, ref, mask);
}

inline void QOpenGLFunctions_ES2::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask)
{
    ::glStencilFuncSeparate(face, func, ref, mask);
}

inline void QOpenGLFunctions_ES2::glStencilMask(GLuint mask)
{
    ::glStencilMask(mask);
}

inline void QOpenGLFunctions_ES2::glStencilMaskSeparate(GLenum face, GLuint mask)
{
    ::glStencilMaskSeparate(face, mask);
}

inline void QOpenGLFunctions_ES2::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass)
{
    ::glStencilOp(fail, zfail, zpass);
}

inline void QOpenGLFunctions_ES2::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass)
{
    ::glStencilOpSeparate(face, fail, zfail, zpass);
}

inline void QOpenGLFunctions_ES2::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid* pixels)
{
    ::glTexImage2D(target, level, internalformat, width, height, border, format, type, pixels);
}

inline void QOpenGLFunctions_ES2::glTexParameterf(GLenum target, GLenum pname, GLfloat param)
{
    ::glTexParameterf(target, pname, param);
}

inline void QOpenGLFunctions_ES2::glTexParameterfv(GLenum target, GLenum pname, const GLfloat* params)
{
    ::glTexParameterfv(target, pname, params);
}

inline void QOpenGLFunctions_ES2::glTexParameteri(GLenum target, GLenum pname, GLint param)
{
    ::glTexParameteri(target, pname, param);
}

inline void QOpenGLFunctions_ES2::glTexParameteriv(GLenum target, GLenum pname, const GLint* params)
{
    ::glTexParameteriv(target, pname, params);
}

inline void QOpenGLFunctions_ES2::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid* pixels)
{
    ::glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels);
}

inline void QOpenGLFunctions_ES2::glUniform1f(GLint location, GLfloat x)
{
    ::glUniform1f(location, x);
}

inline void QOpenGLFunctions_ES2::glUniform1fv(GLint location, GLsizei count, const GLfloat* v)
{
    ::glUniform1fv(location, count, v);
}

inline void QOpenGLFunctions_ES2::glUniform1i(GLint location, GLint x)
{
    ::glUniform1i(location, x);
}

inline void QOpenGLFunctions_ES2::glUniform1iv(GLint location, GLsizei count, const GLint* v)
{
    ::glUniform1iv(location, count, v);
}

inline void QOpenGLFunctions_ES2::glUniform2f(GLint location, GLfloat x, GLfloat y)
{
    ::glUniform2f(location, x, y);
}

inline void QOpenGLFunctions_ES2::glUniform2fv(GLint location, GLsizei count, const GLfloat* v)
{
    ::glUniform2fv(location, count, v);
}

inline void QOpenGLFunctions_ES2::glUniform2i(GLint location, GLint x, GLint y)
{
    ::glUniform2i(location, x, y);
}

inline void QOpenGLFunctions_ES2::glUniform2iv(GLint location, GLsizei count, const GLint* v)
{
    ::glUniform2iv(location, count, v);
}

inline void QOpenGLFunctions_ES2::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z)
{
    ::glUniform3f(location, x, y, z);
}

inline void QOpenGLFunctions_ES2::glUniform3fv(GLint location, GLsizei count, const GLfloat* v)
{
    ::glUniform3fv(location, count, v);
}

inline void QOpenGLFunctions_ES2::glUniform3i(GLint location, GLint x, GLint y, GLint z)
{
    ::glUniform3i(location, x, y, z);
}

inline void QOpenGLFunctions_ES2::glUniform3iv(GLint location, GLsizei count, const GLint* v)
{
    ::glUniform3iv(location, count, v);
}

inline void QOpenGLFunctions_ES2::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w)
{
    ::glUniform4f(location, x, y, z, w);
}

inline void QOpenGLFunctions_ES2::glUniform4fv(GLint location, GLsizei count, const GLfloat* v)
{
    ::glUniform4fv(location, count, v);
}

inline void QOpenGLFunctions_ES2::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w)
{
    ::glUniform4i(location, x, y, z, w);
}

inline void QOpenGLFunctions_ES2::glUniform4iv(GLint location, GLsizei count, const GLint* v)
{
    ::glUniform4iv(location, count, v);
}

inline void QOpenGLFunctions_ES2::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)
{
    ::glUniformMatrix2fv(location, count, transpose, value);
}

inline void QOpenGLFunctions_ES2::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)
{
    ::glUniformMatrix3fv(location, count, transpose, value);
}

inline void QOpenGLFunctions_ES2::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat* value)
{
    ::glUniformMatrix4fv(location, count, transpose, value);
}

inline void QOpenGLFunctions_ES2::glUseProgram(GLuint program)
{
    ::glUseProgram(program);
}

inline void QOpenGLFunctions_ES2::glValidateProgram(GLuint program)
{
    ::glValidateProgram(program);
}

inline void QOpenGLFunctions_ES2::glVertexAttrib1f(GLuint indx, GLfloat x)
{
    ::glVertexAttrib1f(indx, x);
}

inline void QOpenGLFunctions_ES2::glVertexAttrib1fv(GLuint indx, const GLfloat* values)
{
    ::glVertexAttrib1fv(indx, values);
}

inline void QOpenGLFunctions_ES2::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y)
{
    ::glVertexAttrib2f(indx, x, y);
}

inline void QOpenGLFunctions_ES2::glVertexAttrib2fv(GLuint indx, const GLfloat* values)
{
    ::glVertexAttrib2fv(indx, values);
}

inline void QOpenGLFunctions_ES2::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z)
{
    ::glVertexAttrib3f(indx, x, y, z);
}

inline void QOpenGLFunctions_ES2::glVertexAttrib3fv(GLuint indx, const GLfloat* values)
{
    ::glVertexAttrib3fv(indx, values);
}

inline void QOpenGLFunctions_ES2::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w)
{
    ::glVertexAttrib4f(indx, x, y, z, w);
}

inline void QOpenGLFunctions_ES2::glVertexAttrib4fv(GLuint indx, const GLfloat* values)
{
    ::glVertexAttrib4fv(indx, values);
}

inline void QOpenGLFunctions_ES2::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const GLvoid* ptr)
{
    ::glVertexAttribPointer(indx, size, type, normalized, stride, ptr);
}

inline void QOpenGLFunctions_ES2::glViewport(GLint x, GLint y, GLsizei width, GLsizei height)
{
    ::glViewport(x, y, width, height);
}

QT_END_NAMESPACE

#endif // QT_OPENGL_ES_2

#endif
