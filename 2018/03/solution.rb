#!/usr/bin/env ruby

# Read in and convert to integers
dat = File.readlines('input.txt').map(&:strip)

# Solve Part 1
line_re = /^#(?<id>\d+)\s*@\s*(?<x>\d+),(?<y>\d+):\s*(?<w>\d+)x(?<h>\d+)$/i
grid = Array.new(1000) { Array.new(1000) }
overlap = 0

dat.each do |line|
    m = line_re.match(line)
    width = m[:w].to_i
    height = m[:h].to_i
    xoffset = m[:x].to_i
    yoffset = m[:y].to_i

    for x in xoffset..(xoffset + width - 1) do
        for y in yoffset..(yoffset + height - 1) do
            grid[y][x] = grid[y][x].nil? ? m[:id] : 'X'
        end
    end
end

grid.each do |row|
    row.each do |col|
        overlap += 1 if col == 'X'
    end
end

puts "Part 1: #{overlap}"

# Solve Part 2
dat.each do |line |
    m = line_re.match(line)
    width = m[:w].to_i
    height = m[:h].to_i
    xoffset = m[:x].to_i
    yoffset = m[:y].to_i

    found = 0
    expected = width * height

    for x in xoffset..(xoffset + width - 1) do
        for y in yoffset..(yoffset + height - 1) do
            found += 1 if grid[y][x] == m[:id]
        end
    end

    if found == expected
        puts "Part 2: #{m[:id]}"
    end
end
