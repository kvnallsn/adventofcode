#!/usr/bin/env ruby

require 'date'

def reduce(txt)
    combos = /aA|bB|cC|dD|eE|fF|gG|hH|iI|jJ|kK|lL|mM|nN|oO|pP|qQ|rR|sS|tT|uU|vV|wW|xX|yY|zZ|Zz|Yy|Xx|Ww|Vv|Uu|Tt|Ss|Rr|Qq|Pp|Oo|Nn|Mm|Ll|Kk|Jj|Ii|Hh|Gg|Ff|Ee|Dd|Cc|Bb|Aa/

    loop do
        nxt = txt.gsub!(combos, '')
        break if nxt.nil?
        txt = nxt
    end

    return txt
end

# Read in and convert to integers
dat = File.read('input.txt')
#dat = File.read('input_small.txt')

# Solve Part 1

puts "Start: #{dat.length}"
part1 = reduce(dat.dup)
puts "Part 1: #{part1.length}"

# Solve Part 2
part2 = dat.tr('x', '').tr('X', '')
part2 = reduce(part2)
puts "Part 2: x: #{part2.length}"

