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

int get_updates_offset(updates *u, int update_idx)
{
    int total_offset = 0;
    for (int i = 0; i < update_idx; i++)
    {
        total_offset += u->update_groups[i];
    }
    return total_offset;
}

int is_valid(constraint *c, updates *u, int update_idx)
{
    int page_before_idx = -1;
    int page_after_idx = -1;

    int total_offset = get_updates_offset(u, update_idx);

    for (int i = 0; i < u->update_groups[update_idx]; i++)
    {
        if (c->page_before == u->updates[total_offset + i])
        {
            page_before_idx = i;
        }
        if (c->page_after == u->updates[total_offset + i])
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

int get_relevant_constraint(constraints *c, int page_before, int page_after)
{
    for (int i = 0; i < c->length; i++)
    {
        if (c->constraints[i].page_before == page_before && c->constraints[i].page_after == page_after)
        {
            return i;
        }
    }
    return -1;
}

int is_update_valid(constraints *c, updates *u, int update_idx)
{
    for (int i = 0; i < c->length; i++)
    {
        if (!is_valid(&c->constraints[i], u, update_idx))
        {
            return 0;
        }
    }
    return 1;
}

int sort_temp_update_by_constraints(int *u, int length, constraints *c)
{
    int swapped = 0;
    do
    {
        swapped = 0;
        for (int i = 1; i < length; i++)
        {
            int a = u[i];
            int b = u[i - 1];
            int constraint_idx = get_relevant_constraint(c, a, b);
            if (constraint_idx == -1)
            {
                // no constraint, no need to swap
                continue;
            }
            constraint constraint = c->constraints[constraint_idx];
            if (constraint.page_before == a)
            {
                u[i - 1] = a;
                u[i] = b;
                swapped = 1;
            }
        }
    } while (swapped);
    return 0;
}

int fix_update(constraints *c, updates *u, int update_idx)
{
    int *temp_updates = malloc(u->update_groups[update_idx] * sizeof(int));
    if (!temp_updates)
    {
        return ENOMEM;
    }
    int offset = get_updates_offset(u, update_idx);
    for (int i = 0; i < u->update_groups[update_idx]; i++)
    {
        temp_updates[i] = u->updates[offset + i];
    }

    sort_temp_update_by_constraints(temp_updates, u->update_groups[update_idx], c);

    for (int i = 0; i < u->update_groups[update_idx]; i++)
    {
        u->updates[offset + i] = temp_updates[i];
    }
    return 0;
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

    for (int i = 0; i < updates_length; i++)
    {
        u->updates[i] = 0;
    }

    int update_index = 0;

    for (int i = start; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            continue;
        }
        int update = 0;
        while (buffer[i] != ',' && buffer[i] != '\n')
        {
            update = update * 10 + (buffer[i] - '0');
            i++;
        }
        u->updates[update_index] = update;
        update_index++;
    }

    return 0;
}

int get_sum_of_middle_page_numbers(constraints *c, updates *u)
{
    int sum = 0;

    for (int u_idx = 0; u_idx < u->update_group_length; u_idx++)
    {
        if (!is_update_valid(c, u, u_idx))
        {
            continue;
        }
        int total_offset = get_updates_offset(u, u_idx);
        int middle_page = u->updates[total_offset + (u->update_groups[u_idx] / 2)];
        sum += middle_page;
    }
    return sum;
}

int get_sum_of_middle_page_numbers_with_fix(constraints *c, updates *u)
{
    int sum = 0;

    for (int u_idx = 0; u_idx < u->update_group_length; u_idx++)
    {
        if (!is_update_valid(c, u, u_idx))
        {
            fix_update(c, u, u_idx);
            int total_offset = get_updates_offset(u, u_idx);
            int middle_page = u->updates[total_offset + (u->update_groups[u_idx] / 2)];
            sum += middle_page;
        }
    }
    return sum;
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

    int sum = get_sum_of_middle_page_numbers(&constraints, &update);
    printf("Part 1: %d\n", sum);

    int sum2 = get_sum_of_middle_page_numbers_with_fix(&constraints, &update);
    printf("Part 2: %d\n", sum2);

    return 0;
}
