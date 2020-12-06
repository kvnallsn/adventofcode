#!/usr/bin/env python3
"""AoC Day 2"""

import sys

class Counter(dict):
    """Counter"""
    def __missing__(self, key):
        return 0

def load(argc, argv):
    if argc != 2:
        print('usage: {} input-file'.format(argv[0]))
        exit(0)

    with open(argv[1], 'r') as fdd:
        dat = fdd.read().splitlines()

    return dat

def part1(argc, argv):
    """ Part 1 entry point"""

    dat = load(argc, argv)

    two_cnt = 0
    three_cnt =0
    for word in dat:
        cnt = Counter()
        for c in word:
            cnt[c] += 1

        got_two = 0
        got_three = 0
        for c in cnt:
            if cnt[c] == 2 and got_two == 0:
                two_cnt += 1
                got_two = 1
            elif cnt[c] == 3 and got_three == 0:
                three_cnt += 1
                got_three = 1


    print('Part 1: {}'.format(three_cnt * two_cnt))

def part2(argc, argv):
    """ Part 2 entry point"""
    dat = load(argc, argv)

    num_words = len(dat)
    for i in range(num_words):
        for j in range(i + 1, num_words):
            diff = 0
            for (iw, jw) in zip(dat[i], dat[j]):
                if iw != jw:
                    diff += 1

            if diff == 1:
                print('{} : {}'.format(dat[i], dat[j]))


if __name__ == "__main__":
    part1(len(sys.argv), sys.argv)
    part2(len(sys.argv), sys.argv)

