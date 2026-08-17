/*
 * JNI public API functions
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

#ifndef AVCODEC_JNI_H
#define AVCODEC_JNI_H

/*
 * Manually set a Java virtual machine which will be used to retrieve the JNI
 * environment. Once a Java VM is set it cannot be changed afterwards, meaning
 * you can call multiple times av_jni_set_java_vm with the same Java VM pointer
 * however it will error out if you try to set a different Java VM.
 *
 * @param vm Java virtual machine
 * @param log_ctx context used for logging, can be NULL
 * @return 0 on success, < 0 otherwise
 */
int av_jni_set_java_vm(void *vm, void *log_ctx);

/*
 * Get the Java virtual machine which has been set with av_jni_set_java_vm.
 *
 * @param vm Java virtual machine
 * @return a pointer to the Java virtual machine
 */
void *av_jni_get_java_vm(void *log_ctx);

#endif /* AVCODEC_JNI_H */
