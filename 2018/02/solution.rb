#!/usr/bin/env ruby

# Read in and convert to integers
dat = File.readlines('input.txt').map(&:strip)

# Solve Part 1
two_cnt = 0
three_cnt = 0
dat.each do |word|
    dict = { }
    word.each_char do |ch| 
        if dict[ch].nil?
            dict[ch] = 1
        else
            dict[ch] += 1
        end
    end

    if dict.has_value?(2)
        two_cnt += 1
    end

    if dict.has_value?(3)
        three_cnt += 1
    end
end

part1 = two_cnt * three_cnt
puts "Part 1: #{part1}"

# Solve Part 2

for i in 0..(dat.count - 1) do
    for j in (i + 1)..(dat.count - 1) do
        diff = 0
        dat[i].chars.zip(dat[j].chars).each { |ic, jc| diff += 1 if ic != jc }

        if diff == 1
            puts "Part 2: #{dat[i]} : #{dat[j]}"
        end
    end
end
