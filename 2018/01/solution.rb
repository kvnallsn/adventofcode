#!/usr/bin/env ruby
require 'set'

# Read in and convert to integers
dat = File.readlines('input.txt').map { |s|
   Integer(s)
}

# Solve Part 1
sum = 0
dat.each { |i| sum += i }
puts "Part 1: #{sum}"

# Solve Part 2
freqs = Set[0]
sum = 0
stop = 0
while stop == 0
    dat.each { |i|
        sum += i
        if freqs.add?(sum) == nil
            stop = 1
            break
        end
    }
end

puts "Part 2: #{sum}"
