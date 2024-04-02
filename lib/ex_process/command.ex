defmodule ExProcess.Command do
  use Rustler, otp_app: :ex_process, crate: "command"

  # When your NIF is loaded, it will override this function.
  def run(_command, _args, _envs), do: :erlang.nif_error(:nif_not_loaded)
end