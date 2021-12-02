#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char *argv[]) {

    FILE *fp = fopen("input", "r");
    if (!fp) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    char *line = NULL;
    size_t len = 0;
    ssize_t nread;
    int depth = 0, depth2 = 0, position = 0, aim = 0;
    while ((nread = getline(&line, &len, fp)) != -1) {
        char cmd[8] = { 0 };
        int amt = 0;
        sscanf(line, "%s %d", cmd, &amt);

        if (strcmp("forward", cmd) == 0) {
            position += amt;
            depth2 += aim * amt;
        } else if (strcmp("down", cmd) == 0) {
            depth += amt;
            aim += amt;
        } else if (strcmp("up", cmd) == 0) {
            depth -= amt;
            aim -= amt;
        } else {
            printf("unknown command: %s\n", cmd);
        }
    }

    printf("Part 1: %d\n", (depth * position));
    printf("Part 2: %d\n", (depth2 * position));

    if (line) {
        free(line);
    }

    fclose(fp);

    return 0;
}
