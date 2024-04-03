defmodule ExProcess.Command do
  def spawn(command, args, opts \\ []) do
    ExProcess.Nif.spawn(ExProcess.Runtime.get(), command, args, opts |> Keyword.get(:env, %{}))
  end
end
