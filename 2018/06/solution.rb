#!/usr/bin/env ruby

require 'ostruct'

def dist(x1, y1, x2, y2)
    x = x1 - x2
    y = y1 - y2
    return x.abs + y.abs
end

marker = 'A'
line_re = /^(?<x>\d+), (?<y>\d+)$/

# Read in and convert to integers
#dat = File.readlines('input.txt').map(&:strip)
dat = File.readlines('input.txt').map(&:strip).map do |line|
    m = line_re.match(line)
    pair = OpenStruct.new
    pair.x = m[:x].to_i
    pair.y = m[:y].to_i
    pair.l = marker
    marker = marker.succ
    pair
end

sz = 375
grid1 = Array.new(sz) { Array.new(sz) }
grid2 = Array.new(sz) { Array.new(sz) }

for y in 0..(sz-1)
    for x in 0..(sz-1)
        dat.each do |pair|
            m = dist(pair.x, pair.y, x, y)
            if grid1[y][x].nil? or m < grid1[y][x] then
                grid1[y][x] = m
                grid2[y][x] = (m == 0 ? pair.l : pair.l.downcase)
            elsif m == grid1[y][x] then
                grid2[y][x] = '.'
            end
        end
    end
end

score = Hash.new 
grid2.each_with_index do |c, y|
    c.each_with_index do |r, x|
        if r == '.' then
            next
        elsif x == 0 or y == 0 or x == (sz-1) || y == (sz-1) then
            score[r.downcase] = 0
        elsif score.has_key?(r.downcase) and score[r.downcase] != 0 then
            score[r.downcase] += 1
        else
            score[r.downcase] = 1
        end
    end
end

m = score.max_by { |k, v| v }
puts m


# Solve Part 2
