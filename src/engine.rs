use rexpect::spawn;

use crate::config::TestConfig;
use crate::traits::{Reporter, Runner};

pub struct QemuRunner;

impl Runner for QemuRunner {
    fn run(&mut self, config: &TestConfig, reporter: &dyn Reporter) -> anyhow::Result<()> {
        reporter.on_test_start(config);

        let mut p = spawn(&config.command, Some(config.timeout_ms))?;

        for msg in &config.boot_sequence {
            reporter.on_boot_start(msg);
            if let Err(e) = p.exp_string(msg) {
                reporter.on_step_failure("Boot sequence", &e.to_string());
                anyhow::bail!(e.to_string());
            }
            reporter.on_boot_check(msg);
        }

        if let Err(e) = p.exp_string(&config.shell_interactions.prompt) {
            reporter.on_step_failure("Waiting for initial prompt", &e.to_string());
            anyhow::bail!(e.to_string());
        }

        for test in &config.shell_interactions.tests {
            reporter.on_step_start(&test.command);

            if let Err(e) = p.send_line(&test.command) {
                reporter.on_step_failure(&test.command, "Failed to send command");
                anyhow::bail!(e.to_string());
            }

            if let Err(e) = p.exp_string(&test.expect) {
                reporter.on_step_failure(
                    &test.command,
                    &format!("Expected '{}' not found", test.expect),
                );
                anyhow::bail!(e.to_string());
            }

            if let Err(e) = p.exp_string(&config.shell_interactions.prompt) {
                reporter.on_step_failure(
                    &test.command,
                    "Kernel panic / No prompt returned after command",
                );
                anyhow::bail!(e.to_string());
            }

            reporter.on_step_success(&test.command);
        }

        let _ = p.process_mut().kill(rexpect::process::Signal::SIGTERM);
        reporter.on_test_end(true);

        Ok(())
    }
}
