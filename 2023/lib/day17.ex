defmodule Day17 do
  def p1_test() do
    solve("input/day17.test", 1, 3)
  end

  def p1() do
    solve("input/day17.txt", 1, 3)
  end

  def p2_test() do
    solve("input/day17.test", 4, 10)
  end

  def p2() do
    solve("input/day17.txt", 4, 10)
  end

  defp parse_input(file) do
    input = File.read!(file)

    width =
      ~r/\n/
      |> Regex.run(input, return: :index)
      |> Enum.map(&elem(&1, 0))
      |> List.first()

    grid =
      input
      |> String.replace("\n", "")
      |> String.codepoints()
      |> Enum.with_index()
      |> Enum.reduce(%{}, fn {n, idx}, m ->
        Map.put(m, idx, String.to_integer(n))
      end)

    {grid, width, length(Map.keys(grid))}
  end

  defp next({idx, dir}, width, max, dist) do
    1..dist//1
    |> Enum.map(
      &[
        {idx - &1, :left, &1},
        {idx + &1, :right, &1},
        {idx - &1 * width, :up, &1},
        {idx + &1 * width, :down, &1}
      ]
    )
    |> List.flatten()
    |> Enum.filter(fn {i, d, _} ->
      {idx_row, i_row} = {div(idx, width), div(i, width)}

      case d do
        _ when i <= 0 or i >= max -> false
        x when x == dir -> false
        :left when dir == :right -> false
        :right when dir == :left -> false
        :up when dir == :down -> false
        :down when dir == :up -> false
        :left when idx_row != i_row -> false
        :right when idx_row != i_row -> false
        _ -> true
      end
    end)
    |> Enum.sort_by(&elem(&1, 1))
    |> Enum.chunk_by(&elem(&1, 1))
  end

  def find_path(_grid, _mindist, _maxdist, [], _visited, _dists) do
    throw("unable to find path")
  end

  def find_path({grid, width, len}, mindist, maxdist, [node | rest], visited, dists) do
    {cost, current} = node
    {cidx, _} = current

    case cidx == len - 1 do
      true ->
        cost

      false ->
        visited = MapSet.put(visited, current)

        {_, dists, to_visit} =
          next(current, width, len, maxdist)
          |> Enum.reduce({0, dists, []}, fn dirs, {_, dists, to_visit} ->
            Enum.reduce(dirs, {0, dists, to_visit}, fn {idx, dir, d}, {costi, dists, to_visit} ->
              costi = costi + Map.get(grid, idx)

              with true <- d >= mindist,
                   alt <- cost + costi,
                   cc <- Map.get(dists, {idx, dir}, 999_999_999),
                   true <- alt < cc do
                {costi, Map.put(dists, {idx, dir}, alt), [{alt, {idx, dir}} | to_visit]}
              else
                _ -> {costi, dists, to_visit}
              end
            end)
          end)

        # first sort the new nodes, then insertion sort (kind of) into the list
        to_visit =
          to_visit
          |> Enum.sort_by(&elem(&1, 0), :asc)
          |> Enum.reduce(rest, fn {cost, node}, q ->
            q
            |> Enum.with_index()
            |> Enum.drop_while(fn {{qc, _}, _} -> qc < cost end)
            |> List.first()
            |> case do
              nil -> q ++ [{cost, node}]
              {_, idx} -> List.insert_at(q, idx, {cost, node})
            end
          end)
          |> Enum.filter(fn {_, node} -> !MapSet.member?(visited, node) end)

        find_path({grid, width, len}, mindist, maxdist, to_visit, visited, dists)
    end
  end

  def solve(file, mindist, maxdist) do
    # part 1: 1260
    # part 2: 1416

    parse_input(file)
    |> find_path(mindist, maxdist, [{0, {0, :start}}], MapSet.new(), %{{0, :start} => 0})
  end
end
