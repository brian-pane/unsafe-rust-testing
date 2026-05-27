#include <stddef.h>
#include <stdint.h>

extern uint64_t read_buffer_c(const uint8_t *data, size_t size) {
    uint64_t sum = 0;
    size_t i;
    for (i = 0; i < size; i++) {
        sum += *data++;
    }
    return sum;
}

extern void write_buffer_c(uint8_t *data, size_t size, uint8_t pattern) {
    size_t i;
    for (i = 0; i < size; i++) {
        *data++ = pattern;
    }
}
