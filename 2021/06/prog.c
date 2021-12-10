#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define DAYS    9

void read_input(char *filename, uint64_t timers[DAYS]) {

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
    while (token != NULL) {
        uint32_t timer = atoi(token);
        timers[timer] += 1;
        token = strtok(NULL, ",");
    }


    if (line) {
        free(line);
    }

    fclose(fp);
}

int main(int argc, char *argv[]) {

    uint64_t timers[DAYS] = { 0 };
    uint64_t part1 = 0;
    uint64_t part2 = 0;

    read_input("input", timers);

    for (int day = 1; day <= 80; day++) {
        uint64_t saved = timers[0];
        timers[0] = timers[1];
        timers[1] = timers[2];
        timers[2] = timers[3];
        timers[3] = timers[4];
        timers[4] = timers[5];
        timers[5] = timers[6];
        timers[6] = timers[7] + saved;
        timers[7] = timers[8];
        timers[8] = saved;
    }

    for (int i = DAYS - 1; i >= 0; i--) {
        part1 += timers[i];
    }

    for (int day = 81; day <= 256; day++) {
        uint64_t saved = timers[0];
        timers[0] = timers[1];
        timers[1] = timers[2];
        timers[2] = timers[3];
        timers[3] = timers[4];
        timers[4] = timers[5];
        timers[5] = timers[6];
        timers[6] = timers[7] + saved;
        timers[7] = timers[8];
        timers[8] = saved;
    }

    for (int i = DAYS - 1; i >= 0; i--) {
        part2 += timers[i];
    }

    printf("Part 1: %lu\n", part1);
    printf("Part 2: %lu\n", part2);


    return 0;
}
