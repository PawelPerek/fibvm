use std::{path::PathBuf, process::Command, time::Duration};

pub struct Cpp;

impl super::BenchmarkPolicy for Cpp {
    fn compile(&self) {
        println!("Compiling C++ code...");
        let _ = std::fs::create_dir("build");

        let buff = ["build", "main"].iter().collect::<PathBuf>();
        let output_file = buff.as_path();

        dbg!(output_file);

        let mut command = Command::new("g++");
        command.arg("--std=c++20").arg("-o").arg(output_file).arg("main.cpp");
        command.output().expect("failed to compile");
    }

    fn benchmark(&self) -> Vec<(Duration, Duration)> {
        println!("Benchmarking C++ code...");
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