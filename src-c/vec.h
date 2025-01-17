
#ifndef VEC_H
#define VEC_H

#include <stdlib.h>
#include <errno.h>
#include <assert.h>
#include <stdio.h>

#define __VEC_TYPE(name, vecval_t) \
    typedef struct vec_##name##_s  \
    {                              \
        size_t size;               \
        size_t capacity;           \
        vecval_t *data;            \
    } vec_##name##_t;

// typedef struct vec_my_vec_s
// {
//     size_t size;
//     size_t capacity;
//     int *data;
// } vec_my_vec_t;

#define __VEC_IMPL(name, SCOPE, vecval_t)                                           \
    SCOPE vec_##name##_t *vec_init_##name(void)                                     \
    {                                                                               \
        return (vec_##name##_t *)calloc(1, sizeof(vec_##name##_t));                 \
    }                                                                               \
    SCOPE vec_##name##_t *vec_init_with_capacity##name(size_t capacity)             \
    {                                                                               \
        vec_##name##_t *v = vec_init_##name();                                      \
        if (!v)                                                                     \
        {                                                                           \
            errno = ENOMEM;                                                         \
            return NULL;                                                            \
        }                                                                           \
        v->capacity = capacity;                                                     \
        v->data = calloc(capacity, sizeof(vecval_t));                               \
        if (!v->data)                                                               \
        {                                                                           \
            free(v);                                                                \
            errno = ENOMEM;                                                         \
            return NULL;                                                            \
        }                                                                           \
        return v;                                                                   \
    }                                                                               \
    SCOPE void vec_free_##name(vec_##name##_t *v)                                   \
    {                                                                               \
        if (v)                                                                      \
        {                                                                           \
            free(v->data);                                                          \
            free(v);                                                                \
        }                                                                           \
    }                                                                               \
    SCOPE int vec_push_##name(vec_##name##_t *v, vecval_t value)                    \
    {                                                                               \
        if (v->capacity == 0)                                                       \
        {                                                                           \
            v->capacity = 16;                                                       \
            v->data = calloc(v->capacity, sizeof(vecval_t));                        \
            if (!v->data)                                                           \
            {                                                                       \
                errno = ENOMEM;                                                     \
                return ENOMEM;                                                      \
            }                                                                       \
        }                                                                           \
                                                                                    \
        if (v->size == v->capacity)                                                 \
        {                                                                           \
                                                                                    \
            size_t new_capacity = v->capacity * 2;                                  \
            vecval_t *new_data = realloc(v->data, new_capacity * sizeof(vecval_t)); \
            if (!new_data)                                                          \
            {                                                                       \
                errno = ENOMEM;                                                     \
                return ENOMEM;                                                      \
            }                                                                       \
            v->data = new_data;                                                     \
            v->capacity = new_capacity;                                             \
        }                                                                           \
        v->data[v->size++] = value;                                                 \
        return 0;                                                                   \
    }                                                                               \
    SCOPE vecval_t vec_pop_##name(vec_##name##_t *v)                                \
    {                                                                               \
        assert(v->size > 0);                                                        \
        return v->data[--v->size];                                                  \
    }                                                                               \
    SCOPE vecval_t vec_get_##name(vec_##name##_t *v, size_t index)                  \
    {                                                                               \
        assert(index < v->size);                                                    \
        return v->data[index];                                                      \
    }                                                                               \
    SCOPE size_t vec_length_##name(vec_##name##_t *v)                               \
    {                                                                               \
        return v->size;                                                             \
    }

#define vec_t(name) vec_##name##_t
#define vec_init(name) vec_init_##name()
#define vec_init_with_capacity(name, capacity) vec_init_with_capacity##name(capacity)
#define vec_free(name, v) vec_free_##name(v)
#define vec_push(name, v, value) vec_push_##name(v, value)
#define vec_pop(name, v) vec_pop_##name(v)
#define vec_get(name, v, index) vec_get_##name(v, index)
#define vec_length(name, v) vec_length_##name(v)

/*! function
    @abstract Iterate over a vector
    @param name: The name of the vector
    @param v: Pointer to the vector
    @param value: Variable to which the value will be assigned
    @param code: Block of code to execute
*/
#define vec_foreach(name, v, value, code)    \
    {                                        \
        for (size_t i = 0; i < v->size; i++) \
        {                                    \
            vecval_t value = v->data[i];     \
            code;                            \
        }                                    \
    }

/*! function
    @abstract Iterate over a vector with index
    @param name: The name of the vector
    @param v: Pointer to the vector
    @param index: Variable to which the index will be assigned
    @param value: Variable to which the value will be assigned
    @param code: Block of code to execute
*/
#define vec_foreach_index(name, v, index, value, code) \
    {                                                  \
        for (size_t __i = 0; __i < v->size; __i++)     \
        {                                              \
            size_t index = __i;                        \
            vecval_t value = v->data[__i];             \
            code;                                      \
        }                                              \
    }

#define VEC_DECLARE(name, vecval_t) \
    __VEC_TYPE(name, vecval_t)      \
    __VEC_IMPL(name, static inline __attribute__((__unused__)), vecval_t)

#endif