defmodule ExProcess.Child do
  def try_status(child) do
    ExProcess.Nif.try_status(child)
  end

  def kill(child) do
    ExProcess.Nif.kill(child)
  end
end