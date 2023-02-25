use std::{env, time::Duration};
use std::path::Path;
use std::process::Command;
use std::thread::sleep;

use notify::{RecursiveMode, Result, Watcher, Event};

fn reload() {
    let args = env::args().collect::<Vec<_>>();

    let cmd = &args[1];
    Command::new("killall").arg("waybar").output().expect("Could not run killall");
    Command::new(cmd).spawn().expect(&format!("Could not start process {}", cmd));
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("USAGE: autoreload [COMMAND] [PATH] - execute COMMAND whenever something in PATH changes");
        return Ok(());
    }

    reload();

    let mut watcher = notify::recommended_watcher(|res: Result<Event>| match res {
        Ok(event) => {
            if event.kind.is_modify() {
                reload();
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    let path = &args[2];
    println!("Watching {}...", path);
    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    loop {
        sleep(Duration::from_secs(1));
    }
}
