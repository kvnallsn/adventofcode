defmodule PriorityQueue do
  @doc """
  Creates a new priority queue based on erlang's generally balanced tress (gb_trees)
  """
  def new, do: {:gb_trees.empty(), %{}}

  @doc """
  Pushes an element into the priority queue, updating it's position if the new priority
  is lower than the existing priority
  """
  def push(queue, element, priority) do
    {tree, map} = queue

    case Map.get(map, element) do
      nil ->
        {:insert, tree, element, priority}

      x when priority < x ->
        case :gb_trees.is_empty(tree) do
          true ->
            {:insert, tree, element, priority}

          false ->
            case :gb_trees.is_defined(x, tree) do
              false ->
                {:insert, tree, element, priority}

              true ->
                {set, tree} = :gb_trees.take(x, tree)
                set = MapSet.delete(set, element)

                case MapSet.size(set) do
                  0 ->
                    {:insert, tree, element, priority}

                  _ ->
                    tree = :gb_trees.insert(x, set, tree)
                    {:insert, tree, element, priority}
                end
            end
        end

      _ ->
        :skip
    end
    |> case do
      :skip ->
        {tree, map}

      {:insert, tree, element, priority} ->
        val =
          :gb_trees.lookup(priority, tree)
          |> case do
            :none -> MapSet.new([element])
            {:value, set} -> MapSet.put(set, element)
          end

        {:gb_trees.enter(priority, val, tree), Map.put(map, element, priority)}
    end
  end

  @doc """
  Extracts the node with the lowest priority
  """
  def pop(queue) do
    {tree, map} = queue

    case :gb_trees.is_empty(tree) do
      true ->
        {:empty, {tree, map}}

      false ->
        {key, val, tree} = :gb_trees.take_smallest(tree)
        [node | rest] = MapSet.to_list(val)

        tree =
          case rest do
            [] -> tree
            rest -> :gb_trees.enter(key, MapSet.new(rest), tree)
          end

        map = Map.delete(map, node)

        {{node, key}, {tree, map}}
    end
  end
end
