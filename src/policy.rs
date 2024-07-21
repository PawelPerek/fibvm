use std::{env, path::{Path, PathBuf}, time::Duration};

mod c;
mod cpp;
mod csharp;

pub use c::C;
pub use cpp::Cpp;
pub use csharp::CSharp;

pub trait BenchmarkPolicy {
    fn cd(&self, path: &Path) {
        env::set_current_dir(path).expect("failed to change directory");
        dbg!(env::current_dir().unwrap());
    }

    fn compile(&self) {}

    fn benchmark(&self) -> Vec<(Duration, Duration)>;

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
