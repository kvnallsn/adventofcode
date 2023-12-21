defmodule Day21 do
  def parse_input(file) do
    tmp =
      file
      |> File.read!()
      |> String.split("\n")
      |> Enum.map(&(&1 <> &1 <> &1 <> &1 <> &1))
      |> Enum.join()

    tmp = tmp <> tmp <> tmp <> tmp <> tmp

    tmp
    |> String.codepoints()
    |> Enum.with_index()
    |> Enum.reduce(%{}, &Map.put(&2, elem(&1, 1), elem(&1, 0)))
  end

  def step(grid, width, pos) do
    rpos = rem(pos, width)

    [pos - 1, pos + 1, pos - width, pos + width]
    |> Enum.reject(&(&1 < 0))
    |> Enum.reject(&(&1 >= width * width))
    |> Enum.reject(&(rpos == 0 and rem(&1, width) == width - 1))
    |> Enum.reject(&(rpos == width - 1 and rem(&1, width) == 0))
    |> Enum.reject(&(Map.get(grid, &1) == "#"))
  end

  def walk({_grid, _width}, _max, [], visited), do: visited

  def walk({grid, width}, max, {pos, step}, visited) do
    case step do
      s when s > max ->
        visited

      _ ->
        visited = MapSet.put(visited, {pos, step})

        grid
        |> step(width, pos)
        |> Enum.map(&{&1, step + 1})
        |> Enum.reject(&(elem(&1, 1) > max))
        |> Enum.reject(&MapSet.member?(visited, &1))
        |> Enum.reduce(visited, &walk({grid, width}, max, &1, &2))
    end
  end

  def p1() do
    solve("input/day21.txt", 64)
  end

  def p2() do
    solve2(div(26_501_365, 131))
  end

  def solve(file, max) do
    width = 655
    start = 214_512
    grid = parse_input(file)

    walk({grid, width}, max, {start, 0}, MapSet.new())
    |> Enum.filter(&(elem(&1, 1) == max))
    |> length()
  end

  def solve2(n) do
    # a0 = solve(file, 65, 1)
    a0 = 3867
    # a1 = solve(file, 196, 1)
    a1 = 34253
    # a2 = solve(file, 327, 1)
    a2 = 94909

    b0 = a0
    b1 = a1 - a0
    b2 = a2 - a1

    b0 + b1 * n + div(n * (n - 1), 2) * (b2 - b1)
  end
end
