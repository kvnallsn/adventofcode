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

        // offset 2 from the first line and blank line
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

int main(int argc, char *argv[]) {

    uint8_t numbers[100] = { 0 };
    uint8_t boards[100][5][5] = { 0 };

    read_input("input", numbers, boards);

    for (int i = 0; i < 100; i++) {
        printf("%d, ", numbers[i]);

        for (int board = 0; board < 100; board++) {
            // update board with new number
            for (int row = 0; row < 5; row++) {
                for (int col = 0; col < 5; col++) {
                    if (boards[board][row][col] == numbers[i]) {
                        boards[board][row][col] = SET_MARKED(boards[board][row][col]);
                    }
                }
            }

            // check if board is a winner
            for (int row = 0; row < 5; row++) {
                if (IS_MARKED(boards[board][row][0]) &&
                        IS_MARKED(boards[board][row][1]) &&
                        IS_MARKED(boards[board][row][2]) &&
                        IS_MARKED(boards[board][row][3]) &&
                        IS_MARKED(boards[board][row][4])) {

                    // winner
                    printf("row winner\n");
                    print_board(boards[board]);
                    exit(0);

                }
            }

            for (int col = 0; col < 5; col++) {
                if (IS_MARKED(boards[board][0][col]) &&
                        IS_MARKED(boards[board][1][col]) &&
                        IS_MARKED(boards[board][2][col]) &&
                        IS_MARKED(boards[board][3][col]) &&
                        IS_MARKED(boards[board][4][col])) {

                    // winner
                    printf("col winner\n");
                    print_board(boards[board]);
                    exit(0);

                }
            }
        }
    }
    printf("\n");

    //printf("Part 1: %d\n", (depth * position));
    //printf("Part 2: %d\n", (depth2 * position));


    return 0;
}
