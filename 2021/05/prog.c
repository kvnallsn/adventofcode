#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define ROWS    1000
#define COLS    1000

void read_input(char *filename, uint32_t board1[ROWS][COLS], uint32_t board2[ROWS][COLS]) {

    FILE *fp = fopen(filename, "r");
    if (!fp) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    char *line = NULL;
    size_t len = 0;

    //int i = 0;
    while (getline(&line, &len, fp) != -1) {
        uint32_t x1, y1, x2, y2 = 0;
        sscanf(line, "%d,%d -> %d,%d", &x1, &y1, &x2, &y2);

        if (x1 == x2) {
            uint32_t max = y1 > y2 ? y1 : y2;
            uint32_t min = y1 > y2 ? y2 : y1;
            for (int y = min; y <= max; y++) {
                board1[y][x1] += 1;
                board2[y][x1] += 1;
            }
        } else if (y1 == y2) {
            uint32_t max = x1 > x2 ? x1 : x2;
            uint32_t min = x1 > x2 ? x2 : x1;
            for (int x = min; x <= max; x++) {
                board1[y1][x] += 1;
                board2[y1][x] += 1;
            }
        } else {
            while ((x1 != x2) && (y1 != y2)) {
                board2[y1][x1] += 1;
                x1 += x1 < x2 ? 1 : -1;
                y1 += y1 < y2 ? 1 : -1;
            }

            // one more to mark the final position
            board2[y1][x1] += 1;
        }
    }

    if (line) {
        free(line);
    }

    fclose(fp);
}

int main(int argc, char *argv[]) {

    uint32_t board1[ROWS][COLS] = { 0 };
    uint32_t board2[ROWS][COLS] = { 0 };
    uint32_t part1 = 0;
    uint32_t part2 = 0;

    read_input("input", board1, board2);

    for (int r = 0; r < ROWS; r++) {
        for (int c = 0; c < COLS; c++) {
            if (board1[r][c] >= 2) {
                part1++;
            }

            if (board2[r][c] >= 2) {
                part2++;
            }
        }
    }

    printf("Part 1: %d\n", part1);
    printf("Part 2: %d\n", part2);


    return 0;
}
