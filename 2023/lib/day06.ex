defmodule Day06 do
  def p1_test() do
    part1([[7, 9], [15, 40], [30, 200]])
  end

  def p1() do
    part1([[58, 434], [81, 1041], [96, 2219], [76, 1218]])
  end

  def p2_test() do
    part2(71530, 940_200)
  end

  def p2() do
    part2(58_819_676, 434_104_122_191_218)
  end

  def compute_distances(time, accelaration) do
    0..time//1
    |> Enum.map(fn ms -> [time - ms, ms * accelaration] end)
    |> Enum.map(fn [left, acc] -> left * acc end)
    |> IO.inspect(charlists: :as_lists)
  end

  defp part1(races) do
    # answer: 1159152

    races
    |> Enum.map(fn [time, dist] -> [compute_distances(time, 1), dist] end)
    |> Enum.map(fn [dists, max] -> Enum.filter(dists, fn dist -> dist > max end) end)
    |> Enum.map(fn dists -> length(dists) end)
    |> Enum.product()
  end

  defp part2(time, max) do
    # answer: 41513103

    length =
      0..time//1
      |> Stream.take_while(fn ms -> (time - ms) * ms < max end)
      |> Enum.to_list()
      |> length()

    time - length * 2 + 1
  end
end
