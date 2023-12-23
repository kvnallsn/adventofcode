defmodule Aoc do
  @moduledoc """
  Documentation for `Aoc`.
  """

  @doc """
  Builds a graph structure from an 2d input grid represented in 1d space as
  as list of {index, ch} tuples

  Inputs:
  * grid: An enumerable of {index, character} tuples
  * width: width of one row of the 2d input grid
  * adj_fn: An adjacency function that returns a list of next possible nodes for a given input value,
            or an empty list if there are no possible moves from this node

  Outputs:
  * A map of nodes to their neighbors and the weight of each link (e.g., %{1 -> [{2, 10}, {3, 940}]})
  """
  def build_graph({grid, width}, adj_fn) do
    grid
    |> Enum.reduce(%{}, fn {idx, ch}, graph ->
      case adj_fn.({grid, width}, ch, idx) do
        [] -> graph
        coords -> Map.put(graph, idx, coords)
      end
    end)
  end

  @doc """
  Depth First Search (DFS)

  Performs a depth first search to reach all goals.  Only stops
  when the entire graph has been searched.

  Inputs:
  * graph: Map of the index position to a list of connected nodes and their respective weights
  * start: The index of the starting node
  * goals: An single node (integer) or an enumerable of all goal nodes to search for
  
  Outputs:
  A map of the goal nodes and the distance/counts to reach them

  ## Examples

      iex> Aoc.dfs(graph, 0, [3244, 964, 453])
      %{3244 => [324, 4532, 433], 954 => [1234], 453 => []}

  """
  def dfs(graph, start, goals, output \\ %{})

  def dfs(graph, start, goal, output) when is_integer(start) and is_integer(goal),
    do: dfs(graph, [{start, 0, MapSet.new()}], MapSet.new([goal]), output)

  def dfs(graph, start, goals, output) when is_integer(start),
    do: dfs(graph, [{start, 0, MapSet.new()}], goals, output)

  def dfs(_graph, [], _goal, output), do: output

  def dfs(graph, [{tile, count, visited} | rest], goals, output) do
    case Enum.member?(goals, tile) do
      true ->
        dfs(graph, rest, goals, Map.update(output, tile, [count], &[count | &1]))

      false ->
        nxt =
          Map.get(graph, tile)
          |> Enum.reject(&MapSet.member?(visited, elem(&1, 0)))
          |> Enum.map(&{elem(&1, 0), count + elem(&1, 1), MapSet.put(visited, tile)})

        dfs(graph, nxt ++ rest, goals, output)
    end
  end
end
