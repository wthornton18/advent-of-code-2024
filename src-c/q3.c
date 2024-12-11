#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "common.h"

typedef enum TokenType
{
    MUL,
    LEFT_PAREN,
    RIGHT_PAREN,
    NUMERIC_LITERAL,
    COMMA,
    DO,
    DONT,
    OTHER,
} token_type;

typedef struct Token
{
    token_type type;
    int position;
    int length;
} token;

typedef enum ExprType
{
    MUL_EXPR,
    DO_EXPR,
    DONT_EXPR,
} expr_type;

typedef struct Expr
{
    expr_type type;
    union data
    {
        struct mul_expr
        {
            int left;
            int right;
        } mul;
        int do_expr;
        int dont_expr;
    } data;

} expr;

void display_token_type(token_type type)
{
    switch (type)
    {
    case MUL:
        printf("MUL");
        break;
    case LEFT_PAREN:
        printf("LEFT_PAREN");
        break;
    case RIGHT_PAREN:
        printf("RIGHT_PAREN");
        break;
    case NUMERIC_LITERAL:
        printf("NUMERIC_LITERAL");
        break;
    case COMMA:
        printf("COMMA");
        break;
    case DO:
        printf("DO");
        break;
    case DONT:
        printf("DONT");
        break;
    case OTHER:
        printf("OTHER");
        break;
    default:
        printf("UNKNOWN");
        break;
    }
}

void display_expr_type(expr_type type)
{
    switch (type)
    {
    case MUL_EXPR:
        printf("MUL_EXPR");
        break;
    case DO_EXPR:
        printf("DO_EXPR");
        break;
    case DONT_EXPR:
        printf("DONT_EXPR");
        break;
    default:
        printf("UNKNOWN_EXPR");
        break;
    }
}

void display_token(token t)
{
    printf("Token type: ");
    display_token_type(t.type);
    printf(", position: %d, length: %d\n", t.position, t.length);
}

void display_expr(expr e)
{
    printf("Expression type: ");

    display_expr_type(e.type);
    if (e.type == MUL_EXPR)
    {
        printf(", left: %d, right: %d\n", e.data.mul.left, e.data.mul.right);
    }
    printf("\n");
}

int startswith(char *buffer, long length, char *prefix, int offset)
{
    int prefix_length = strlen(prefix);
    if ((length - offset) < prefix_length)
    {
        return 0;
    }
    for (int i = 0; i < prefix_length; i++)
    {
        if (buffer[offset + i] != prefix[i])
        {
            return 0;
        }
    }
    return 1;
}

int tokens_startswith(token *tokens, long token_count, token_type *token_types, long token_type_count, int offset)
{
    if ((token_count - offset) < token_type_count)
    {
        return 0;
    }

    for (int i = 0; i < token_type_count; i++)
    {

        if (tokens[offset + i].type != token_types[i])
        {

            return 0;
        }
    }
    return 1;
}
int tokenize(char *buffer, long length, token **tokens, long *token_count)
{

    *tokens = malloc(length * sizeof(token)); // worst case scenario, every character is a token
    if (!*tokens)
    {
        return ENOMEM;
    }

    for (int i = 0; i < length; i++)
    {
        token t = {OTHER, i, 1};
        (*tokens)[i] = t;
    }

    long token_index = 0;
    long i = 0;
    while (i < length)
    {
        if (startswith(buffer, length, "don't", i))
        {
            token t = {DONT, i, 5};
            (*tokens)[token_index] = t;
            i += 5;
        }
        else if (startswith(buffer, length, "do", i))
        {
            token t = {DO, i, 2};
            (*tokens)[token_index] = t;
            i += 2;
        }
        else if (startswith(buffer, length, "mul", i))
        {
            token t = {MUL, i, 3};
            (*tokens)[token_index] = t;
            i += 3;
        }
        else if (buffer[i] == '(')
        {
            token t = {LEFT_PAREN, i, 1};
            (*tokens)[token_index] = t;
            i++;
        }
        else if (buffer[i] == ')')
        {
            token t = {RIGHT_PAREN, i, 1};
            (*tokens)[token_index] = t;
            i++;
        }
        else if (buffer[i] == ',')
        {
            token t = {COMMA, i, 1};
            (*tokens)[token_index] = t;
            i++;
        }
        else if (buffer[i] >= '0' && buffer[i] <= '9')
        {
            int j = i;
            while (j < length && buffer[j] >= '0' && buffer[j] <= '9')
            {
                j++;
            }
            token t = {NUMERIC_LITERAL, i, j - i};
            (*tokens)[token_index] = t;
            i = j;
        }
        else
        {
            token t = {OTHER, i, 1};
            (*tokens)[token_index] = t;
            i++;
        }
        token_index++;
    }
    *token_count = token_index;

    tokens = realloc(*tokens, *token_count * sizeof(token));

    if (!*tokens)
    {
        return ENOMEM;
    }

    return 0;
}

int get_number(char *buffer, token token)
{
    int number = 0;
    for (int i = 0; i < token.length; i++)
    {
        number = number * 10 + (buffer[token.position + i] - '0');
    }
    return number;
}

int compile(char *buffer, token *tokens, long token_count, expr **expressions, long *expr_count)
{
    *expressions = malloc(token_count * sizeof(expr)); // worst case scenario, every token is an expression
    if (!*expressions)
    {
        return ENOMEM;
    }

    long expr_index = 0;
    long i = 0;

    while (i < token_count)
    {

        if (tokens_startswith(tokens, token_count, (token_type[]){MUL, LEFT_PAREN, NUMERIC_LITERAL, COMMA, NUMERIC_LITERAL, RIGHT_PAREN}, 6, i))
        {
            token first_num = tokens[i + 2];
            token second_num = tokens[i + 4];
            expr e = {MUL_EXPR, {{.left = get_number(buffer, first_num), .right = get_number(buffer, second_num)}}};
            (*expressions)[expr_index] = e;
            i += 6;
            expr_index++;
        }
        else if (tokens_startswith(tokens, token_count, (token_type[]){DO, LEFT_PAREN, RIGHT_PAREN}, 3, i))
        {
            expr e = {DO_EXPR, {0}};
            (*expressions)[expr_index] = e;
            i += 3;
            expr_index++;
        }
        else if (tokens_startswith(tokens, token_count, (token_type[]){DONT, LEFT_PAREN, RIGHT_PAREN}, 3, i))
        {
            expr e = {DONT_EXPR, {0}};
            (*expressions)[expr_index] = e;
            i += 3;
            expr_index++;
        }
        else
        {
            i++;
        }
    }
    *expr_count = expr_index;
    *expressions = realloc(*expressions, *expr_count * sizeof(expr));

    if (!*expressions)
    {
        return ENOMEM;
    }

    return 0;
}

int run_prog(expr *expressions, long expression_count, int enable_do_and_dont)
{
    int sum = 0;
    int mul_enabled = 1;

    for (int i = 0; i < expression_count; i++)
    {
        expr e = expressions[i];
        switch (e.type)
        {
        case MUL_EXPR:
            if (mul_enabled)
            {
                sum += e.data.mul.left * e.data.mul.right;
            }

            break;
        case DO_EXPR:
            if (enable_do_and_dont)
            {
                mul_enabled = 1;
            }
            break;
        case DONT_EXPR:
            if (enable_do_and_dont)
            {
                mul_enabled = 0;
            }
            break;
        default:
            break;
        }
    }
    return sum;
}

int main(void)
{
    char *buffer = 0;
    long length;

    token *tokens = {0};
    long token_count = 0;

    expr *expressions = {0};
    long expression_count = 0;

    int read = read_file_to_buffer(&buffer, "data/q3.txt", &length);
    if (read != 0)
    {
        printf("Error: %s\n", strerror(errno));
        return read;
    }

    int tokenized = tokenize(buffer, length, &tokens, &token_count);
    if (tokenized != 0)
    {
        printf("Error: %s\n", strerror(tokenized));
        return tokenized;
    }

    int compiled = compile(buffer, tokens, token_count, &expressions, &expression_count);

    free(buffer);
    free(tokens);

    if (compiled != 0)
    {
        printf("Error: %s\n", strerror(compiled));
        return compiled;
    }

    int result = run_prog(expressions, expression_count, 0);
    printf("Part 1: %d\n", result);
    int result2 = run_prog(expressions, expression_count, 1);
    printf("Part 2: %d\n", result2);
}
