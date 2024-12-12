#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "common.h"

typedef struct Constraint
{
    int page_before;
    int page_after;

} constraint;

int is_valid(constraint *c, int *updates, int start, int end)
{
    int page_before_idx = -1;
    int page_after_idx = -1;

    for (int i = start; i < end; i++)
    {
        if (updates[i] == c->page_before)
        {
            page_before_idx = i;
        }
        if (updates[i] == c->page_after)
        {
            page_after_idx = i;
        }
    }

    if (page_before_idx == -1 || page_after_idx == -1)
    {
        return 1;
    }

    return page_before_idx < page_after_idx;
}

int main(void)
{
    long length;
    char *buffer = 0;

    long update_groups_length;
    int *update_groups = 0;

    long updates_length;
    int *updates = 0;

    long constraints_length;
    constraint *constraints = 0;

    int read = read_file_to_buffer(&buffer, "data/q5.txt", &length);

    if (read != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return read;
    }
}