use std::{env, fs::File, process};

use walkman::{
    config::TestConfig,
    engine::QemuRunner,
    reporter::ConsoleReporter,
    traits::{Reporter, Runner},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: walkman <path_to_test.yml>");
        process::exit(1);
    }

    let file_path = &args[1];

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open test file: {}", e);
            process::exit(1);
        }
    };

    let config: TestConfig = match serde_yaml::from_reader(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Invalid Yaml format : {}", e);
            process::exit(1);
        }
    };

    let reporter = ConsoleReporter;
    let mut runner = QemuRunner;

    if runner.run(&config, &reporter).is_err() {
        reporter.on_test_end(false);
        process::exit(1);
    }
}
