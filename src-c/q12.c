#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdbool.h>
#include "common.h"
#include "vec.h"
#include "grid.h"

typedef struct Coord
{
    size_t x;
    size_t y;
} coord;

VEC_DECLARE(coords, coord)

typedef struct Region
{
    char plant_type;
    vec_t(coords) coords;
} region;

VEC_DECLARE(regions, region)

GRID_DECLARE(c, char)

int main(void)
{
    char *buffer = 0;
    long length;

    int parse = read_file_to_buffer(&buffer, "data/q12.txt", &length);

    if (parse != 0)
    {
        fprintf(stderr, "Error reading file: %s\n", strerror(errno));
        return errno;
    }
}