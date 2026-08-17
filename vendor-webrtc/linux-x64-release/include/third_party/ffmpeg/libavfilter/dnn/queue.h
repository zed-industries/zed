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

#include <stddef.h>

#ifndef AVFILTER_DNN_QUEUE_H
#define AVFILTER_DNN_QUEUE_H

/**
 * Linear double-ended data structure
 */
typedef struct Queue Queue;

/**
 * @brief Create a Queue instance.
 * It initializes the length of the Queue as 0.
 *
 * @return Pointer to the Queue
 * @retval NULL if allocation fails
 */
Queue *ff_queue_create(void);

/**
 * @brief Destroy the Queue instance.
 * It also frees all elements of the Queue.
 */
void ff_queue_destroy(Queue *q);

/**
 * @brief Return the length of the Queue
 */
size_t ff_queue_size(Queue *q);

/**
 * @brief Return a pointer to the data
 * at the head of the queue.
 *
 * @retval NULL if null pointer was passed
 * or queue is empty
 */
void *ff_queue_peek_front(Queue *q);

/**
 * @brief Return a pointer to the data at
 * the tail of the queue.
 *
 * @retval NULL if null pointer was passed
 * or queue is empty
 */
void *ff_queue_peek_back(Queue *q);

/**
 * @brief Add data to the head of the queue.
 * It increases the length of Queue by one.
 *
 * @param q pointer to the Queue.
 * @param v data to be added
 * @return The length of the Queue
 * @retval 0 if the pointer to queue is NULL
 * @retval -1 if new entry cannot be created
 */
int ff_queue_push_front(Queue *q, void *v);

/**
 * @brief Add data to the tail of the queue.
 * It increases the length of Queue by one.
 *
 * @param q pointer to the Queue
 * @param v data to be added
 * @return The length of the Queue
 * @retval 0 if the pointer to queue is NULL
 * @retval -1 if new entry cannot be created
 */
int ff_queue_push_back(Queue *q, void *v);

/**
 * @brief Remove and free first element from
 * the Queue. It shrinks the length of Queue
 * by one.
 *
 * @param q pointer to the Queue.
 * @return The value of first element as void.
 * If a null pointer or empty queue is passed,
 * it returns NULL
 */
void *ff_queue_pop_front(Queue *q);

/**
 * @brief Remove and free last element from
 * the Queue. It shrinks the length of Queue
 * by one.
 *
 * @param q pointer to the Queue.
 * @return The value of last element as void.
 * If a null pointer or empty queue is passed,
 * it returns NULL
 */
void *ff_queue_pop_back(Queue *q);

#endif
