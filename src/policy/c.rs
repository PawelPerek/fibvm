use std::{path::PathBuf, process::Command};

pub struct C;

impl super::BenchmarkPolicy for C {
    fn compile(&self) {
        println!("Compiling C code...");
        let _ = std::fs::create_dir("build");

        let buff = ["build", "main"].iter().collect::<PathBuf>();
        let output_file = buff.as_path();

        let mut command = Command::new("gcc");
        command.arg("-o").arg(output_file).arg("main.c");
        command.output().expect("failed to compile");
    }

    fn benchmark_command(&self) -> Command {
        let buff: PathBuf = [".", "build", "main"].iter().collect();
        let command = Command::new(&buff);

        command
    }
}