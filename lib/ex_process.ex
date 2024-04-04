defmodule ExProcess do
  def process_state(pid) do
    ExProcess.Nif.process_state(pid)
  end
end
