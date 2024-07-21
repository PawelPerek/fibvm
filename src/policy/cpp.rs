use std::{path::PathBuf, process::Command};

pub struct Cpp;

impl super::BenchmarkPolicy for Cpp {
    fn compile(&self) {
        println!("Compiling C++ code...");
        let _ = std::fs::create_dir("build");

        let buff = ["build", "main"].iter().collect::<PathBuf>();
        let output_file = buff.as_path();

        let mut command = Command::new("g++");
        command.arg("--std=c++20").arg("-o").arg(output_file).arg("main.cpp");
        command.output().expect("failed to compile");
    }

    fn benchmark_command(&self) -> Command {
        let buff: PathBuf = [".", "build", "main"].iter().collect();
        let command = Command::new(&buff);

        command
    }
}