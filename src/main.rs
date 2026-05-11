use std::{fs::File, process};

use walkman::{
    config::TestConfig,
    engine::QemuRunner,
    reporter::{console::ConsoleReporter, tui::TuiReporter},
    traits::Runner,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    test_file: String,

    #[arg(long, default_value_t = false)]
    nui: bool,
}

fn main() {
    let cli = Cli::parse();

    let file = match File::open(cli.test_file) {
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

    // let reporter = ConsoleReporter;
    let mut runner = QemuRunner;

    let result = if cli.nui {
        let reporter = ConsoleReporter;
        runner.run(&config, &reporter)
    } else {
        let reporter = TuiReporter::new(&config);
        runner.run(&config, &reporter)
    };

    if result.is_err() {
        process::exit(1);
    }
}
