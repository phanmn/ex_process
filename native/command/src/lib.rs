use rustler::{Env, Error, ResourceArc, Return, Term};
use std::process::Command;
use std::sync::Mutex;

struct ProcessResource {
    pub child: Mutex<std::process::Child>,
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(ProcessResource, env);
    true
}

#[rustler::nif(schedule = "DirtyCpu")]
fn run(
    program: String,
    arg_list: Vec<String>,
    envs: std::collections::HashMap<String, String>,
) -> Result<ResourceArc<ProcessResource>, Error> {
    let spawn_result = Command::new(program).args(arg_list).envs(envs).spawn();
    match spawn_result {
        Ok(child) => {
            let resource = ResourceArc::new(ProcessResource {
                child: Mutex::new(child),
            });

            Ok(resource)
            // Ok(Return::Term(resource))
        }
        Err(e) => Err(Error::Term(Box::new(format!("{:#}", e)))),
    }
}

rustler::init!("Elixir.ExProcess.Command", [run], load = load);
