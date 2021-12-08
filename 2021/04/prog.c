#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define MARK 0x80
#define SET_MARKED(x) (MARK | x)
#define IS_MARKED(x) ((x & MARK) == MARK)

void read_input(char *filename, uint8_t numbers[100], uint8_t boards[100][5][5]) {

    FILE *fp = fopen(filename, "r");
    if (!fp) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    char *line = NULL;
    size_t len = 0;

    // read in first line of called numbers
    if (getline(&line, &len, fp) != -1) {
        char *token = strtok(line, ",");
        for (int j = 0; token != NULL; j++) {
            numbers[j] = atoi(token) & 0xFF;
            token = strtok(NULL, ",");
        }
    }

    int i = 0;
    while (getline(&line, &len, fp) != -1) {
        if (strcmp("\n", line) == 0) {
            continue;
        }

        int board = i / 5;
        int row = i % 5;
        char *token = strtok(line, " ");
        for (int col = 0; token != NULL; col++) {
            boards[board][row][col] = atoi(token) & 0xFF;
            token = strtok(NULL, " ");
        }

        i += 1;
    }

    if (line) {
        free(line);
    }

    fclose(fp);
}

void print_board(uint8_t board[5][5]) {
    printf("%d %d %d %d %d\n", board[0][0], board[0][1], board[0][2], board[0][3], board[0][4]);
    printf("%d %d %d %d %d\n", board[1][0], board[1][1], board[1][2], board[1][3], board[1][4]);
    printf("%d %d %d %d %d\n", board[2][0], board[2][1], board[2][2], board[2][3], board[2][4]);
    printf("%d %d %d %d %d\n", board[3][0], board[3][1], board[3][2], board[3][3], board[3][4]);
    printf("%d %d %d %d %d\n", board[4][0], board[4][1], board[4][2], board[4][3], board[4][4]);
}

uint32_t sum_unmarked(uint8_t board[5][5]) {
    uint32_t sum = 0;
    for (int row = 0; row < 5; row++) {
        for (int col = 0; col < 5; col++) {
            if (!IS_MARKED(board[row][col])) {
                sum += board[row][col];
            }
        }
    }

    return sum;
}

int main(int argc, char *argv[]) {

    uint8_t numSolved = 0;
    uint8_t numbers[100] = { 0 };
    uint8_t solved[100] = { 0 };
    uint8_t boards[100][5][5] = { 0 };
    uint32_t part1 = 0;
    uint32_t part2 = 0;

    read_input("input", numbers, boards);

    for (int i = 0; i < 100; i++) {
        for (int board = 0; board < 100; board++) {
            if (solved[board]) {
                // this board has already been solved/is a winner
                continue;
            }

            // update board with new number
            for (int row = 0; row < 5; row++) {
                for (int col = 0; col < 5; col++) {
                    if (boards[board][row][col] == numbers[i]) {
                        boards[board][row][col] = SET_MARKED(boards[board][row][col]);
                    }
                }
            }

            uint8_t winner = 0;

            // check if board is a winner
            for (int row = 0; row < 5; row++) {
                if (IS_MARKED(boards[board][row][0]) &&
                        IS_MARKED(boards[board][row][1]) &&
                        IS_MARKED(boards[board][row][2]) &&
                        IS_MARKED(boards[board][row][3]) &&
                        IS_MARKED(boards[board][row][4])) {

                    // winner
                    winner = 1;
                }
            }

            for (int col = 0; col < 5; col++) {
                if (IS_MARKED(boards[board][0][col]) &&
                        IS_MARKED(boards[board][1][col]) &&
                        IS_MARKED(boards[board][2][col]) &&
                        IS_MARKED(boards[board][3][col]) &&
                        IS_MARKED(boards[board][4][col])) {

                    // winner
                    winner = 1;
                }
            }

            if (winner) {
               if (numSolved == 0) {
                    // first winner (part 1 answer)
                    part1 = numbers[i] * sum_unmarked(boards[board]);
                } else if (numSolved == 99) {
                    part2 = numbers[i] * sum_unmarked(boards[board]); 
                }

                solved[board] = 1;
                numSolved += 1;
            }
        }
    }

    printf("Part 1: %d\n", part1);
    printf("Part 2: %d\n", part2);


    return 0;
}
