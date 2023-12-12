defmodule Day10 do
  @vertical_pipes ["|", "L", "J", "7", "F"]

  def p1_test() do
    part1("input/day10.test", 7, "F")
  end

  def p1() do
    part1("input/day10.txt", 142, "7")
  end

  def p2_test() do
    # part2("input/day10.test", 7, "F")
    # part2("input/day10.2.test", 11, "F")
    part2("input/day10.3.test", 22, "F")
  end

  def p2() do
    part2("input/day10.txt", 142, "7")
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
      |> Enum.map(fn {ch, idx} ->
        case Enum.member?(path, idx) do
          true -> ch
          false -> "."
        end
      end)
      |> Enum.map(fn ch ->
        case ch do
          "-" -> <<0x2500::utf8>>
          "|" -> <<0x2502::utf8>>
          "F" -> <<0x250C::utf8>>
          "7" -> <<0x2510::utf8>>
          "L" -> <<0x2514::utf8>>
          "J" -> <<0x2518::utf8>>
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

    print_grid("#{file}.out", grid, width, path)

    path
    |> length()
    |> Kernel.div(2)
  end

  defp part2(file, width, start_pipe) do
    # answer: 

    {grid, start} =
      common(file, start_pipe)

    path =
      trace_path(grid, width, start, start_pipe)

    locs =
      path
      |> Enum.reduce(%{}, fn {idx, _pipe, loc}, m -> Map.put(m, idx, loc) end)

    sorter = fn {idx, _pipe, _loc} -> idx end
    # chunker = fn {idx, _pipe, _loc} -> Kernel.div(idx, width) end
    chunker = fn {idx, _} -> Kernel.div(idx, width) end

    walls =
      path
      |> Enum.sort_by(&sorter.(&1), :asc)
      |> Enum.filter(fn {_idx, pipe, _loc} -> pipe != "-" end)
      |> Enum.reduce(%{}, fn {idx, _pipe, loc}, m ->
        if Map.get(locs, idx + width) == loc + 1 do
          Map.put(m, idx, :enter)
        else
          Map.put(m, idx, :leave)
        end
      end)

    walls
    |> Map.to_list()
    |> Enum.sort()
    |> Enum.chunk_by(&chunker.(&1))
    |> IO.inspect(charlists: :as_lists)

    path = path |> Enum.map(fn {idx, _, _} -> idx end)

    grid
    |> Enum.with_index()
    |> Enum.map(fn {ch, idx} ->
      case Enum.member?(path, idx) do
        true -> {ch, idx}
        false -> {".", idx}
      end
    end)
    |> Enum.reduce({0, false}, fn {ch, idx}, {count, state} ->
      case Map.get(walls, idx) do
        :enter ->
          {count, true}

        :leave ->
          {count, false}

        nil ->
          case {ch, state} do
            {".", true} -> {count + 1, state}
            _ -> {count, state}
          end
      end
    end)

    # path =
    #  path
    #  |> Enum.map(fn {idx, _} -> idx end)

    # grid
    # |> Enum.with_index()
    # |> Enum.map(fn {ch, idx} ->
    #  case Enum.member?(path, idx) do
    #    true -> ch
    #    false -> "."
    #  end
    # end)
    # |> Enum.chunk_every(width)
    # |> IO.inspect(charlists: :as_lists)
    # |> Enum.map(fn row ->
    #  Enum.reduce(row, {0, false}, fn ch, {count, state} ->
    #    c = count + 1

    #    case ch do
    #      "L" -> {count, !state}
    #      "F" -> {count, !state}
    #      "|" -> {count, !state}
    #      "J" -> {count, !state}
    #      "." when state -> {c, state}
    #      _ -> {count, state}
    #    end
    #  end)
    # end)
    # |> IO.inspect(charlists: :as_lists)

    # fout = File.open!("input/day10.out", [:write])

    # path
    # |> Enum.filter(fn {_, pipe} -> Enum.member?(@vertical_pipes, pipe) end)
    # |> Enum.sort_by(&sorter.(&1))
    # |> Enum.chunk_by(&chunker.(&1))
    # |> IO.inspect(charlists: :as_lists)
    # |> Enum.reduce(0, fn [start | rest], count ->
    #  Enum.reduce(rest, start, fn coord, acc ->
    #    acc
    #  end)
    #  |> IO.inspect(charlists: :as_lists)
    #
    #  count
    # end)
  end
end
