defmodule ExProcess.CommandTest do
  use ExUnit.Case
  require Logger

  test "greets the world" do
    ExProcess.Command.run("123", ["456", "789"], %{"abc" => "xyz"})
    |> inspect()
    |> Logger.error()
  end
end