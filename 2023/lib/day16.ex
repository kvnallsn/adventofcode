defmodule Day16 do
  def p1_test() do
    solve("input/day16.test")
  end

  def p1() do
    solve("input/day16.txt")
  end

  def p2_test() do
    solve2("input/day16.test")
  end

  def p2() do
    solve2("input/day16.txt")
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

    {grid, width}
  end

  defp next(idx, dir, width) do
    nxt =
      case dir do
        :right -> idx + 1
        :left -> idx - 1
        :up -> idx - width
        :down -> idx + width
      end

    {nxt, dir}
  end

  defp navigate(_idx, {grid, width, beams, energized}) do
    energized = beams ++ energized

    beams =
      beams
      |> Enum.map(fn {idx, dir} ->
        case Enum.at(grid, idx) do
          "." -> next(idx, dir, width)
          "/" when dir == :right -> next(idx, :up, width)
          "/" when dir == :left -> next(idx, :down, width)
          "/" when dir == :up -> next(idx, :right, width)
          "/" when dir == :down -> next(idx, :left, width)
          "\\" when dir == :right -> next(idx, :down, width)
          "\\" when dir == :left -> next(idx, :up, width)
          "\\" when dir == :up -> next(idx, :left, width)
          "\\" when dir == :down -> next(idx, :right, width)
          "|" when dir == :up or dir == :down -> next(idx, dir, width)
          "-" when dir == :left or dir == :right -> next(idx, dir, width)
          "|" -> [next(idx, :up, width), next(idx, :down, width)]
          "-" -> [next(idx, :left, width), next(idx, :right, width)]
        end
      end)
      |> List.flatten()
      |> Enum.filter(fn {idx, dir} ->
        !(idx < 0 or idx >= length(grid) or (dir == :left and rem(idx, width) == width - 1) or
            (dir == :right and rem(idx, width) == 0))
      end)
      |> Enum.filter(&(!Enum.member?(energized, &1)))

    next = {grid, width, beams, energized}

    case length(beams) do
      0 -> {:halt, next}
      _ -> {:cont, next}
    end
  end

  defp fire_beam({grid, width}, start) do
    0..length(grid)//1
    |> Enum.reduce_while({grid, width, [start], []}, &navigate/2)
    |> elem(3)
    |> Enum.map(&elem(&1, 0))
    |> Enum.sort()
    |> Enum.dedup()
    |> length()
  end

  def solve(file) do
    # part 1: 8389

    parse_input(file)
    |> fire_beam({0, :right})
  end

  def solve2(file) do
    # part 1: 8564

    {grid, width} = parse_input(file)

    left =
      0..(length(grid) - 1)//width
      |> Enum.to_list()
      |> Enum.map(&{&1, :right})

    right =
      (width - 1)..length(grid)//width
      |> Enum.to_list()
      |> Enum.map(&{&1, :left})

    top =
      0..(width - 1)//1
      |> Enum.to_list()
      |> Enum.map(&{&1, :down})

    bottom =
      (length(grid) - width)..(length(grid) - 1)//1
      |> Enum.to_list()
      |> Enum.map(&{&1, :up})

    (left ++ right ++ top ++ bottom)
    |> Enum.map(fn start -> fire_beam({grid, width}, start) end)
    |> Enum.max()
  end
end
