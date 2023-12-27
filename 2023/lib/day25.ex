defmodule Day25 do
  def parse_input(file) do
    file
    |> File.read!()
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      [node, bridges] = String.split(line, ":", trim: true)
      {node, String.split(bridges, " ", trim: true)}
    end)
    |> Enum.reduce(%{}, fn {node, bridges}, graph ->
      Enum.reduce(bridges, graph, fn bridge, graph ->
        Map.update(graph, node, [{bridge, 1}], &[{bridge, 1} | &1])
        |> Map.update(bridge, [{node, 1}], &[{node, 1} | &1])
      end)
    end)
  end

  def hot_link(graph, top) do
    graph
    |> Enum.reduce(MapSet.new(), fn {node, neighbors}, pairs ->
      graph
      |> Enum.map(&elem(&1, 0))
      |> Enum.reject(&(&1 == node or Enum.member?(neighbors, {&1, 1})))
      |> Enum.reduce(pairs, fn node2, pairs ->
        {a, b} = if(node < node2, do: {node, node2}, else: {node2, node})
        MapSet.put(pairs, {a, b})
      end)
    end)
    |> Enum.shuffle()
    |> Enum.take(top)
    |> Enum.map(&Aoc.dijkstra(graph, elem(&1, 0), elem(&1, 1)))
    |> Enum.reduce(%{}, fn edges, m ->
      Enum.reduce(tl(edges), {m, hd(edges)}, fn cur, {m, node} ->
        {a, b} = if node < cur, do: {node, cur}, else: {cur, node}
        {Map.update(m, {a, b}, 1, fn c -> c + 1 end), cur}
      end)
      |> elem(0)
    end)
    |> Enum.sort_by(&elem(&1, 1), :desc)
    |> List.first()
    |> elem(0)
  end

  def count_nodes(_graph, [], seen), do: seen

  def count_nodes(graph, [node | nodes], seen) do
    nodes =
      Map.get(graph, node)
      |> Enum.map(&elem(&1, 0))
      |> Enum.reject(&MapSet.member?(seen, &1))
      |> Enum.concat(nodes)

    count_nodes(graph, nodes, MapSet.put(seen, node))
  end

  def solve(file, top \\ 100) do
    graph =
      parse_input(file)

    {graph, cuts} =
      1..3//1
      |> Enum.reduce({graph, []}, fn _, {graph, cuts} ->
        {a, b} =
          hot_link(graph, top)

        al =
          graph
          |> Map.get(a)
          |> List.delete({b, 1})

        bl =
          graph
          |> Map.get(b)
          |> List.delete({a, 1})

        {graph
         |> Map.put(a, al)
         |> Map.put(b, bl), [{a, b} | cuts]}
      end)

    {a, b} = List.first(cuts)
    a = count_nodes(graph, [a], MapSet.new())
    b = count_nodes(graph, [b], MapSet.new())

    MapSet.size(a) * MapSet.size(b)
  end
end
