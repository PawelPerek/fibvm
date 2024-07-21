use std::{env, path::{Path, PathBuf}, process::Command, time::Duration};

mod c;
mod cpp;
mod csharp;
mod python;

pub use c::C;
pub use cpp::Cpp;
pub use csharp::CSharp;
pub use python::Python;

pub trait BenchmarkPolicy {
    fn cd(&self, path: &Path) {
        env::set_current_dir(path).expect("failed to change directory");
        dbg!(env::current_dir().unwrap());
    }

    fn compile(&self) {}

    fn benchmark(&self) -> Vec<(Duration, Duration)> {
        for flag in ["", "--native"] {
            let mut n = 0;
            let mut execution_time = Duration::ZERO;
            
            while execution_time < Duration::from_secs(1) {
                let mut command = self.benchmark_command();
               
                let start = std::time::Instant::now();
            
                command.arg(n.to_string()).arg(flag).output().expect("failed to run");
    
                execution_time = start.elapsed();
                
                println!("n = {}, flag = {}, execution time = {:?}", n, flag, execution_time);
    
                n += 1;
            }
        } 
    
        vec![]
    }

    fn benchmark_command(&self) -> Command;

    fn run(&self, path: &mut PathBuf) -> Vec<(Duration, Duration)> {
        self.cd(path);
        self.compile();
        let results = self.benchmark();
        self.re_cd(path);

        results
    }

    fn re_cd(&self, path: &Path) {
        let depth = path.iter().count();
        let mut path = env::current_dir().expect("failed to get current directory");

        for _ in 0..depth {
            path.pop();
        }

        env::set_current_dir(path).expect("failed to change directory");
    }
}
