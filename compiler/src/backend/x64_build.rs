#![allow(unused)]

use std::env::{temp_dir};
use std::fs::{File, create_dir_all};
use std::env;
use std::io::prelude::*;
use std::process::{Command};

use std::collections::HashMap;

// WINDOWS:
// these dependencies are required so that the compiler can be self contained
// this means the user doesn't have to download Visual Studio just to be able to invoke the Microsoft linker from the command line
// (this was figured out by trial and error)

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

#[cfg(target_os = "windows")]
static UCRT: &'static [u8] = include_bytes!("bin_include/win64/ucrt.lib");

#[cfg(target_os = "windows")]
static VCRUNTIME: &'static [u8] = include_bytes!("bin_include/win64/libvcruntime.lib");

#[cfg(target_os = "windows")]
static CMT: &'static [u8] = include_bytes!("bin_include/win64/libcmt.lib");

#[cfg(target_os = "windows")]
static KERNEL32: &'static [u8] = include_bytes!("bin_include/win64/kernel32.lib");

#[cfg(target_os = "windows")]
static UUID: &'static [u8] = include_bytes!("bin_include/win64/uuid.lib");

pub struct X64Builder {
    filename: String,
    content: String,
}

impl X64Builder {

    #[cfg(target_os = "windows")]
    fn copy_dependencies(&self, folder: String) -> HashMap<String, String> {

        let mut dependencies: HashMap<String, String> = HashMap::new();

        let mut temp_dir = temp_dir();
        temp_dir.push(folder);

        // create folder if it doesn't exist
        let _ = create_dir_all(temp_dir.clone()).unwrap();

        let mut pairs: Vec<(&'static str, &'static [u8])> = vec!();

        pairs.push(("runtime.lib", RUNTIME));
        pairs.push(("nasm.exe", NASM));
        pairs.push(("link.exe", WINLINK));
        pairs.push(("mspdbcore.dll", MSPDBCORE));
        pairs.push(("tbbmalloc.dll", TBBMALLOC));
        pairs.push(("ucrt.lib", UCRT));
        pairs.push(("libvcruntime.lib", VCRUNTIME));
        pairs.push(("libcmt.lib", CMT));
        pairs.push(("kernel32.lib", KERNEL32));
        pairs.push(("uuid.lib", UUID));

        for pair in pairs {
            let mut file_path = temp_dir.clone();

            file_path.push(pair.0);

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

            if !file_path.is_file() {
                let mut file = File::create(file_path).unwrap();
                let _ = file.write_all(pair.1).unwrap();
                drop(file);
            }
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

        let folder_to_install = "rustcomp".to_owned();

        let mut base_folder = temp_dir();
        base_folder.push(folder_to_install.to_owned());

        let dependency_map = self.copy_dependencies(folder_to_install);

        // the output folder shoulder later be a command line option,
        // or default to the users current working directory
        let mut file_path = base_folder.clone();
        file_path.push(self.filename.clone());

        let mut asm_file_path = file_path.clone();
        asm_file_path.set_extension("asm");

        let mut asm_file = File::create(asm_file_path.clone()).unwrap();
        asm_file.write_all(self.content.as_bytes());
        drop(asm_file);

        let mut obj_file_path = file_path.clone();
        obj_file_path.set_extension("obj");

        let nasm_args = &[
            "-f",
            "win64",
            asm_file_path.to_str().unwrap(),
            "-o",
            obj_file_path.to_str().unwrap()
        ];

        let nasm_output =
            Command::new(dependency_map.get("nasm").unwrap())
            .args(nasm_args)
            .output()
            .expect("Failed to call nasm");

        if nasm_output.status.code().unwrap() != 0 {
            println!("{}", std::str::from_utf8(&nasm_output.stderr).unwrap().to_owned());
        }

        let mut runtime_path = base_folder.clone();
        runtime_path.push("runtime.lib");

        let mut exe_file_path = file_path.clone();
        exe_file_path.set_extension("exe");

        // this current working directory change is needed because the linker
        // looks for certain files in the same directory
        let previous_working_dir = env::current_dir().unwrap();

        env::set_current_dir(base_folder.clone()).unwrap();

        let linker_output =
            Command::new(dependency_map.get("link").unwrap())
            .args(&[
                    obj_file_path.to_str().unwrap(),
                    runtime_path.to_str().unwrap(),
                    "/Entry:__runtime_startup",
                    &("/Out:".to_owned() + exe_file_path.to_str().unwrap()),
                    "/Subsystem:console",
                ])
            .output()
            .expect("link.exe was not found");

        if linker_output.status.code().unwrap() != 0 {
            println!("{}", std::str::from_utf8(&linker_output.stderr).unwrap().to_owned());
            println!("{}", std::str::from_utf8(&linker_output.stdout).unwrap().to_owned());
        }

        println!("output excutable to: {}", base_folder.to_str().unwrap());

        env::set_current_dir(previous_working_dir).unwrap();
    }
}
