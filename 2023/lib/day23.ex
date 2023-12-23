defmodule Day23 do
  def parse_input(file, xform \\ fn x -> x end) do
    input = File.read!(file)
    width = Regex.run(~r/\n/, input, return: :index) |> List.first() |> elem(0)

    grid =
      input
      |> String.codepoints()
      |> Enum.reject(&(&1 == "\n"))
      |> Enum.map(&xform.(&1))
      |> Enum.with_index()
      |> Enum.reduce(%{}, &Map.put(&2, elem(&1, 1), elem(&1, 0)))

    {grid, width}
  end

  def next({grid, width}, ch, pos, weight \\ 1) do
    case ch do
      ">" -> [pos + 1]
      "<" -> [pos - 1]
      "^" -> [pos - width]
      "v" -> [pos + width]
      "." -> [pos - 1, pos + 1, pos - width, pos + width]
      _ -> []
    end
    |> Enum.reject(&(Map.get(grid, &1) == "#" or Map.get(grid, &1) == nil))
    |> Enum.map(&{&1, weight})
  end

  def solve(file) do
    graph =
      file
      |> parse_input()
      |> Aoc.build_graph(&next/3)

    {start, goal} =
      graph
      |> Map.keys()
      |> Enum.min_max()

    Aoc.dfs(graph, start, goal)
    |> Map.to_list()
    |> List.first()
    |> elem(1)
    |> Enum.max()
  end

  def solve2(file) do
    graph =
      file
      |> parse_input(&if(&1 == "#", do: "#", else: "."))
      |> Aoc.build_graph(&next/3)

    {start, goal} =
      graph
      |> Map.keys()
      |> Enum.min_max()

    verts =
      graph
      |> Enum.filter(&(length(elem(&1, 1)) > 2))
      |> Enum.reduce(MapSet.new([start, goal]), &MapSet.put(&2, elem(&1, 0)))

    edges =
      verts
      |> Enum.map(&{&1, Aoc.dfs(graph, &1, MapSet.delete(verts, &1))})
      |> Enum.reduce(%{}, fn {pos, coords}, edges ->
        Map.put(edges, pos, Enum.map(coords, &{elem(&1, 0), Enum.max(elem(&1, 1))}))
      end)

    Aoc.dfs(edges, start, goal)
    |> Map.to_list()
    |> List.first()
    |> elem(1)
    |> Enum.max()
  end
end
