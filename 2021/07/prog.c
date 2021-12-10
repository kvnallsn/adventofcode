#define _GNU_SOURCE
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifndef TEST
#define COUNT       1000
#define FILENAME    "input"
#else
#define COUNT       10
#define FILENAME    "input.test"
#endif

void read_input(char *filename, int32_t crabs[COUNT]) {
    FILE *fp = fopen(filename, "r");
    if (!fp) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    char *line = NULL;
    size_t len = 0;

    if (getline(&line, &len, fp) == -1) {
        perror("getline");
        exit(EXIT_FAILURE);
    }

    char *token = strtok(line, ",");
    for (int i = 0; token != NULL && i < COUNT; i++) {
        crabs[i] = atoi(token);
        token = strtok(NULL, ",");
    }

    if (line) {
        free(line);
    }

    fclose(fp);
}

int compare_ints(const void *a, const void *b) {
    int arg1 = *(uint32_t*)a;
    int arg2 = *(uint32_t*)b;

    if (arg1 < arg2) return -1;
    if (arg1 > arg2) return 1;
    return 0;
}

int main(int argc, char *argv[]) {

    int32_t crabs[COUNT] = { 0 };
    int32_t part1 = 0;
    int32_t part2 = 0;

    read_input(FILENAME, crabs);

    // part 1, find median
    qsort(crabs, COUNT, sizeof(*crabs), compare_ints);
    int32_t median = crabs[COUNT / 2];
    for (int i = 0; i < COUNT; i++) {
        part1 += abs(crabs[i] - median);
    }

    // part 2, find average and sum with triangle numbers
    int32_t sum = 0, mean = 0;
    for (int i = 0; i < COUNT; i++) {
        sum += crabs[i];
    }
    mean = sum / COUNT;
    for (int i = 0; i < COUNT; i++) {
        int32_t distance = abs(crabs[i] - mean);
        part2 += (distance * (distance + 1)) / 2;
    }

    printf("Part 1: %u\n", part1);
    printf("Part 2: %u\n", part2);


    return 0;
}
