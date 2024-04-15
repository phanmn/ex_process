defmodule ExProcess.Nif do
  use Rustler, otp_app: :ex_process, crate: "command", target: System.get_env("RUSTLER_TARGET")

  # When your NIF is loaded, it will override this function.
  def spawn(_runtime, _command, _args, _envs), do: :erlang.nif_error(:nif_not_loaded)
  def try_status(_resource), do: :erlang.nif_error(:nif_not_loaded)
  def kill(_resource), do: :erlang.nif_error(:nif_not_loaded)
  def pid(_resource), do: :erlang.nif_error(:nif_not_loaded)

  def start_runtime(_Pid), do: :erlang.nif_error(:nif_not_loaded)
  def stop_runtime(_resource), do: :erlang.nif_error(:nif_not_loaded)
  def process_state(_pid), do: :erlang.nif_error(:nif_not_loaded)
end
