/*
 * Copyright (c) 2020
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

#ifndef AVFILTER_DNN_SAFE_QUEUE_H
#define AVFILTER_DNN_SAFE_QUEUE_H

/**
 * Double-ended queue with mutex locks ensuring
 * data consistency while multithreading.
 */
typedef struct SafeQueue SafeQueue;

/**
 * @brief Create and initialize a SafeQueue instance.
 *
 * @return Pointer to the SafeQueue
 * @retval NULL if initialization fails
 */
SafeQueue *ff_safe_queue_create(void);

/**
 * @brief Destroy the SafeQueue instance.
 * It also frees all elements of the queue,
 * destroys the mutex and condition variable.
 */
void ff_safe_queue_destroy(SafeQueue *sq);

/**
 * @brief Return the length of the SafeQueue
 */
size_t ff_safe_queue_size(SafeQueue *sq);

/**
 * @brief Add data to the head of queue in the
 * SafeQueue after locking mutex. After adding
 * the data, it signals the condition variable
 * and unlocks the mutex. It increases the length
 * of queue in the SafeQueue by one.
 *
 * @param sq pointer to the SafeQueue
 * @param v data to be added
 * @return The length of the queue
 * @retval 0 if the queue is not initialized
 * @retval -1 if new entry cannot be created
 */
int ff_safe_queue_push_front(SafeQueue *sq, void *v);

/**
 * @brief Add data to the tail of queue in the
 * SafeQueue after locking mutex. After adding
 * the data, it signals the condition variable
 * and unlocks the mutex. It increases the length
 * of queue in the SafeQueue by one.
 *
 * @param sq pointer to the SafeQueue
 * @param v data to be added
 * @return The length of the queue
 * @retval 0 if the queue is not initialized
 * @retval -1 if new entry cannot be created
 */
int ff_safe_queue_push_back(SafeQueue *sq, void *v);

/**
 * @brief Remove and free first element from
 * the queue in SafeQueue. Before removing, it
 * waits for the condition variable to signal and
 * acquires the mutex. Finally, it signals the
 * condition and unlocks the mutex.
 * It shrinks the length of queue in the SafeQueue
 * by one.
 *
 * @param sq pointer to the SafeQueue.
 * @return The value of first element as void.
 * If a null pointer or empty queue is passed,
 * it returns NULL
 */
void *ff_safe_queue_pop_front(SafeQueue *sq);

#endif
