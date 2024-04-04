defmodule ExProcess.Command do
  def spawn(command, args, opts \\ []) do
    env = opts |> Keyword.get(:env, %{})

    env =
      :maps.filter(
        fn _, v -> v != nil end,
        env
      )

    ExProcess.Nif.spawn(ExProcess.Runtime.get(), command, args, env)
  end
end
