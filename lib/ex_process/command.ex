defmodule ExProcess.Command do
  use Rustler, otp_app: :ex_process, crate: "command"

  # When your NIF is loaded, it will override this function.
  def spawn(_command, _args, _envs), do: :erlang.nif_error(:nif_not_loaded)
  def try_wait(_resource), do: :erlang.nif_error(:nif_not_loaded)
end