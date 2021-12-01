#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
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
    int part1 = 0, last = 10000;
    int part2 = 0, i = 0, data[2000];
    while ((nread = getline(&line, &len, fp)) != -1) {
        int current = atoi(line);
        if (current > last) {
            part1++;
        }
        last = current;
        data[i++] = current;
    }

    for (i = 3; i < 2000; i++) {
        int a = data[i - 3] + data[i - 2] + data[i - 1];
        int b = data[i - 2] + data[i - 1] + data[i];
        if (b > a) {
            part2++;
        }
    }

    printf("Part 1: %d\n", part1);
    printf("Part 2: %d\n", part2);

    if (line) {
        free(line);
    }

    fclose(fp);

    return 0;
}
