defmodule Day11 do
  def p1_test() do
    solve("input/day11.test", 10, 2)
  end

  def p1() do
    solve("input/day11.txt", 140, 2)
  end

  def p2_test() do
    # solve("input/day11.test", 10, 10)
    solve("input/day11.test", 10, 100)
  end

  def p2() do
    solve("input/day11.txt", 140, 1_000_000)
  end

  defp common(file) do
    File.read!(file)
    |> String.replace("\n", "")
    |> String.codepoints()
  end

  defp compute_offsets(lst, width, offset) do
    lst
    |> Enum.chunk_every(width)
    |> Enum.map(&Enum.all?(&1, fn x -> x == "." end))
    |> Enum.with_index()
    |> Enum.reduce({%{}, 0}, fn {has_offset, row}, {m, total_offset} ->
      offset = total_offset + if has_offset, do: offset, else: 0
      {Map.put(m, row, offset), offset}
    end)
    |> elem(0)
  end

  defp expand(universe, width, offset) do
    row_offsets =
      universe
      |> compute_offsets(width, offset)

    col_offsets =
      universe
      |> Enum.with_index()
      |> Enum.map(fn {ch, idx} -> {ch, rem(idx, width) * width + div(idx, width)} end)
      |> Enum.sort_by(&elem(&1, 1), :asc)
      |> Enum.map(fn {ch, _} -> ch end)
      |> compute_offsets(width, offset)

    {universe |> Enum.join(), row_offsets, col_offsets}
  end

  defp shortest_paths(galaxies) do
    galaxies
    |> Enum.reduce({[], galaxies}, fn galaxy, {lst, meta} ->
      [_ | rest] = meta
      {[{galaxy, rest} | lst], rest}
    end)
    |> elem(0)
    |> Enum.map(fn {{{x1, y1}, _}, lst} ->
      Enum.reduce(lst, 0, fn {{x2, y2}, _}, acc ->
        acc + abs(x2 - x1) + abs(y2 - y1)
      end)
    end)
  end

  defp solve(file, width, offset) do
    # part 1 answer: 10313550
    # part 2 answer: 611998089572

    {universe, row_offsets, col_offsets} =
      common(file)
      |> expand(width, offset - 1)

    Regex.scan(~r/(#)/, universe, capture: :all_but_first, return: :index)
    |> List.flatten()
    |> Enum.map(&elem(&1, 0))
    |> Enum.map(&{rem(&1, width), div(&1, width)})
    |> Enum.map(fn {col, row} ->
      {col + Map.get(col_offsets, col, 0), row + Map.get(row_offsets, row, 0)}
    end)
    |> Enum.with_index()
    |> shortest_paths()
    |> Enum.sum()
  end
end
