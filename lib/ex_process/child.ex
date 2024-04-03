defmodule ExProcess.Child do
  def try_wait(child) do
    ExProcess.Nif.try_wait(child)
  end

  def kill(child) do
    ExProcess.Nif.kill(child)
  end
end