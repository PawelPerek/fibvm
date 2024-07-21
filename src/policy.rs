use std::{env, path::{Path, PathBuf}, time::Duration};

mod c;
mod cpp;

pub use c::C;
pub use cpp::Cpp;

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
        self.benchmark()
    }
}
