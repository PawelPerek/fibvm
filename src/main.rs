mod policy;

use std::collections::HashMap;
use std::path::Path;


fn main() {
    let path = Path::new("modules");

    let mut policies: HashMap<&str, Box<dyn policy::BenchmarkPolicy>> = HashMap::new();
    
    if std::env::args().any(|arg| arg == "--enable-c#") {
        policies.insert("c#", Box::new(policy::CSharp));
    }

    policies.insert("c", Box::new(policy::C));
    policies.insert("c++", Box::new(policy::Cpp));

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let mut path = entry.path();
            let lang = path.into_iter().last().unwrap().to_str().unwrap();

            if let Some(policy) = policies.get(lang) {
                policy.run(&mut path);
            } else {
                println!("Unsupported language: {}", lang);
            }
        }
    }

    
}
