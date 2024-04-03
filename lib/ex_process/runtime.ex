defmodule ExProcess.Runtime do
  use GenServer

  def start_link(args) do
    GenServer.start_link(__MODULE__, args)
  end

  def init(_) do
    Process.flag(:trap_exit, true)

    runtime = ExProcess.Nif.start_runtime(self())
    :persistent_term.put(__MODULE__, runtime)

    {:ok, %{runtime: runtime}}
  end

  def get() do
    :persistent_term.get(__MODULE__)
  end

  def handle_info(:ex_process_runtime_stopped, state) do
    {:stop, :ex_process_runtime_failure, state |> Map.put(:runtime, nil)}
  end

  def handle_info(_msg, state) do
    {:noreply, state}
  end

  def terminate(_reason, %{runtime: nil}) do
    :ok
  end

  def terminate(_reason, %{runtime: runtime}) do
    ExProcess.Nif.stop_runtime(runtime)

    receive do
      :ex_process_runtime_stopped -> :ok
    end
  end
end
