
#ifndef GRID_H
#define GRID_H

#include <stdlib.h>
#include <stdio.h>
#include <errno.h>

typedef struct cGrid
{
    long rows;
    long cols;
    char *data;
} cgrid;

typedef struct iGrid
{
    long rows;
    long cols;
    int *data;
} igrid;

char get_ccell(cgrid *g, long row, long col)
{
    return g->data[row * g->cols + col];
}

void set_ccell(cgrid *g, long row, long col, char value)
{
    g->data[row * g->cols + col] = value;
}

void display_cgrid(cgrid *g)
{
    for (int i = 0; i < g->rows; i++)
    {
        for (int j = 0; j < g->cols; j++)
        {
            printf("%c", get_ccell(g, i, j));
        }
        printf("\n");
    }
}

int copy_cgrid(cgrid *src, cgrid *dest)
{
    dest->rows = src->rows;
    dest->cols = src->cols;
    dest->data = (char *)malloc(dest->rows * dest->cols * sizeof(char));
    if (!dest->data)
    {
        errno = ENOMEM;
        return ENOMEM;
    }
    for (int i = 0; i < dest->rows; i++)
    {
        for (int j = 0; j < dest->cols; j++)
        {
            set_ccell(dest, i, j, get_ccell(src, i, j));
        }
    }
    return 0;
}

int allocated_cgrid(cgrid *g, long rows, long cols)
{
    g->rows = rows;
    g->cols = cols;
    g->data = (char *)malloc(rows * cols * sizeof(char));
    if (!g->data)
    {
        errno = ENOMEM;
        return ENOMEM;
    }
    return 0;
}

int allocated_and_default_cgrid(cgrid *g, long rows, long cols, char default_value)
{
    g->rows = rows;
    g->cols = cols;
    g->data = (char *)malloc(rows * cols * sizeof(char));
    if (!g->data)
    {
        errno = ENOMEM;
        return ENOMEM;
    }
    for (int i = 0; i < rows; i++)
    {
        for (int j = 0; j < cols; j++)
        {
            set_ccell(g, i, j, default_value);
        }
    }
    return 0;
}

int free_cgrid(cgrid *g)
{
    free(g->data);
    g->cols = 0;
    g->rows = 0;
    return 0;
}

int get_icell(igrid *g, long row, long col)
{
    return g->data[row * g->cols + col];
}

void set_icell(igrid *g, long row, long col, int value)
{
    g->data[row * g->cols + col] = value;
}

void display_igrid(igrid *g)
{
    for (int i = 0; i < g->rows; i++)
    {
        for (int j = 0; j < g->cols; j++)
        {
            printf("%d", get_icell(g, i, j));
        }
        printf("\n");
    }
}

int copy_igrid(igrid *src, igrid *dest)
{
    dest->rows = src->rows;
    dest->cols = src->cols;
    dest->data = (int *)malloc(dest->rows * dest->cols * sizeof(int));
    if (!dest->data)
    {
        errno = ENOMEM;
        return ENOMEM;
    }
    for (int i = 0; i < dest->rows; i++)
    {
        for (int j = 0; j < dest->cols; j++)
        {
            set_icell(dest, i, j, get_icell(src, i, j));
        }
    }
    return 0;
}

int allocated_igrid(igrid *g, long rows, long cols)
{
    g->rows = rows;
    g->cols = cols;
    g->data = (int *)malloc(rows * cols * sizeof(int));
    if (!g->data)
    {
        errno = ENOMEM;
        return ENOMEM;
    }
    return 0;
}

int allocated_and_default_igrid(igrid *g, long rows, long cols, int default_value)
{
    g->rows = rows;
    g->cols = cols;
    g->data = (int *)malloc(rows * cols * sizeof(int));
    if (!g->data)
    {
        errno = ENOMEM;
        return ENOMEM;
    }
    for (int i = 0; i < rows; i++)
    {
        for (int j = 0; j < cols; j++)
        {
            set_icell(g, i, j, default_value);
        }
    }
    return 0;
}

int free_igrid(igrid *g)
{
    free(g->data);
    g->cols = 0;
    g->rows = 0;
    return 0;
}

#endif
