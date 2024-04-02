use rustler::{Encoder, Env, ResourceArc, Term};
use std::process::{Command, Stdio};
use std::sync::Mutex;

struct ProcessResource {
    pub child: Mutex<std::process::Child>,
}

rustler::atoms! { error, ok, none}

enum SpawnResult {
    Success(ResourceArc<ProcessResource>),
    Failure(String),
}

impl<'a> Encoder for SpawnResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            SpawnResult::Success(arc) => (ok(), arc).encode(env),
            SpawnResult::Failure(msg) => (error(), msg).encode(env),
        }
    }
}

#[rustler::nif]
fn spawn(
    program: String,
    arg_list: Vec<String>,
    envs: std::collections::HashMap<String, String>,
) -> SpawnResult {
    let spawn_result = Command::new(program).args(arg_list).envs(envs).stdin(Stdio::null()).stdout(Stdio::null()).spawn();
    match spawn_result {
        Ok(child) => {
            let resource = ResourceArc::new(ProcessResource {
                child: Mutex::new(child),
            });

            SpawnResult::Success(resource)
        }
        Err(e) => SpawnResult::Failure(format!("{:#}", e)),
    }
}

enum TryWaitResult {
    Success(i32),
    None,
    Failure(String),
}

impl<'a> Encoder for TryWaitResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            TryWaitResult::Success(arc) => (ok(), arc).encode(env),
            TryWaitResult::Failure(msg) => (error(), msg).encode(env),
            TryWaitResult::None => (ok(), none()).encode(env),
        }
    }
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(ProcessResource, env);
    true
}

#[rustler::nif]
fn try_wait(resource: ResourceArc<ProcessResource>) -> TryWaitResult {
    let child = &mut *resource.child.lock().unwrap();
    match child.try_wait() {
        Ok(Some(status)) => TryWaitResult::Success(status.code().unwrap()),
        Ok(None) => TryWaitResult::None,
        Err(e) => TryWaitResult::Failure(format!("{:#}", e)), // Err(format!("{:#}", e)),
    }
}

#[rustler::nif]
fn kill(resource: ResourceArc<ProcessResource>) -> bool {
    let child = &mut *resource.child.lock().unwrap();
    let _ = child.kill();
    true
}

rustler::init!("Elixir.ExProcess.Command", [spawn, try_wait, kill], load = load);
