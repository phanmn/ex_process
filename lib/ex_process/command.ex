defmodule ExProcess.Command do
  def spawn(command, args, opts \\ []) do
    ExProcess.Nif.spawn(command, args, opts |> Keyword.get(:env, %{}))
  end
end
