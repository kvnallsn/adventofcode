defmodule Day24 do
  def parse_input(file) do
    file
    |> File.read!()
    |> String.split("\n", trim: true)
    |> Enum.map(
      &Regex.run(~r/(-?\d+),\s+(-?\d+),\s+?(-?\d+)\s+@\s+(-?\d+),\s+(-?\d+),\s+(-?\d+)/, &1,
        capture: :all_but_first
      )
    )
    |> Enum.map(fn line -> Enum.map(line, &String.to_integer(&1)) end)
  end

  def compute_intersection({a, b}) do
    [x1, y1, _z1, ax1, ay1, _az1] = a
    [x2, y2, _z2, ax2, ay2, _az2] = b

    {s1, s2} = {ay1 / ax1, ay2 / ax2}

    case s1 == s2 do
      true ->
        :never

      false ->
        c1 = y1 - s1 * x1
        c2 = y2 - s2 * x2

        xi = (c2 - c1) / (s1 - s2)
        yi = s1 * xi + c1
        {xi, yi}
    end
    |> case do
      :never -> :parallel
      {x, _y} when ax1 < 0 and x > x1 -> :past
      {x, _y} when ax1 > 0 and x < x1 -> :past
      {_x, y} when ay1 < 0 and y > y1 -> :past
      {_x, y} when ay1 > 0 and y < y1 -> :past
      {x, _y} when ax2 < 0 and x > x2 -> :past
      {x, _y} when ax2 > 0 and x < x2 -> :past
      {_x, y} when ay2 < 0 and y > y2 -> :past
      {_x, y} when ay2 > 0 and y < y2 -> :past
      coord -> coord
    end
  end

  def solve(file, low, high) do
    inputs = parse_input(file)

    inputs
    |> Enum.with_index()
    |> Enum.map(fn {line, idx} -> Enum.drop(inputs, idx + 1) |> Enum.map(&{line, &1}) end)
    |> List.flatten()
    |> Enum.map(&compute_intersection/1)
    |> Enum.reject(&(&1 == :parallel or &1 == :past))
    |> Enum.reject(&(elem(&1, 0) < low or elem(&1, 0) > high))
    |> Enum.reject(&(elem(&1, 1) < low or elem(&1, 1) > high))
    |> length()
  end

  def solve2(file) do
    # This code should be good but the elixir Z3 bindings seem off...
    # solved using the python bindings
    x = ExSMT.env_var(:x)
    y = ExSMT.env_var(:y)
    z = ExSMT.env_var(:z)
    vx = ExSMT.env_var(:vx)
    vy = ExSMT.env_var(:vy)
    vz = ExSMT.env_var(:vz)

    [a, b, c] =
      file
      |> parse_input()
      |> Enum.take(3)
      |> Enum.with_index()
      |> Enum.map(fn {[x0, y0, z0, xv, yv, zv], idx} ->
        t = ExSMT.env_var("t#{idx}")

        eq =
          ExSMT.expr(
            :and,
            ExSMT.expr(
              :=,
              ExSMT.expr(:+, ExSMT.expr(:*, t, vx), x),
              ExSMT.expr(:+, ExSMT.expr(:*, t, xv), x0)
            ),
            ExSMT.expr(
              :=,
              ExSMT.expr(:+, ExSMT.expr(:*, t, vy), y),
              ExSMT.expr(:+, ExSMT.expr(:*, t, yv), y0)
            ),
            ExSMT.expr(
              :=,
              ExSMT.expr(:+, ExSMT.expr(:*, t, vz), z),
              ExSMT.expr(:+, ExSMT.expr(:*, t, zv), z0)
            )
          )

        ExSMT.expr(:and, ExSMT.expr(:>, t, 0), eq)
      end)

    ExSMT.expr(:and, a, b, c)
    |> IO.inspect()
    |> ExSMT.solve()

    :ok
  end
end
