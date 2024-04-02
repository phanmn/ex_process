use rustler::{Encoder, Env, ResourceArc, Term};
use std::process::Command;
use std::sync::Mutex;

struct ProcessResource {
    pub child: Mutex<std::process::Child>,
}

rustler::atoms! { error, ok, }

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

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(ProcessResource, env);
    true
}

#[rustler::nif(schedule = "DirtyCpu")]
fn spawn(
    program: String,
    arg_list: Vec<String>,
    envs: std::collections::HashMap<String, String>,
) -> SpawnResult {
    let spawn_result = Command::new(program).args(arg_list).envs(envs).spawn();
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

rustler::init!("Elixir.ExProcess.Command", [spawn], load = load);
