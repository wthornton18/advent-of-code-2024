#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <math.h>
#include "common.h"

typedef struct Equation
{
    int *values;
    int values_length;
    long target;
} equation;

typedef enum Operation
{
    ADD,
    MUL,
    CONCAT,

} op;

void display_equation(equation eq)
{
    printf("Equation {");
    printf("target: %ld, ", eq.target);
    printf("numbers: ");
    printf("[");
    for (int i = 0; i < eq.values_length; i++)
    {
        printf("%d", eq.values[i]);
        if (i < eq.values_length - 1)
        {
            printf(", ");
        }
    }
    printf("]");
    printf(" }");
    printf("\n");
}

static int inline quick_pow_10(int n)
{
    static const int pow_10[] = {
        1, 10, 100, 1000, 10000, 100000, 1000000,
        10000000, 100000000, 1000000000};
    return pow_10[n];
}

static int inline num_digits(int n)
{
    int places;
    long x;
    if (n < 10)
    {
        return 1;
    }
    for (x = 100, places = 1; (x * 10) < INT64_MAX - 1; x *= 10, places++)
    {
        if (n < x)
        {
            return places;
        }
    }
    return places;
}

static long inline evaluate(op operation, long current, int next)
{
    switch (operation)
    {
    case ADD:
        return current + next;
    case MUL:
        return current * next;
    case CONCAT:
        return (current * (quick_pow_10(num_digits(next)))) + next;
    default:
        return 0;
    }
}

static int inline is_satisfiable_add_mul(equation eq, long current, int index)
{

    if (index >= eq.values_length)
    {
        return current == eq.target;
    }

    if (current > eq.target)
    {
        return 0;
    }

    int next = eq.values[index];
    int next_index = index + 1;

    long add = evaluate(ADD, current, next);
    long mul = evaluate(MUL, current, next);
    return is_satisfiable_add_mul(eq, add, next_index) || is_satisfiable_add_mul(eq, mul, next_index);
}

static int inline is_satisfiable_add_mul_concat(equation eq, long current, int index)
{
    if (index >= eq.values_length)
    {
        return current == eq.target;
    }

    if (current > eq.target)
    {
        return 0;
    }

    int next = eq.values[index];
    int next_index = index + 1;

    long add = evaluate(ADD, current, next);
    long mul = evaluate(MUL, current, next);
    long concat = evaluate(CONCAT, current, next);
    return is_satisfiable_add_mul_concat(eq, add, next_index) || is_satisfiable_add_mul_concat(eq, mul, next_index) || is_satisfiable_add_mul_concat(eq, concat, next_index);
}

int parse_input(char *buffer, long length, equation **equations, long *equations_length)
{
    assert(buffer != 0);
    assert(length > 0);

    long equations_count = 0;
    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == '\n')
        {
            equations_count++;
        }
    }

    equation *eqs = malloc(equations_count * sizeof(equation));

    if (!eqs)
    {
        errno = ENOMEM;
        return ENOMEM;
    }

    *equations = eqs;
    *equations_length = equations_count;
    int equation_index = 0;

    char *end_line;

    char *line = strtok_r(buffer, "\n", &end_line);

    while (line)
    {
        char *colon = strchr(line, ' ');
        if (!colon)
        {
            errno = EINVAL;
            return EINVAL;
        }

        long target = strtol(line, NULL, 10);
        char *values_str = colon + 1;

        while (*values_str && *values_str == ' ')
        {
            values_str++;
        }

        int values_count = 0;
        for (unsigned long i = 0; i < strlen(values_str); i++)
        {
            if (values_str[i] == ' ')
            {
                values_count++;
            }
        }
        values_count++;

        int *values = malloc(values_count * sizeof(int));
        if (!values)
        {
            errno = ENOMEM;
            return ENOMEM;
        }

        char *end_values;
        char *value = strtok_r(values_str, " ", &end_values);
        int value_index = 0;

        while (value)
        {
            values[value_index] = strtol(value, NULL, 10);
            value_index++;
            value = strtok_r(NULL, " ", &end_values);
        }

        eqs[equation_index].target = target;
        eqs[equation_index].values = values;
        eqs[equation_index].values_length = values_count;
        equation_index++;

        line = strtok_r(NULL, "\n", &end_line);
    }
    return 0;
}

long sum_satisfiable_target_add_mul(equation *equations, long equations_length)
{
    long sum = 0;
    for (int i = 0; i < equations_length; i++)
    {
        equation eq = equations[i];
        if (is_satisfiable_add_mul(eq, eq.values[0], 1))
        {
            sum += eq.target;
        }
    }
    return sum;
}

long sum_satisfiable_add_mul_concat(equation *equations, long equations_length)
{
    long sum = 0;
    for (int i = 0; i < equations_length; i++)
    {
        equation eq = equations[i];
        if (is_satisfiable_add_mul_concat(eq, eq.values[0], 1))
        {
            sum += eq.target;
        }
    }
    return sum;
}
int main(void)
{
    char *buffer = 0;
    long length;

    // Multiple equations arr
    equation *equations;
    long equations_length;

    int read = read_file_to_buffer(&buffer, "data/q7.txt", &length);
    if (read != 0)
    {
        printf("Error reading file: %s\n", strerror(errno));
        return errno;
    }

    int parse = parse_input(buffer, length, &equations, &equations_length);
    if (parse != 0)
    {
        printf("Error parsing input: %s\n", strerror(errno));
        return errno;
    }

    long sum = sum_satisfiable_target_add_mul(equations, equations_length);
    printf("Part 1: %ld\n", sum);

    long sum_concat = sum_satisfiable_add_mul_concat(equations, equations_length);
    printf("Part 2: %ld\n", sum_concat);
}
