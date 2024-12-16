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

typedef struct Constraints
{
    constraint *constraints;
    int length;
} constraints;

typedef struct Updates
{
    int *update_groups;
    int *updates;
    int update_group_length;
    int updates_length;

} updates;

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

int parse_input(char *buffer, long length, constraints *c, updates *u)
{
    int constraints_section = 1;

    int constraints_length = 0;
    int updates_groups_length = 0;
    for (int i = 0; i < length; i++)
    {

        if (buffer[i] == '\n')
        {
            if (buffer[i - 1] == '\n')
            {
                constraints_section = 0;
            }

            if (constraints_section)
            {
                constraints_length++;
            }
            else
            {
                updates_groups_length++;
            }
        }
    }

    if (constraints_length == 0)
    {
        return EINVAL;
    }
    c->length = constraints_length;
    c->constraints = malloc(constraints_length * sizeof(constraint));

    if (!c->constraints)
    {
        return ENOMEM;
    }

    for (int i = 0; i < constraints_length; i++)
    {
        constraint constraint = {0, 0};
        c->constraints[i] = constraint;
    }

    if (updates_groups_length == 0)
    {
        return EINVAL;
    }

    u->update_group_length = updates_groups_length;
    u->update_groups = malloc(updates_groups_length * sizeof(int));

    if (!u->update_groups)
    {
        return ENOMEM;
    }

    for (int i = 0; i < updates_groups_length; i++)
    {
        u->update_groups[i] = 0;
    }

    int constraint_index = 0;
    for (int i = 0; i < (constraints_length * 6) + 1; i++)
    {
        if (constraint_index >= constraints_length)
        {
            break;
        }
        int page_before = ((buffer[i] - '0') * 10) + buffer[i + 1] - '0';
        int page_after = ((buffer[i + 3] - '0') * 10) + buffer[i + 4] - '0';
        constraint constraint = {.page_before = page_before, .page_after = page_after};
        c->constraints[constraint_index] = constraint;
        i += 5;
        constraint_index++;
    };

    int start = (constraints_length * 6) + 1;
    int update_groups_index = 0;

    for (int i = start; i < length; i++)
    {
        int update_group_length = 1;
        int k = 0;
        while ((i + k < length) && buffer[i + k] != '\n')
        {
            if (buffer[i + k] == ',')
            {
                update_group_length++;
            }
            k++;
        }
        i += k;
        u->update_groups[update_groups_index] = update_group_length;
        update_groups_index++;
    }
    int updates_length = 0;

    for (int i = 0; i < u->update_group_length; i++)
    {
        updates_length += u->update_groups[i];
    }

    if (updates_length == 0)
    {
        return EINVAL;
    }

    u->updates_length = updates_length;

    u->updates = malloc(updates_length * sizeof(int));

    if (!u->updates)
    {
        return ENOMEM;
    }

    return 0;
}

int main(void)
{
    long length;
    char *buffer = 0;

    updates update = {0};
    constraints constraints = {0};

    int read = read_file_to_buffer(&buffer, "data/q5.txt", &length);

    if (read != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return read;
    }

    int parse = parse_input(buffer, length, &constraints, &update);

    if (parse != 0)
    {
        printf("Error: %s\n", strerror(parse));
        return read;
    }

    return 0;
}
