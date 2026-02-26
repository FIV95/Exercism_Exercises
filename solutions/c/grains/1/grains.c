#include "grains.h"
#include "math.h"

uint64_t square(uint8_t index) {
    if (index == 0) {
        return 0;
    } else if (index == 1) {
        return 1;
    }
    else {
        return pow(2, index-1);
    }
}

uint64_t total(void) {
    int squares = 64;
    uint64_t sum = 0;
    int grains = 1;

    while (squares > 0) {
        sum += grains;
        grains *= 2;
        squares = squares -1;
    }
    return sum;
}