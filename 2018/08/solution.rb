#!/usr/bin/env ruby

require 'ostruct'

$idx = 0

def process_node(dat, info)
    num_children = dat[info.i]
    info.i += 1
    num_meta = dat[info.i]
    info.i += 1

    puts "Children #{num_children}, Meta: #{num_meta}"

    dat[info.i..(info.i+num_children-1)].each do 
        #puts ">> processing child #{dat[info.i]}"
        process_node(dat, info)
    end

    dat[info.i..(info.i+num_meta-1)].each do 
        #puts ">> processing meta #{dat[info.i]}"
        info.sum += dat[info.i]
        info.i += 1
    end
end

# Read in and convert to integers
dat = File.read('input.txt').split(/\s/).map(&:to_i)
#dat = File.read('input_small.txt').split(/\s/).map(&:to_i)

puts dat

# Solve Part 1
info = OpenStruct.new
info.i = 0
info.sum = 0
process_node(dat, info)

puts info

# Solve Part 2

