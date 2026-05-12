use crate::config::TestConfig;
use crate::traits::Reporter;
use crate::{GREEN, GREY, RED, RESET};

pub struct ConsoleReporter;

impl Reporter for ConsoleReporter {
    fn on_test_start(&self, config: &TestConfig) {
        println!("\n▶ 📼 WALKMAN PLAYING: {}", config.name);
        println!("  ⚙️ Command: {}", config.command);
        println!("  ⏳ Timeout: {}ms", config.timeout_ms);
        println!("--------------------------------------------------");
    }

    fn on_boot_start(&self, _msg: &str) {}

    fn on_boot_check(&self, msg: &str) {
        println!("  [BOOT] Waiting for: '{}' ...", msg);
    }

    fn on_step_start(&self, step_name: &str) {
        print!("  [TEST] Executing: '{}' ... ", step_name);
    }

    fn on_step_success(&self, _step_name: &str, details: Option<String>) {
        println!("{}SUCCESS{}", GREEN, RESET);
        if let Some(msg) = details {
            println!("{GREY}{msg}{RESET}");
        }
    }

    fn on_step_failure(&self, _step_name: &str, reason: &str) {
        println!("{}FAILED{}\n  ↳ Reason: {}", RED, RESET, reason);
    }

    fn on_test_end(&self, success: bool) {
        println!("--------------------------------------------------");
        if success {
            println!("{}RESULT: PASS\n{}", GREEN, RESET);
        } else {
            println!("{}RESULT: FAIL\n{}", RED, RESET);
        }
    }
}
