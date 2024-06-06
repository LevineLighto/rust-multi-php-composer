use std::{env, process::Command};

fn main() {
    let args_count = env::args().count();
    let mut args = vec![String::new(); args_count];
    let mut index = 0;
    for arg in env::args() {
        index += 1;
        if index <= 1 {
            continue;
        } else if index == 2 {
            args[0] = arg;
            args[1] = String::from("C:/ProgramData/ComposerSetup/bin/composer.phar")
        } else {
            args[index - 1] = arg;
        }
    }

    let mut process = Command::new("multiphp")
        .args(args)
        .spawn()
        .expect("Multi Composer: Failed to execute command");

    process.wait().unwrap();
}
