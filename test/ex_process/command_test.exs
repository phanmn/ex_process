defmodule ExProcess.CommandTest do
  use ExUnit.Case
  require Logger

  @command_path "abc.exe"

  setup_all do
    {:ok, _runtime} = ExProcess.Runtime.start_link(nil)

    :ok
  end

  test "invalid" do
    assert {:error, _} = ExProcess.Command.spawn("123", ["456", "789"], env: %{"abc" => "xyz"})
  end

  test "valid" do
    assert {:ok, ref} =
             ExProcess.Command.spawn(
               @command_path,
               ["456", "789"]
             )

    assert true == ExProcess.Child.pid(ref) |> is_integer()
    assert {:ok, :running} = ExProcess.Child.try_status(ref)
  end

  test "kill" do
    assert {:ok, ref} =
             ExProcess.Command.spawn(
               @command_path,
               ["456", "789"]
             )

    assert true = ExProcess.Child.kill(ref)
    assert {:ok, :running} = ExProcess.Child.try_status(ref)

    Process.sleep(500)
    assert {:ok, :none} = ExProcess.Child.try_status(ref)
  end
end
