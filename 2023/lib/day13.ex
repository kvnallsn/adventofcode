defmodule Day13 do
  def p1_test() do
    solve("input/day13.test", 0)
  end

  def p1() do
    solve("input/day13.txt", 0)
  end

  def p2_test() do
    solve("input/day13.test", 1)
  end

  def p2() do
    solve("input/day13.txt", 1)
  end

  defp parse_input(file) do
    file
    |> File.read!()
    |> String.split("\n\n")
    |> Enum.map(&parse_grid/1)
  end

  defp parse_grid(grid) do
    [{width, _}] =
      ~r/\n/
      |> Regex.run(grid, return: :index)
      |> Enum.take(1)

    rows =
      grid
      |> String.split("\n", trim: true)
      |> Enum.map(&String.codepoints/1)

    height = length(rows)

    cols =
      grid
      |> String.replace("\n", "")
      |> String.codepoints()
      |> Enum.with_index()
      |> Enum.map(fn {ch, idx} -> {ch, rem(idx, width) * height + div(idx, width)} end)
      |> Enum.sort_by(&elem(&1, 1), :asc)
      |> Enum.map(&elem(&1, 0))
      |> Enum.chunk_every(height)

    {rows, cols, width, height}
  end

  defp calc_difference(l, r, acc), do: acc + if(l == r, do: 0, else: 1)

  defp find_reflect(grid, target, skip) do
    [first | rest] =
      Enum.drop(grid, skip)

    rest
    |> Enum.with_index()
    |> Enum.reduce_while({nil, first}, fn {line, idx}, {_reflect, prev} ->
      case Enum.zip_reduce(line, prev, 0, &calc_difference/3) do
        0 -> {:halt, {idx, []}}
        1 when target == 1 -> {:halt, {idx, []}}
        _ -> {:cont, {nil, line}}
      end
    end)
    |> elem(0)
    |> case do
      nil ->
        :not_found

      idx ->
        rest
        |> Enum.drop(idx)
        |> Enum.zip(grid |> Enum.take(skip + idx + 1) |> Enum.reverse())
        |> Enum.map(fn {l, r} -> Enum.zip_reduce(l, r, 0, &calc_difference/3) end)
        |> Enum.sum()
        |> case do
          x when x == target -> {:found, idx}
          _ -> {:bad, idx}
        end
    end
  end

  defp reflections(grid, max, target) do
    1..max//1
    |> Enum.reduce_while(0, fn _, offset ->
      case find_reflect(grid, target, offset) do
        :not_found -> {:halt, :not_found}
        {:found, idx} -> {:halt, offset + idx + 1}
        {:bad, idx} -> {:cont, offset + idx + 1}
      end
    end)
  end

  defp solve(file, target) do
    # part 1 answer: 37975
    # part 2 answer: 32497

    parse_input(file)
    |> Enum.with_index()
    |> Enum.map(fn {{rows, cols, w, h}, idx} ->
      reflections(rows, h, target)
      |> case do
        :not_found ->
          case reflections(cols, w, target) do
            :not_found -> throw("not found #{idx}")
            x -> x
          end

        x ->
          x * 100
      end
    end)
    |> Enum.sum()
  end
end
