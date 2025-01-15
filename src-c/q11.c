#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdbool.h>
#include "common.h"
#include "khash.h"

KHASH_MAP_INIT_INT64(memo, unsigned long)

typedef struct StoneKey
{
    unsigned long stone;
    unsigned long blink;
} stone_key;

typedef struct BlinkResult
{
    unsigned long stone;
    optional_type(unsigned long) other_stone;
} blink_result;

unsigned long stk_hash(stone_key *key)
{

    // We use elegant pair hashing - should be very fast
    // http://szudzik.com/ElegantPairing.pdf

    unsigned long x = key->stone;
    unsigned long y = key->blink;

    long res = x >= y ? x * x + x + y : x + y * y;
    return res;
}

int parse(char *buffer, long length, int **initial_numbers, long *initial_numbers_length)
{
    // Input such as 5178527 8525 22 376299 3 69312 0 275

    // Count the number of spaces
    int spaces = 0;

    for (int i = 0; i < length; i++)
    {
        if (buffer[i] == ' ')
        {
            spaces++;
        }
    }

    // Allocate the initial numbers array

    int *numbers = malloc(sizeof(int) * (spaces + 1));

    if (!numbers)
    {
        errno = ENOMEM;
        return ENOMEM;
    }

    char *token = strtok(buffer, " ");

    int i = 0;

    while (token)
    {
        numbers[i] = atoi(token);
        token = strtok(NULL, " ");
        i++;
    }

    *initial_numbers = numbers;
    *initial_numbers_length = spaces + 1;

    return 0;
}

inline static long quick_pow_10(long n)
{
    static const long pow_10[] = {
        1, 10, 100, 1000, 10000, 100000, 1000000,
        10000000, 100000000, 1000000000};

    if (n < 10)
    {
        return pow_10[n];
    }
    else
    {
        return pow_10[9];
    }

    return pow_10[n];
}

inline static long num_digits(long n)
{
    long places;
    long x;
    if (n < 10)
    {
        return 1;
    }
    for (x = 100, places = 2;; x *= 10, places++)
    {
        if (n < x)
        {
            return places;
        }
    }
    return places;
}

blink_result blink(unsigned long blink)
{
    switch (blink)
    {
    case 0:
        return (blink_result){1, {.present = false}};
    default:
    {
        long digit_count = num_digits(blink);
        if (digit_count % 2 == 0)
        {
            long half_count = digit_count / 2;
            long divisor = quick_pow_10(half_count);

            return (blink_result){blink / divisor, {.present = true, .value = blink % divisor}};
        }
        else
        {
            return (blink_result){blink * 2024, {.present = false}};
        }
    }
    }
}

long get_stone_count(khash_t(memo) * table, stone_key *key)
{
    khiter_t k;
    int ret;

    k = kh_get(memo, table, stk_hash(key));
    if (k != kh_end(table))
    {
        return kh_value(table, k);
    }

    if (key->blink == 0)
    {
        k = kh_put(memo, table, stk_hash(key), &ret);
        kh_value(table, k) = 1;

        return 1;
    }

    blink_result result = blink(key->stone);

    long stone_count = get_stone_count(table, &(stone_key){result.stone, key->blink - 1});

    if (result.other_stone.present)
    {
        stone_count += get_stone_count(table, &(stone_key){result.other_stone.value, key->blink - 1});
    }
    k = kh_put(memo, table, stk_hash(key), &ret);
    kh_value(table, k) = stone_count;

    return stone_count;
}

long count_total_stones(int *numbers, long length, int blink)
{
    long total = 0;

    khash_t(memo) *table = kh_init(memo);

    for (int i = 0; i < length; i++)
    {
        total += get_stone_count(table, &(stone_key){numbers[i], blink});
    }

    return total;
}

int main(void)
{
    char *buffer = 0;
    long length;

    int *initial_numbers = 0;
    long initial_numbers_length = 0;

    int read = read_file_to_buffer(&buffer, "data/q11.txt", &length);

    if (read != 0)
    {
        fprintf(stderr, "Error reading file: %s\n", strerror(errno));
        return errno;
    }

    int parse_result = parse(buffer, length, &initial_numbers, &initial_numbers_length);

    if (parse_result != 0)
    {
        fprintf(stderr, "Error parsing input: %s\n", strerror(errno));
        return errno;
    }

    long total = count_total_stones(initial_numbers, initial_numbers_length, 25);

    printf("Total stones: %ld\n", total);

    long total_2 = count_total_stones(initial_numbers, initial_numbers_length, 75);

    printf("Total stones: %ld\n", total_2);

    return 0;
}