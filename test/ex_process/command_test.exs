defmodule ExProcess.CommandTest do
  use ExUnit.Case
  require Logger

  test "invalid" do
    assert {:error, _} = ExProcess.Command.spawn("123", ["456", "789"], %{"abc" => "xyz"})
  end
end