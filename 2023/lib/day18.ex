defmodule Day18 do
  def p1_test() do
    solve("input/day18.test", false)
  end

  def p1() do
    solve("input/day18.txt", false)
  end

  def p2_test() do
    solve("input/day18.test", true)
  end

  def p2() do
    solve("input/day18.txt", true)
  end

  defp parse_dir(i) do
    case i do
      "0" -> "R"
      "1" -> "D"
      "2" -> "L"
      "3" -> "U"
    end
  end

  defp parse_input(file, decode) do
    input =
      file
      |> File.read!()
      |> String.split("\n", trim: true)
      |> Enum.map(
        &Regex.run(~r/(R|U|D|L)\s+([0-9]+)\s+\(#([0-9a-z]{5})([0-3])\)/, &1,
          capture: :all_but_first
        )
      )

    case decode do
      true ->
        Enum.map(input, fn [_, _, amt, dir] -> [parse_dir(dir), String.to_integer(amt, 16)] end)

      false ->
        Enum.map(input, fn [dir, amt, _, _] -> [dir, String.to_integer(amt)] end)
    end
  end

  def shoelace(points) do
    points
    |> Enum.reduce({0, List.last(points)}, fn {x1, y1}, {sum, {x0, y0}} ->
      {sum + (y0 * x1 - x0 * y1), {x1, y1}}
    end)
    |> elem(0)
    |> div(2)
  end

  def picks(area, edges) do
    area - edges / 2 + 1
  end

  def solve(file, decode) do
    {_last, len, coords} =
      parse_input(file, decode)
      |> Enum.reduce({{0, 0}, 0, []}, fn [dir, amt], {{x, y}, len, coords} ->
        coord =
          case dir do
            "R" -> {x + amt, y}
            "L" -> {x - amt, y}
            "D" -> {x, y + amt}
            "U" -> {x, y - amt}
          end

        {coord, len + amt, [coord | coords]}
      end)

    coords
    |> shoelace()
    |> picks(len)
    |> Kernel.+(len)
  end
end
