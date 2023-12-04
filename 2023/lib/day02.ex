defmodule Day02 do
  defp day02 do
    File.read!("input/day02.txt")
    |> String.split("\n", trim: true)
    |> Enum.with_index()
    |> Enum.map(fn {line, idx} ->
      Regex.scan(~r/((?<count>\d+) (?<color>blue|red|green))+/, line)
      |> Enum.reduce(%{"red" => 0, "green" => 0, "blue" => 0}, fn [_, _, count, color], acc ->
        icount = String.to_integer(count)

        if acc[color] < icount do
          Map.put(acc, color, icount)
        else
          acc
        end
      end)
      |> Map.put("index", idx + 1)
    end)
  end

  def day02p1 do
    day02()
    |> Enum.filter(fn game ->
      game["red"] <= 12 and game["green"] <= 13 and game["blue"] <= 14
    end)
    |> Enum.reduce(0, fn %{"index" => idx}, acc -> acc + idx end)
  end

  def day02p2 do
    day02()
    |> Enum.map(fn %{"red" => red, "green" => green, "blue" => blue} -> red * green * blue end)
    |> Enum.reduce(0, fn power, acc -> power + acc end)
  end

end
