// rustling have a nice build system:
// every time you save file it automatically builds and runs tests.
// I think that would be very convinient to have a such build system for random testing


// I found this in rustlings source code:
/*
    Mode::Compile => Command::new("rustc")
        .args(&[self.path.to_str().unwrap(), "-o", &temp_file()])
        .args(RUSTC_COLOR_ARGS)
        .output(),
*/

//Code for temporary names
/*
#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    format!("./temp_{}_{}", process::id(), thread_id)
}
*/

// I found that rustlings uses something called Watch
// I can add that by using Cargo
// notify = "4.0"
use notify::{RecommendedWatcher, Watcher};
use crate::rust_analyser_project_gen::RustAnalyserProject;

mod rust_analyser_project_gen;

enum Subcommands {
    Run,
    Lsp
}

fn lsp(){
    let mut project = RustAnalyserProject::new();
            project
                .get_sysroot_src()
                .expect("Couldn't find toolchain path, do you have `rustc` installed?");
            project
                .files_to_json()
                .expect("Couldn't parse rustlings exercises files");

            if project.crates.is_empty() {
                println!("Failed find any exercises, make sure you're in the `rustlings` folder");
            } else if project.write_to_disk().is_err() {
                println!("Failed to write rust-project.json to disk for rust-analyzer");
            } else {
                println!("Successfully generated rust-project.json");
                println!("rust-analyzer will now parse exercises, restart your language server or editor")
            }
}

#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    format!("./temp_{}_{}", process::id(), thread_id)
}

use std::{process::{Command, self, Output}, path::{PathBuf, Path}, fs::remove_file, time::Duration, sync::mpsc::{channel, RecvTimeoutError}, ffi::OsStr};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];

fn cargo_build(path: PathBuf) -> Result<OutputStr, OutputStr>{
    let cmd = Command::new("cargo").arg("run").arg("--bin").arg(path.file_stem().unwrap())
        .args(RUSTC_COLOR_ARGS)
        .output()
        .expect("Error on build");

    let output = OutputStr{
        stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
        stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
    };

    if cmd.status.success() {
        Ok(output)
    } else {
        Err(output)
    }
}

fn compile(path : PathBuf) -> Result<String, OutputStr>{
    let cmd = Command::new("rustc").args(&[path.to_str().unwrap(), "-o", &temp_file()])
        .args(RUSTC_COLOR_ARGS).output()
        .expect("Error on compilation");

    if cmd.status.success() {
        Ok("..".to_owned())
    } else {
        Err(OutputStr {
            stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
            stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
        })
    }
}

struct OutputStr{
    stdout: String,
    stderr: String
}

fn run() -> Result<OutputStr, OutputStr>{
    let arg = "--show-output";
    let cmd = Command::new(&temp_file())
        .arg(arg)
        .output()
        .expect("Failed to execute process!");

    let output = OutputStr {
        stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
        stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
    };

    if cmd.status.success() {
        Ok(output)
    } else {
        Err(output)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_tests() {
        println!("Testing!");
    }
}

fn cargo_file(path: PathBuf){
    let cargo_res = cargo_build(path.to_owned());
    match cargo_res {
        Ok(output) => {
            println!("{}", output.stdout);
        },
        Err(output) => {
            println!("Error on compilation!");
            println!("{}", output.stderr);
        }
    }
}

fn run_file(path: PathBuf){
    let compile_res = compile(path.to_owned());
    match compile_res {
        Ok(_) => {},
        Err(output) => {
            println!("Error on compilation!");
            println!("{}", output.stderr);
            return;
        },
    }

    let result = run();
    _ = remove_file(&temp_file());
    match result {
        Ok(output) => {
            println!("{}", output.stdout);
        },
        Err(output) => {
            println!(
                "Testing of {} failed! Please try again. Here's the output:",
                path.to_str().unwrap()
            );
            println!("{}", output.stderr);
        },
    }
}


fn watch(){
    fn clear_screen() {
        println!("\x1Bc");
    }

    clear_screen();
    println!("Started watcher!");

    let (tx, rx) = channel();
    let mut watcher : RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch(Path::new("./src/bin"), notify::RecursiveMode::NonRecursive).unwrap();



    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                notify::DebouncedEvent::Create(b) 
                | notify::DebouncedEvent::Write(b)
                | notify::DebouncedEvent::Chmod(b) => {
                  //When you create / edit a file
                  if b.extension() == Some(OsStr::new("rs")) && b.exists() {
                    let path = b.as_path().canonicalize().unwrap();
                    clear_screen();
                    cargo_file(path);
                  }  
                 },
                _ => {}
            },
            Err(RecvTimeoutError::Timeout) => {

            },
            Err(e) => println!("watch error {:?}", e),
        }


    }
}

// I made it working, by using the source code of rustlings, and documentation I glued this solution for building and running whenever I save the file!
// After a quick search I found the clue
// rust-analyser uses Cargo default settings, however.. I created a custom build system.
// Now I need to create a rust-project.json, or generate it.
// https://rust-analyzer.github.io/manual.html#non-cargo-based-projects 
fn main() {
    
    let command : Subcommands = Subcommands::Lsp;

    match command {
        Subcommands::Run => {
            watch()
        },
        Subcommands::Lsp => {
            lsp()
        },
    }


    println!("Look! Recursion!");
}
