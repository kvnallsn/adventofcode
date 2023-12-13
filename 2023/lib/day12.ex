defmodule Day12 do
  use Memoize

  def p1_test() do
    solve("input/day12.test", false)
  end

  def p1() do
    solve("input/day12.txt", false)
  end

  def p2_test() do
    solve("input/day12.test", true)
  end

  def p2() do
    solve("input/day12.txt", true)
  end

  defp common(file, unfold) do
    File.read!(file)
    |> String.split("\n", trim: true)
    |> Enum.map(&String.split(&1, " ", trim: true))
    |> Enum.map(fn [springs, pattern] ->
      nums =
        ~r/(\d{1,2})/
        |> Regex.scan(pattern, capture: :all_but_first)
        |> List.flatten()
        |> Enum.map(&String.to_integer/1)

      springs = String.codepoints(springs)

      case unfold do
        false ->
          {springs, nums}

        true ->
          1..4//1
          |> Enum.reduce({springs, nums}, fn _, {s, p} ->
            {s ++ ["?"] ++ springs, p ++ nums}
          end)
      end
    end)
  end

  defmemo(count({[], []}), do: 1)
  defmemo(count({[], _pattern}), do: 0)

  defmemo count({springs, []}) do
    springs
    |> Enum.member?("#")
    |> case do
      true -> 0
      false -> 1
    end
  end

  defmemo count({["." | springs], pattern}) do
    count({springs, pattern})
  end

  defmemo count({["?" | springs], pattern}) do
    count({["#" | springs], pattern}) + count({["." | springs], pattern})
  end

  defmemo count({["#" | springs], [count | pattern]}) do
    min_length = Enum.sum(pattern) + count + length(pattern) - 1

    with true <- length(springs) >= min_length,
         false <- Enum.take(springs, count - 1) |> Enum.member?("."),
         false <- Enum.drop(springs, count - 1) |> Enum.take(1) |> Enum.member?("#") do
      count({Enum.drop(springs, count), pattern})
    else
      _ -> 0
    end
  end

  defp solve(file, unfold) do
    # part 1 answer: 8180
    # part 2 answer: 620189727003627

    common(file, unfold)
    |> Enum.map(&count/1)
    |> Enum.sum()
  end
end
