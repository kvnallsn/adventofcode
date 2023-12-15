defmodule Day15 do
  def p1_test() do
    solve("input/day15.test")
  end

  def p1() do
    solve("input/day15.txt")
  end

  def p2_test() do
    solve2("input/day15.test")
  end

  def p2() do
    solve2("input/day15.txt")
  end

  defp hash(s) when is_binary(s), do: hash(String.to_charlist(s))

  defp hash(s) when is_list(s) do
    Enum.reduce(s, 0, fn ch, acc -> rem((acc + ch) * 17, 256) end)
  end

  def solve(file) do
    file
    |> File.read!()
    |> String.replace("\n", "")
    |> String.split(",", trim: true)
    |> Enum.map(&hash/1)
    |> Enum.sum()
  end

  def solve2(file) do
    input =
      file
      |> File.read!()
      |> String.replace("\n", "")

    ~r/([a-z]+)(=|-)([1-9]?)/
    |> Regex.scan(input, capture: :all_but_first)
    |> Enum.reduce(%{}, fn [label, op, lens], m ->
      idx = hash(label)
      box = Map.get(m, idx, [])

      box =
        case op do
          "=" ->
            case Enum.find_index(box, &(elem(&1, 0) == label)) do
              nil -> box ++ [{label, lens}]
              i -> List.replace_at(box, i, {label, lens})
            end

          "-" ->
            Enum.filter(box, &(elem(&1, 0) != label))
        end

      Map.put(m, idx, box)
    end)
    |> Enum.map(fn {box, lenses} ->
      lenses
      |> Enum.with_index()
      |> Enum.map(fn {{_, lense}, idx} -> (box + 1) * (idx + 1) * String.to_integer(lense) end)
      |> Enum.sum()
    end)
    |> Enum.sum()
  end
end
