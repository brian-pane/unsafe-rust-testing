#include <stddef.h>

extern long checksum(const char *data) {
    long sum = 0;
    while (*data != 0) {
        sum += *data++;
    }
    return sum;
}
