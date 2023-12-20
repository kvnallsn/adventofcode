defmodule Day19 do
  def p1_test() do
    solve("input/day19.test")
  end

  def p1() do
    solve("input/day19.txt")
  end

  def p2_test() do
    solve2("input/day19.test")
  end

  def p2() do
    solve2("input/day19.txt")
  end

  defp parse_input(file) do
    [workflows, parts] =
      file
      |> File.read!()
      |> String.split("\n\n", trim: true)

    workflows =
      workflows
      |> String.split("\n", trim: true)
      |> Enum.map(fn workflow ->
        [name, flows] =
          ~r/^([a-z]+){(.*)}$/
          |> Regex.run(workflow, capture: :all_but_first)

        {last, flows} = flows |> String.split(",", trim: true) |> List.pop_at(-1)

        flows =
          flows
          |> Enum.map(fn flow ->
            ~r/([a|m|s|x])(<|>)(\d+):(\w+)/
            |> Regex.run(flow, capture: :all_but_first)
          end)
          |> Enum.map(fn [cat, op, val, step] -> {cat, op, String.to_integer(val), step} end)

        {name, flows, last}
      end)
      |> Enum.reduce(%{}, fn {name, flows, last}, m ->
        Map.put(m, name, {flows, last})
      end)

    parts =
      parts
      |> String.split("\n", trim: true)
      |> Enum.map(fn part ->
        [x, m, a, s] =
          ~r/{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}/
          |> Regex.run(part, capture: :all_but_first)
          |> Enum.map(&String.to_integer(&1))

        %{}
        |> Map.put("x", x)
        |> Map.put("m", m)
        |> Map.put("a", a)
        |> Map.put("s", s)
        |> Map.put("sum", x + m + a + s)
      end)

    {workflows, parts}
  end

  defp run_workflow(workflows, part) do
    0..1_000_000//1
    |> Enum.reduce_while({"in", 0}, fn _, {workflow, sum} ->
      # get the workflow
      {steps, last} =
        Map.get(workflows, workflow)

      # evalute the steps
      steps
      |> Enum.reduce_while(nil, fn {cat, op, val, tgt}, _ ->
        rating = Map.get(part, cat)

        case op do
          ">" -> rating > val
          "<" -> rating < val
        end
        |> case do
          true -> {:halt, tgt}
          false -> {:cont, nil}
        end
      end)
      |> case do
        nil -> last
        tgt -> tgt
      end
      |> case do
        "A" ->
          {:halt, {last, Map.get(part, "sum")}}

        "R" ->
          {:halt, {last, 0}}

        tgt ->
          {:cont, {tgt, sum}}
      end
    end)
    |> elem(1)
  end

  def solve(file) do
    {workflows, parts} =
      parse_input(file)

    parts
    |> Enum.map(&run_workflow(workflows, &1))
    |> Enum.sum()
  end

  defp diff({high, low}), do: high - low

  defp calc_total(vals, cat, val) do
    case cat do
      "x" ->
        diff(val) * diff(Map.get(vals, "m")) * diff(Map.get(vals, "a")) * diff(Map.get(vals, "s"))

      "m" ->
        diff(val) * diff(Map.get(vals, "x")) * diff(Map.get(vals, "a")) * diff(Map.get(vals, "s"))

      "a" ->
        diff(val) * diff(Map.get(vals, "m")) * diff(Map.get(vals, "x")) * diff(Map.get(vals, "s"))

      "s" ->
        diff(val) * diff(Map.get(vals, "m")) * diff(Map.get(vals, "a")) * diff(Map.get(vals, "x"))
    end
  end

  def solve2(file) do
    {workflows, _parts} =
      parse_input(file)

    # first split all workflows so they only contain one step
    workflows =
      workflows
      |> Enum.reduce(%{}, fn {key, {steps, last}}, m ->
        steps
        |> Enum.with_index()
        |> Enum.reduce(m, fn {{cat, op, val, tgt}, idx}, m ->
          nxt =
            case length(steps) - 1 == idx do
              true when last == "A" or last == "R" -> last
              true -> "#{last}1"
              false -> "#{key}#{idx + 2}"
            end

          tgt = if tgt == "A" or tgt == "B", do: tgt, else: "#{tgt}1"
          name = if key == "in", do: "in", else: "#{key}#{idx + 1}"

          Map.put(m, name, {cat, op, val, tgt, nxt})
        end)
      end)

    start =
      %{"x" => {4000, 0}, "m" => {4000, 0}, "a" => {4000, 0}, "s" => {4000, 0}}

    # now traverse the list, keeping track of the min/max values for each part of the tree
    0..1_000_000//1
    |> Enum.reduce_while({[{"in", start}], 0}, fn _idx, {[{step, vals} | rest], count} ->
      IO.puts(step)
      {cat, op, val, tgt, nxt} = Map.get(workflows, step)
      {cmax, cmin} = Map.get(vals, cat)

      case op do
        ">" -> [{tgt, {cmax, val}}, {nxt, {val, cmin}}]
        "<" -> [{tgt, {val - 1, cmin}}, {nxt, {cmax, val - 1}}]
      end
      |> Enum.reduce({rest, count}, fn {step, val}, {rest, count} ->
        case step do
          "A" -> {rest, count + calc_total(vals, cat, val)}
          "R" -> {rest, count}
          _ -> {[{step, Map.put(vals, cat, val)} | rest], count}
        end
      end)
      |> case do
        {[], count} -> {:halt, count}
        x -> {:cont, x}
      end
    end)
  end
end
