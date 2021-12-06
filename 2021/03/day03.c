#define _GNU_SOURCE
#include <stdint.h>
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
    int data[12] = { 0 };
    int values[1000][12] = { 0 };
    int i = 0;
    while ((nread = getline(&line, &len, fp)) != -1) {
        for (int j = 0; j < 12; j++) {
            values[i][j] = line[j] == '1' ? 1 : 0;
            data[j] += line[j] == '1' ? 1 : 0;
        }
        i += 1;
    }

    // Part 1
    int gamma = 0;
    int epsilon = 0;
    for (int i = 0; i < 12; i++) {
        gamma |= (data[i] > 500 ? (1 << (11 - i)) : 0);
        epsilon |= (data[i] < 500 ? (1 << (11 - i)) : 0);
    }

    // Part 2
    uint8_t o_selected[1000] = { 0 };
    uint8_t co2_selected[1000] = { 0 };
    memset(o_selected, 1, 1000);
    memset(co2_selected, 1, 1000);

    for (int i = 0; i < 12; i++) {
        int o_n = 0;
        int o_v = 0;

        int co2_n = 0;
        int co2_v = 0;

        for (int j = 0; j < 1000; j++) {
            if (o_selected[j]) {
                o_v += values[j][i];
                o_n += 1;
            }

            if (co2_selected[j]) {
                co2_v += values[j][i];
                co2_n += 1;
            }
        }

        // add in remainder if n value is odd
        int most = (o_v >= ((o_n / 2) + (o_n % 2))) ? 1 : 0;
        int least = (co2_v < ((co2_n / 2) + (co2_n % 2))) ? 1 : 0;

        int o2_done = o_n == 1;
        int co2_done = co2_n == 1;

#if DEBUG
        if (!o2_done) {
            printf("oxygen: m = %d, v = %d, n = %d (%d)\n", most, o_v, o_n, o_n / 2);
        }

        if (!co2_done) {
            printf("co2:    l = %d, v = %d, n = %d (%d)\n", least, co2_v, co2_n, co2_n / 2);
        }
#endif

        for (int j = 0; j < 1000; j++) {
            if (!o2_done && o_selected[j]) {
                o_selected[j] = most == values[j][i];
            }

            if (!co2_done && co2_selected[j]) {
                co2_selected[j] = least == values[j][i];
            }
        }


        if (o2_done && co2_done) {
            break;
        }
    }

    // find the only value left selected
    int oxygen = 0;
    int co2 = 0;
    for (int i = 0; i < 1000; i++) {
        if (o_selected[i]) {
            for (int j = 0; j < 12; j++) {
                oxygen |= values[i][j] << (11 - j);
            }
        }

        if (co2_selected[i]) {
            for (int j = 0; j < 12; j++) {
                co2 |= values[i][j] << (11 - j);
            }
        }
    }

    printf("Part 1: %u\n", (gamma * epsilon));
    printf("Part 2: %d\n", (oxygen * co2));

    if (line) {
        free(line);
    }

    fclose(fp);

    return 0;
}
