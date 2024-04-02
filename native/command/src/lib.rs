use rustler::types::map::MapIterator;
use std::process::Command;

// #[derive(Debug, NifReturnable)]
// struct Output {
//     pub status: i32,
//     pub stdout: String,
//     pub stderr: String,
// }

#[rustler::nif(schedule = "DirtyCpu")]
fn run(program: String, arg_list: Vec<String>, envs: MapIterator) -> i32 {
    let mut child = Command::new(program);
    add_arg(&mut child, arg_list);
    add_env(&mut child, envs);

    match child.output() {
        Ok(output) => output.status.code().unwrap_or(-1),
        _ => -2,
    }
}

rustler::init!("Elixir.ExProcess.Command", [run]);

fn add_arg(command: &mut Command, mut v: Vec<String>) {
    for arg in v.iter_mut() {
        command.arg(arg);
    }
}

fn add_env(command: &mut Command, envs: MapIterator) {
    for (key, value) in envs {
        let key_string = key.decode::<String>().unwrap();
        let value_string = value.decode::<String>().unwrap();

        command.env(key_string, value_string);
    }
}
