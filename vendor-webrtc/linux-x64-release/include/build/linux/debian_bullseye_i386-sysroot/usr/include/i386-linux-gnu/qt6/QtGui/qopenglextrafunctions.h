// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QOPENGLEXTRAFUNCTIONS_H
#define QOPENGLEXTRAFUNCTIONS_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#include <QtGui/qopenglfunctions.h>

// MemoryBarrier is a macro on some architectures on Windows
#ifdef Q_OS_WIN
#pragma push_macro("MemoryBarrier")
#undef MemoryBarrier
#endif

// GLES build without having included gl32.h -> GLDEBUGPROC is still need for the protos, define it here
#if QT_CONFIG(opengles2) && !QT_CONFIG(opengles32)
typedef void (QOPENGLF_APIENTRY  *GLDEBUGPROC)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);
#endif

QT_BEGIN_NAMESPACE

class QOpenGLExtraFunctionsPrivate;

#undef glReadBuffer
#undef glDrawRangeElements
#undef glTexImage3D
#undef glTexSubImage3D
#undef glCopyTexSubImage3D
#undef glCompressedTexImage3D
#undef glCompressedTexSubImage3D
#undef glGenQueries
#undef glDeleteQueries
#undef glIsQuery
#undef glBeginQuery
#undef glEndQuery
#undef glGetQueryiv
#undef glGetQueryObjectuiv
#undef glUnmapBuffer
#undef glGetBufferPointerv
#undef glDrawBuffers
#undef glUniformMatrix2x3fv
#undef glUniformMatrix3x2fv
#undef glUniformMatrix2x4fv
#undef glUniformMatrix4x2fv
#undef glUniformMatrix3x4fv
#undef glUniformMatrix4x3fv
#undef glBlitFramebuffer
#undef glRenderbufferStorageMultisample
#undef glFramebufferTextureLayer
#undef glMapBufferRange
#undef glFlushMappedBufferRange
#undef glBindVertexArray
#undef glDeleteVertexArrays
#undef glGenVertexArrays
#undef glIsVertexArray
#undef glGetIntegeri_v
#undef glBeginTransformFeedback
#undef glEndTransformFeedback
#undef glBindBufferRange
#undef glBindBufferBase
#undef glTransformFeedbackVaryings
#undef glGetTransformFeedbackVarying
#undef glVertexAttribIPointer
#undef glGetVertexAttribIiv
#undef glGetVertexAttribIuiv
#undef glVertexAttribI4i
#undef glVertexAttribI4ui
#undef glVertexAttribI4iv
#undef glVertexAttribI4uiv
#undef glGetUniformuiv
#undef glGetFragDataLocation
#undef glUniform1ui
#undef glUniform2ui
#undef glUniform3ui
#undef glUniform4ui
#undef glUniform1uiv
#undef glUniform2uiv
#undef glUniform3uiv
#undef glUniform4uiv
#undef glClearBufferiv
#undef glClearBufferuiv
#undef glClearBufferfv
#undef glClearBufferfi
#undef glGetStringi
#undef glCopyBufferSubData
#undef glGetUniformIndices
#undef glGetActiveUniformsiv
#undef glGetUniformBlockIndex
#undef glGetActiveUniformBlockiv
#undef glGetActiveUniformBlockName
#undef glUniformBlockBinding
#undef glDrawArraysInstanced
#undef glDrawElementsInstanced
#undef glFenceSync
#undef glIsSync
#undef glDeleteSync
#undef glClientWaitSync
#undef glWaitSync
#undef glGetInteger64v
#undef glGetSynciv
#undef glGetInteger64i_v
#undef glGetBufferParameteri64v
#undef glGenSamplers
#undef glDeleteSamplers
#undef glIsSampler
#undef glBindSampler
#undef glSamplerParameteri
#undef glSamplerParameteriv
#undef glSamplerParameterf
#undef glSamplerParameterfv
#undef glGetSamplerParameteriv
#undef glGetSamplerParameterfv
#undef glVertexAttribDivisor
#undef glBindTransformFeedback
#undef glDeleteTransformFeedbacks
#undef glGenTransformFeedbacks
#undef glIsTransformFeedback
#undef glPauseTransformFeedback
#undef glResumeTransformFeedback
#undef glGetProgramBinary
#undef glProgramBinary
#undef glProgramParameteri
#undef glInvalidateFramebuffer
#undef glInvalidateSubFramebuffer
#undef glTexStorage2D
#undef glTexStorage3D
#undef glGetInternalformativ

#undef glDispatchCompute
#undef glDispatchComputeIndirect
#undef glDrawArraysIndirect
#undef glDrawElementsIndirect
#undef glFramebufferParameteri
#undef glGetFramebufferParameteriv
#undef glGetProgramInterfaceiv
#undef glGetProgramResourceIndex
#undef glGetProgramResourceName
#undef glGetProgramResourceiv
#undef glGetProgramResourceLocation
#undef glUseProgramStages
#undef glActiveShaderProgram
#undef glCreateShaderProgramv
#undef glBindProgramPipeline
#undef glDeleteProgramPipelines
#undef glGenProgramPipelines
#undef glIsProgramPipeline
#undef glGetProgramPipelineiv
#undef glProgramUniform1i
#undef glProgramUniform2i
#undef glProgramUniform3i
#undef glProgramUniform4i
#undef glProgramUniform1ui
#undef glProgramUniform2ui
#undef glProgramUniform3ui
#undef glProgramUniform4ui
#undef glProgramUniform1f
#undef glProgramUniform2f
#undef glProgramUniform3f
#undef glProgramUniform4f
#undef glProgramUniform1iv
#undef glProgramUniform2iv
#undef glProgramUniform3iv
#undef glProgramUniform4iv
#undef glProgramUniform1uiv
#undef glProgramUniform2uiv
#undef glProgramUniform3uiv
#undef glProgramUniform4uiv
#undef glProgramUniform1fv
#undef glProgramUniform2fv
#undef glProgramUniform3fv
#undef glProgramUniform4fv
#undef glProgramUniformMatrix2fv
#undef glProgramUniformMatrix3fv
#undef glProgramUniformMatrix4fv
#undef glProgramUniformMatrix2x3fv
#undef glProgramUniformMatrix3x2fv
#undef glProgramUniformMatrix2x4fv
#undef glProgramUniformMatrix4x2fv
#undef glProgramUniformMatrix3x4fv
#undef glProgramUniformMatrix4x3fv
#undef glValidateProgramPipeline
#undef glGetProgramPipelineInfoLog
#undef glBindImageTexture
#undef glGetBooleani_v
#undef glMemoryBarrier
#undef glMemoryBarrierByRegion
#undef glTexStorage2DMultisample
#undef glGetMultisamplefv
#undef glSampleMaski
#undef glGetTexLevelParameteriv
#undef glGetTexLevelParameterfv
#undef glBindVertexBuffer
#undef glVertexAttribFormat
#undef glVertexAttribIFormat
#undef glVertexAttribBinding
#undef glVertexBindingDivisor

#undef glBlendBarrier
#undef glCopyImageSubData
#undef glDebugMessageControl
#undef glDebugMessageInsert
#undef glDebugMessageCallback
#undef glGetDebugMessageLog
#undef glPushDebugGroup
#undef glPopDebugGroup
#undef glObjectLabel
#undef glGetObjectLabel
#undef glGetObjectPtrLabel
#undef glGetPointerv
#undef glEnablei
#undef glDisablei
#undef glBlendEquationi
#undef glBlendEquationSeparatei
#undef glBlendFunci
#undef glBlendFuncSeparatei
#undef glColorMaski
#undef glIsEnabledi
#undef glDrawElementsBaseVertex
#undef glDrawRangeElementsBaseVertex
#undef glDrawElementsInstancedBaseVertex
#undef glFrameBufferTexture
#undef glPrimitiveBoundingBox
#undef glGetGraphicsResetStatus
#undef glReadnPixels
#undef glGetnUniformfv
#undef glGetnUniformiv
#undef glGetnUniformuiv
#undef glMinSampleShading
#undef glPatchParameteri
#undef glTexParameterIiv
#undef glTexParameterIuiv
#undef glGetTexParameterIiv
#undef glGetTexParameterIuiv
#undef glSamplerParameterIiv
#undef glSamplerParameterIuiv
#undef glGetSamplerParameterIiv
#undef glGetSamplerParameterIuiv
#undef glTexBuffer
#undef glTexBufferRange
#undef glTexStorage3DMultisample

class Q_GUI_EXPORT QOpenGLExtraFunctions : public QOpenGLFunctions
{
    Q_DECLARE_PRIVATE(QOpenGLExtraFunctions)

public:
    QOpenGLExtraFunctions();
    QOpenGLExtraFunctions(QOpenGLContext *context);
    ~QOpenGLExtraFunctions() {}

    // GLES3
    void glReadBuffer(GLenum mode);
    void glDrawRangeElements(GLenum mode, GLuint start, GLuint end, GLsizei count, GLenum type, const void *indices);
    void glTexImage3D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLenum format, GLenum type, const void *pixels);
    void glTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLenum type, const void *pixels);
    void glCopyTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLint x, GLint y, GLsizei width, GLsizei height);
    void glCompressedTexImage3D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLsizei imageSize, const void *data);
    void glCompressedTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLsizei imageSize, const void *data);
    void glGenQueries(GLsizei n, GLuint *ids);
    void glDeleteQueries(GLsizei n, const GLuint *ids);
    GLboolean glIsQuery(GLuint id);
    void glBeginQuery(GLenum target, GLuint id);
    void glEndQuery(GLenum target);
    void glGetQueryiv(GLenum target, GLenum pname, GLint *params);
    void glGetQueryObjectuiv(GLuint id, GLenum pname, GLuint *params);
    GLboolean glUnmapBuffer(GLenum target);
    void glGetBufferPointerv(GLenum target, GLenum pname, void **params);
    void glDrawBuffers(GLsizei n, const GLenum *bufs);
    void glUniformMatrix2x3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glUniformMatrix3x2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glUniformMatrix2x4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glUniformMatrix4x2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glUniformMatrix3x4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glUniformMatrix4x3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glBlitFramebuffer(GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0, GLint dstX1, GLint dstY1, GLbitfield mask, GLenum filter);
    void glRenderbufferStorageMultisample(GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height);
    void glFramebufferTextureLayer(GLenum target, GLenum attachment, GLuint texture, GLint level, GLint layer);
    void *glMapBufferRange(GLenum target, GLintptr offset, GLsizeiptr length, GLbitfield access);
    void glFlushMappedBufferRange(GLenum target, GLintptr offset, GLsizeiptr length);
    void glBindVertexArray(GLuint array);
    void glDeleteVertexArrays(GLsizei n, const GLuint *arrays);
    void glGenVertexArrays(GLsizei n, GLuint *arrays);
    GLboolean glIsVertexArray(GLuint array);
    void glGetIntegeri_v(GLenum target, GLuint index, GLint *data);
    void glBeginTransformFeedback(GLenum primitiveMode);
    void glEndTransformFeedback(void);
    void glBindBufferRange(GLenum target, GLuint index, GLuint buffer, GLintptr offset, GLsizeiptr size);
    void glBindBufferBase(GLenum target, GLuint index, GLuint buffer);
    void glTransformFeedbackVaryings(GLuint program, GLsizei count, const GLchar *const*varyings, GLenum bufferMode);
    void glGetTransformFeedbackVarying(GLuint program, GLuint index, GLsizei bufSize, GLsizei *length, GLsizei *size, GLenum *type, GLchar *name);
    void glVertexAttribIPointer(GLuint index, GLint size, GLenum type, GLsizei stride, const void *pointer);
    void glGetVertexAttribIiv(GLuint index, GLenum pname, GLint *params);
    void glGetVertexAttribIuiv(GLuint index, GLenum pname, GLuint *params);
    void glVertexAttribI4i(GLuint index, GLint x, GLint y, GLint z, GLint w);
    void glVertexAttribI4ui(GLuint index, GLuint x, GLuint y, GLuint z, GLuint w);
    void glVertexAttribI4iv(GLuint index, const GLint *v);
    void glVertexAttribI4uiv(GLuint index, const GLuint *v);
    void glGetUniformuiv(GLuint program, GLint location, GLuint *params);
    GLint glGetFragDataLocation(GLuint program, const GLchar *name);
    void glUniform1ui(GLint location, GLuint v0);
    void glUniform2ui(GLint location, GLuint v0, GLuint v1);
    void glUniform3ui(GLint location, GLuint v0, GLuint v1, GLuint v2);
    void glUniform4ui(GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3);
    void glUniform1uiv(GLint location, GLsizei count, const GLuint *value);
    void glUniform2uiv(GLint location, GLsizei count, const GLuint *value);
    void glUniform3uiv(GLint location, GLsizei count, const GLuint *value);
    void glUniform4uiv(GLint location, GLsizei count, const GLuint *value);
    void glClearBufferiv(GLenum buffer, GLint drawbuffer, const GLint *value);
    void glClearBufferuiv(GLenum buffer, GLint drawbuffer, const GLuint *value);
    void glClearBufferfv(GLenum buffer, GLint drawbuffer, const GLfloat *value);
    void glClearBufferfi(GLenum buffer, GLint drawbuffer, GLfloat depth, GLint stencil);
    const GLubyte *glGetStringi(GLenum name, GLuint index);
    void glCopyBufferSubData(GLenum readTarget, GLenum writeTarget, GLintptr readOffset, GLintptr writeOffset, GLsizeiptr size);
    void glGetUniformIndices(GLuint program, GLsizei uniformCount, const GLchar *const*uniformNames, GLuint *uniformIndices);
    void glGetActiveUniformsiv(GLuint program, GLsizei uniformCount, const GLuint *uniformIndices, GLenum pname, GLint *params);
    GLuint glGetUniformBlockIndex(GLuint program, const GLchar *uniformBlockName);
    void glGetActiveUniformBlockiv(GLuint program, GLuint uniformBlockIndex, GLenum pname, GLint *params);
    void glGetActiveUniformBlockName(GLuint program, GLuint uniformBlockIndex, GLsizei bufSize, GLsizei *length, GLchar *uniformBlockName);
    void glUniformBlockBinding(GLuint program, GLuint uniformBlockIndex, GLuint uniformBlockBinding);
    void glDrawArraysInstanced(GLenum mode, GLint first, GLsizei count, GLsizei instancecount);
    void glDrawElementsInstanced(GLenum mode, GLsizei count, GLenum type, const void *indices, GLsizei instancecount);
    GLsync glFenceSync(GLenum condition, GLbitfield flags);
    GLboolean glIsSync(GLsync sync);
    void glDeleteSync(GLsync sync);
    GLenum glClientWaitSync(GLsync sync, GLbitfield flags, GLuint64 timeout);
    void glWaitSync(GLsync sync, GLbitfield flags, GLuint64 timeout);
    void glGetInteger64v(GLenum pname, GLint64 *data);
    void glGetSynciv(GLsync sync, GLenum pname, GLsizei bufSize, GLsizei *length, GLint *values);
    void glGetInteger64i_v(GLenum target, GLuint index, GLint64 *data);
    void glGetBufferParameteri64v(GLenum target, GLenum pname, GLint64 *params);
    void glGenSamplers(GLsizei count, GLuint *samplers);
    void glDeleteSamplers(GLsizei count, const GLuint *samplers);
    GLboolean glIsSampler(GLuint sampler);
    void glBindSampler(GLuint unit, GLuint sampler);
    void glSamplerParameteri(GLuint sampler, GLenum pname, GLint param);
    void glSamplerParameteriv(GLuint sampler, GLenum pname, const GLint *param);
    void glSamplerParameterf(GLuint sampler, GLenum pname, GLfloat param);
    void glSamplerParameterfv(GLuint sampler, GLenum pname, const GLfloat *param);
    void glGetSamplerParameteriv(GLuint sampler, GLenum pname, GLint *params);
    void glGetSamplerParameterfv(GLuint sampler, GLenum pname, GLfloat *params);
    void glVertexAttribDivisor(GLuint index, GLuint divisor);
    void glBindTransformFeedback(GLenum target, GLuint id);
    void glDeleteTransformFeedbacks(GLsizei n, const GLuint *ids);
    void glGenTransformFeedbacks(GLsizei n, GLuint *ids);
    GLboolean glIsTransformFeedback(GLuint id);
    void glPauseTransformFeedback(void);
    void glResumeTransformFeedback(void);
    void glGetProgramBinary(GLuint program, GLsizei bufSize, GLsizei *length, GLenum *binaryFormat, void *binary);
    void glProgramBinary(GLuint program, GLenum binaryFormat, const void *binary, GLsizei length);
    void glProgramParameteri(GLuint program, GLenum pname, GLint value);
    void glInvalidateFramebuffer(GLenum target, GLsizei numAttachments, const GLenum *attachments);
    void glInvalidateSubFramebuffer(GLenum target, GLsizei numAttachments, const GLenum *attachments, GLint x, GLint y, GLsizei width, GLsizei height);
    void glTexStorage2D(GLenum target, GLsizei levels, GLenum internalformat, GLsizei width, GLsizei height);
    void glTexStorage3D(GLenum target, GLsizei levels, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth);
    void glGetInternalformativ(GLenum target, GLenum internalformat, GLenum pname, GLsizei bufSize, GLint *params);

    // GLES 3.1
    void glDispatchCompute(GLuint num_groups_x, GLuint num_groups_y, GLuint num_groups_z);
    void glDispatchComputeIndirect(GLintptr indirect);
    void glDrawArraysIndirect(GLenum mode, const void *indirect);
    void glDrawElementsIndirect(GLenum mode, GLenum type, const void *indirect);
    void glFramebufferParameteri(GLenum target, GLenum pname, GLint param);
    void glGetFramebufferParameteriv(GLenum target, GLenum pname, GLint *params);
    void glGetProgramInterfaceiv(GLuint program, GLenum programInterface, GLenum pname, GLint *params);
    GLuint glGetProgramResourceIndex(GLuint program, GLenum programInterface, const GLchar *name);
    void glGetProgramResourceName(GLuint program, GLenum programInterface, GLuint index, GLsizei bufSize, GLsizei *length, GLchar *name);
    void glGetProgramResourceiv(GLuint program, GLenum programInterface, GLuint index, GLsizei propCount, const GLenum *props, GLsizei bufSize, GLsizei *length, GLint *params);
    GLint glGetProgramResourceLocation(GLuint program, GLenum programInterface, const GLchar *name);
    void glUseProgramStages(GLuint pipeline, GLbitfield stages, GLuint program);
    void glActiveShaderProgram(GLuint pipeline, GLuint program);
    GLuint glCreateShaderProgramv(GLenum type, GLsizei count, const GLchar *const*strings);
    void glBindProgramPipeline(GLuint pipeline);
    void glDeleteProgramPipelines(GLsizei n, const GLuint *pipelines);
    void glGenProgramPipelines(GLsizei n, GLuint *pipelines);
    GLboolean glIsProgramPipeline(GLuint pipeline);
    void glGetProgramPipelineiv(GLuint pipeline, GLenum pname, GLint *params);
    void glProgramUniform1i(GLuint program, GLint location, GLint v0);
    void glProgramUniform2i(GLuint program, GLint location, GLint v0, GLint v1);
    void glProgramUniform3i(GLuint program, GLint location, GLint v0, GLint v1, GLint v2);
    void glProgramUniform4i(GLuint program, GLint location, GLint v0, GLint v1, GLint v2, GLint v3);
    void glProgramUniform1ui(GLuint program, GLint location, GLuint v0);
    void glProgramUniform2ui(GLuint program, GLint location, GLuint v0, GLuint v1);
    void glProgramUniform3ui(GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2);
    void glProgramUniform4ui(GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3);
    void glProgramUniform1f(GLuint program, GLint location, GLfloat v0);
    void glProgramUniform2f(GLuint program, GLint location, GLfloat v0, GLfloat v1);
    void glProgramUniform3f(GLuint program, GLint location, GLfloat v0, GLfloat v1, GLfloat v2);
    void glProgramUniform4f(GLuint program, GLint location, GLfloat v0, GLfloat v1, GLfloat v2, GLfloat v3);
    void glProgramUniform1iv(GLuint program, GLint location, GLsizei count, const GLint *value);
    void glProgramUniform2iv(GLuint program, GLint location, GLsizei count, const GLint *value);
    void glProgramUniform3iv(GLuint program, GLint location, GLsizei count, const GLint *value);
    void glProgramUniform4iv(GLuint program, GLint location, GLsizei count, const GLint *value);
    void glProgramUniform1uiv(GLuint program, GLint location, GLsizei count, const GLuint *value);
    void glProgramUniform2uiv(GLuint program, GLint location, GLsizei count, const GLuint *value);
    void glProgramUniform3uiv(GLuint program, GLint location, GLsizei count, const GLuint *value);
    void glProgramUniform4uiv(GLuint program, GLint location, GLsizei count, const GLuint *value);
    void glProgramUniform1fv(GLuint program, GLint location, GLsizei count, const GLfloat *value);
    void glProgramUniform2fv(GLuint program, GLint location, GLsizei count, const GLfloat *value);
    void glProgramUniform3fv(GLuint program, GLint location, GLsizei count, const GLfloat *value);
    void glProgramUniform4fv(GLuint program, GLint location, GLsizei count, const GLfloat *value);
    void glProgramUniformMatrix2fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glProgramUniformMatrix3fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glProgramUniformMatrix4fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glProgramUniformMatrix2x3fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glProgramUniformMatrix3x2fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glProgramUniformMatrix2x4fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glProgramUniformMatrix4x2fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glProgramUniformMatrix3x4fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glProgramUniformMatrix4x3fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void glValidateProgramPipeline(GLuint pipeline);
    void glGetProgramPipelineInfoLog(GLuint pipeline, GLsizei bufSize, GLsizei *length, GLchar *infoLog);
    void glBindImageTexture(GLuint unit, GLuint texture, GLint level, GLboolean layered, GLint layer, GLenum access, GLenum format);
    void glGetBooleani_v(GLenum target, GLuint index, GLboolean *data);
    void glMemoryBarrier(GLbitfield barriers);
    void glMemoryBarrierByRegion(GLbitfield barriers);
    void glTexStorage2DMultisample(GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height, GLboolean fixedsamplelocations);
    void glGetMultisamplefv(GLenum pname, GLuint index, GLfloat *val);
    void glSampleMaski(GLuint maskNumber, GLbitfield mask);
    void glGetTexLevelParameteriv(GLenum target, GLint level, GLenum pname, GLint *params);
    void glGetTexLevelParameterfv(GLenum target, GLint level, GLenum pname, GLfloat *params);
    void glBindVertexBuffer(GLuint bindingindex, GLuint buffer, GLintptr offset, GLsizei stride);
    void glVertexAttribFormat(GLuint attribindex, GLint size, GLenum type, GLboolean normalized, GLuint relativeoffset);
    void glVertexAttribIFormat(GLuint attribindex, GLint size, GLenum type, GLuint relativeoffset);
    void glVertexAttribBinding(GLuint attribindex, GLuint bindingindex);
    void glVertexBindingDivisor(GLuint bindingindex, GLuint divisor);

    // GLES 3.2
    void glBlendBarrier(void);
    void glCopyImageSubData(GLuint srcName, GLenum srcTarget, GLint srcLevel, GLint srcX, GLint srcY, GLint srcZ, GLuint dstName, GLenum dstTarget, GLint dstLevel, GLint dstX, GLint dstY, GLint dstZ, GLsizei srcWidth, GLsizei srcHeight, GLsizei srcDepth);
    void glDebugMessageControl(GLenum source, GLenum type, GLenum severity, GLsizei count, const GLuint *ids, GLboolean enabled);
    void glDebugMessageInsert(GLenum source, GLenum type, GLuint id, GLenum severity, GLsizei length, const GLchar *buf);
    void glDebugMessageCallback(GLDEBUGPROC callback, const void *userParam);
    GLuint glGetDebugMessageLog(GLuint count, GLsizei bufSize, GLenum *sources, GLenum *types, GLuint *ids, GLenum *severities, GLsizei *lengths, GLchar *messageLog);
    void glPushDebugGroup(GLenum source, GLuint id, GLsizei length, const GLchar *message);
    void glPopDebugGroup(void);
    void glObjectLabel(GLenum identifier, GLuint name, GLsizei length, const GLchar *label);
    void glGetObjectLabel(GLenum identifier, GLuint name, GLsizei bufSize, GLsizei *length, GLchar *label);
    void glObjectPtrLabel(const void *ptr, GLsizei length, const GLchar *label);
    void glGetObjectPtrLabel(const void *ptr, GLsizei bufSize, GLsizei *length, GLchar *label);
    void glGetPointerv(GLenum pname, void **params);
    void glEnablei(GLenum target, GLuint index);
    void glDisablei(GLenum target, GLuint index);
    void glBlendEquationi(GLuint buf, GLenum mode);
    void glBlendEquationSeparatei(GLuint buf, GLenum modeRGB, GLenum modeAlpha);
    void glBlendFunci(GLuint buf, GLenum src, GLenum dst);
    void glBlendFuncSeparatei(GLuint buf, GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
    void glColorMaski(GLuint index, GLboolean r, GLboolean g, GLboolean b, GLboolean a);
    GLboolean glIsEnabledi(GLenum target, GLuint index);
    void glDrawElementsBaseVertex(GLenum mode, GLsizei count, GLenum type, const void *indices, GLint basevertex);
    void glDrawRangeElementsBaseVertex(GLenum mode, GLuint start, GLuint end, GLsizei count, GLenum type, const void *indices, GLint basevertex);
    void glDrawElementsInstancedBaseVertex(GLenum mode, GLsizei count, GLenum type, const void *indices, GLsizei instancecount, GLint basevertex);
    void glFramebufferTexture(GLenum target, GLenum attachment, GLuint texture, GLint level);
    void glPrimitiveBoundingBox(GLfloat minX, GLfloat minY, GLfloat minZ, GLfloat minW, GLfloat maxX, GLfloat maxY, GLfloat maxZ, GLfloat maxW);
    GLenum glGetGraphicsResetStatus(void);
    void glReadnPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLsizei bufSize, void *data);
    void glGetnUniformfv(GLuint program, GLint location, GLsizei bufSize, GLfloat *params);
    void glGetnUniformiv(GLuint program, GLint location, GLsizei bufSize, GLint *params);
    void glGetnUniformuiv(GLuint program, GLint location, GLsizei bufSize, GLuint *params);
    void glMinSampleShading(GLfloat value);
    void glPatchParameteri(GLenum pname, GLint value);
    void glTexParameterIiv(GLenum target, GLenum pname, const GLint *params);
    void glTexParameterIuiv(GLenum target, GLenum pname, const GLuint *params);
    void glGetTexParameterIiv(GLenum target, GLenum pname, GLint *params);
    void glGetTexParameterIuiv(GLenum target, GLenum pname, GLuint *params);
    void glSamplerParameterIiv(GLuint sampler, GLenum pname, const GLint *param);
    void glSamplerParameterIuiv(GLuint sampler, GLenum pname, const GLuint *param);
    void glGetSamplerParameterIiv(GLuint sampler, GLenum pname, GLint *params);
    void glGetSamplerParameterIuiv(GLuint sampler, GLenum pname, GLuint *params);
    void glTexBuffer(GLenum target, GLenum internalformat, GLuint buffer);
    void glTexBufferRange(GLenum target, GLenum internalformat, GLuint buffer, GLintptr offset, GLsizeiptr size);
    void glTexStorage3DMultisample(GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth, GLboolean fixedsamplelocations);

private:
    static bool isInitialized(const QOpenGLExtraFunctionsPrivate *d) { return d != nullptr; }
};


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

class QOpenGLExtraFunctionsPrivate : public QOpenGLFunctionsPrivate
{
public:
    QOpenGLExtraFunctionsPrivate(QOpenGLContext *ctx);

    // GLES3
#define QT_OPENGL_EXTRA_FUNCTIONS(F) \
    F(void, ReadBuffer, (GLenum mode)) \
    F(void, DrawRangeElements, (GLenum mode, GLuint start, GLuint end, GLsizei count, GLenum type, const void *indices)) \
    F(void, TexImage3D, (GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLenum format, GLenum type, const void *pixels)) \
    F(void, TexSubImage3D, (GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLenum type, const void *pixels)) \
    F(void, CopyTexSubImage3D, (GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLint x, GLint y, GLsizei width, GLsizei height)) \
    F(void, CompressedTexImage3D, (GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLsizei imageSize, const void *data)) \
    F(void, CompressedTexSubImage3D, (GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLsizei imageSize, const void *data)) \
    F(void, GenQueries, (GLsizei n, GLuint *ids)) \
    F(void, DeleteQueries, (GLsizei n, const GLuint *ids)) \
    F(GLboolean, IsQuery, (GLuint id)) \
    F(void, BeginQuery, (GLenum target, GLuint id)) \
    F(void, EndQuery, (GLenum target)) \
    F(void, GetQueryiv, (GLenum target, GLenum pname, GLint *params)) \
    F(void, GetQueryObjectuiv, (GLuint id, GLenum pname, GLuint *params)) \
    F(GLboolean, UnmapBuffer, (GLenum target)) \
    F(void, GetBufferPointerv, (GLenum target, GLenum pname, void **params)) \
    F(void, DrawBuffers, (GLsizei n, const GLenum *bufs)) \
    F(void, UniformMatrix2x3fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, UniformMatrix3x2fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, UniformMatrix2x4fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, UniformMatrix4x2fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, UniformMatrix3x4fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, UniformMatrix4x3fv, (GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, BlitFramebuffer, (GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0, GLint dstX1, GLint dstY1, GLbitfield mask, GLenum filter)) \
    F(void, RenderbufferStorageMultisample, (GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height)) \
    F(void, FramebufferTextureLayer, (GLenum target, GLenum attachment, GLuint texture, GLint level, GLint layer)) \
    F(void *,MapBufferRange, (GLenum target, GLintptr offset, GLsizeiptr length, GLbitfield access)) \
    F(void, FlushMappedBufferRange, (GLenum target, GLintptr offset, GLsizeiptr length)) \
    F(void, BindVertexArray, (GLuint array)) \
    F(void, DeleteVertexArrays, (GLsizei n, const GLuint *arrays)) \
    F(void, GenVertexArrays, (GLsizei n, GLuint *arrays)) \
    F(GLboolean, IsVertexArray, (GLuint array)) \
    F(void, GetIntegeri_v, (GLenum target, GLuint index, GLint *data)) \
    F(void, BeginTransformFeedback, (GLenum primitiveMode)) \
    F(void, EndTransformFeedback, (void)) \
    F(void, BindBufferRange, (GLenum target, GLuint index, GLuint buffer, GLintptr offset, GLsizeiptr size)) \
    F(void, BindBufferBase, (GLenum target, GLuint index, GLuint buffer)) \
    F(void, TransformFeedbackVaryings, (GLuint program, GLsizei count, const GLchar *const*varyings, GLenum bufferMode)) \
    F(void, GetTransformFeedbackVarying, (GLuint program, GLuint index, GLsizei bufSize, GLsizei *length, GLsizei *size, GLenum *type, GLchar *name)) \
    F(void, VertexAttribIPointer, (GLuint index, GLint size, GLenum type, GLsizei stride, const void *pointer)) \
    F(void, GetVertexAttribIiv, (GLuint index, GLenum pname, GLint *params)) \
    F(void, GetVertexAttribIuiv, (GLuint index, GLenum pname, GLuint *params)) \
    F(void, VertexAttribI4i, (GLuint index, GLint x, GLint y, GLint z, GLint w)) \
    F(void, VertexAttribI4ui, (GLuint index, GLuint x, GLuint y, GLuint z, GLuint w)) \
    F(void, VertexAttribI4iv, (GLuint index, const GLint *v)) \
    F(void, VertexAttribI4uiv, (GLuint index, const GLuint *v)) \
    F(void, GetUniformuiv, (GLuint program, GLint location, GLuint *params)) \
    F(GLint, GetFragDataLocation, (GLuint program, const GLchar *name)) \
    F(void, Uniform1ui, (GLint location, GLuint v0)) \
    F(void, Uniform2ui, (GLint location, GLuint v0, GLuint v1)) \
    F(void, Uniform3ui, (GLint location, GLuint v0, GLuint v1, GLuint v2)) \
    F(void, Uniform4ui, (GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3)) \
    F(void, Uniform1uiv, (GLint location, GLsizei count, const GLuint *value)) \
    F(void, Uniform2uiv, (GLint location, GLsizei count, const GLuint *value)) \
    F(void, Uniform3uiv, (GLint location, GLsizei count, const GLuint *value)) \
    F(void, Uniform4uiv, (GLint location, GLsizei count, const GLuint *value)) \
    F(void, ClearBufferiv, (GLenum buffer, GLint drawbuffer, const GLint *value)) \
    F(void, ClearBufferuiv, (GLenum buffer, GLint drawbuffer, const GLuint *value)) \
    F(void, ClearBufferfv, (GLenum buffer, GLint drawbuffer, const GLfloat *value)) \
    F(void, ClearBufferfi, (GLenum buffer, GLint drawbuffer, GLfloat depth, GLint stencil)) \
    F(const GLubyte *, GetStringi, (GLenum name, GLuint index)) \
    F(void, CopyBufferSubData, (GLenum readTarget, GLenum writeTarget, GLintptr readOffset, GLintptr writeOffset, GLsizeiptr size)) \
    F(void, GetUniformIndices, (GLuint program, GLsizei uniformCount, const GLchar *const*uniformNames, GLuint *uniformIndices)) \
    F(void, GetActiveUniformsiv, (GLuint program, GLsizei uniformCount, const GLuint *uniformIndices, GLenum pname, GLint *params)) \
    F(GLuint, GetUniformBlockIndex, (GLuint program, const GLchar *uniformBlockName)) \
    F(void, GetActiveUniformBlockiv, (GLuint program, GLuint uniformBlockIndex, GLenum pname, GLint *params)) \
    F(void, GetActiveUniformBlockName, (GLuint program, GLuint uniformBlockIndex, GLsizei bufSize, GLsizei *length, GLchar *uniformBlockName)) \
    F(void, UniformBlockBinding, (GLuint program, GLuint uniformBlockIndex, GLuint uniformBlockBinding)) \
    F(void, DrawArraysInstanced, (GLenum mode, GLint first, GLsizei count, GLsizei instancecount)) \
    F(void, DrawElementsInstanced, (GLenum mode, GLsizei count, GLenum type, const void *indices, GLsizei instancecount)) \
    F(GLsync, FenceSync, (GLenum condition, GLbitfield flags)) \
    F(GLboolean, IsSync, (GLsync sync)) \
    F(void, DeleteSync, (GLsync sync)) \
    F(GLenum, ClientWaitSync, (GLsync sync, GLbitfield flags, GLuint64 timeout)) \
    F(void, WaitSync, (GLsync sync, GLbitfield flags, GLuint64 timeout)) \
    F(void, GetInteger64v, (GLenum pname, GLint64 *data)) \
    F(void, GetSynciv, (GLsync sync, GLenum pname, GLsizei bufSize, GLsizei *length, GLint *values)) \
    F(void, GetInteger64i_v, (GLenum target, GLuint index, GLint64 *data)) \
    F(void, GetBufferParameteri64v, (GLenum target, GLenum pname, GLint64 *params)) \
    F(void, GenSamplers, (GLsizei count, GLuint *samplers)) \
    F(void, DeleteSamplers, (GLsizei count, const GLuint *samplers)) \
    F(GLboolean, IsSampler, (GLuint sampler)) \
    F(void, BindSampler, (GLuint unit, GLuint sampler)) \
    F(void, SamplerParameteri, (GLuint sampler, GLenum pname, GLint param)) \
    F(void, SamplerParameteriv, (GLuint sampler, GLenum pname, const GLint *param)) \
    F(void, SamplerParameterf, (GLuint sampler, GLenum pname, GLfloat param)) \
    F(void, SamplerParameterfv, (GLuint sampler, GLenum pname, const GLfloat *param)) \
    F(void, GetSamplerParameteriv, (GLuint sampler, GLenum pname, GLint *params)) \
    F(void, GetSamplerParameterfv, (GLuint sampler, GLenum pname, GLfloat *params)) \
    F(void, VertexAttribDivisor, (GLuint index, GLuint divisor)) \
    F(void, BindTransformFeedback, (GLenum target, GLuint id)) \
    F(void, DeleteTransformFeedbacks, (GLsizei n, const GLuint *ids)) \
    F(void, GenTransformFeedbacks, (GLsizei n, GLuint *ids)) \
    F(GLboolean, IsTransformFeedback, (GLuint id)) \
    F(void, PauseTransformFeedback, (void)) \
    F(void, ResumeTransformFeedback, (void)) \
    F(void, GetProgramBinary, (GLuint program, GLsizei bufSize, GLsizei *length, GLenum *binaryFormat, void *binary)) \
    F(void, ProgramBinary, (GLuint program, GLenum binaryFormat, const void *binary, GLsizei length)) \
    F(void, ProgramParameteri, (GLuint program, GLenum pname, GLint value)) \
    F(void, InvalidateFramebuffer, (GLenum target, GLsizei numAttachments, const GLenum *attachments)) \
    F(void, InvalidateSubFramebuffer, (GLenum target, GLsizei numAttachments, const GLenum *attachments, GLint x, GLint y, GLsizei width, GLsizei height)) \
    F(void, TexStorage2D, (GLenum target, GLsizei levels, GLenum internalformat, GLsizei width, GLsizei height)) \
    F(void, TexStorage3D, (GLenum target, GLsizei levels, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth)) \
    F(void, GetInternalformativ, (GLenum target, GLenum internalformat, GLenum pname, GLsizei bufSize, GLint *params)) \
    F(void, DispatchCompute, (GLuint num_groups_x, GLuint num_groups_y, GLuint num_groups_z)) \
    F(void, DispatchComputeIndirect, (GLintptr indirect)) \
    F(void, DrawArraysIndirect, (GLenum mode, const void *indirect)) \
    F(void, DrawElementsIndirect, (GLenum mode, GLenum type, const void *indirect)) \
    F(void, FramebufferParameteri, (GLenum target, GLenum pname, GLint param)) \
    F(void, GetFramebufferParameteriv, (GLenum target, GLenum pname, GLint *params)) \
    F(void, GetProgramInterfaceiv, (GLuint program, GLenum programInterface, GLenum pname, GLint *params)) \
    F(GLuint, GetProgramResourceIndex, (GLuint program, GLenum programInterface, const GLchar *name)) \
    F(void, GetProgramResourceName, (GLuint program, GLenum programInterface, GLuint index, GLsizei bufSize, GLsizei *length, GLchar *name)) \
    F(void, GetProgramResourceiv, (GLuint program, GLenum programInterface, GLuint index, GLsizei propCount, const GLenum *props, GLsizei bufSize, GLsizei *length, GLint *params)) \
    F(GLint, GetProgramResourceLocation, (GLuint program, GLenum programInterface, const GLchar *name)) \
    F(void, UseProgramStages, (GLuint pipeline, GLbitfield stages, GLuint program)) \
    F(void, ActiveShaderProgram, (GLuint pipeline, GLuint program)) \
    F(GLuint, CreateShaderProgramv, (GLenum type, GLsizei count, const GLchar *const*strings)) \
    F(void, BindProgramPipeline, (GLuint pipeline)) \
    F(void, DeleteProgramPipelines, (GLsizei n, const GLuint *pipelines)) \
    F(void, GenProgramPipelines, (GLsizei n, GLuint *pipelines)) \
    F(GLboolean, IsProgramPipeline, (GLuint pipeline)) \
    F(void, GetProgramPipelineiv, (GLuint pipeline, GLenum pname, GLint *params)) \
    F(void, ProgramUniform1i, (GLuint program, GLint location, GLint v0)) \
    F(void, ProgramUniform2i, (GLuint program, GLint location, GLint v0, GLint v1)) \
    F(void, ProgramUniform3i, (GLuint program, GLint location, GLint v0, GLint v1, GLint v2)) \
    F(void, ProgramUniform4i, (GLuint program, GLint location, GLint v0, GLint v1, GLint v2, GLint v3)) \
    F(void, ProgramUniform1ui, (GLuint program, GLint location, GLuint v0)) \
    F(void, ProgramUniform2ui, (GLuint program, GLint location, GLuint v0, GLuint v1)) \
    F(void, ProgramUniform3ui, (GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2)) \
    F(void, ProgramUniform4ui, (GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3)) \
    F(void, ProgramUniform1f, (GLuint program, GLint location, GLfloat v0)) \
    F(void, ProgramUniform2f, (GLuint program, GLint location, GLfloat v0, GLfloat v1)) \
    F(void, ProgramUniform3f, (GLuint program, GLint location, GLfloat v0, GLfloat v1, GLfloat v2)) \
    F(void, ProgramUniform4f, (GLuint program, GLint location, GLfloat v0, GLfloat v1, GLfloat v2, GLfloat v3)) \
    F(void, ProgramUniform1iv, (GLuint program, GLint location, GLsizei count, const GLint *value)) \
    F(void, ProgramUniform2iv, (GLuint program, GLint location, GLsizei count, const GLint *value)) \
    F(void, ProgramUniform3iv, (GLuint program, GLint location, GLsizei count, const GLint *value)) \
    F(void, ProgramUniform4iv, (GLuint program, GLint location, GLsizei count, const GLint *value)) \
    F(void, ProgramUniform1uiv, (GLuint program, GLint location, GLsizei count, const GLuint *value)) \
    F(void, ProgramUniform2uiv, (GLuint program, GLint location, GLsizei count, const GLuint *value)) \
    F(void, ProgramUniform3uiv, (GLuint program, GLint location, GLsizei count, const GLuint *value)) \
    F(void, ProgramUniform4uiv, (GLuint program, GLint location, GLsizei count, const GLuint *value)) \
    F(void, ProgramUniform1fv, (GLuint program, GLint location, GLsizei count, const GLfloat *value)) \
    F(void, ProgramUniform2fv, (GLuint program, GLint location, GLsizei count, const GLfloat *value)) \
    F(void, ProgramUniform3fv, (GLuint program, GLint location, GLsizei count, const GLfloat *value)) \
    F(void, ProgramUniform4fv, (GLuint program, GLint location, GLsizei count, const GLfloat *value)) \
    F(void, ProgramUniformMatrix2fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ProgramUniformMatrix3fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ProgramUniformMatrix4fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ProgramUniformMatrix2x3fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ProgramUniformMatrix3x2fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ProgramUniformMatrix2x4fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ProgramUniformMatrix4x2fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ProgramUniformMatrix3x4fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ProgramUniformMatrix4x3fv, (GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)) \
    F(void, ValidateProgramPipeline, (GLuint pipeline)) \
    F(void, GetProgramPipelineInfoLog, (GLuint pipeline, GLsizei bufSize, GLsizei *length, GLchar *infoLog)) \
    F(void, BindImageTexture, (GLuint unit, GLuint texture, GLint level, GLboolean layered, GLint layer, GLenum access, GLenum format)) \
    F(void, GetBooleani_v, (GLenum target, GLuint index, GLboolean *data)) \
    F(void, MemoryBarrier, (GLbitfield barriers)) \
    F(void, MemoryBarrierByRegion, (GLbitfield barriers)) \
    F(void, TexStorage2DMultisample, (GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height, GLboolean fixedsamplelocations)) \
    F(void, GetMultisamplefv, (GLenum pname, GLuint index, GLfloat *val)) \
    F(void, SampleMaski, (GLuint maskNumber, GLbitfield mask)) \
    F(void, GetTexLevelParameteriv, (GLenum target, GLint level, GLenum pname, GLint *params)) \
    F(void, GetTexLevelParameterfv, (GLenum target, GLint level, GLenum pname, GLfloat *params)) \
    F(void, BindVertexBuffer, (GLuint bindingindex, GLuint buffer, GLintptr offset, GLsizei stride)) \
    F(void, VertexAttribFormat, (GLuint attribindex, GLint size, GLenum type, GLboolean normalized, GLuint relativeoffset)) \
    F(void, VertexAttribIFormat, (GLuint attribindex, GLint size, GLenum type, GLuint relativeoffset)) \
    F(void, VertexAttribBinding, (GLuint attribindex, GLuint bindingindex)) \
    F(void, VertexBindingDivisor, (GLuint bindingindex, GLuint divisor)) \
    F(void, BlendBarrier, (void)) \
    F(void, BlendEquationSeparatei, (GLuint buf, GLenum modeRGB, GLenum modeAlpha)) \
    F(void, BlendEquationi, (GLuint buf, GLenum mode))              \
    F(void, BlendFuncSeparatei, (GLuint buf, GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha)) \
    F(void, BlendFunci, (GLuint buf, GLenum src, GLenum dst)) \
    F(void, ColorMaski, (GLuint index, GLboolean r, GLboolean g, GLboolean b, GLboolean a)) \
    F(void, CopyImageSubData, (GLuint srcName, GLenum srcTarget, GLint srcLevel, GLint srcX, GLint srcY, GLint srcZ, GLuint dstName, GLenum dstTarget, GLint dstLevel, GLint dstX, GLint dstY, GLint dstZ, GLsizei srcWidth, GLsizei srcHeight, GLsizei srcDepth)) \
    F(void, DebugMessageCallback, (GLDEBUGPROC callback, const void * userParam)) \
    F(void, DebugMessageControl, (GLenum source, GLenum type, GLenum severity, GLsizei count, const GLuint * ids, GLboolean enabled)) \
    F(void, DebugMessageInsert, (GLenum source, GLenum type, GLuint id, GLenum severity, GLsizei length, const GLchar * buf)) \
    F(void, Disablei, (GLenum target, GLuint index)) \
    F(void, DrawElementsBaseVertex, (GLenum mode, GLsizei count, GLenum type, const void * indices, GLint basevertex)) \
    F(void, DrawElementsInstancedBaseVertex, (GLenum mode, GLsizei count, GLenum type, const void * indices, GLsizei instancecount, GLint basevertex)) \
    F(void, DrawRangeElementsBaseVertex, (GLenum mode, GLuint start, GLuint end, GLsizei count, GLenum type, const void * indices, GLint basevertex)) \
    F(void, Enablei, (GLenum target, GLuint index)) \
    F(void, FramebufferTexture, (GLenum target, GLenum attachment, GLuint texture, GLint level)) \
    F(GLuint, GetDebugMessageLog, (GLuint count, GLsizei bufSize, GLenum* sources, GLenum* types, GLuint* ids, GLenum* severities, GLsizei* lengths, GLchar* messageLog)) \
    F(GLenum, GetGraphicsResetStatus, (void)) \
    F(void, GetObjectLabel, (GLenum identifier, GLuint name, GLsizei bufSize, GLsizei* length, GLchar* label)) \
    F(void, GetObjectPtrLabel, (const void * ptr, GLsizei bufSize, GLsizei* length, GLchar* label)) \
    F(void, GetPointerv, (GLenum pname, void ** params)) \
    F(void, GetSamplerParameterIiv, (GLuint sampler, GLenum pname, GLint* params)) \
    F(void, GetSamplerParameterIuiv, (GLuint sampler, GLenum pname, GLuint* params)) \
    F(void, GetTexParameterIiv, (GLenum target, GLenum pname, GLint* params)) \
    F(void, GetTexParameterIuiv, (GLenum target, GLenum pname, GLuint* params)) \
    F(void, GetnUniformfv, (GLuint program, GLint location, GLsizei bufSize, GLfloat* params)) \
    F(void, GetnUniformiv, (GLuint program, GLint location, GLsizei bufSize, GLint* params)) \
    F(void, GetnUniformuiv, (GLuint program, GLint location, GLsizei bufSize, GLuint* params)) \
    F(GLboolean, IsEnabledi, (GLenum target, GLuint index)) \
    F(void, MinSampleShading, (GLfloat value)) \
    F(void, ObjectLabel, (GLenum identifier, GLuint name, GLsizei length, const GLchar * label)) \
    F(void, ObjectPtrLabel, (const void * ptr, GLsizei length, const GLchar * label)) \
    F(void, PatchParameteri, (GLenum pname, GLint value)) \
    F(void, PopDebugGroup, (void)) \
    F(void, PrimitiveBoundingBox, (GLfloat minX, GLfloat minY, GLfloat minZ, GLfloat minW, GLfloat maxX, GLfloat maxY, GLfloat maxZ, GLfloat maxW)) \
    F(void, PushDebugGroup, (GLenum source, GLuint id, GLsizei length, const GLchar * message)) \
    F(void, ReadnPixels, (GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLsizei bufSize, void * data)) \
    F(void, SamplerParameterIiv, (GLuint sampler, GLenum pname, const GLint * param)) \
    F(void, SamplerParameterIuiv, (GLuint sampler, GLenum pname, const GLuint * param)) \
    F(void, TexBuffer, (GLenum target, GLenum internalformat, GLuint buffer)) \
    F(void, TexBufferRange, (GLenum target, GLenum internalformat, GLuint buffer, GLintptr offset, GLsizeiptr size)) \
    F(void, TexParameterIiv, (GLenum target, GLenum pname, const GLint * params)) \
    F(void, TexParameterIuiv, (GLenum target, GLenum pname, const GLuint * params)) \
    F(void, TexStorage3DMultisample, (GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth, GLboolean fixedsamplelocations)) \

    QT_OPENGL_DECLARE(QT_OPENGL_EXTRA_FUNCTIONS)
};

// GLES 3.0 and 3.1

inline void QOpenGLExtraFunctions::glBeginQuery(GLenum target, GLuint id)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BeginQuery(target, id);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBeginTransformFeedback(GLenum primitiveMode)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BeginTransformFeedback(primitiveMode);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBindBufferBase(GLenum target, GLuint index, GLuint buffer)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BindBufferBase(target, index, buffer);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBindBufferRange(GLenum target, GLuint index, GLuint buffer, GLintptr offset, GLsizeiptr size)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BindBufferRange(target, index, buffer, offset, size);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBindSampler(GLuint unit, GLuint sampler)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BindSampler(unit, sampler);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBindTransformFeedback(GLenum target, GLuint id)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BindTransformFeedback(target, id);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBindVertexArray(GLuint array)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BindVertexArray(array);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBlitFramebuffer(GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0, GLint dstX1, GLint dstY1, GLbitfield mask, GLenum filter)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glClearBufferfi(GLenum buffer, GLint drawbuffer, GLfloat depth, GLint stencil)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ClearBufferfi(buffer, drawbuffer, depth, stencil);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glClearBufferfv(GLenum buffer, GLint drawbuffer, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ClearBufferfv(buffer, drawbuffer, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glClearBufferiv(GLenum buffer, GLint drawbuffer, const GLint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ClearBufferiv(buffer, drawbuffer, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glClearBufferuiv(GLenum buffer, GLint drawbuffer, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ClearBufferuiv(buffer, drawbuffer, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLenum QOpenGLExtraFunctions::glClientWaitSync(GLsync sync, GLbitfield flags, GLuint64 timeout)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLenum result = d->f.ClientWaitSync(sync, flags, timeout);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glCompressedTexImage3D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLsizei imageSize, const void * data)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.CompressedTexImage3D(target, level, internalformat, width, height, depth, border, imageSize, data);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glCompressedTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLsizei imageSize, const void * data)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.CompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glCopyBufferSubData(GLenum readTarget, GLenum writeTarget, GLintptr readOffset, GLintptr writeOffset, GLsizeiptr size)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.CopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glCopyTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLint x, GLint y, GLsizei width, GLsizei height)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.CopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDeleteQueries(GLsizei n, const GLuint * ids)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DeleteQueries(n, ids);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDeleteSamplers(GLsizei count, const GLuint * samplers)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DeleteSamplers(count, samplers);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDeleteSync(GLsync sync)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DeleteSync(sync);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDeleteTransformFeedbacks(GLsizei n, const GLuint * ids)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DeleteTransformFeedbacks(n, ids);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDeleteVertexArrays(GLsizei n, const GLuint * arrays)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DeleteVertexArrays(n, arrays);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawArraysInstanced(GLenum mode, GLint first, GLsizei count, GLsizei instancecount)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawArraysInstanced(mode, first, count, instancecount);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawBuffers(GLsizei n, const GLenum * bufs)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawBuffers(n, bufs);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawElementsInstanced(GLenum mode, GLsizei count, GLenum type, const void * indices, GLsizei instancecount)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawElementsInstanced(mode, count, type, indices, instancecount);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawRangeElements(GLenum mode, GLuint start, GLuint end, GLsizei count, GLenum type, const void * indices)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawRangeElements(mode, start, end, count, type, indices);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glEndQuery(GLenum target)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.EndQuery(target);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glEndTransformFeedback()
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.EndTransformFeedback();
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLsync QOpenGLExtraFunctions::glFenceSync(GLenum condition, GLbitfield flags)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLsync result = d->f.FenceSync(condition, flags);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glFlushMappedBufferRange(GLenum target, GLintptr offset, GLsizeiptr length)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.FlushMappedBufferRange(target, offset, length);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glFramebufferTextureLayer(GLenum target, GLenum attachment, GLuint texture, GLint level, GLint layer)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.FramebufferTextureLayer(target, attachment, texture, level, layer);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGenQueries(GLsizei n, GLuint* ids)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GenQueries(n, ids);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGenSamplers(GLsizei count, GLuint* samplers)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GenSamplers(count, samplers);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGenTransformFeedbacks(GLsizei n, GLuint* ids)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GenTransformFeedbacks(n, ids);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGenVertexArrays(GLsizei n, GLuint* arrays)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GenVertexArrays(n, arrays);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetActiveUniformBlockName(GLuint program, GLuint uniformBlockIndex, GLsizei bufSize, GLsizei* length, GLchar* uniformBlockName)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetActiveUniformBlockName(program, uniformBlockIndex, bufSize, length, uniformBlockName);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetActiveUniformBlockiv(GLuint program, GLuint uniformBlockIndex, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetActiveUniformBlockiv(program, uniformBlockIndex, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetActiveUniformsiv(GLuint program, GLsizei uniformCount, const GLuint * uniformIndices, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetActiveUniformsiv(program, uniformCount, uniformIndices, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetBufferParameteri64v(GLenum target, GLenum pname, GLint64* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetBufferParameteri64v(target, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetBufferPointerv(GLenum target, GLenum pname, void ** params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetBufferPointerv(target, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLint QOpenGLExtraFunctions::glGetFragDataLocation(GLuint program, const GLchar * name)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLint result = d->f.GetFragDataLocation(program, name);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glGetInteger64i_v(GLenum target, GLuint index, GLint64* data)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetInteger64i_v(target, index, data);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetInteger64v(GLenum pname, GLint64* data)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetInteger64v(pname, data);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetIntegeri_v(GLenum target, GLuint index, GLint* data)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetIntegeri_v(target, index, data);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetInternalformativ(GLenum target, GLenum internalformat, GLenum pname, GLsizei bufSize, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetInternalformativ(target, internalformat, pname, bufSize, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetProgramBinary(GLuint program, GLsizei bufSize, GLsizei* length, GLenum* binaryFormat, void * binary)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetProgramBinary(program, bufSize, length, binaryFormat, binary);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetQueryObjectuiv(GLuint id, GLenum pname, GLuint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetQueryObjectuiv(id, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetQueryiv(GLenum target, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetQueryiv(target, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetSamplerParameterfv(GLuint sampler, GLenum pname, GLfloat* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetSamplerParameterfv(sampler, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetSamplerParameteriv(GLuint sampler, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetSamplerParameteriv(sampler, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline const GLubyte * QOpenGLExtraFunctions::glGetStringi(GLenum name, GLuint index)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    const GLubyte * result = d->f.GetStringi(name, index);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glGetSynciv(GLsync sync, GLenum pname, GLsizei bufSize, GLsizei* length, GLint* values)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetSynciv(sync, pname, bufSize, length, values);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetTransformFeedbackVarying(GLuint program, GLuint index, GLsizei bufSize, GLsizei* length, GLsizei* size, GLenum* type, GLchar* name)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetTransformFeedbackVarying(program, index, bufSize, length, size, type, name);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLuint QOpenGLExtraFunctions::glGetUniformBlockIndex(GLuint program, const GLchar * uniformBlockName)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLuint result = d->f.GetUniformBlockIndex(program, uniformBlockName);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glGetUniformIndices(GLuint program, GLsizei uniformCount, const GLchar *const* uniformNames, GLuint* uniformIndices)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetUniformIndices(program, uniformCount, uniformNames, uniformIndices);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetUniformuiv(GLuint program, GLint location, GLuint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetUniformuiv(program, location, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetVertexAttribIiv(GLuint index, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetVertexAttribIiv(index, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetVertexAttribIuiv(GLuint index, GLenum pname, GLuint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetVertexAttribIuiv(index, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glInvalidateFramebuffer(GLenum target, GLsizei numAttachments, const GLenum * attachments)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.InvalidateFramebuffer(target, numAttachments, attachments);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glInvalidateSubFramebuffer(GLenum target, GLsizei numAttachments, const GLenum * attachments, GLint x, GLint y, GLsizei width, GLsizei height)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.InvalidateSubFramebuffer(target, numAttachments, attachments, x, y, width, height);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLboolean QOpenGLExtraFunctions::glIsQuery(GLuint id)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLboolean result = d->f.IsQuery(id);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLExtraFunctions::glIsSampler(GLuint sampler)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLboolean result = d->f.IsSampler(sampler);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLExtraFunctions::glIsSync(GLsync sync)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLboolean result = d->f.IsSync(sync);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLExtraFunctions::glIsTransformFeedback(GLuint id)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLboolean result = d->f.IsTransformFeedback(id);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLboolean QOpenGLExtraFunctions::glIsVertexArray(GLuint array)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLboolean result = d->f.IsVertexArray(array);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void * QOpenGLExtraFunctions::glMapBufferRange(GLenum target, GLintptr offset, GLsizeiptr length, GLbitfield access)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    void *result = d->f.MapBufferRange(target, offset, length, access);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glPauseTransformFeedback()
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.PauseTransformFeedback();
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramBinary(GLuint program, GLenum binaryFormat, const void * binary, GLsizei length)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramBinary(program, binaryFormat, binary, length);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramParameteri(GLuint program, GLenum pname, GLint value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramParameteri(program, pname, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glReadBuffer(GLenum src)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ReadBuffer(src);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glRenderbufferStorageMultisample(GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.RenderbufferStorageMultisample(target, samples, internalformat, width, height);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glResumeTransformFeedback()
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ResumeTransformFeedback();
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glSamplerParameterf(GLuint sampler, GLenum pname, GLfloat param)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.SamplerParameterf(sampler, pname, param);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glSamplerParameterfv(GLuint sampler, GLenum pname, const GLfloat * param)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.SamplerParameterfv(sampler, pname, param);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glSamplerParameteri(GLuint sampler, GLenum pname, GLint param)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.SamplerParameteri(sampler, pname, param);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glSamplerParameteriv(GLuint sampler, GLenum pname, const GLint * param)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.SamplerParameteriv(sampler, pname, param);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexImage3D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLenum format, GLenum type, const void * pixels)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexImage3D(target, level, internalformat, width, height, depth, border, format, type, pixels);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexStorage2D(GLenum target, GLsizei levels, GLenum internalformat, GLsizei width, GLsizei height)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexStorage2D(target, levels, internalformat, width, height);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexStorage3D(GLenum target, GLsizei levels, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexStorage3D(target, levels, internalformat, width, height, depth);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLenum type, const void * pixels)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTransformFeedbackVaryings(GLuint program, GLsizei count, const GLchar *const* varyings, GLenum bufferMode)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TransformFeedbackVaryings(program, count, varyings, bufferMode);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniform1ui(GLint location, GLuint v0)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Uniform1ui(location, v0);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniform1uiv(GLint location, GLsizei count, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Uniform1uiv(location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniform2ui(GLint location, GLuint v0, GLuint v1)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Uniform2ui(location, v0, v1);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniform2uiv(GLint location, GLsizei count, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Uniform2uiv(location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniform3ui(GLint location, GLuint v0, GLuint v1, GLuint v2)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Uniform3ui(location, v0, v1, v2);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniform3uiv(GLint location, GLsizei count, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Uniform3uiv(location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniform4ui(GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Uniform4ui(location, v0, v1, v2, v3);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniform4uiv(GLint location, GLsizei count, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Uniform4uiv(location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniformBlockBinding(GLuint program, GLuint uniformBlockIndex, GLuint uniformBlockBinding)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.UniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniformMatrix2x3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.UniformMatrix2x3fv(location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniformMatrix2x4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.UniformMatrix2x4fv(location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniformMatrix3x2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.UniformMatrix3x2fv(location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniformMatrix3x4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.UniformMatrix3x4fv(location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniformMatrix4x2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.UniformMatrix4x2fv(location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUniformMatrix4x3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.UniformMatrix4x3fv(location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLboolean QOpenGLExtraFunctions::glUnmapBuffer(GLenum target)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLboolean result = d->f.UnmapBuffer(target);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glVertexAttribDivisor(GLuint index, GLuint divisor)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribDivisor(index, divisor);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexAttribI4i(GLuint index, GLint x, GLint y, GLint z, GLint w)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribI4i(index, x, y, z, w);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexAttribI4iv(GLuint index, const GLint * v)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribI4iv(index, v);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexAttribI4ui(GLuint index, GLuint x, GLuint y, GLuint z, GLuint w)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribI4ui(index, x, y, z, w);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexAttribI4uiv(GLuint index, const GLuint * v)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribI4uiv(index, v);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexAttribIPointer(GLuint index, GLint size, GLenum type, GLsizei stride, const void * pointer)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribIPointer(index, size, type, stride, pointer);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glWaitSync(GLsync sync, GLbitfield flags, GLuint64 timeout)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.WaitSync(sync, flags, timeout);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glActiveShaderProgram(GLuint pipeline, GLuint program)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ActiveShaderProgram(pipeline, program);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBindImageTexture(GLuint unit, GLuint texture, GLint level, GLboolean layered, GLint layer, GLenum access, GLenum format)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BindImageTexture(unit, texture, level, layered, layer, access, format);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBindProgramPipeline(GLuint pipeline)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BindProgramPipeline(pipeline);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBindVertexBuffer(GLuint bindingindex, GLuint buffer, GLintptr offset, GLsizei stride)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BindVertexBuffer(bindingindex, buffer, offset, stride);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLuint QOpenGLExtraFunctions::glCreateShaderProgramv(GLenum type, GLsizei count, const GLchar *const* strings)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLuint result = d->f.CreateShaderProgramv(type, count, strings);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glDeleteProgramPipelines(GLsizei n, const GLuint * pipelines)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DeleteProgramPipelines(n, pipelines);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDispatchCompute(GLuint num_groups_x, GLuint num_groups_y, GLuint num_groups_z)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DispatchCompute(num_groups_x, num_groups_y, num_groups_z);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDispatchComputeIndirect(GLintptr indirect)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DispatchComputeIndirect(indirect);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawArraysIndirect(GLenum mode, const void * indirect)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawArraysIndirect(mode, indirect);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawElementsIndirect(GLenum mode, GLenum type, const void * indirect)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawElementsIndirect(mode, type, indirect);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glFramebufferParameteri(GLenum target, GLenum pname, GLint param)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.FramebufferParameteri(target, pname, param);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGenProgramPipelines(GLsizei n, GLuint* pipelines)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GenProgramPipelines(n, pipelines);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetBooleani_v(GLenum target, GLuint index, GLboolean* data)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetBooleani_v(target, index, data);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetFramebufferParameteriv(GLenum target, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetFramebufferParameteriv(target, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetMultisamplefv(GLenum pname, GLuint index, GLfloat* val)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetMultisamplefv(pname, index, val);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetProgramInterfaceiv(GLuint program, GLenum programInterface, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetProgramInterfaceiv(program, programInterface, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetProgramPipelineInfoLog(GLuint pipeline, GLsizei bufSize, GLsizei* length, GLchar* infoLog)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetProgramPipelineInfoLog(pipeline, bufSize, length, infoLog);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetProgramPipelineiv(GLuint pipeline, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetProgramPipelineiv(pipeline, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLuint QOpenGLExtraFunctions::glGetProgramResourceIndex(GLuint program, GLenum programInterface, const GLchar * name)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLuint result = d->f.GetProgramResourceIndex(program, programInterface, name);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLint QOpenGLExtraFunctions::glGetProgramResourceLocation(GLuint program, GLenum programInterface, const GLchar * name)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLint result = d->f.GetProgramResourceLocation(program, programInterface, name);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glGetProgramResourceName(GLuint program, GLenum programInterface, GLuint index, GLsizei bufSize, GLsizei* length, GLchar* name)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetProgramResourceName(program, programInterface, index, bufSize, length, name);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetProgramResourceiv(GLuint program, GLenum programInterface, GLuint index, GLsizei propCount, const GLenum * props, GLsizei bufSize, GLsizei* length, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetProgramResourceiv(program, programInterface, index, propCount, props, bufSize, length, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetTexLevelParameterfv(GLenum target, GLint level, GLenum pname, GLfloat* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetTexLevelParameterfv(target, level, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetTexLevelParameteriv(GLenum target, GLint level, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetTexLevelParameteriv(target, level, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLboolean QOpenGLExtraFunctions::glIsProgramPipeline(GLuint pipeline)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLboolean result = d->f.IsProgramPipeline(pipeline);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glMemoryBarrier(GLbitfield barriers)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.MemoryBarrier(barriers);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glMemoryBarrierByRegion(GLbitfield barriers)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.MemoryBarrierByRegion(barriers);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform1f(GLuint program, GLint location, GLfloat v0)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform1f(program, location, v0);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform1fv(GLuint program, GLint location, GLsizei count, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform1fv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform1i(GLuint program, GLint location, GLint v0)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform1i(program, location, v0);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform1iv(GLuint program, GLint location, GLsizei count, const GLint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform1iv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform1ui(GLuint program, GLint location, GLuint v0)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform1ui(program, location, v0);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform1uiv(GLuint program, GLint location, GLsizei count, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform1uiv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform2f(GLuint program, GLint location, GLfloat v0, GLfloat v1)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform2f(program, location, v0, v1);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform2fv(GLuint program, GLint location, GLsizei count, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform2fv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform2i(GLuint program, GLint location, GLint v0, GLint v1)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform2i(program, location, v0, v1);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform2iv(GLuint program, GLint location, GLsizei count, const GLint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform2iv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform2ui(GLuint program, GLint location, GLuint v0, GLuint v1)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform2ui(program, location, v0, v1);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform2uiv(GLuint program, GLint location, GLsizei count, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform2uiv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform3f(GLuint program, GLint location, GLfloat v0, GLfloat v1, GLfloat v2)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform3f(program, location, v0, v1, v2);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform3fv(GLuint program, GLint location, GLsizei count, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform3fv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform3i(GLuint program, GLint location, GLint v0, GLint v1, GLint v2)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform3i(program, location, v0, v1, v2);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform3iv(GLuint program, GLint location, GLsizei count, const GLint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform3iv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform3ui(GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform3ui(program, location, v0, v1, v2);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform3uiv(GLuint program, GLint location, GLsizei count, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform3uiv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform4f(GLuint program, GLint location, GLfloat v0, GLfloat v1, GLfloat v2, GLfloat v3)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform4f(program, location, v0, v1, v2, v3);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform4fv(GLuint program, GLint location, GLsizei count, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform4fv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform4i(GLuint program, GLint location, GLint v0, GLint v1, GLint v2, GLint v3)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform4i(program, location, v0, v1, v2, v3);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform4iv(GLuint program, GLint location, GLsizei count, const GLint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform4iv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform4ui(GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform4ui(program, location, v0, v1, v2, v3);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniform4uiv(GLuint program, GLint location, GLsizei count, const GLuint * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniform4uiv(program, location, count, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix2fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix2fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix2x3fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix2x3fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix2x4fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix2x4fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix3fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix3fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix3x2fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix3x2fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix3x4fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix3x4fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix4fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix4fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix4x2fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix4x2fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glProgramUniformMatrix4x3fv(GLuint program, GLint location, GLsizei count, GLboolean transpose, const GLfloat * value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ProgramUniformMatrix4x3fv(program, location, count, transpose, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glSampleMaski(GLuint maskNumber, GLbitfield mask)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.SampleMaski(maskNumber, mask);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexStorage2DMultisample(GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height, GLboolean fixedsamplelocations)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexStorage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glUseProgramStages(GLuint pipeline, GLbitfield stages, GLuint program)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.UseProgramStages(pipeline, stages, program);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glValidateProgramPipeline(GLuint pipeline)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ValidateProgramPipeline(pipeline);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexAttribBinding(GLuint attribindex, GLuint bindingindex)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribBinding(attribindex, bindingindex);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexAttribFormat(GLuint attribindex, GLint size, GLenum type, GLboolean normalized, GLuint relativeoffset)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribFormat(attribindex, size, type, normalized, relativeoffset);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexAttribIFormat(GLuint attribindex, GLint size, GLenum type, GLuint relativeoffset)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexAttribIFormat(attribindex, size, type, relativeoffset);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glVertexBindingDivisor(GLuint bindingindex, GLuint divisor)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.VertexBindingDivisor(bindingindex, divisor);
    Q_OPENGL_FUNCTIONS_DEBUG
}

// GLES 3.2

inline void QOpenGLExtraFunctions::glBlendBarrier()
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BlendBarrier();
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBlendEquationSeparatei(GLuint buf, GLenum modeRGB, GLenum modeAlpha)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BlendEquationSeparatei(buf, modeRGB, modeAlpha);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBlendEquationi(GLuint buf, GLenum mode)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BlendEquationi(buf, mode);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBlendFuncSeparatei(GLuint buf, GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BlendFuncSeparatei(buf, srcRGB, dstRGB, srcAlpha, dstAlpha);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glBlendFunci(GLuint buf, GLenum src, GLenum dst)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.BlendFunci(buf, src, dst);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glColorMaski(GLuint index, GLboolean r, GLboolean g, GLboolean b, GLboolean a)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ColorMaski(index, r, g, b, a);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glCopyImageSubData(GLuint srcName, GLenum srcTarget, GLint srcLevel, GLint srcX, GLint srcY, GLint srcZ, GLuint dstName, GLenum dstTarget, GLint dstLevel, GLint dstX, GLint dstY, GLint dstZ, GLsizei srcWidth, GLsizei srcHeight, GLsizei srcDepth)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.CopyImageSubData(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDebugMessageCallback(GLDEBUGPROC callback, const void * userParam)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DebugMessageCallback(callback, userParam);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDebugMessageControl(GLenum source, GLenum type, GLenum severity, GLsizei count, const GLuint * ids, GLboolean enabled)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DebugMessageControl(source, type, severity, count, ids, enabled);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDebugMessageInsert(GLenum source, GLenum type, GLuint id, GLenum severity, GLsizei length, const GLchar * buf)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DebugMessageInsert(source, type, id, severity, length, buf);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDisablei(GLenum target, GLuint index)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Disablei(target, index);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawElementsBaseVertex(GLenum mode, GLsizei count, GLenum type, const void * indices, GLint basevertex)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawElementsBaseVertex(mode, count, type, indices, basevertex);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawElementsInstancedBaseVertex(GLenum mode, GLsizei count, GLenum type, const void * indices, GLsizei instancecount, GLint basevertex)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawElementsInstancedBaseVertex(mode, count, type, indices, instancecount, basevertex);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glDrawRangeElementsBaseVertex(GLenum mode, GLuint start, GLuint end, GLsizei count, GLenum type, const void * indices, GLint basevertex)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.DrawRangeElementsBaseVertex(mode, start, end, count, type, indices, basevertex);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glEnablei(GLenum target, GLuint index)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.Enablei(target, index);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glFramebufferTexture(GLenum target, GLenum attachment, GLuint texture, GLint level)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.FramebufferTexture(target, attachment, texture, level);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLuint QOpenGLExtraFunctions::glGetDebugMessageLog(GLuint count, GLsizei bufSize, GLenum* sources, GLenum* types, GLuint* ids, GLenum* severities, GLsizei* lengths, GLchar* messageLog)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLuint result = d->f.GetDebugMessageLog(count, bufSize, sources, types, ids, severities, lengths, messageLog);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline GLenum QOpenGLExtraFunctions::glGetGraphicsResetStatus()
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLenum result = d->f.GetGraphicsResetStatus();
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glGetObjectLabel(GLenum identifier, GLuint name, GLsizei bufSize, GLsizei* length, GLchar* label)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetObjectLabel(identifier, name, bufSize, length, label);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetObjectPtrLabel(const void * ptr, GLsizei bufSize, GLsizei* length, GLchar* label)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetObjectPtrLabel(ptr, bufSize, length, label);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetPointerv(GLenum pname, void ** params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetPointerv(pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetSamplerParameterIiv(GLuint sampler, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetSamplerParameterIiv(sampler, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetSamplerParameterIuiv(GLuint sampler, GLenum pname, GLuint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetSamplerParameterIuiv(sampler, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetTexParameterIiv(GLenum target, GLenum pname, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetTexParameterIiv(target, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetTexParameterIuiv(GLenum target, GLenum pname, GLuint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetTexParameterIuiv(target, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetnUniformfv(GLuint program, GLint location, GLsizei bufSize, GLfloat* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetnUniformfv(program, location, bufSize, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetnUniformiv(GLuint program, GLint location, GLsizei bufSize, GLint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetnUniformiv(program, location, bufSize, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glGetnUniformuiv(GLuint program, GLint location, GLsizei bufSize, GLuint* params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.GetnUniformuiv(program, location, bufSize, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline GLboolean QOpenGLExtraFunctions::glIsEnabledi(GLenum target, GLuint index)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    GLboolean result = d->f.IsEnabledi(target, index);
    Q_OPENGL_FUNCTIONS_DEBUG
    return result;
}

inline void QOpenGLExtraFunctions::glMinSampleShading(GLfloat value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.MinSampleShading(value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glObjectLabel(GLenum identifier, GLuint name, GLsizei length, const GLchar * label)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ObjectLabel(identifier, name, length, label);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glObjectPtrLabel(const void * ptr, GLsizei length, const GLchar * label)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ObjectPtrLabel(ptr, length, label);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glPatchParameteri(GLenum pname, GLint value)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.PatchParameteri(pname, value);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glPopDebugGroup()
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.PopDebugGroup();
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glPrimitiveBoundingBox(GLfloat minX, GLfloat minY, GLfloat minZ, GLfloat minW, GLfloat maxX, GLfloat maxY, GLfloat maxZ, GLfloat maxW)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.PrimitiveBoundingBox(minX, minY, minZ, minW, maxX, maxY, maxZ, maxW);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glPushDebugGroup(GLenum source, GLuint id, GLsizei length, const GLchar * message)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.PushDebugGroup(source, id, length, message);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glReadnPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLsizei bufSize, void * data)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.ReadnPixels(x, y, width, height, format, type, bufSize, data);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glSamplerParameterIiv(GLuint sampler, GLenum pname, const GLint * param)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.SamplerParameterIiv(sampler, pname, param);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glSamplerParameterIuiv(GLuint sampler, GLenum pname, const GLuint * param)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.SamplerParameterIuiv(sampler, pname, param);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexBuffer(GLenum target, GLenum internalformat, GLuint buffer)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexBuffer(target, internalformat, buffer);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexBufferRange(GLenum target, GLenum internalformat, GLuint buffer, GLintptr offset, GLsizeiptr size)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexBufferRange(target, internalformat, buffer, offset, size);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexParameterIiv(GLenum target, GLenum pname, const GLint * params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexParameterIiv(target, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexParameterIuiv(GLenum target, GLenum pname, const GLuint * params)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexParameterIuiv(target, pname, params);
    Q_OPENGL_FUNCTIONS_DEBUG
}

inline void QOpenGLExtraFunctions::glTexStorage3DMultisample(GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth, GLboolean fixedsamplelocations)
{
    Q_D(QOpenGLExtraFunctions);
    Q_ASSERT(QOpenGLExtraFunctions::isInitialized(d));
    d->f.TexStorage3DMultisample(target, samples, internalformat, width, height, depth, fixedsamplelocations);
    Q_OPENGL_FUNCTIONS_DEBUG
}

QT_END_NAMESPACE

#undef QT_OPENGL_DECLARE_FUNCTIONS
#undef QT_OPENGL_COUNT_FUNCTIONS
#undef QT_OPENGL_DECLARE

#ifdef Q_OS_WIN
#pragma pop_macro("MemoryBarrier")
#endif

#endif // QT_NO_OPENGL

#endif
