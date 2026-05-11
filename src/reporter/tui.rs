use std::{cell::RefCell, io::stdout};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::{config::TestConfig, traits::Reporter};

enum Status {
    Pending,
    Running,
    Success,
    Failed(String),
}

struct TuiState {
    name: String,
    command: String,
    boot_steps: Vec<(String, Status)>,
    shell_steps: Vec<(String, Status)>,
    lines_printed: u16,
}

pub struct TuiReporter {
    state: RefCell<TuiState>,
}

impl TuiReporter {
    pub fn new(config: &TestConfig) -> Self {
        let _ = execute!(stdout(), cursor::Hide);

        let boot_steps = config
            .boot_sequence
            .iter()
            .map(|s| (s.clone(), Status::Pending))
            .collect();

        let shell_steps = config
            .shell_interactions
            .tests
            .iter()
            .map(|t| (t.command.clone(), Status::Pending))
            .collect();

        Self {
            state: RefCell::new(TuiState {
                name: config.name.clone(),
                command: config.command.clone(),
                boot_steps,
                shell_steps,
                lines_printed: 0,
            }),
        }
    }

    fn redraw(&self) {
        let mut state = self.state.borrow_mut();
        let mut out = stdout();

        if state.lines_printed > 0 {
            let _ = execute!(
                out,
                cursor::MoveUp(state.lines_printed),
                cursor::MoveToColumn(0),
                Clear(ClearType::FromCursorDown)
            );
        }

        let mut lines = 0;

        let _ = execute!(
            out,
            Print("\n📼 WALKMAN PLAYING: "),
            SetForegroundColor(Color::Cyan),
            Print(&state.name),
            ResetColor,
            Print("\n"),
            Print("⚙ Command: "),
            SetForegroundColor(Color::DarkGrey),
            Print(&state.command),
            ResetColor,
            Print("\n"),
            Print(
                "--------------------------------------------------------------------------------\n\n"
            ),
            SetForegroundColor(Color::Yellow),
            Print("[ BOOT SEQUENCE ]\n"),
            ResetColor
        );
        lines += 6;

        for (name, status) in &state.boot_steps {
            lines += Self::draw_step(&mut out, name, status, "Waiting for: ");
        }

        let _ = execute!(
            out,
            Print("\n"),
            SetForegroundColor(Color::Yellow),
            Print("[ SHELL INTERACTIONS ]\n"),
            ResetColor
        );
        lines += 2;

        for (name, status) in &state.shell_steps {
            lines += Self::draw_step(&mut out, name, status, "command: ");
        }

        let _ = execute!(
            out,
            Print(
                "\n--------------------------------------------------------------------------------\n"
            )
        );
        lines += 2;

        state.lines_printed = lines;
    }

    fn draw_step(out: &mut std::io::Stdout, name: &str, status: &Status, prefix: &str) -> u16 {
        let mut lines = 1;
        match status {
            Status::Pending => {
                let _ = execute!(
                    out,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("  · {}{}\n", prefix, name)),
                    ResetColor
                );
            }
            Status::Success => {
                let _ = execute!(
                    out,
                    SetForegroundColor(Color::Green),
                    Print("  ● "),
                    ResetColor,
                    Print(format!("{}{}\n", prefix, name))
                );
            }
            Status::Running => {
                let _ = execute!(
                    out,
                    SetForegroundColor(Color::Cyan),
                    Print("  ▶ "),
                    ResetColor,
                    Print(format!("{}{}\n", prefix, name))
                );
                let _ = execute!(
                    out,
                    SetForegroundColor(Color::DarkGrey),
                    Print("    │ ⌨️  Sending payload...\n"),
                    Print("    │ ⏳ Waiting for OS response...\n"),
                    ResetColor
                );
                lines += 2;
            }
            Status::Failed(reason) => {
                let _ = execute!(
                    out,
                    SetForegroundColor(Color::Red),
                    Print("  ● "),
                    ResetColor,
                    Print(format!("{}{}\n", prefix, name))
                );
                let _ = execute!(
                    out,
                    SetForegroundColor(Color::Red),
                    Print(format!("    │ Reason: {}\n", reason)),
                    ResetColor
                );
                lines += 1;
            }
        }
        lines
    }

    fn update_boot_status(&self, name: &str, new_status: Status) {
        if let Some(step) = self
            .state
            .borrow_mut()
            .boot_steps
            .iter_mut()
            .find(|s| s.0 == name)
        {
            step.1 = new_status;
        }
        self.redraw();
    }

    fn update_shell_status(&self, name: &str, new_status: Status) {
        if let Some(step) = self
            .state
            .borrow_mut()
            .shell_steps
            .iter_mut()
            .find(|s| s.0 == name)
        {
            step.1 = new_status;
        }
        self.redraw();
    }
}

impl Reporter for TuiReporter {
    fn on_test_start(&self, _config: &TestConfig) {
        self.redraw();
    }

    fn on_boot_start(&self, msg: &str) {
        self.update_boot_status(msg, Status::Running);
    }

    fn on_boot_check(&self, msg: &str) {
        self.update_boot_status(msg, Status::Success);
    }

    fn on_step_start(&self, step_name: &str) {
        self.update_shell_status(step_name, Status::Running);
    }

    fn on_step_success(&self, step_name: &str) {
        self.update_shell_status(step_name, Status::Success);
    }

    fn on_step_failure(&self, step_name: &str, reason: &str) {
        self.update_boot_status(step_name, Status::Failed(reason.to_string()));
        self.update_shell_status(step_name, Status::Failed(reason.to_string()));
    }

    fn on_test_end(&self, success: bool) {
        let mut out = stdout();
        if success {
            let _ = execute!(
                out,
                SetForegroundColor(Color::Green),
                Print("RESULT: ALL TESTS PASSED\n\n"),
                ResetColor
            );
        } else {
            let _ = execute!(
                out,
                SetForegroundColor(Color::Red),
                Print("RESULT: TEST SUITE FAILED\n\n"),
                ResetColor
            );
        }

        let _ = execute!(out, cursor::Show);
    }
}

impl Drop for TuiReporter {
    fn drop(&mut self) {
        let _ = execute!(stdout(), cursor::Show);
    }
}
