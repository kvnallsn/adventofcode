defmodule Day01 do
  defp name_to_num(name) do
    case name do
      "one" -> "1"
      "two" -> "2"
      "three" -> "3"
      "four" -> "4"
      "five" -> "5"
      "six" -> "6"
      "seven" -> "7"
      "eight" -> "8"
      "nine" -> "9"
      x -> x
    end
  end

  def day01 do
    File.read!("input/day01.txt")
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      Regex.named_captures(
        ~r/^[a-z]*?(?<first>one|two|three|four|five|six|seven|eight|nine|\d)[a-z0-9]*(?<second>one|two|three|four|five|six|seven|eight|nine|\d)[a-z]*?$/,
        line
      )
      |> case do
        nil ->
          Regex.named_captures(~r/^[a-z]*(?<num>\d)[a-z]*$/, line)
          |> case do
            nil -> raise "bad input, no regex match"
            cap -> cap["num"] <> cap["num"]
          end

        cap ->
          name_to_num(cap["first"]) <> name_to_num(cap["second"])
      end
    end)
    |> Enum.map(fn line -> String.to_integer(line) end)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end
end
