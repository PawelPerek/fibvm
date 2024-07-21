use std::{path::PathBuf, process::Command, time::Duration};

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

    fn benchmark(&self) -> Vec<(Duration, Duration)> {
        println!("Benchmarking C# code...");
        let buff: PathBuf = [".", "build", "main"].iter().collect();

        for flag in ["", "--native"] {
            let mut n = 0;
            let mut execution_time = Duration::ZERO;
            
            while execution_time < Duration::from_secs(1) {
                let mut command = Command::new(&buff);
               
                let start = std::time::Instant::now();
            
                command.arg(n.to_string()).arg(flag).output().expect("failed to run");
    
                execution_time = start.elapsed();
                
                println!("n = {}, flag = {}, execution time = {:?}", n, flag, execution_time);
    
                n += 1;
            }
        } 
    
        vec![]
    }
}