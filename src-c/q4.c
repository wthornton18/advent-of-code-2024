#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "common.h"

const int XMAS_DELTAS[8][3][2] = {
    {{0, 1}, {0, 2}, {0, 3}},
    {{1, 0}, {2, 0}, {3, 0}},
    {{1, 1}, {2, 2}, {3, 3}},
    {{1, -1}, {2, -2}, {3, -3}},
    {{-1, 1}, {-2, 2}, {-3, 3}},
    {{-1, -1}, {-2, -2}, {-3, -3}},
    {{-1, 0}, {-2, 0}, {-3, 0}},
    {{0, -1}, {0, -2}, {0, -3}}};

const int X_MAS_DELTAS[2][2][2] = {
    {{-1, 1}, {1, -1}},
    {{1, 1}, {-1, -1}}};

const char XMAS[4] = {'X', 'M', 'A', 'S'};

typedef struct Grid
{
    long rows;
    long cols;
    char *data;
} grid;

char get_cell(grid *g, long row, long col)
{
    return g->data[row * g->cols + col];
}

void set_cell(grid *g, long row, long col, char value)
{
    g->data[row * g->cols + col] = value;
}

void display_grid(grid *g)
{
    for (int i = 0; i < g->rows; i++)
    {
        for (int j = 0; j < g->cols; j++)
        {
            printf("%c", get_cell(g, i, j));
        }
        printf("\n");
    }
}

int parse_input(char *buffer, long length, grid *g)
{
    assert(buffer);
    assert(length > 0);

    g->rows = 0;
    g->cols = 0;

    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            break;
        }
        g->cols++;
    }

    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            g->rows++;
        }
    }
    long total_cells = g->rows * g->cols;

    char *data = malloc(total_cells * sizeof(char));
    if (!data)
    {
        return ENOMEM;
    }
    for (int i = 0; i < total_cells; i++)
    {
        data[i] = 'a';
    }

    g->data = data;

    int grid_index = 0;
    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            continue;
        }
        g->data[grid_index] = buffer[i];
        grid_index++;
    }

    return 0;
}

int count_xmas(grid *g, int i, int j)
{
    int count = 0;

    for (int k = 0; k < 8; k++)
    {
        int found = 1;
        for (int l = 0; l < 3; l++)
        {
            int row = i + XMAS_DELTAS[k][l][0];
            int col = j + XMAS_DELTAS[k][l][1];
            if (row < 0 || row >= g->rows || col < 0 || col >= g->cols)
            {
                found = 0;
                break;
            }

            if (get_cell(g, row, col) != XMAS[l + 1])
            {
                found = 0;
                break;
            }
        }
        if (found)
        {
            count++;
        }
    }

    return count;
}

int is_valid_x_mas(grid *g, int i, int j)
{
    int valid_x_mas = 1;

    for (int k = 0; k < 2; k++)
    {
        int characters_found = 0;
        for (int l = 0; l < 2; l++)
        {
            int row = i + X_MAS_DELTAS[k][l][0];
            int col = j + X_MAS_DELTAS[k][l][1];
            if (row < 0 || row >= g->rows || col < 0 || col >= g->cols)
            {
                valid_x_mas = 0;
                break;
            }

            char c = get_cell(g, row, col);
            if (c == 'M')
            {
                characters_found += 1;
            }
            else if (c == 'S')
            {
                characters_found += 2;
            }
        }
        if (characters_found != 3)
        {
            valid_x_mas = 0;
            break;
        }
    }

    return valid_x_mas;
}

int count_total_xmas(grid *g)
{
    int count = 0;
    for (int i = 0; i < g->rows; i++)
    {
        for (int j = 0; j < g->cols; j++)
        {
            if (get_cell(g, i, j) == 'X')
            {
                count += count_xmas(g, i, j);
            }
        }
    }
    return count;
}

int count_total_x_mas(grid *g)
{
    int count = 0;
    for (int i = 0; i < g->rows; i++)
    {
        for (int j = 0; j < g->cols; j++)
        {
            if (get_cell(g, i, j) == 'A')
            {
                count += is_valid_x_mas(g, i, j);
            }
        }
    }
    return count;
}

int main(void)
{
    char *buffer = 0;
    long length;
    grid g = {0};

    int read = read_file_to_buffer(&buffer, "data/q4.txt", &length);
    if (read != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return read;
    }

    int parse = parse_input(buffer, length, &g);
    if (parse != 0)
    {
        printf("Error: %s\n", strerror(parse));
        return parse;
    }

    int count_xmas = count_total_xmas(&g);
    printf("Part1: %d\n", count_xmas);
    int count_x_mas = count_total_x_mas(&g);
    printf("Part2: %d\n", count_x_mas);
}
