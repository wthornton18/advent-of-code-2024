#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <math.h>
#include "common.h"
#include "grid.h"

GRID_DECLARE(c, char)
GRID_DELCARE_INT(antinode)

typedef struct Pos
{
    int x;
    int y;
} pos;

pos sub(pos a, pos b)
{
    pos result = {a.x - b.x, a.y - b.y};
    return result;
}

pos add(pos a, pos b)
{
    pos result = {a.x + b.x, a.y + b.y};
    return result;
}

pos mul(pos a, int scalar)
{
    pos result = {a.x * scalar, a.y * scalar};
    return result;
}

int parse_input(char *buffer, long length, grid_t(c) * grid)
{
    assert(buffer != 0);
    assert(length > 0);

    int rows = 0;
    int cols = 0;
    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            rows++;
        }
    }

    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            break;
        }
        cols++;
    }

    grid->rows = rows;
    grid->cols = cols;
    grid->data = malloc(rows * cols * sizeof(char));
    if (!grid->data)
    {
        errno = ENOMEM;
        return errno;
    }

    for (int i = 0; i < rows; i++)
    {
        for (int j = 0; j < cols; j++)
        {
            grid_set(c, grid, i, j, '.');
        }
    }

    char *line = strtok(buffer, "\n");

    int row = 0;
    while (line)
    {

        for (int i = 0; i < cols; i++)
        {
            grid_set(c, grid, row, i, line[i]);
        }

        line = strtok(NULL, "\n");
        row++;
    }

    return 0;
}
int generate_antinodes(grid_t(antinode) * antinode_grid, pos *valid_positions, int valid_positions_length, int keep_iterating)
{
    for (int i = 0; i < valid_positions_length; i++)
    {
        for (int j = 0; (j < valid_positions_length); j++)
        {
            if (i == j)
            {
                continue;
            }
            pos x_i = valid_positions[i];
            pos x_j = valid_positions[j];
            pos d = sub(x_j, x_i);

            int k = keep_iterating ? 0 : 1;

            do
            {
                pos a = sub(x_i, mul(d, k));

                if (a.x < 0 || a.y < 0 || a.x >= (int)antinode_grid->rows || a.y >= (int)antinode_grid->cols)
                {
                    break;
                }

                grid_set(antinode, antinode_grid, a.x, a.y, 1);
                k++;
            } while (keep_iterating);
        }
    }
    return 0;
}

int count_unique_antinodes(grid_t(c) * grid, int keep_iterating)
{

    grid_t(antinode) *antinode_grid = grid_init_with_dimensions_and_default(antinode, grid->rows, grid->cols, 0);

    int unique_chars[62] = {0};

    for (int i = 0; i < 62; i++)
    {
        unique_chars[i] = 0;
    }

    for (size_t i = 0; i < grid->rows; i++)
    {
        for (size_t j = 0; j < grid->cols; j++)
        {
            char c = grid_get(c, grid, i, j);
            if (c == '.')
            {
                continue;
            }
            if (c >= 'A' && c <= 'Z')
            {
                unique_chars[c - 'A']++;
            }
            else if (c >= 'a' && c <= 'z')
            {
                unique_chars[c - 'a' + 26]++;
            }
            else if (c >= '0' && c <= '9')
            {
                unique_chars[c - '0' + 52]++;
            }
        }
    }

    for (int i = 0; i < 62; i++)
    {
        if (unique_chars[i] == 0)
        {
            continue;
        }

        char c = 0;
        if (i < 26)
        {
            c = 'A' + i;
        }
        else if (i < 52)
        {
            c = 'a' + i - 26;
        }
        else
        {
            c = '0' + i - 52;
        }

        pos *valid_positions = malloc(unique_chars[i] * sizeof(pos));
        if (!valid_positions)
        {
            errno = ENOMEM;
            return errno;
        }

        int valid_positions_length = 0;
        for (size_t j = 0; j < grid->rows; j++)
        {
            for (size_t k = 0; k < grid->cols; k++)
            {
                if (grid_get(c, grid, j, k) == c)
                {
                    pos p = {j, k};
                    valid_positions[valid_positions_length] = p;
                    valid_positions_length++;
                }
            }
        }

        generate_antinodes(antinode_grid, valid_positions, valid_positions_length, keep_iterating);
        free(valid_positions);
    }
    int sum = grid_sum(antinode, antinode_grid);
    grid_free(antinode, antinode_grid);
    return sum;
}

int main(void)
{
    char *buffer = 0;
    long length;

    grid_t(c) signal_grid = {0};

    int err = read_file_to_buffer(&buffer, "data/q8.txt", &length);
    if (err != 0)
    {
        printf("Error reading file: %s\n", strerror(errno));
        return errno;
    }

    int parse_err = parse_input(buffer, length, &signal_grid);
    if (parse_err != 0)
    {
        printf("Error parsing input: %s\n", strerror(errno));
        return errno;
    }
    int unique_antinodes = count_unique_antinodes(&signal_grid, 0);
    printf("Part 1: %d\n", unique_antinodes);
    int unique_antinodes_iterating = count_unique_antinodes(&signal_grid, 1);
    printf("Part 2: %d\n", unique_antinodes_iterating);

    return 0;
}
