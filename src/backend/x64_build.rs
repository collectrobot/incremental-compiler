use std::env::{temp_dir};
use std::fs::{File};
use std::io::prelude::*;
use std::process::{Command};

use std::collections::HashMap;

#[cfg(target_os = "windows")]
static RUNTIME: &'static [u8] = include_bytes!("bin_include/win64/runtime.lib");

#[cfg(target_os = "windows")]
static NASM: &'static [u8] = include_bytes!("bin_include/win64/nasm.exe");

#[cfg(target_os = "windows")]
static WINLINK: &'static [u8] = include_bytes!("bin_include/win64/link.exe");

#[cfg(target_os = "windows")]
static MSPDBCORE: &'static [u8] = include_bytes!("bin_include/win64/mspdbcore.dll");

#[cfg(target_os = "windows")]
static TBBMALLOC: &'static [u8] = include_bytes!("bin_include/win64/tbbmalloc.dll");

pub struct X64Builder {
    filename: String,
    content: String,
}

impl X64Builder {

    #[cfg(target_os = "windows")]
    fn copy_dependencies(&self) -> HashMap<String, String> {

        let mut dependencies: HashMap<String, String> = HashMap::new();

        let mut temp_dir = temp_dir();
        temp_dir.push("rustcomp");

        let mut pairs: Vec<(&'static str, &'static [u8])> = vec!();

        pairs.push(("runtime.lib", RUNTIME));
        pairs.push(("nasm.exe", NASM));
        pairs.push(("link.exe", WINLINK));
        pairs.push(("mspdbcore.dll", MSPDBCORE));
        pairs.push(("tbbmalloc.dll", TBBMALLOC));

        for pair in pairs {
            let mut file_path = temp_dir.clone();

            file_path.set_file_name(pair.0);

            let name_as_str = pair.0.to_string();

            let filename_only =
                name_as_str
                .split(".")
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .first()
                .unwrap()
                .clone();

            dependencies.insert(filename_only.clone(), file_path.to_str().unwrap().to_string());

            let mut file = File::create(file_path).unwrap();
            let _ = file.write_all(pair.1).unwrap();
            drop(file);
        } 

        dependencies
    }

    pub fn new(filename: String, content: String) -> Self {
        X64Builder {
            filename: filename,
            content: content
        }
    }

    #[cfg(target_os = "windows")]
    pub fn build(&self) {

        let dependency_map = self.copy_dependencies();

        let nasm_output =
            Command::new(dependency_map.get("nasm").unwrap())
            .args(&["-f", "win64", &(self.filename.clone() + ".asm")])
            .output()
            .expect("Failed to call nasm");

        if nasm_output.status.code().unwrap() != 0 {
            println!("{}", std::str::from_utf8(&nasm_output.stderr).unwrap().to_owned());
        }

        let linker_output =
            Command::new(dependency_map.get("link").unwrap())
            .args(&[
                    &(self.filename.clone() + "obj"),
                    "runtime.lib",
                    &("Out:\"".to_owned() + &(self.filename.clone() + ".exe") + "\""),
                    "/Entrypoint:__runtime_startup",
                    "/Subsystem:console",
                ])
            .output()
            .expect("link.exe was not found");

        if linker_output.status.code().unwrap() != 0 {
            println!("{}", std::str::from_utf8(&linker_output.stderr).unwrap().to_owned());
        }
    }
}
