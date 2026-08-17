/*
 * JNI utility functions
 *
 * Copyright (c) 2015-2016 Matthieu Bouron <matthieu.bouron stupeflix.com>
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_FFJNI_H
#define AVCODEC_FFJNI_H

#include <jni.h>
#include <stddef.h>

/*
 * Attach permanently a JNI environment to the current thread and retrieve it.
 *
 * If successfully attached, the JNI environment will automatically be detached
 * at thread destruction.
 *
 * @param attached pointer to an integer that will be set to 1 if the
 * environment has been attached to the current thread or 0 if it is
 * already attached.
 * @param log_ctx context used for logging, can be NULL
 * @return the JNI environment on success, NULL otherwise
 */
JNIEnv *ff_jni_get_env(void *log_ctx);

/*
 * Convert a jstring to its utf characters equivalent.
 *
 * @param env JNI environment
 * @param string Java string to convert
 * @param log_ctx context used for logging, can be NULL
 * @return a pointer to an array of unicode characters on success, NULL
 * otherwise
 */
char *ff_jni_jstring_to_utf_chars(JNIEnv *env, jstring string, void *log_ctx);

/*
 * Convert utf chars to its jstring equivalent.
 *
 * @param env JNI environment
 * @param utf_chars a pointer to an array of unicode characters
 * @param log_ctx context used for logging, can be NULL
 * @return a Java string object on success, NULL otherwise
 */
jstring ff_jni_utf_chars_to_jstring(JNIEnv *env, const char *utf_chars, void *log_ctx);

/*
 * Extract the error summary from a jthrowable in the form of "className: errorMessage"
 *
 * @param env JNI environment
 * @param exception exception to get the summary from
 * @param error address pointing to the error, the value is updated if a
 * summary can be extracted
 * @param log_ctx context used for logging, can be NULL
 * @return 0 on success, < 0 otherwise
 */
int ff_jni_exception_get_summary(JNIEnv *env, jthrowable exception, char **error, void *log_ctx);

/*
 * Check if an exception has occurred,log it using av_log and clear it.
 *
 * @param env JNI environment
 * @param log value used to enable logging if an exception has occurred,
 * 0 disables logging, != 0 enables logging
 * @param log_ctx context used for logging, can be NULL
 */
int ff_jni_exception_check(JNIEnv *env, int log, void *log_ctx);

/*
 * Jni field type.
 */
enum FFJniFieldType {

    FF_JNI_CLASS,
    FF_JNI_FIELD,
    FF_JNI_STATIC_FIELD,
    FF_JNI_METHOD,
    FF_JNI_STATIC_METHOD

};

/*
 * Jni field describing a class, a field or a method to be retrieved using
 * the ff_jni_init_jfields method.
 */
struct FFJniField {

    const char *name;
    const char *method;
    const char *signature;
    enum FFJniFieldType type;
    size_t offset;
    int mandatory;

};

/*
 * Retrieve class references, field ids and method ids to an arbitrary structure.
 *
 * @param env JNI environment
 * @param jfields a pointer to an arbitrary structure where the different
 * fields are declared and where the FFJNIField mapping table offsets are
 * pointing to
 * @param jfields_mapping null terminated array of FFJNIFields describing
 * the class/field/method to be retrieved
 * @param global make the classes references global. It is the caller
 * responsibility to properly release global references.
 * @param log_ctx context used for logging, can be NULL
 * @return 0 on success, < 0 otherwise
 */
int ff_jni_init_jfields(JNIEnv *env, void *jfields, const struct FFJniField *jfields_mapping, int global, void *log_ctx);

/*
 * Delete class references, field ids and method ids of an arbitrary structure.
 *
 * @param env JNI environment
 * @param jfields a pointer to an arbitrary structure where the different
 * fields are declared and where the FFJNIField mapping table offsets are
 * pointing to
 * @param jfields_mapping null terminated array of FFJNIFields describing
 * the class/field/method to be deleted
 * @param global threat the classes references as global and delete them
 * accordingly
 * @param log_ctx context used for logging, can be NULL
 * @return 0 on success, < 0 otherwise
 */
int ff_jni_reset_jfields(JNIEnv *env, void *jfields, const struct FFJniField *jfields_mapping, int global, void *log_ctx);

#endif /* AVCODEC_FFJNI_H */
