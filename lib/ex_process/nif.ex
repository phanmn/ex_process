defmodule ExProcess.Nif do
  # use Rustler, otp_app: :ex_process, crate: "command"

  version = Mix.Project.config()[:version]

  @rustler_precompiled otp_app: :ex_process,
                       crate: "command",
                       base_url:
                         "https://github.com/phanmn/ex_process/releases/download/v#{version}",
                       force_build: System.get_env("EX_PROCESS_BUILD") in ["1", "true"],
                       version: version

  use RustlerPrecompiled, @rustler_precompiled

  def rustler_precompile_config() do
    @rustler_precompiled
    |> Keyword.put_new(:module, __MODULE__)
    |> RustlerPrecompiled.Config.new()
  end

  # When your NIF is loaded, it will override this function.
  def spawn(_runtime, _command, _args, _envs), do: :erlang.nif_error(:nif_not_loaded)
  def try_status(_resource), do: :erlang.nif_error(:nif_not_loaded)
  def kill(_resource), do: :erlang.nif_error(:nif_not_loaded)
  def pid(_resource), do: :erlang.nif_error(:nif_not_loaded)

  def start_runtime(_Pid), do: :erlang.nif_error(:nif_not_loaded)
  def stop_runtime(_resource), do: :erlang.nif_error(:nif_not_loaded)
  def process_state(_pid), do: :erlang.nif_error(:nif_not_loaded)
end
