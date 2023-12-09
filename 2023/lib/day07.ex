defmodule Day07 do
  def p1_test() do
    part1("input/day07.test")
  end

  def p1() do
    part1("input/day07.txt")
  end

  def p2_test() do
    part2("input/day07.test")
  end

  def p2() do
    part2("input/day07.txt")
  end

  defp score_cards(hand, j) do 
    hand
    |> String.to_charlist()
    |> Enum.map(fn c ->
      case c do
        65 -> 14
        75 -> 13
        81 -> 12
        74 -> j
        84 -> 10
        x -> x - 48
      end
    end)
  end

  defp sort_order([hand, type, _bid]) do
    [hand, type]
  end

  defp sort_hands([h1, t1], [h2, t2], j) do
    if (t1 == t2) do
        score_cards(h1, j)
        |> Enum.zip(score_cards(h2, j))
        |> Enum.drop_while(fn {l, r} -> l == r end)
        |> Enum.take(1)
        |> Enum.reduce(false, fn {l, r}, _ -> l < r end)
      
    else
      t1 <= t2
    end
  end

  defp common(file) do
    File.read!(file)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line -> String.split(line, " ", trim: true) end)
  end

  defp hand_type(hand) do
    hand
    |> case do
      [5] -> 7              # five of a kind
      [4, 1] -> 6           # four of a kind
      [3, 2] -> 5           # full house
      [3, 1, 1] -> 4        # three of a kind
      [2, 2, 1] -> 3        # two pair
      [2, 1, 1, 1] -> 2     # one pair
      [1, 1, 1, 1, 1] -> 1  # high card
      _ -> 0                # bad input
    end
  end
  
  defp parse_hand([hand, bid]) do
    type = hand
    |> String.to_charlist()
    |> Enum.group_by(&{&1})
    |> Enum.map(fn {{_}, v} -> length(v) end)
    |> Enum.sort(:desc)
    |> hand_type()

    [hand, type, String.to_integer(bid)]
  end

  defp parse_hand_p2([hand, bid]) do
    {cards, jokers} = hand
    |> String.to_charlist()
    |> Enum.group_by(&{&1})
    |> Enum.map(fn {{card}, v} -> [card, length(v)] end)
    |> Enum.reduce({[], 0}, fn [card, len], {tmp, count} -> 
      if card == 74 do
        # joker
        {tmp, len}
      else
        {[len | tmp], count}
      end
    end)

    [best | rest] = if jokers < 5, do: cards |> Enum.sort(:desc), else: [0]

    type = [best + jokers | rest]
    |> hand_type()

    [hand, type, String.to_integer(bid)]
  end

  defp part1(file) do
    # answer: 251029473

    common(file)
    |> Enum.map(&parse_hand/1)
    |> Enum.sort_by(&sort_order/1, fn l, r -> sort_hands(l, r, 11) end)
    |> Enum.with_index(1)
    |> Enum.reduce(0, fn {[_, _, bid], idx}, acc -> acc + (bid * idx) end )
  end

  defp part2(file) do
    # answer: 251003917

    common(file)
    |> Enum.map(&parse_hand_p2/1)
    |> Enum.sort_by(&sort_order/1, fn l, r -> sort_hands(l, r, 0) end)
    |> Enum.with_index(1)
    |> Enum.reduce(0, fn {[_, _, bid], idx}, acc -> acc + (bid * idx) end )
  end
end
