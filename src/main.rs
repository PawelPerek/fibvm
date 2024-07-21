use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

trait BenchmarkPolicy {
    fn cd(&self, path: &Path) {
        env::set_current_dir(path).expect("failed to change directory");
        dbg!(env::current_dir().unwrap());
    }

    fn compile(&self) {}

    fn benchmark(&self) -> Vec<(Duration, Duration)>;

    fn run(&self, path: &mut PathBuf) -> Vec<(Duration, Duration)> {
        self.cd(path);
        self.compile();
        self.benchmark()
    }
}

struct C;

impl BenchmarkPolicy for C {
    fn compile(&self) {
        let mut command = Command::new("gcc");
        command.arg("-o").arg("output").arg("main.c");
        command.output().expect("failed to compile");
    }

    fn benchmark(&self) -> Vec<(Duration, Duration)> {
        let buff: PathBuf = [".", "output"].iter().collect();

        for flag in ["", "--native"] {
            let mut n = 0;
            let mut execution_time = Duration::new(0, 0);
            
            while execution_time < Duration::new(10, 0) {
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

struct Cpp;

impl BenchmarkPolicy for Cpp {
    fn benchmark(&self) -> Vec<(Duration, Duration)> {
        vec![]
    }
}

fn main() {
    let path = Path::new("modules");

    let mut policies: HashMap<&str, Box<dyn BenchmarkPolicy>> = HashMap::new();
    
    policies.insert("c", Box::new(C));
    policies.insert("c++", Box::new(Cpp));

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let mut path = entry.path();
            let lang = path.into_iter().last().unwrap().to_str().unwrap();

            println!("Benchmarking {}...", lang);

            let policy = policies.get(lang).unwrap();

            policy.run(&mut path);
        }
    }

    
}
