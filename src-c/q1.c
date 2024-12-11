#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "common.h"

int compare(const void *a, const void *b)
{
    return (*(int *)a - *(int *)b);
}

int parse_input(char *buffer, long length, int **arr, int **other_arr, long *lines)
{
    assert(buffer);
    assert(length > 0);

    *lines = 0;

    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            (*lines)++;
        }
    }

    *arr = malloc(*lines * sizeof(int));
    *other_arr = malloc(*lines * sizeof(int));

    if (!*arr || !*other_arr)
    {
        return ENOMEM;
    }

    for (int i = 0; i < *lines; i++)
    {
        (*arr)[i] = 0;
        (*other_arr)[i] = 0;
    }

    int arr_index = 0;
    int for_first_arr = 1;

    for (int i = 0; i < length; i++)
    {

        if (buffer[i] == '\n')
        {
            arr_index++;

            for_first_arr = 1;
        }

        if (buffer[i] == ' ')
        {
            for_first_arr = 0;
        }

        if (buffer[i] >= '0' && buffer[i] <= '9')
        {
            if (for_first_arr)
            {
                (*arr)[arr_index] = (*arr)[arr_index] * 10 + (buffer[i] - '0');
            }
            else
            {
                (*other_arr)[arr_index] = (*other_arr)[arr_index] * 10 + (buffer[i] - '0');
            }
        }
    }
    qsort(*arr, *lines, sizeof(int), compare);
    qsort(*other_arr, *lines, sizeof(int), compare);

    return 0;
}

int compute_distance_sum(int *arr, int *other_arr, long lines)
{
    int sum = 0;
    for (int i = 0; i < lines; i++)
    {
        sum += abs(arr[i] - other_arr[i]);
    }
    return sum;
}

int get_occurrences(int *arr, long lines, int target)

{

    int *elem = bsearch(&target, arr, lines, sizeof(arr[0]), compare);

    if (elem == NULL)
    {
        return 0;
    }

    int occurrences = 1;

    int idx = (elem - arr);

    for (int i = idx - 1; i >= 0; i--)
    {
        if (arr[i] == target)
        {
            occurrences++;
        }
        else
        {
            break;
        }
    }
    for (int i = idx + 1; i < lines; i++)
    {
        if (arr[i] == target)
        {
            occurrences++;
        }
        else
        {
            break;
        }
    }

    return occurrences;
}

int compute_similarity_scores(int *arr, int *other_arr, long lines)
{
    int sum = 0;
    for (int i = 0; i < lines; i++)
    {

        int occurrences = get_occurrences(other_arr, lines, arr[i]);

        sum += occurrences * arr[i];
    }
    return sum;
}

int main(void)
{
    char *buffer = 0;
    long length;
    long lines;
    int *arr = 0;
    int *other_arr = 0;

    int read = read_file_to_buffer(&buffer, "data/q1.txt", &length);
    if (read != 0)
    {
        printf("Error: %s\n", strerror(read));
        return read;
    }

    int parse = parse_input(buffer, length, &arr, &other_arr, &lines);
    if (parse != 0)
    {
        printf("Error: %s\n", strerror(parse));
        return parse;
    }

    printf("Sum of distances: %d\n", compute_distance_sum(arr, other_arr, lines));
    printf("Similarity score: %d\n", compute_similarity_scores(arr, other_arr, lines));
}
