
use glob::glob;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct RustAnalyserProject {
    sysroot_src : String,
    pub crates: Vec<Crate>
}

#[derive(Serialize, Deserialize)]
pub struct Crate{
    root_module: String,
    edition: String,
    deps: Vec<String>,
    cfg: Vec<String>    
}


impl RustAnalyserProject {
    pub fn new() -> RustAnalyserProject{
        RustAnalyserProject{
            sysroot_src: String::new(),
            crates: Vec::new(),
        }
    }

    pub fn write_to_disk(&self) -> Result<(), std::io::Error> {
        std::fs::write(
            "./rust-project.json",
            serde_json::to_vec(&self).expect("failed to serialize to JSON"),
        )?;
        Ok(())
    }

    fn path_to_json(&mut self, path : String){
        if let Some((_, ext)) = path.split_once('.') {
            if ext == "rs" {
                self.crates.push(Crate {
                    root_module: path,
                    edition: "2021".to_string(),
                    deps: Vec::new(),
                    cfg: vec!["test".to_string()],
                })
            }
        }
    }

    pub fn files_to_json(&mut self) -> Result<(), Box<dyn Error>> {
        for e in glob("./src/bin/**/*")? {
            let path = e?.to_string_lossy().to_string();
            self.path_to_json(path);
        }
        Ok(())
    }

    pub fn get_sysroot_src(&mut self) ->  Result<(), Box<dyn Error>>{
        let toolchain = Command::new("rustc")
            .arg("--print")
            .arg("sysroot")
            .output()?
            .stdout;

        let toolchain = String::from_utf8_lossy(&toolchain);
        let mut whitespace_iter = toolchain.split_whitespace();

        let toolchain = whitespace_iter.next().unwrap_or(&toolchain);

        println!("Determined toolchain: {}\n", &toolchain);

        self.sysroot_src = (std::path::Path::new(&*toolchain)
            .join("lib")
            .join("rustlib")
            .join("src")
            .join("rust")
            .join("library")
            .to_string_lossy())
        .to_string();
        Ok(())
    }
}

fn main(){
    println!("Hello world!");   
}