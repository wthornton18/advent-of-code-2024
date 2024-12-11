#include <errno.h>
#include <stdio.h>

#ifndef COMMON_H
#define COMMON_H

int read_file_to_buffer(char **buffer, char *filename, long *length)
{
    int seek;
    FILE *f = fopen(filename, "r");
    if (!f)
    {
        printf("Error: %s\n", strerror(errno));
        return errno;
    };

    seek = fseek(f, 0, SEEK_END);
    if (seek != 0)
    {
        fclose(f);
        return errno;
    }
    *length = ftell(f);
    if (*length < 0)
    {
        fclose(f);
        return errno;
    }

    seek = fseek(f, 0, SEEK_SET);

    if (seek != 0)
    {
        fclose(f);
        return errno;
    }

    *buffer = malloc(*length);
    if (!*buffer)
    {
        fclose(f);
        return ENOMEM;
    }

    int read = fread(*buffer, 1, *length, f);
    if (read != *length)
    {
        fclose(f);
        return errno;
    }
    fclose(f);

    return 0;
}

#endif
