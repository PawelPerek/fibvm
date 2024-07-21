use std::process::Command;

pub struct Python;

impl super::BenchmarkPolicy for Python {
    fn benchmark_command(&self) -> Command {
        let mut command = Command::new("python3");
        command.arg("main.py");

        command
    }
}