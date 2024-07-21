use std::{path::PathBuf, process::Command};

pub struct CSharp;

impl super::BenchmarkPolicy for CSharp {
    fn compile(&self) {
        println!("Compiling C# code...");
        let _ = std::fs::create_dir("build");
        
        let buff: PathBuf = [".", "build", "main"].iter().collect();
        let output_file = buff.as_path();

        let mut command = Command::new("csc");
        command.arg(format!("/out:{}", output_file.display())).arg("main.cs");
        command.output().expect("failed to compile");
    }

    fn benchmark_command(&self) -> Command {
        let buff: PathBuf = [".", "build", "main"].iter().collect();
        let mut command = Command::new("mono");

        command.arg(&buff);

        command
    }
}