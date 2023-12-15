defmodule AocUtil do
  @doc """
  Computes the amount of time it takes a function to run in seconds
  """
  def benchmark(function) do
    function
    |> :timer.tc()
    |> elem(0)
    |> Kernel./(1_000_000)
  end
end
