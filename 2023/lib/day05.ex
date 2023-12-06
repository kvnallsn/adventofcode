defmodule Day05 do
  def p1_test() do
    part1("input/day05.test")
  end

  def p1() do
    part1("input/day05.txt")
  end

  def p2_test() do
    part2("input/day05.test")
  end

  def p2() do
    part2("input/day05.txt")
  end

  defp extract_nums(s) do
    Regex.scan(~r/(\d+)/, s)
    |> Enum.map(fn [_, num] -> String.to_integer(num) end)
  end

  def common(file) do
    [seeds | maps] =
      File.read!(file)
      |> String.split("\n\n")

    seeds = extract_nums(seeds)

    maps =
      maps
      |> Enum.map(fn lines ->
        [_name | nums] =
          String.split(lines, "\n") |> Enum.filter(fn line -> String.length(line) > 0 end)

        nums
        |> Enum.map(fn line ->
          Regex.run(~r/(\d+)\s+(\d+)\s+(\d+)/, line, capture: :all_but_first)
          |> Enum.map(fn n -> String.to_integer(n) end)
        end)
        |> Enum.map(fn [dst, start, len] ->
          [start, start + len - 1, dst - start]
        end)
      end)

    {seeds, maps}
  end

  defp find_min(seeds, maps) do
    seeds
    |> Stream.map(fn seed ->
      maps
      |> Enum.reduce(seed, fn maps, acc ->
        maps
        |> Enum.filter(fn [start, stop, _] -> acc >= start and acc <= stop end)
        |> case do
          [] -> acc
          [[_, _, offset] | _] -> acc + offset
        end
      end)
    end)
    |> Enum.min()
  end

  defp part1(file) do
    # answer: 806029445

    {seeds, maps} =
      common(file)

    find_min(seeds, maps)
  end

  defp part2(file) do
    # answer: 59370572

    {seeds, maps} =
      common(file)

    seeds
    |> Enum.chunk_every(2)
    |> Task.async_stream(fn [start, stop] -> find_min(start..(start + stop - 1)//1, maps) end,
      timeout: :infinity
    )
    |> Enum.map(fn {_, val} -> val end)
    |> Enum.min()
  end
end
