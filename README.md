<h1>
  <img width="30" height="30" alt="From KlickPin CF Fresh side hustle ideas that can help you create a more curated classy and Pinterest-worthy result for busy people who still want gorgeous results - Pin-822962531938830764" src="https://github.com/user-attachments/assets/d2ac270f-c518-4878-976b-8b5dbe31e91e" />
  Walkman
</h1>

Walkman is a robust, YAML-driven integration test framework designed specifically for AuriOS and bare-metal environments. It automates QEMU interactions via the serial port, providing real-time feedback and CI/CD ready reporting.

## Features

* Declarative Testing: Write test scenarios in clean, readable YAML files.
* Directory Runner: Execute a single test file or a full directory of tests sequentially.
* Dynamic TUI: Real-time terminal user interface tracking boot sequences and shell commands.
* CI/CD Ready: A flat, script-friendly output mode (`--nui`) designed for GitHub Actions and automated pipelines.
* Smart Matchers: Support for exact string matching and sequential multi-line expectations.
* Visual Diffs: Built-in text differencing to quickly spot discrepancies between expected and received outputs.

## Installation

Walkman is written in Rust. You can compile and install it using Cargo.

From source:
```bash
git clone https://github.com/Auri-OS/walkman.git
cd walkman
cargo install --path .
```

Directly from GitHub (ideal for CI pipelines):
```bash
cargo install --git https://github.com/Auri-OS/walkman.git --tag v0.1.1
```

## Usage

Run a single test file:
```bash
walkman tests/basic_test.yml
```

Run an entire test suite from a directory:
```bash
walkman tests/integrations/
```

### CLI Options

* `--nui`: Disables the dynamic TUI. Forces a flat console output suitable for CI logs.
* `--verbose`: Displays the raw output captured from the serial port for successful tests, highlighting the matched strings.

## Writing a Test

Test configurations are written in YAML. They are divided into two main parts: the boot sequence (waiting for the OS to initialize) and the shell interactions.

```yaml
name: "Basic Shell Integration"
command: "qemu-system-i386 -cdrom output/AuriOS.iso -m 512M -display none -serial stdio"
timeout_ms: 8000

boot_sequence:
  - "Starting AuriOS boot sequence..."
  - "System ready."

shell_interactions:
  prompt: "auri-os"
  tests:
    - command: "uptime -s"
      expect: "s\r\n"
    - command: "memdump 16"
      expect: 
        - "PMM BITMAP DUMP"
        - "Legend:"
```

## Architecture

Walkman leverages `rexpect` to spawn and manage the QEMU process, capturing its `stdio` output. It acts as a state machine, moving from pending, to running, to success/failure based on the standard output emitted by the target operating system's serial port.
