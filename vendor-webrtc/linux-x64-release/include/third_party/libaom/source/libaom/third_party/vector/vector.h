/*
The MIT License(MIT)
Copyright(c) 2016 Peter Goldsborough

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions :

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

#ifndef VECTOR_H
#define VECTOR_H

#include <stdbool.h>
#include <stddef.h>

/***** DEFINITIONS *****/

#define VECTOR_MINIMUM_CAPACITY 2
#define VECTOR_GROWTH_FACTOR 2
#define VECTOR_SHRINK_THRESHOLD (1 / 4)

#define VECTOR_ERROR -1
#define VECTOR_SUCCESS 0

#define VECTOR_UNINITIALIZED NULL
#define VECTOR_INITIALIZER \
  { 0, 0, 0, VECTOR_UNINITIALIZED }

/***** STRUCTURES *****/

typedef struct Vector {
  size_t size;
  size_t capacity;
  size_t element_size;

  void *data;
} Vector;

typedef struct Iterator {
  void *pointer;
  size_t element_size;
} Iterator;

/***** METHODS *****/

/* Constructor */
int aom_vector_setup(Vector *vector, size_t capacity, size_t element_size);

#if 0
/* Copy Constructor */
int aom_vector_copy(Vector *destination, Vector *source);

/* Copy Assignment */
int aom_vector_copy_assign(Vector *destination, Vector *source);

/* Move Constructor */
int aom_vector_move(Vector *destination, Vector *source);

/* Move Assignment */
int aom_vector_move_assign(Vector *destination, Vector *source);

int aom_vector_swap(Vector *destination, Vector *source);
#endif  // 0

/* Destructor */
int aom_vector_destroy(Vector *vector);

/* Insertion */
int aom_vector_push_back(Vector *vector, void *element);
#if 0
int aom_vector_push_front(Vector *vector, void *element);
int aom_vector_insert(Vector *vector, size_t index, void *element);
int aom_vector_assign(Vector *vector, size_t index, void *element);
#endif  // 0

#if 0
/* Deletion */
int aom_vector_pop_back(Vector *vector);
int aom_vector_pop_front(Vector *vector);
int aom_vector_erase(Vector *vector, size_t index);
int aom_vector_clear(Vector *vector);

/* Lookup */
void *aom_vector_get(Vector *vector, size_t index);
const void *aom_vector_const_get(const Vector *vector, size_t index);
void *aom_vector_front(Vector *vector);
void *aom_vector_back(Vector *vector);
#define VECTOR_GET_AS(type, aom_vector_pointer, index) \
  *((type *)aom_vector_get((aom_vector_pointer), (index)))
#endif  // 0

/* Information */
bool aom_vector_is_initialized(const Vector *vector);
size_t aom_vector_byte_size(const Vector *vector);
#if 0
size_t aom_vector_free_space(const Vector *vector);
bool aom_vector_is_empty(const Vector *vector);

/* Memory management */
int aom_vector_resize(Vector *vector, size_t new_size);
int aom_vector_reserve(Vector *vector, size_t minimum_capacity);
int aom_vector_shrink_to_fit(Vector *vector);
#endif  // 0

/* Iterators */
Iterator aom_vector_begin(Vector *vector);
#if 0
Iterator aom_vector_end(Vector *vector);
#endif  // 0
Iterator aom_vector_iterator(Vector *vector, size_t index);

void *aom_iterator_get(Iterator *iterator);
#if 0
#define ITERATOR_GET_AS(type, iterator) *((type *)aom_iterator_get((iterator)))

int aom_iterator_erase(Vector *vector, Iterator *iterator);
#endif  // 0

void aom_iterator_increment(Iterator *iterator);
#if 0
void aom_iterator_decrement(Iterator *iterator);

void *aom_iterator_next(Iterator *iterator);
void *aom_iterator_previous(Iterator *iterator);

bool aom_iterator_equals(Iterator *first, Iterator *second);
bool aom_iterator_is_before(Iterator *first, Iterator *second);
bool aom_iterator_is_after(Iterator *first, Iterator *second);

size_t aom_iterator_index(Vector *vector, Iterator *iterator);

#define VECTOR_FOR_EACH(aom_vector_pointer, iterator_name)               \
  for (Iterator(iterator_name) = aom_vector_begin((aom_vector_pointer)), \
      end = aom_vector_end((aom_vector_pointer));                        \
       !aom_iterator_equals(&(iterator_name), &end);                     \
       aom_iterator_increment(&(iterator_name)))
#endif  // 0

#endif /* VECTOR_H */
