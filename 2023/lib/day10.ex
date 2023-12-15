defmodule Day10 do
  def p1_test() do
    part1("input/day10.test", 7, "F")
  end

  def p1() do
    part1("input/day10.txt", 142, "7")
  end

  def p2() do
    part2("input/day10.txt", 142, "7")
  end

  def print(file, width, start_pipe) do
    {grid, start} =
      common("input/#{file}", start_pipe)

    path =
      trace_path(grid, width, start, start_pipe)

    print_grid("#{file}.out", grid, width, path)
  end

  defp common(file, start_pipe) do
    grid =
      File.read!(file)
      |> String.replace("\n", "")
      |> String.codepoints()

    start = Enum.find_index(grid, fn x -> x == "S" end)

    {grid |> Enum.map(fn c -> if c == "S", do: start_pipe, else: c end), start}
  end

  defp next(grid, width, pipe, idx, loc) do
    case pipe do
      "|" -> [idx - width, idx + width]
      "-" -> [idx - 1, idx + 1]
      "L" -> [idx - width, idx + 1]
      "J" -> [idx - width, idx - 1]
      "7" -> [idx + width, idx - 1]
      "F" -> [idx + width, idx + 1]
    end
    |> Enum.map(fn i -> {i, Enum.at(grid, i), loc + 1} end)
  end

  defp trace_path(grid, width, start, start_pipe) do
    1..(width * width)//1
    |> Enum.reduce_while([{start, start_pipe, 0}], fn _, [{idx, pipe, loc} | rest] ->
      {prev, _, _} = List.first(rest, {-1, ".", -1})

      next_pipe =
        next(grid, width, pipe, idx, loc)
        |> Enum.drop_while(fn {i, _, _} -> i == prev or i == start end)
        |> Enum.take(1)

      acc =
        [next_pipe | [{idx, pipe, loc} | rest]]
        |> List.flatten()

      case length(next_pipe) do
        0 -> {:halt, acc}
        _ -> {:cont, acc}
      end
    end)
  end

  defp print_grid(file, grid, width, path) do
    path =
      Enum.map(path, fn {idx, _, _} -> idx end)

    grid =
      grid
      |> Enum.with_index()
      |> Enum.map(&if Enum.member?(path, elem(&1, 1)), do: elem(&1, 0), else: ".")
      |> Enum.map(fn ch ->
        case ch do
          "-" -> <<0x2500::utf8>>
          "|" -> <<0x2502::utf8>>
          "F" -> <<0x256D::utf8>>
          "7" -> <<0x256E::utf8>>
          "L" -> <<0x2570::utf8>>
          "J" -> <<0x256F::utf8>>
          "." -> <<0x25CB::utf8>>
          c -> c
        end
      end)
      |> Enum.chunk_every(width)
      |> Enum.map(fn row -> Enum.join(row) end)
      |> Enum.join("\n")

    File.write!(file, grid)
  end

  defp part1(file, width, start_pipe) do
    # answer: 7005

    {grid, start} =
      common(file, start_pipe)

    path =
      trace_path(grid, width, start, start_pipe)

    path
    |> length()
    |> Kernel.div(2)
  end

  defp part2(file, width, start_pipe) do
    # answer: 417

    {grid, start} =
      common(file, start_pipe)

    path =
      trace_path(grid, width, start, start_pipe)
      |> Enum.map(&elem(&1, 0))

    grid
    |> Enum.with_index()
    |> Enum.map(&if Enum.member?(path, elem(&1, 1)), do: elem(&1, 0), else: ".")
    |> Enum.reduce({0, false}, fn ch, {count, state} ->
      case ch do
        "|" -> {count, !state}
        "J" -> {count, !state}
        "L" -> {count, !state}
        "." when state -> {count + 1, state}
        _ -> {count, state}
      end
    end)
    |> elem(0)
  end
end
