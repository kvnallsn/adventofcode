defmodule Day09 do
  def p1_test() do
    part1("input/day09.test")
  end

  def p1() do
    part1("input/day09.txt")
  end

  def p2_test() do
    part2("input/day09.test")
  end

  def p2() do
    part2("input/day09.txt")
  end

  defp common(file) do
    File.read!(file)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line -> Regex.scan(~r/(-?\d+)/, line, capture: :all_but_first) end)
    |> Enum.map(fn lists -> List.flatten(lists) end)
    |> Enum.map(fn line -> Enum.map(line, fn num -> String.to_integer(num) end) end)
    |> Enum.map(&reduce_oasis/1)
  end

  @doc """
  Takes a list of numbers (i.e., [0, 3, 6, 9, 12, 15] as input along with
  a max iteration count (defaults to 100).

  How this function works:
  Iterates over the provided list (`oasis`), computing the different between
  each element. Additionally, it stores the first and last element of of each 
  intermediate list, which are later used to compute either the next or previous
  element.
  """
  def reduce_oasis(oasis, max_iter \\ 100) do
    [st | _] = oasis

    Enum.reduce_while(0..max_iter//1, {[], [st], oasis}, fn _, {e, b, [h | t]} ->
      {tail, lst} =
        Enum.reduce(t, {h, []}, fn num, {p, lst} ->
          {num, [num - p | lst]}
        end)

      lst = Enum.reverse(lst)
      [head | _] = lst

      case Enum.all?(lst, fn x -> x == 0 end) do
        true -> {:halt, {[tail | e], [head | b], lst}}
        false -> {:cont, {[tail | e], [head | b], lst}}
      end
    end)
  end

  defp part1(file) do
    # answer: 1666172641

    common(file)
    |> Enum.map(fn {tail, _, _} -> Enum.sum(tail) end)
    |> Enum.sum()
  end

  defp part2(file) do
    # answer: 933

    common(file)
    |> Enum.map(fn {_, [f | r], _} ->
      Enum.reduce(r, f, fn v, acc -> v - acc end)
    end)
    |> Enum.sum()
  end
end
