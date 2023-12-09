defmodule Day08 do
  @input_test_1 """
  RL

  AAA = (BBB, CCC)
  BBB = (DDD, EEE)
  CCC = (ZZZ, GGG)
  DDD = (DDD, DDD)
  EEE = (EEE, EEE)
  GGG = (GGG, GGG)
  ZZZ = (ZZZ, ZZZ)
  """

  @input_test_2 """
  LLR

  AAA = (BBB, BBB)
  BBB = (AAA, ZZZ)
  ZZZ = (ZZZ, ZZZ)
  """

  @input_test_3 """
  LR

  11A = (11B, XXX)
  11B = (XXX, 11Z)
  11Z = (11B, XXX)
  22A = (22B, XXX)
  22B = (22C, 22C)
  22C = (22Z, 22Z)
  22Z = (22B, 22B)
  XXX = (XXX, XXX)
  """

  def p1_test() do
    IO.puts("---- test 1 ----")
    part1(@input_test_1)
    IO.puts("---- test 2 ----")
    part1(@input_test_2)
  end

  def p1() do
    part1(File.read!("input/day08.txt"))
  end

  def p2_test() do
    part2(@input_test_3)
  end

  def p2() do
    part2(File.read!("input/day08.txt"))
  end

  defp common(input) do
    [commands, map] =
      input
      |> String.split("\n\n", trim: true)

    map =
      map
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        Regex.run(~r/([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)/, line,
          capture: :all_but_first
        )
      end)
      |> Enum.reduce(%{}, fn [start, left, right], m -> Map.put(m, start, {left, right}) end)

    [commands |> String.to_charlist(), map]
  end

  defp run([commands, map], start, stop_fn, max_iter \\ 100) do
    0..max_iter//1
    |> Enum.reduce_while([0, start, false], fn _, [count, node, _] ->
      commands
      |> Enum.reduce_while([count, node, false], fn dir, [count, node, _] ->
        case stop_fn.(node) do
          true ->
            {:halt, [count, node, true]}

          false ->
            {l, r} =
              Map.get(map, node)

            case dir do
              76 -> {:cont, [count + 1, l, false]}
              82 -> {:cont, [count + 1, r, false]}
            end
        end
      end)
      |> case do
        [steps, node, true] -> {:halt, [steps, node, true]}
        x -> {:cont, x}
      end
    end)
  end

  defp part1(input) do
    # answer: 14429

    common(input)
    |> run("AAA", fn node -> node == "ZZZ" end)
  end

  defp part2(input) do
    # answer: 10921547990923

    [commands, map] =
      common(input)

    [start | rest] =
      map
      |> Enum.filter(fn {start, _} -> String.ends_with?(start, "A") end)
      |> Enum.map(fn {start, _} -> start end)
      |> Enum.map(fn start ->
        run([commands, map], start, fn node -> String.ends_with?(node, "Z") end)
      end)
      |> Enum.map(fn [steps, _, _] -> steps end)

    rest
    |> Enum.reduce(start, fn step, acc ->
      Integer.floor_div(step * acc, Integer.gcd(step, acc))
    end)
  end
end
