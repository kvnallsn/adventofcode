#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifdef TEST
#define COUNT       10
#define FILENAME    "input.test"
#else
#define COUNT       200
#define FILENAME    "input"
#endif

int main(int argc, char *argv[]) {

    int32_t part1 = 0;
    int32_t part2 = 0;

    FILE *fp = fopen(FILENAME, "r");
    if (!fp) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    char *line = NULL;
    size_t len = 0;

    while (getline(&line, &len, fp) != -1) {
        // strip newline
        line[strlen(line) - 1] = '\0';

        // save pointers for strtok_r
        char *save, *input, *output;

        // get input half of line (before the pipe)
        char *token = strtok_r(line, "|", &save);

        // parse inputs
        uint8_t nums[10] = { 0 };
        uint8_t data[255] = { 0 };
        uint8_t fives[3] = { 0 };
        uint8_t sixes[3] = { 0 };
        size_t fives_idx = 0, sixes_idx = 0;
        token = strtok_r(token, " ", &input);
        for (int i = 0; i < 10 && token != NULL; i++) {
            uint8_t val  = 0;
            for (size_t j = 0; j < strlen(token); j++) {
                val |= 1 << (token[j] - 0x61);
            }

            switch (__builtin_popcount(val)) {
            case 2:
                nums[1] = val;  // only "1" uses two segments
                data[val] = 1;
                break;
            case 3:
                nums[7] = val;  // only "7" uses three segments
                data[val] = 7;
                break;
            case 4:
                nums[4] = val;  // only "4" uses three segments
                data[val] = 4;
                break;
            case 5:
                fives[fives_idx++] = val;
                break;
            case 6:
                sixes[sixes_idx++] = val;
                break;
            case 7:
                nums[8] = val;  // only "8" uses seven segments
                data[val] = 8;
                break;
            default:
                fprintf(stderr, "impossible popcount");
                exit(EXIT_FAILURE);
            }
            token = strtok_r(NULL, " ", &input);
        }

        // solve digits that use five segments
        for (int i = 0; i < 3; i++) {
            if ((fives[i] & nums[1]) == nums[1]) {
                nums[3] = fives[i];
                data[fives[i]] = 3;
            } else {
                uint8_t mask = nums[4] ^ nums[1];
                if ((fives[i] & mask) == mask) {
                    nums[5] = fives[i];
                    data[fives[i]] = 5;
                } else {
                    nums[2] = fives[i];
                    data[fives[i]] = 2;
                }
            }
        }

        // solve digits that use six segments
        for (int i = 0; i < 3; i++) {
            if ((sixes[i] & nums[3]) == nums[3]) {
                nums[9] = fives[i];
                data[sixes[i]] = 9;
            } else if ((sixes[i] & nums[1]) == nums[1]) {
                nums[0] = sixes[i];
                data[sixes[i]] = 0;
            } else {
                nums[6] = sixes[i];
                data[sixes[i]] = 6;
            }
        }

        // get output half of line (after the pipe)
        token = strtok_r(NULL, "|", &save);

        // parse outputs
        uint32_t num = 0;
        token = strtok_r(token, " ", &output);
        for (int i = 1000; token != NULL; i /= 10) {
            uint8_t val  = 0;
            for (; *token != '\0'; token++) {
                val |= 1 << (*token - 0x61);
            }

            // part 1, count unique length outputs
            switch (__builtin_popcount(val)) {
            case 2: // "1"
            case 3: // "7"
            case 4: // "4"
            case 7: // "8"
                part1 += 1;
                break;
            default:
                /* do nothing */
                break;
            }

            // part 2, recreate output number
            num += data[val] * i;
            token = strtok_r(NULL, " ", &output);
        }

        // part 2, sum all output numbers
        part2 += num;
    }


    if (line) {
        free(line);
    }

    fclose(fp);

    printf("Part 1: %u\n", part1);
    printf("Part 2: %u\n", part2);
}
