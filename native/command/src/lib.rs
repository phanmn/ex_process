use rustler::{Encoder, Env, ResourceArc, Term};
use async_process::{Command, Stdio, Child};
use std::sync::Mutex;

mod runtime;

mod atoms {
    rustler::atoms! {
        error,
        ok,
        none,
        running,
        ex_process_runtime_stopped
    }
}

struct ProcessResource {
    pub child: Mutex<Child>,
    // pub runtime: ResourceArc<runtime::RuntimeResource>,
    // pub exit_status: Mutex<Option<ExitStatus>>
}

enum SpawnResult {
    Success(ResourceArc<ProcessResource>),
    Failure(String),
}

impl<'a> Encoder for SpawnResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            SpawnResult::Success(arc) => (atoms::ok(), arc).encode(env),
            SpawnResult::Failure(msg) => (atoms::error(), msg).encode(env),
        }
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn spawn(
    env: Env,
    runtime: ResourceArc<runtime::RuntimeResource>,
    program: String,
    arg_list: Vec<String>,
    envs: std::collections::HashMap<String, String>,
) -> SpawnResult {
    let _ = env;
    if runtime.is_closed() {
        return SpawnResult::Failure("bad_arg".to_string());
    }

    let spawn_result = Command::new(program)
        .args(arg_list)
        .envs(envs)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn();
    match spawn_result {
        Ok(mut child) => {
            let status = child.status();
            runtime.spawn(async move {
                let _ = status.await;
            });

            let resource = ResourceArc::new(ProcessResource {
                child: Mutex::new(child),
                // runtime: runtime,
            });

            SpawnResult::Success(resource)
        }
        Err(e) => SpawnResult::Failure(format!("{:#}", e)),
    }
}

enum TryStatusResult {
    Success(i32),
    None,
    Running,
    Failure(String),
}

impl<'a> Encoder for TryStatusResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            TryStatusResult::Success(arc) => (atoms::ok(), arc).encode(env),
            TryStatusResult::Failure(msg) => (atoms::error(), msg).encode(env),
            TryStatusResult::None => (atoms::ok(), atoms::none()).encode(env),
            TryStatusResult::Running => (atoms::ok(), atoms::running()).encode(env)
        }
    }
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(ProcessResource, env);
    rustler::resource!(runtime::RuntimeResource, env);
    true
}

#[rustler::nif(schedule = "DirtyCpu")]
fn try_status(resource: ResourceArc<ProcessResource>) -> TryStatusResult {
    let child = &mut *resource.child.lock().unwrap();
    match child.try_status() {
        Ok(Some(status)) => {
            let code = status.code();
            if code.is_none() {
                TryStatusResult::None
            } else {
                TryStatusResult::Success(code.unwrap())
            }
        },
        Ok(None) => TryStatusResult::Running,
        Err(e) => TryStatusResult::Failure(format!("{:#}", e)), // Err(format!("{:#}", e)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn kill(resource: ResourceArc<ProcessResource>) -> bool {
    let child = &mut *resource.child.lock().unwrap();
    let _ = child.kill();
    true
}

#[rustler::nif]
fn pid(resource: ResourceArc<ProcessResource>) -> u32 {
    let child = &mut *resource.child.lock().unwrap();
    child.id()
}

rustler::init!(
    "Elixir.ExProcess.Nif",
    [
        runtime::start_runtime,
        runtime::stop_runtime,
        spawn,
        try_status,
        kill,
        pid
    ],
    load = load
);
