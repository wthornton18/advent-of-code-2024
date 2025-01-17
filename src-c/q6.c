#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdbool.h>
#include "common.h"
#include "grid.h"

GRID_DECLARE(c, char)
GRID_DECLARE(dir, int)

typedef enum Direction
{
    UP = 1 << 0,
    DOWN = 1 << 1,
    LEFT = 1 << 2,
    RIGHT = 1 << 3
} direction;

#define EMPTY_CELL '.'
#define OBSTACLE '#'
#define GUARD '^'
#define TRAVERSED 'X'

int next_position(int x, int y, direction d, int *new_x, int *new_y)
{
    switch (d)
    {
    case UP:
        *new_x = x - 1;
        *new_y = y;
        break;
    case DOWN:
        *new_x = x + 1;
        *new_y = y;
        break;
    case LEFT:
        *new_x = x;
        *new_y = y - 1;
        break;
    case RIGHT:
        *new_x = x;
        *new_y = y + 1;
        break;
    default:
        errno = EINVAL;
        return EINVAL;
    }
    return 0;
}

direction rotate_90(direction d)
{
    switch (d)
    {
    case UP:
        return RIGHT;
    case DOWN:
        return LEFT;
    case LEFT:
        return UP;
    case RIGHT:
        return DOWN;
    default:
        errno = EINVAL;
        return EINVAL;
    }
}

int print_direction(direction d)
{
    switch (d)
    {
    case UP:
        printf("UP\n");
        break;
    case DOWN:
        printf("DOWN\n");
        break;
    case LEFT:
        printf("LEFT\n");
        break;
    case RIGHT:
        printf("RIGHT\n");
        break;
    default:
        errno = EINVAL;
        return EINVAL;
    }
    return 0;
}

int parse_input(char *buffer, long length, grid_t(c) * g, int *guard_x, int *guard_y)
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
        errno = ENOMEM;
        return errno;
    }

    for (int i = 0; i < total_cells; i++)
    {
        data[i] = '.';
    }

    g->data = data;

    int cgrid_col = 0;
    int cgrid_row = 0;

    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            cgrid_row++;
            cgrid_col = 0;
            continue;
        }
        if (buffer[i] == GUARD)
        {
            *guard_x = cgrid_row;
            *guard_y = cgrid_col;
            grid_set(c, g, cgrid_row, cgrid_col, EMPTY_CELL);
        }
        else if (buffer[i] == EMPTY_CELL || buffer[i] == OBSTACLE)
        {
            grid_set(c, g, cgrid_row, cgrid_col, buffer[i]);
        }
        else
        {
            errno = EINVAL;
            return EINVAL;
        }

        cgrid_col++;
    }

    return 0;
}

int compute_guard_path(grid_t(c) * g, int guard_x, int guard_y)
{
    int position_x = guard_x;
    int position_y = guard_y;
    direction d = UP;

    while (1)
    {

        if (grid_get(c, g, position_x, position_y) == EMPTY_CELL)
        {
            grid_set(c, g, position_x, position_y, TRAVERSED);
        }

        int new_x;
        int new_y;

        int next = next_position(position_x, position_y, d, &new_x, &new_y);
        if (next != 0)
        {
            errno = next;
            return next;
        }
        if (new_x < 0 || new_x >= (int)g->cols || new_y < 0 || new_y >= (int)g->rows)
        {
            break;
        }

        char next_cell = grid_get(c, g, new_x, new_y);
        if (next_cell == EMPTY_CELL || next_cell == TRAVERSED)
        {
            position_x = new_x;
            position_y = new_y;
        }
        else if (next_cell == OBSTACLE)
        {
            d = rotate_90(d);
        }
    }

    return 0;
}

int contains_cycle(grid_t(c) * c, int guard_x, int guard_y)
{
    int position_x = guard_x;
    int position_y = guard_y;
    direction d = UP;

    grid_t(dir) *dir_grid = grid_init_with_dimensions_and_default(dir, c->rows, c->cols, 0);
    if (dir_grid == NULL)
    {
        errno = ENOMEM;
        return 0;
    }

    grid_set(dir, dir_grid, position_x, position_y, d);

    while (1)
    {

        int new_x;
        int new_y;

        int next = next_position(position_x, position_y, d, &new_x, &new_y);
        if (next != 0)
        {
            printf("Error: %s\n", strerror(next));
            errno = next;
            break;
        }
        if (new_x < 0 || new_x >= (int)c->cols || new_y < 0 || new_y >= (int)c->rows)
        {

            break;
        }

        char next_cell = grid_get(c, c, new_x, new_y);
        if (next_cell == EMPTY_CELL || next_cell == TRAVERSED)
        {
            position_x = new_x;
            position_y = new_y;
        }
        else if (next_cell == OBSTACLE)
        {
            d = rotate_90(d);
        }

        int dir = grid_get(dir, dir_grid, position_x, position_y);

        if ((dir & d) != 0)
        {
            grid_free(dir, dir_grid);
            return 1;
        }

        dir |= d;

        grid_set(dir, dir_grid, position_x, position_y, dir);
    }
    grid_free(dir, dir_grid);

    return 0;
}

int get_total_number_of_cycles(grid_t(c) * c, int guard_x, int guard_y)
{
    int cycles = 0;
    char value;
    size_t i, j;
    grid_enumerate(c, c, value, i, j, {
        if (value == EMPTY_CELL)
        {
            grid_set(c, c, i, j, OBSTACLE);
            int cycle = contains_cycle(c, guard_x, guard_y);
            if (cycle == 1)
            {
                cycles++;
            }
            grid_set(c, c, i, j, EMPTY_CELL);
        } });
    return cycles;
}

int count_traversed_cells(grid_t(c) * g)
{
    int count = 0;
    char value;
    grid_foreach(c, g, value, {
        if (value == TRAVERSED)
        {
            count++;
        }
    });
    return count;
}

int main(void)
{
    long length;
    char *buffer = 0;

    grid_t(c) g = {0};
    int guard_x = 0;
    int guard_y = 0;

    int read = read_file_to_buffer(&buffer, "data/q6.txt", &length);
    if (read != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return errno;
    }

    int parse = parse_input(buffer, length, &g, &guard_x, &guard_y);

    if (parse != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return errno;
    }

    grid_t(c) g_copy = {0};
    int copy = grid_copy(c, &g, &g_copy);
    if (copy != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return errno;
    }

    int compute = compute_guard_path(&g_copy, guard_x, guard_y);

    if (compute != 0)
    {
        printf("Error: %s\n", strerror(compute));
        return compute;
    }

    int traversed = count_traversed_cells(&g_copy);

    printf("Part 1: %d\n", traversed);

    int total_cycles = get_total_number_of_cycles(&g, guard_x, guard_y);
    printf("Part 2: %d\n", total_cycles);

    return 0;
}
