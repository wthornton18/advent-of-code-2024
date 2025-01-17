#ifndef GRID_H
#define GRID_H

#include <stdlib.h>
#include <errno.h>
#include <assert.h>
#include <stdio.h>

#define __GRID_TYPE(name, gridval_t) \
    typedef struct grid_##name##_s   \
    {                                \
        size_t rows;                 \
        size_t cols;                 \
        gridval_t *data;             \
    } grid_##name##_t;

#define __GRID_IMPL(name, SCOPE, gridval_t)                                                                       \
    SCOPE grid_##name##_t *grid_init_##name(void)                                                                 \
    {                                                                                                             \
        return (grid_##name##_t *)calloc(1, sizeof(grid_##name##_t));                                             \
    }                                                                                                             \
    SCOPE grid_##name##_t *grid_init_with_dimensions_##name(size_t rows, size_t cols)                             \
    {                                                                                                             \
        grid_##name##_t *g = grid_init_##name();                                                                  \
        if (!g)                                                                                                   \
        {                                                                                                         \
            errno = ENOMEM;                                                                                       \
            return NULL;                                                                                          \
        }                                                                                                         \
        g->rows = rows;                                                                                           \
        g->cols = cols;                                                                                           \
        g->data = (gridval_t *)calloc(rows * cols, sizeof(gridval_t));                                            \
        if (!g->data)                                                                                             \
        {                                                                                                         \
            free(g);                                                                                              \
            errno = ENOMEM;                                                                                       \
            return NULL;                                                                                          \
        }                                                                                                         \
        return g;                                                                                                 \
    }                                                                                                             \
    SCOPE gridval_t grid_get_##name(grid_##name##_t *g, size_t row, size_t col)                                   \
    {                                                                                                             \
        return g->data[row * g->cols + col];                                                                      \
    }                                                                                                             \
    SCOPE void grid_set_##name(grid_##name##_t *g, size_t row, size_t col, gridval_t value)                       \
    {                                                                                                             \
        g->data[row * g->cols + col] = value;                                                                     \
    }                                                                                                             \
    SCOPE grid_##name##_t *grid_init_with_dimensions_and_default_##name(size_t rows, size_t cols, gridval_t dval) \
    {                                                                                                             \
        grid_##name##_t *g = grid_init_with_dimensions_##name(rows, cols);                                        \
        if (!g)                                                                                                   \
        {                                                                                                         \
            return NULL;                                                                                          \
        }                                                                                                         \
        for (size_t i = 0; i < rows; i++)                                                                         \
        {                                                                                                         \
            for (size_t j = 0; j < cols; j++)                                                                     \
            {                                                                                                     \
                grid_set_##name(g, i, j, dval);                                                                   \
            }                                                                                                     \
        }                                                                                                         \
        return g;                                                                                                 \
    }                                                                                                             \
    SCOPE void grid_free_##name(grid_##name##_t *g)                                                               \
    {                                                                                                             \
        if (g)                                                                                                    \
        {                                                                                                         \
            free(g->data);                                                                                        \
            free(g);                                                                                              \
        }                                                                                                         \
    }                                                                                                             \
    SCOPE int grid_copy_##name(grid_##name##_t *src, grid_##name##_t *dest)                                       \
    {                                                                                                             \
        dest->rows = src->rows;                                                                                   \
        dest->cols = src->cols;                                                                                   \
        dest->data = (gridval_t *)malloc(dest->rows * dest->cols * sizeof(gridval_t));                            \
        if (!dest->data)                                                                                          \
        {                                                                                                         \
            errno = ENOMEM;                                                                                       \
            return ENOMEM;                                                                                        \
        }                                                                                                         \
        for (size_t i = 0; i < dest->rows; i++)                                                                   \
        {                                                                                                         \
            for (size_t j = 0; j < dest->cols; j++)                                                               \
            {                                                                                                     \
                grid_set_##name(dest, i, j, grid_get_##name(src, i, j));                                          \
            }                                                                                                     \
        }                                                                                                         \
        return 0;                                                                                                 \
    }                                                                                                             \
    SCOPE int grid_push##name(grid_##name##_t *g, gridval_t *vals, size_t length)                                 \
    {                                                                                                             \
        assert(length == g->cols);                                                                                \
        g->data = (gridval_t *)realloc(g->data, (g->rows + 1) * g->cols * sizeof(gridval_t));                     \
        if (!g->data)                                                                                             \
        {                                                                                                         \
            errno = ENOMEM;                                                                                       \
            return ENOMEM;                                                                                        \
        }                                                                                                         \
        for (size_t i = 0; i < length; i++)                                                                       \
        {                                                                                                         \
            grid_set_##name(g, g->rows, i, vals[i]);                                                              \
        }                                                                                                         \
        g->rows++;                                                                                                \
                                                                                                                  \
        return 0;                                                                                                 \
    }

#define __GRID_INT_IMPL(name, SCOPE)              \
    SCOPE int grid_sum_##name(grid_##name##_t *g) \
    {                                             \
        int sum = 0;                              \
        for (size_t i = 0; i < g->rows; i++)      \
        {                                         \
            for (size_t j = 0; j < g->cols; j++)  \
            {                                     \
                sum += grid_get_##name(g, i, j);  \
            }                                     \
        }                                         \
        return sum;                               \
    }

#define grid_t(name) grid_##name##_t
#define grid_init(name) grid_init_##name()
#define grid_init_with_dimensions(name, rows, cols) grid_init_with_dimensions_##name(rows, cols)
#define grid_init_with_dimensions_and_default(name, rows, cols, dval) grid_init_with_dimensions_and_default_##name(rows, cols, dval)
#define grid_free(name, g) grid_free_##name(g)
#define grid_get(name, g, row, col) grid_get_##name(g, row, col)
#define grid_set(name, g, row, col, value) grid_set_##name(g, row, col, value)
#define grid_copy(name, src, dest) grid_copy_##name(src, dest)
#define grid_cols(name, g) g->cols
#define grid_rows(name, g) g->rows
#define grid_sum(name, g) grid_sum_##name(g)

/*! function
    @abstract  Iterate over a grid
    @param     name: The name of the grid
    @param     g: Pointer to the grid
    @param     value: The value at the current cell (set by the macro)
    @param     code: The code to execute at each cell
*/
#define grid_foreach(name, g, value, code)    \
    for (size_t i = 0; i < g->rows; i++)      \
    {                                         \
        for (size_t j = 0; j < g->cols; j++)  \
        {                                     \
            value = grid_get_##name(g, i, j); \
            code;                             \
        }                                     \
    }

/*! function
    @abstract  Enumerate over a grid
    @param     name: The name of the grid
    @param     g: Pointer to the grid
    @param     value: The value at the current cell (set by the macro)
    @param     i: The row index (set by the macro)
    @param     j: The column index (set by the macro)
    @param     code: The code to execute at each cell
*/
#define grid_enumerate(name, g, value, i, j, code) \
    for (size_t __i = 0; __i < g->rows; __i++)     \
    {                                              \
        for (size_t __j = 0; __j < g->cols; __j++) \
        {                                          \
            value = grid_get_##name(g, __i, __j);  \
            i = __i;                               \
            j = __j;                               \
            code;                                  \
        }                                          \
    }

#define GRID_DECLARE(name, gridval_t) \
    __GRID_TYPE(name, gridval_t)      \
    __GRID_IMPL(name, static inline __attribute__((__unused__)), gridval_t)

#define GRID_DELCARE_INT(NAME)                                        \
    __GRID_TYPE(NAME, int)                                            \
    __GRID_IMPL(NAME, static inline __attribute__((__unused__)), int) \
    __GRID_INT_IMPL(NAME, static inline __attribute__((__unused__)))

#endif
