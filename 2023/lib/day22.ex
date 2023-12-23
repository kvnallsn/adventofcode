defmodule Day22 do
  def parse_input(file) do
    file
    |> File.read!()
    |> String.split("\n", trim: true)
    |> Enum.map(&Regex.run(~r/(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)/, &1, capture: :all_but_first))
    |> Enum.map(&Enum.map(&1, fn i -> String.to_integer(i) end))
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {[x1, y1, z1, x2, y2, z2], idx}, grid ->
      case {x2 - x1, y2 - y1, z2 - z1} do
        {_, 0, 0} -> Enum.map(x1..x2//1, &{&1, y1, z1})
        {0, _, 0} -> Enum.map(y1..y2//1, &{x1, &1, z1})
        {0, 0, _} -> Enum.map(z1..z2//1, &{x1, y1, &1})
      end
      |> Enum.reduce(grid, &Map.put(&2, &1, idx + 1))
    end)
  end

  def compute_move(_grid, _block, {_x, _y, z}) when z - 1 == 0, do: 0

  def compute_move(grid, block, {x, y, z}) do
    case Map.get(grid, {x, y, z - 1}) do
      nil -> compute_move(grid, block, {x, y, z - 1}) + 1
      b when b == block -> compute_move(grid, block, {x, y, z - 1}) + 1
      _ -> 0
    end
  end

  def fall(grid) do
    grid
    |> Enum.group_by(&elem(&1, 1), &elem(&1, 0))
    |> Enum.to_list()
    |> Enum.map(fn {block, coords} ->
      {block, elem(Enum.min_by(coords, &elem(&1, 2)), 2), coords}
    end)
    |> Enum.sort_by(&elem(&1, 1), :asc)
    |> Enum.reduce({grid, 0}, fn {block, _z, coords}, {grid, count} ->
      # compute how far this block can move
      coords
      |> Enum.map(&compute_move(grid, block, &1))
      |> Enum.min()
      |> case do
        0 ->
          {grid, 0}

        d ->
          {Enum.reduce(coords, grid, fn {x, y, z}, grid ->
             Map.delete(grid, {x, y, z}) |> Map.put({x, y, z - d}, block)
           end), count + 1}
      end
    end)
  end

  def find_singles(grid) do
    grid
    |> Enum.group_by(&elem(&1, 1), &elem(&1, 0))
    |> Enum.to_list()
    |> Enum.reduce(%{}, fn {block, coords}, m ->
      coords
      |> Enum.map(fn {x, y, z} -> Map.get(grid, {x, y, z + 1}) end)
      |> Enum.reject(&(&1 == nil))
      |> Enum.reject(&(&1 == block))
      |> Enum.reduce(m, fn i, m ->
        Map.update(m, i, [block], fn s -> [block | s] end)
      end)
    end)
    |> Enum.map(&elem(&1, 1))
    |> Enum.map(&Enum.dedup(&1))
    |> Enum.reject(&(length(&1) > 1))
    |> List.flatten()
    |> Enum.reduce(MapSet.new(), &MapSet.put(&2, &1))
    |> MapSet.to_list()
  end

  def solve(file) do
    grid = file |> parse_input() |> fall() |> elem(0)

    singles =
      find_singles(grid)
      |> length()
      |> IO.inspect()

    Map.values(grid)
    |> Enum.sort()
    |> Enum.dedup()
    |> length()
    |> Kernel.-(singles)
  end

  def solve2(file) do
    grid = file |> parse_input() |> fall() |> elem(0)

    grid
    |> Enum.group_by(&elem(&1, 1), &elem(&1, 0))
    |> Enum.to_list()
    |> Enum.map(fn {_block, coords} ->
      coords
      |> Enum.reduce(grid, &Map.delete(&2, &1))
      |> fall()
      |> elem(1)
    end)
    |> IO.inspect(limit: :infinity, charlists: :as_lists)
    |> Enum.sum()
  end
end
