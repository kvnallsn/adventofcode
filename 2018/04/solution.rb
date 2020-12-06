#!/usr/bin/env ruby

require 'date'

# Read in and convert to integers
dat = File.readlines('input.txt').map(&:strip)
#dat = File.readlines('input_small.txt').map(&:strip)

# Solve Part 1
line_re = /^\[(?<date>.*)\](?<rest>.*)$/
guard_re = /(?<id>\d+)/
sleep_re = /falls asleep$/
wake_re = /wakes up$/

sorted = Hash.new()

dat.each do |line|
    m = line_re.match(line)
    d = DateTime.parse(m[:date])
    sorted[d] = m[:rest]
end

id = 0
start_sleep = 0
sleep = {}
minutes = {}
sorted.sort.each do |key, value|
    m = guard_re.match(value)
    if m
        id = m[:id]

        if sleep[id].nil?
            sleep[id] = 0
        end

        if minutes[id].nil?
            minutes[id] = Array.new(60, 0)
        end
    elsif sleep_re.match(value)
        start_sleep = key.min
    elsif wake_re.match(value)
        sleep[id] += key.min - start_sleep

        for i in start_sleep..(key.min - 1) do
            minutes[id][i] += 1
        end
    end
end

guard = sleep.max_by { |k, v| v }
min = minutes[guard[0]].each_with_index.max[1]
part1 = guard[0].to_i * min

puts "Guard #{guard[0]} slept for #{guard[1]}, mostly at minute #{min}"
puts "Part 1: #{part1}"

# Solve Part 2

guard = minutes.max_by do |gid, slept|
    slept.each_with_index.max
end

part2 = guard[0].to_i * guard[1].each_with_index.max[1]
puts "Guard #{guard[0]} slept the most at minute #{guard[1].each_with_index.max[1]}"
puts "Part 2: #{part2}"
