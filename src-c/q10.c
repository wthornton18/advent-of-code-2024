#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include "common.h"
#include "grid.h"

typedef struct Pos
{
    int x;
    int y;
} pos;

pos directions[] = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

typedef struct Node
{
    struct Node *next;
    struct Node *prev;
    pos value;
} node;

typedef struct Queue
{
    node *head;
    node *tail;
} queue;

queue *new()
{
    queue *q = malloc(sizeof(queue));
    q->head = NULL;
    q->tail = NULL;
    return q;
}

int is_empty(queue *q)
{
    return q->head == NULL;
}

int enqueue(queue *q, pos p)
{
    node *n = malloc(sizeof(node));
    if (!n)
    {
        errno = ENOMEM;
        return ENOMEM;
    }

    n->next = NULL;
    n->prev = NULL;
    n->value = p;

    if (is_empty(q))
    {
        q->head = n;
        q->tail = n;
        n->prev = NULL;
        n->next = NULL;
        return 0;
    }

    q->tail->next = n;
    n->prev = q->tail;
    q->tail = n;

    return 0;
}

node popright(queue *q)
{
    node *n = q->tail;
    q->tail = n->prev;
    if (q->tail != NULL)
    {
        q->tail->next = NULL;
    }
    if (q->head == n)
    {
        q->head = NULL;
    }

    return *n;
}

int parse_input(char *buffer, long length, igrid *g)
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

    g->rows = rows;
    g->cols = cols;
    g->data = malloc(rows * cols * sizeof(int));
    if (!g->data)
    {
        errno = ENOMEM;
        return ENOMEM;
    }

    for (int i = 0; i < rows; i++)
    {
        for (int j = 0; j < cols; j++)
        {
            set_icell(g, i, j, 0);
        }
    }

    char *line = strtok(buffer, "\n");

    int row = 0;
    while (line)
    {
        for (int i = 0; i < cols; i++)
        {
            set_icell(g, row, i, line[i] - '0');
        }

        line = strtok(NULL, "\n");
        row++;
    }

    return 0;
}

int get_trailhead_score(igrid g, pos trailhead)
{
    int trailhead_score = 0;

    queue *q = new ();

    enqueue(q, trailhead);

    igrid discovered = {0};
    allocated_and_default_igrid(&discovered, g.rows, g.cols, 0);

    while (1)
    {
        // Take the last element from the queue

        if (is_empty(q))
        {
            break;
        }
        pos p = popright(q).value;

        if (get_icell(&discovered, p.x, p.y) == 1)
        {
            continue;
        }

        set_icell(&discovered, p.x, p.y, 1);

        int v = get_icell(&g, p.x, p.y);

        if (v == 9)
        {
            trailhead_score++;
        }

        for (int i = 0; i < 4; i++)
        {
            pos direction = directions[i];
            pos new_pos = {p.x + direction.x, p.y + direction.y};

            if (new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= g.rows || new_pos.y >= g.cols)
            {
                continue;
            }

            if (get_icell(&g, new_pos.x, new_pos.y) - v == 1)
            {
                enqueue(q, new_pos);
            }
        }
    }

    return trailhead_score;
}

int get_trailhead_rating(igrid g, pos trailhead)
{
    int trailhead_rating = 0;
    queue *q = new ();

    enqueue(q, trailhead);

    while (1)
    {
        if (is_empty(q))
        {
            break;
        }

        pos p = popright(q).value;

        int v = get_icell(&g, p.x, p.y);

        if (v == 9)
        {
            trailhead_rating++;
        }

        for (int i = 0; i < 4; i++)
        {
            pos direction = directions[i];
            pos new_pos = {p.x + direction.x, p.y + direction.y};

            if (new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= g.rows || new_pos.y >= g.cols)
            {
                continue;
            }

            if (get_icell(&g, new_pos.x, new_pos.y) - v == 1)
            {
                enqueue(q, new_pos);
            }
        }
    }

    return trailhead_rating;
}

int get_total_trailhead_score(igrid g)
{

    int trailhead_score = 0;

    for (int i = 0; i < g.rows; i++)
    {
        for (int j = 0; j < g.cols; j++)
        {
            if (get_icell(&g, i, j) == 0)
            {
                pos trailhead = {i, j};
                int score = get_trailhead_score(g, trailhead);
                trailhead_score += score;
            }
        }
    }

    return trailhead_score;
}

int get_total_trailhead_rating(igrid g)
{
    int trailhead_rating = 0;

    for (int i = 0; i < g.rows; i++)
    {
        for (int j = 0; j < g.cols; j++)
        {
            if (get_icell(&g, i, j) == 0)
            {
                pos trailhead = {i, j};
                int rating = get_trailhead_rating(g, trailhead);
                trailhead_rating += rating;
            }
        }
    }

    return trailhead_rating;
}

int main(void)
{
    char *buffer = 0;
    long length;

    igrid grid;

    int read = read_file_to_buffer(&buffer, "data/q10.txt", &length);

    if (read != 0)
    {
        printf("Error reading file: %s\n", strerror(errno));
        return errno;
    }

    int parse = parse_input(buffer, length, &grid);

    if (parse != 0)
    {
        printf("Error parsing input: %s\n", strerror(errno));
        return errno;
    }

    int score = get_total_trailhead_score(grid);

    printf("Part 1: %d\n", score);

    int rating = get_total_trailhead_rating(grid);

    printf("Part 2: %d\n", rating);
}