use regex::Regex;
use rexpect::spawn;
use similar::{ChangeTag, TextDiff};

use crate::config::TestConfig;
use crate::traits::{Reporter, Runner};
use crate::{RED, RESET};

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

            for expect in &test.expect {
                if let Err(e) = p.exp_string(expect) {
                    // let clean_msg = clean_error_msg(&e.to_string());
                    let fail_msg = match &e {
                        rexpect::error::Error::Timeout {
                            expected: exp, got, ..
                        } => format_diff(exp, got),
                        rexpect::error::Error::EOF {
                            expected: exp, got, ..
                        } => {
                            format!(
                                "EOF (OS Crashed)!\n    {RED}│{RESET} Expected: '{}'\n    {RED}│{RESET} Got:      '{}'",
                                exp,
                                clean_output(got)
                            )
                        }
                        _ => e.to_string(),
                    };

                    reporter.on_step_failure(&test.command, &fail_msg);
                    anyhow::bail!(e.to_string());
                }
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

fn clean_output(raw: &str) -> String {
    let mut msg = raw.replace("\r\n", " ↵ ").replace('\n', " ↵ ");

    if let Ok(re) = Regex::new(r"\x1b\[[0-9;]*[mK]|\\u\{1b\}\[[0-9;]*[mK]") {
        msg = re.replace_all(&msg, "").to_string();
    }
    msg
}

fn format_diff(expected: &str, got: &str) -> String {
    let clean_got = clean_output(got);

    let diff = TextDiff::from_chars(expected, &clean_got);
    let mut colored_got = String::new();

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {}
            ChangeTag::Insert => {
                colored_got.push_str(&format!("\x1b[31m{}\x1b[0m", change.value()));
            }
            ChangeTag::Equal => {
                colored_got.push_str(&format!("\x1b[32m{}\x1b[0m", change.value()));
            }
        }
    }
    format!(
        "Timeout!\n    {RED}│{RESET} Expected: '{}'\n    {RED}│{RESET} Got:      '{}'",
        expected, colored_got
    )
}
