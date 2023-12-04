defmodule Day04 do
 
  def p1_test() do
    part1("input/day04.test")
  end

  def p1() do
    part1("input/day04.txt")
  end

  def p2_test() do
    part2("input/day04.test")
  end

  def p2() do
    part2("input/day04.txt")
  end

  defp extract_nums(s) do
    Regex.scan(~r/(\d+)/, s) 
    |> Enum.map(fn [_, num] -> String.to_integer(num) end)
  end

  defp card_num(s) do
    [_, num] = Regex.run(~r/Card\s+(\d+)/, s)
    String.to_integer(num)
  end

  defp common(file) do
    File.read!(file)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line -> String.split(line, [":", "|"], trim: true) end)
    |> Enum.map(fn [card, win, mine] -> [card, extract_nums(win) ++ extract_nums(mine)] end)
    |> Enum.map(fn [card, nums] -> [card, Enum.frequencies(nums)] end)
    |> Enum.map(fn [card, freqs] -> [card, Enum.filter(freqs, fn {_, count} -> count > 1 end)] end)
  end

  defp part1(file) do
    common(file)
    |> Enum.filter(fn [_, freqs] -> !Enum.empty?(freqs) end)
    |> Enum.map(fn [card, freqs] -> [card, Integer.pow(2, length(freqs) - 1)] end)
    |> Enum.map(fn [_, points] -> points end)
    |> Enum.sum()
  end

  defp part2(file) do
    # answer 6874754

    common(file)
      |> Enum.map(fn [card, wins] -> [card_num(card), length(wins)] end)
      |> Enum.reduce(%{}, fn [card, wins], acc ->
        acc = Map.update(acc, card, 1, fn current -> current + 1 end)
        count = Map.get(acc, card, 1)

        (card + 1)..(card + wins)//1
        |> Enum.to_list()
        |> Enum.reduce(acc, fn c, bcc ->
          Map.update(bcc, c, 1, fn v -> v + count end)
        end)
    end)
    |> Enum.map(fn {_card, count} -> count end)
    |> Enum.sum()
  end
end
