defmodule Day20 do
  def p1_test() do
    solve("input/day20.test", 1000)
  end

  def p1_test_2() do
    solve("input/day20.test.2", 1000)
  end

  def p1() do
    solve("input/day20.txt", 1000)
  end

  def p2() do
    solve2()
  end

  defp init_state(:broadcaster), do: nil
  defp init_state(:flipflop), do: :off
  defp init_state(:conjunction), do: %{}

  def parse_input(file) do
    cfg =
      file
      |> File.read!()
      |> String.split("\n", trim: true)
      |> Enum.map(
        &(~r/^(broadcaster|[%&][a-z]+)\s+->\s+((?:[a-z]+(?:, )?)+)$/
          |> Regex.run(&1, capture: :all_but_first))
      )
      |> Enum.reduce(%{}, fn [input, output], cfg ->
        {type, name} =
          case String.at(input, 0) do
            "b" -> {:broadcaster, input}
            "%" -> {:flipflop, String.slice(input, 1..-1//1)}
            "&" -> {:conjunction, String.slice(input, 1..-1//1)}
          end

        outputs =
          output
          |> String.split(", ", trim: true)
          |> Enum.map(&String.trim(&1))

        Map.put(cfg, name, {type, outputs, init_state(type)})
      end)

    # trace back initial state for the conjuction modules
    cfg
    |> Enum.filter(&(elem(elem(&1, 1), 0) == :conjunction))
    |> Enum.map(&elem(&1, 0))
    |> Enum.map(fn module ->
      {module,
       cfg
       |> Enum.filter(&Enum.member?(elem(elem(&1, 1), 1), module))
       |> Enum.map(&elem(&1, 0))}
    end)
    |> Enum.reduce(cfg, fn {module, sources}, cfg ->
      {type, outputs, state} = Map.get(cfg, module)
      state = Enum.reduce(sources, state, fn src, state -> Map.put(state, src, :low) end)
      Map.put(cfg, module, {type, outputs, state})
    end)
  end

  defp build_pulse({state, pulse}, cfg, type, module, outputs, rest) do
    case pulse do
      :none ->
        {cfg, rest}

      :high ->
        {Map.put(cfg, module, {type, outputs, state}),
         rest ++ Enum.map(outputs, &{module, &1, :high})}

      :low ->
        {Map.put(cfg, module, {type, outputs, state}),
         rest ++ Enum.map(outputs, &{module, &1, :low})}
    end
  end

  defp send_pulse({cfg, []}, count, _idx), do: {cfg, count}

  defp send_pulse({cfg, [{src, module, pulse} | rest]}, {low, high}, idx) do
    # IO.puts("#{src} -#{pulse}-> #{module}")

    count =
      case pulse do
        :low -> {low + 1, high}
        :high -> {low, high + 1}
      end

    if (module == "mr" or module == "kk" or module == "gl" or module == "bb") and pulse == :low do
      IO.puts("#{module}: #{pulse}: #{idx}")
    end

    case Map.get(cfg, module) do
      nil ->
        {cfg, rest}

      {type, outputs, state} ->
        case type do
          :broadcaster ->
            {nil, pulse}

          :flipflop when pulse == :low and state == :off ->
            {:on, :high}

          :flipflop when pulse == :low and state == :on ->
            {:off, :low}

          :conjunction ->
            state =
              Map.put(state, src, pulse)

            pulse =
              case Enum.all?(state, &(elem(&1, 1) == :high)) do
                true -> :low
                false -> :high
              end

            {state, pulse}

          _ ->
            {state, :none}
        end
        |> build_pulse(cfg, type, module, outputs, rest)
    end
    |> send_pulse(count, idx)
  end

  def push_button(cfg, times) do
    init = cfg

    {_, cycle, {low, high}} =
      1..times//1
      |> Enum.reduce_while({cfg, 0, {0, 0}}, fn idx, {cfg, _cycle, count} ->
        {cfg, count} =
          send_pulse({cfg, [{"button", "broadcaster", :low}]}, count, idx)

        case cfg == init do
          true -> {:halt, {cfg, idx, count}}
          false -> {:cont, {cfg, idx, count}}
        end
      end)

    # |> IO.inspect(limit: 150, custom_options: [sort_maps: true])
    IO.puts("cycle length: #{cycle}")

    cycle = times / cycle
    low * cycle * (high * cycle)
  end

  defp lcm(a, b) do
    div(a * b, Integer.gcd(a, b))
  end

  def solve(file, times) do
    parse_input(file)
    |> push_button(times)
  end

  def solve2() do
    # we know that for a single low pulse to be sent to rx
    # then all inputs to the preceeding conjunction (qt) must be high.
    # from there, we can see all inputs to qt are conjunctions as well
    # and in order to output a high pulse, at least one input must be low.
    # so, we run push the button until we see a low pulse on each of qt's
    # inputs.  Then we can assume it will eventually cycle (hopefully) and
    # compute the LCM of all 4 qt inputs to find out on what button press
    # they all receive a low signal to output a high to qt and finally
    # send a low signal to rx.
    #
    ## mr: 3907
    ## kk: 3931
    ## bb: 3967
    ## gl: 3989

    a = lcm(3907, 3931)
    b = lcm(3967, 3989)
    lcm(a, b)
  end
end
