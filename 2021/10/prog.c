#define _GNU_SOURCE
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifdef TEST
#define CAP                 10
#define FILENAME            "input.test"
#else
#define CAP                 106
#define FILENAME            "input"
#endif

#define ARRAY_SIZE(x)      (sizeof(x)/sizeof(*x))

int compare_ints(const void *a, const void *b) {
    uint64_t arg1 = *(const uint64_t*)a;
    uint64_t arg2 = *(const uint64_t*)b;

    return (arg1 > arg2) - (arg1 < arg2);
}

int main(int argc, char *argv[]) {
    char stack[CAP] = { 0 };
    int32_t score[127] = { 0 };
    uint64_t scores[CAP] = { 0 };
    int32_t part1 = 0;
    int32_t part2 = 0;
    int32_t sidx = 0;

    FILE *fp = fopen(FILENAME, "r");
    if (!fp) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    // part 1 scoring
    score[')'] = 3;
    score[']'] = 57;
    score['}'] = 1197;
    score['>'] = 25137;

    char *line = NULL;
    size_t len = 0;

    while (getline(&line, &len, fp) != -1) {
        memset(stack, 0, ARRAY_SIZE(stack));
        line[strlen(line) - 1] = '\0';

        int error = 0, i = 0;
        char *iter = line;
        for (; *iter && !error; iter++) {
            char offset = 1;
            switch (*iter) {
            case '(':
            case '[':
            case '{':
            case '<':
                stack[i++] = *iter;
                break;
            case ']':
            case '}':
            case '>':
                offset += 1;
            case ')':
                if ((*iter - offset) != stack[--i]) {
                    // corrupted sequence
                    error = 1;
                    part1 += score[*iter & 0xFF];
                }
                break;
            default:
                fprintf(stderr, "illegal character: %d\n", *iter);
                exit(EXIT_FAILURE);
            }
        }

        if (error) {
            // corrupted sequence, don't try to fix
            continue;
        }

        // finish incomplete sequence
        while (i > 0) {
            char c = stack[--i];
            scores[sidx] *= 5;

            switch (c) {
            case '<':
                scores[sidx] += 1;
            case '{':
                scores[sidx] += 1;
            case '[':
                scores[sidx] += 1;
            case '(':
                scores[sidx] += 1;
                break;
            default:
                fprintf(stderr, "illegal character: %c\n", c);
                exit(EXIT_FAILURE);
            }
        }

        // increment score counter
        sidx += 1;
    }

    qsort(scores, sidx, sizeof(*scores), compare_ints);
    part2 = scores[sidx / 2];

    if (line) {
        free(line);
    }

    fclose(fp);

    printf("Part 1: %u\n", part1);
    printf("Part 2: %u\n", part2);

    return 0;
}
