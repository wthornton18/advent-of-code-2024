#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "common.h"

int parse_input(char *buffer, long length, int **arr, int **report_lengths, long *arr_length, long *lines)
{
    assert(buffer);
    assert(length > 0);
    *arr_length = 0;
    *lines = 0;

    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            (*lines)++;
        }
    }

    *report_lengths = malloc(*lines * sizeof(int)); // initialize report_lengths this will contain the relative lengths of each report within arr

    if (!*report_lengths)
    {
        return ENOMEM;
    }

    for (int i = 0; i < *lines; i++)
    {
        (*report_lengths)[i] = 0;
    }

    int arr_index = 0;
    int report_length = 0;

    for (int i = 0; i < length && arr_index < *lines; i++)
    {
        if (buffer[i] == '\n')
        {
            (*report_lengths)[arr_index] = report_length + 1;
            arr_index++;
            report_length = 0;
        }
        if (buffer[i] == ' ' && (i < length - 1 && buffer[i + 1] >= '0' && buffer[i + 1] <= '9'))
        {
            report_length++;
        }
    }
    if (report_length > 0)
    {
        (*report_lengths)[arr_index] = report_length + 1;
    }

    for (int i = 0; i < *lines; i++)
    {
        *arr_length += (*report_lengths)[i];
    }

    *arr = malloc(*arr_length * sizeof(int));
    if (!*arr)
    {
        return ENOMEM;
    }

    for (int i = 0; i < *arr_length; i++)
    {
        (*arr)[i] = 0;
    }

    arr_index = 0;

    for (int i = 0; i < length && arr_index < *arr_length; i++)
    {
        if (buffer[i] == '\n' || buffer[i] == ' ')
        {
            arr_index++;
        }
        if (buffer[i] >= '0' && buffer[i] <= '9')
        {
            (*arr)[arr_index] = (*arr)[arr_index] * 10 + (buffer[i] - '0');
        }
    }

    return 0;
}

int is_report_safe(int *arr, int starting_idx, int length)
{
    int window_start = 0;
    int window_end = 1;
    int previous = -1;

    while (window_start < length && window_end < length)
    {
        int delta = arr[starting_idx + window_end] - arr[starting_idx + window_start];
        int is_positive = delta > 0;
        int abs_delta = abs(delta);

        if (abs_delta > 3 || abs_delta == 0)
        {
            return 0;
        }

        if (previous == -1)
        {
            previous = is_positive;
        }
        else if (previous != is_positive)
        {
            return 0;
        }

        window_start++;
        window_end++;
    }

    return 1;
}

int is_report_safe_with_replacement(int *arr, int starting_idx, int length)
{
    int *new_arr = malloc((length - 1) * sizeof(arr[0]));

    if (!new_arr)
    {
        return ENOMEM;
    }

    for (int skip_idx = 0; skip_idx < length; skip_idx++)
    {
        int k = 0;
        for (int i = 0; i < length; i++)
        {
            if (i == skip_idx)
            {
                continue;
            }
            new_arr[k] = arr[starting_idx + i];
            k++;
        }

        int is_safe = is_report_safe(new_arr, 0, length - 1);

        if (is_safe)
        {

            free(new_arr);
            return 1;
        }
    }

    free(new_arr);
    return 0;
}

int count_safe_reports(int *arr, int *report_lengths, long lines)
{
    int safe_reports = 0;
    int starting_idx = 0;

    for (int i = 0; i < lines; i++)
    {
        int report_is_safe = is_report_safe(arr, starting_idx, report_lengths[i]);

        if (report_is_safe)
        {
            safe_reports++;
        }
        starting_idx += report_lengths[i];
    }

    return safe_reports;
}

int count_safe_reports_with_replacement(int *arr, int *report_lengths, long lines)
{
    int safe_reports = 0;
    int starting_idx = 0;

    for (int i = 0; i < lines; i++)
    {

        if (is_report_safe(arr, starting_idx, report_lengths[i]) || is_report_safe_with_replacement(arr, starting_idx, report_lengths[i]))
        {
            safe_reports++;
        }
        starting_idx += report_lengths[i];
    }

    return safe_reports;
}

int main(void)
{
    char *buffer = 0;
    long length;
    int *arr = 0;
    int *report_lengths = 0;
    long arr_length;
    long lines;

    int read = read_file_to_buffer(&buffer, "data/q2.txt", &length);
    if (read != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return read;
    }

    int parse = parse_input(buffer, length, &arr, &report_lengths, &arr_length, &lines);

    if (parse != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return parse;
    }

    int safe_reports = count_safe_reports(arr, report_lengths, lines);
    printf("Safe reports: %d\n", safe_reports);
    int safe_reports_with_replacement = count_safe_reports_with_replacement(arr, report_lengths, lines);
    printf("Safe reports with replacement: %d\n", safe_reports_with_replacement);
}
