#!/usr/bin/env ruby

require 'set'

def complete(graph, deps, completed, node)
    if not deps[node].nil?
        if not deps[node].subset?(completed)
            return
        end
    end

    completed.add(node) 
    if not graph[node].nil?
        graph[node].sort.each do |s|
            complete(graph, deps, completed, s)
        end
    end
end

def can_complete(graph, deps, completed, node)
    # Search for next lexographical node
    if not deps[node].nil?
        return deps[node].subset?(completed)
    end

    return true
end

def find_next(graph, deps, completed, node)
    if not deps[node].nil?
        if not deps[node].subset?(completed)
            return nil
        end 
    end

    if not completed.include?(node)
        return node
    else
        if not graph[node].nil?
        end

    end
end

dat = File.readlines('input.txt').map(&:strip)
#dat = File.readlines('input_small.txt').map(&:strip)

# Solve Part 1
deps = Hash.new
graph2 = Hash.new

has_preqs = Set.new
all_nodes = Set.new
completed = Set.new

line_re = /^Step (?<step>[A-Z]) must be finished before step (?<pred>[A-Z]) can begin.$/
dat.each do |line|
    m = line_re.match(line) 
    #puts "> #{m[:step]} before #{m[:pred]}"
    has_preqs.add(m[:pred])
    all_nodes.add(m[:pred])
    all_nodes.add(m[:step])

    if not deps.has_key?(m[:pred])
        deps[m[:pred]] = Set.new
    end
    deps[m[:pred]].add(m[:step])

    if not graph2.has_key?(m[:step])
        graph2[m[:step]] = SortedSet.new
    end
    graph2[m[:step]].add(m[:pred])
end

graph2.sort_by { |k, v| k }.each do |k, v|
    print "#{k}: "
    v.each { |s| print "#{s} " }
    print "\n"
end
    
puts ""

start = (all_nodes - has_preqs).sort
while not all_nodes == completed
    start.each do |node| 
        # traverse to find next node
        if can_complete(graph2, deps, completed, node)
            completed.add(node)
        end
    end
end

#start.each do |node|
#    puts "Start #{node}"
    #complete(graph2, deps, completed, node)
#    completed.each { |c| print "#{c}" }
#    print "\n"
#end

completed.each { |c| print "#{c}" }
print "\n"

# Solve Part 2
