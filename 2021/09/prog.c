#define _GNU_SOURCE
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifdef TEST
#define ROWS        5
#define COLS        10
#define FILENAME    "input.test"
#else
#define ROWS        100
#define COLS        100
#define FILENAME    "input"
#endif

#define SET_VISITED(x)  (x | 0x80)
#define IS_VISITED(x)   ((x & 0x80) == 0x80)
#define VAL(x)          (x & 0x7F)

void read_input(char *filename, uint8_t board[ROWS][COLS]) {
    FILE *fp = fopen(filename, "r");
    if (!fp) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    char *line = NULL;
    size_t len = 0;

    for (int r = 0; r < ROWS; r++) {
        if (getline(&line, &len, fp) == -1) {
            perror("getline");
            exit(EXIT_FAILURE);
        }

        for (int c = 0; c < COLS; c++) {
            board[r][c] = (line[c] - 0x30) & 0xFF;
        }
    }

    if (line) {
        free(line);
    }

    fclose(fp);
}

uint32_t dfs(uint8_t board[ROWS][COLS], int row, int col) {
    uint8_t val = VAL(board[row][col]);

    // mark as visited
    board[row][col] = SET_VISITED(board[row][col]);

    if (val == 9) {
        return 0;
    }

    uint32_t count = 0;
    if ((row > 0) && !IS_VISITED(board[row - 1][col])) {
        count += dfs(board, row - 1, col);
    }

    if ((col > 0) && !IS_VISITED(board[row][col - 1])) {
        count += dfs(board, row, col - 1);
    }

    if ((row < (ROWS - 1)) && !IS_VISITED(board[row + 1][col])) {
        count += dfs(board, row + 1, col);
    }

    if ((col < (COLS - 1)) && !IS_VISITED(board[row][col + 1])) {
        count += dfs(board, row, col + 1);
    }

    return count + 1;
}

int main(int argc, char *argv[]) {

    uint8_t board[ROWS][COLS] = { 0 };
    int32_t part1 = 0;
    int32_t part2 = 0;

    read_input(FILENAME, board);

    uint32_t max[3] = { 0 };
    for (int r = 0; r < ROWS; r++) {
        for (int c = 0; c < COLS; c++) {
            if (r > 0) {
                // look up
                if (VAL(board[r][c]) >= VAL(board[r - 1][c])) {
                    continue;
                }
            }

            if (c > 0) {
                // look left
                if (VAL(board[r][c]) >= VAL(board[r][c -1])) {
                    continue;
                }
            }

            if (r < (ROWS - 1)) {
                // look down
                if (VAL(board[r][c]) >= VAL(board[r + 1][c])) {
                    continue;
                }
            }

            if (c < (COLS - 1)) {
                // look right
                if (VAL(board[r][c]) >= VAL(board[r][c + 1])) {
                    continue;
                }
            }

            // Part 1: sum of risk level
            part1 += 1 + VAL(board[r][c]);

            // Part 2: depth first search until 9's are encountered
            uint32_t basin = dfs(board, r, c);
            if (basin > max[0]) {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = basin;
            } else if (basin > max[1]) {
                max[2] = max[1];
                max[1] = basin;
            } else if (basin > max[2]) {
                max[2] = basin;
            }
        }
    }

    part2 = max[0] * max[1] * max[2];

    printf("Part 1: %u\n", part1);
    printf("Part 2: %u\n", part2);

    return 0;
}
