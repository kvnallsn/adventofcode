defmodule Day03 do
  defp neighbors(start, len, width, num) do
    before = Enum.to_list((start - 1 - width)..(start + len - width))
    line = [start - 1, start + len]
    aft = Enum.to_list((start - 1 + width)..(start + len + width))
    (before ++ line ++ aft) |> Enum.map(fn idx -> {idx, num} end)
  end

  def day03p1 do
    # answer: 544433

    width = 142
    # width = 12
    input = File.read!("input/day03.txt") |> String.replace("\n", "")

    symbols =
      Regex.scan(~r/[$*#+@=%\/&-]/, input, return: :index)
      |> Enum.reduce(%{}, fn [{idx, len}], acc ->
        Map.put(acc, idx, String.slice(input, idx, len))
      end)

    Regex.scan(~r/\d+/, input, return: :index)
    |> Enum.reduce([], fn [{idx, len}], acc ->
      acc ++ neighbors(idx, len, width, input |> String.slice(idx, len) |> String.to_integer())
    end)
    |> Enum.map(fn {idx, num} -> {Map.get(symbols, idx, nil), num} end)
    |> Enum.filter(fn {sym, _} -> sym != nil end)
    |> Enum.map(fn {_, num} -> num end)
    |> Enum.sum()
  end

  def day03p2 do
    # answer: 544433

    width = 142
    # width = 12
    input = File.read!("input/day03.txt") |> String.replace("\n", "")

    symbols =
      Regex.scan(~r/[*]/, input, return: :index)
      |> Enum.reduce(%{}, fn [{idx, len}], acc ->
        Map.put(acc, idx, String.slice(input, idx, len))
      end)

    Regex.scan(~r/\d+/, input, return: :index)
    |> Enum.reduce([], fn [{idx, len}], acc ->
      acc ++ neighbors(idx, len, width, input |> String.slice(idx, len) |> String.to_integer())
    end)
    |> Enum.map(fn {idx, num} -> {Map.get(symbols, idx, nil), idx, num} end)
    |> Enum.filter(fn {sym, _, _} -> sym != nil end)
    |> Enum.group_by(fn {_, idx, _} -> idx end, fn {_, _, num} -> num end)
    |> Enum.filter(fn {_, lst} -> length(lst) == 2 end)
    |> Enum.map(fn {_, lst} -> Enum.reduce(lst, 1, fn i, acc -> i * acc end) end)
    |> Enum.sum()
  end
end
