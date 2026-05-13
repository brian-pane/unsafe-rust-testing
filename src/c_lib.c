#include <stddef.h>

extern void write_buffer(char *data, size_t size, char pattern) {
    size_t i;
    for (i = 0; i < size; i++) {
        *data++ = pattern;
    }
}
