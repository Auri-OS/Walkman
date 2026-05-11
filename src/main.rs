use std::{
    fs::{self},
    process,
};

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
    path: String,

    #[arg(long, default_value_t = false)]
    nui: bool,
}

fn main() {
    let cli = Cli::parse();

    let metadata = fs::metadata(&cli.path).unwrap_or_else(|e| {
        eprintln!("Failed to read path {}: {}", cli.path, e);
        process::exit(1);
    });

    let mut test_files = vec![];

    if metadata.is_dir() {
        for entry in fs::read_dir(&cli.path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if path.is_file()
                && let Some(ext) = path.extension()
                && (ext == "yml" || ext == "yaml")
            {
                test_files.push(path.to_string_lossy().to_string());
            }
        }
        test_files.sort();

        if test_files.is_empty() {
            println!("No YAML tests files found in directory: {}", cli.path);
            process::exit(0);
        }
    } else {
        test_files.push(cli.path.clone());
    }

    let mut all_passed = true;
    let mut summary = Vec::new();

    for test_file in &test_files {
        let file = fs::File::open(test_file).unwrap_or_else(|e| {
            eprintln!("Failed to open test file {}: {}", test_file, e);
            process::exit(1);
        });

        let config: TestConfig = serde_yaml::from_reader(file).unwrap_or_else(|e| {
            eprintln!("Invalid Yaml format in {}: {}", test_file, e);
            process::exit(1);
        });

        let mut runner = QemuRunner;

        let result = if cli.nui {
            let reporter = ConsoleReporter;
            runner.run(&config, &reporter)
        } else {
            let reporter = TuiReporter::new(&config);
            runner.run(&config, &reporter)
        };

        let success = result.is_ok();
        summary.push((config.name.clone(), success));

        if !success {
            all_passed = false;
        }
    }

    println!("\n==================================================");
    println!("📼  WALKMAN TEST SUITE SUMMARY");
    println!("==================================================");
    for (name, success) in summary {
        if success {
            println!("\x1b[32m  ▶● {}\x1b[0m", name);
        } else {
            println!("\x1b[31m  ● {}\x1b[0m", name);
        }
    }
    println!("==================================================\n");

    if !all_passed {
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
