#!/usr/bin/env python3
"""Advent of Code 2019 - Day 1"""

import sys

def usage():
    """Prints usage string"""
    print("usage: solution [input-file]")

def main(argc, argv):
    """Main entry point"""
    if argc != 2:
        usage()
        exit(0)

    with open(argv[1], 'r') as fd:
        dat = [int(l) for l in fd.read().splitlines()]

    print('Part 1: {}'.format(sum(dat)))


    tmp = 0
    found = 0
    freqs = { 0 }
    while not found:
        for i in dat:
            tmp += i

            if tmp in freqs:
                found = 1
                break
            else:
                freqs.add(tmp)

    print('Part 2: {}'.format(tmp))

if __name__ == "__main__":
    main(len(sys.argv), sys.argv)
