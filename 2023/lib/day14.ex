defmodule Day14 do
  def p1_test() do
    solve("input/day14.test")
  end

  def p1() do
    solve("input/day14.txt")
  end

  def p2_test() do
    solve2("input/day14.test", 3)
  end

  def p2() do
    solve2("input/day14.txt", 1_000_000_000)
  end

  defp parse_input(file) do
    input = File.read!(file)

    width =
      ~r/\n/
      |> Regex.run(input, return: :index)
      |> Enum.map(&elem(&1, 0))
      |> List.first()

    input = String.replace(input, "\n", "")

    height = div(String.length(input), width)

    input =
      input
      |> String.codepoints()
      |> Enum.with_index()
      |> Enum.map(fn {ch, idx} -> {ch, rem(idx, width) * height + div(idx, width)} end)
      |> Enum.sort_by(&elem(&1, 1), :asc)
      |> Enum.map(&elem(&1, 0))
      |> Enum.chunk_every(height)

    {input, width, height}
  end

  defp transpose(grid, size) do
    grid
    |> List.flatten()
    |> Enum.with_index()
    |> Enum.map(fn {ch, idx} -> {ch, rem(idx, size) * size + div(idx, size)} end)
    |> Enum.sort_by(&elem(&1, 1))
    |> Enum.map(&elem(&1, 0))
    |> Enum.chunk_every(size)
  end

  defp reverse(grid) do
    grid
    |> Enum.map(&Enum.reverse(&1))
  end

  defp tilt(grid, size) do
    grid
    |> Enum.map(fn lane ->
      lane =
        lane
        |> Enum.reduce({[], []}, fn ch, {gap, nxt} ->
          case ch do
            "." ->
              {["." | gap], nxt}

            "O" ->
              {gap, ["O" | nxt]}

            "#" ->
              case length(gap) do
                0 -> {[], ["#" | nxt]}
                _ -> {[], ["#" | [gap | nxt]]}
              end
          end
        end)
        |> elem(1)
        |> List.flatten()
        |> Enum.reverse()

      case length(lane) < size do
        true ->
          [lane | 1..(size - length(lane))//1 |> Enum.map(fn _ -> "." end) |> Enum.to_list()]
          |> List.flatten()

        false ->
          lane
      end
    end)
  end

  defp cycle(grid, size) do
    # assume start in north position
    grid
    |> tilt(size)
    |> transpose(size)
    |> tilt(size)
    |> transpose(size)
    |> reverse()
    |> tilt(size)
    |> reverse()
    |> transpose(size)
    |> reverse()
    |> tilt(size)
    |> reverse()
    |> transpose(size)
  end

  defp calc_load(grid, size) do
    grid
    |> Enum.map(fn lane ->
      lane
      |> Enum.with_index()
      |> Enum.reduce(0, fn {ch, idx}, acc ->
        case ch do
          "." -> acc
          "O" -> acc + (size - idx)
          "#" -> acc
        end
      end)
    end)
    |> Enum.sum()
  end

  defp solve(file) do
    # part 1 answer: 109755

    {input, _width, height} =
      parse_input(file)

    input
    |> Enum.map(fn col ->
      col
      |> Enum.with_index()
      |> Enum.reduce({0, height}, fn {ch, idx}, {load, val} ->
        case ch do
          "." -> {load, val}
          "O" -> {load + val, val - 1}
          "#" -> {load, height - idx - 1}
        end
      end)
    end)
    |> Enum.map(&elem(&1, 0))
    |> Enum.sum()
  end

  defp solve2(file, cycles) do
    # part 2 answer: 90928

    {input, width, _height} =
      parse_input(file)

    grids =
      1..cycles//1
      |> Enum.reduce_while([input], fn _idx, lst ->
        [grid | rest] = lst

        case Enum.member?(rest, grid) do
          true ->
            {:halt, lst}

          false ->
            {:cont, [cycle(grid, width) | lst]}
        end
      end)
      |> Enum.reverse()

    cycle_offset = Enum.find_index(grids, &(&1 == List.last(grids)))
    cycle_len = length(grids) - cycle_offset - 1
    cycle_idx = rem(cycles - cycle_offset, cycle_len)

    grids
    |> Enum.drop(cycle_offset + cycle_idx)
    |> List.first()
    |> calc_load(width)
  end
end
