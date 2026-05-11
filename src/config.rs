use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub name: String,
    pub command: String,
    pub timeout_ms: u64,
    pub boot_sequence: Vec<String>,
    pub shell_interactions: ShellInteractions,
}

#[derive(Debug, Deserialize)]
pub struct ShellInteractions {
    pub prompt: String,
    pub tests: Vec<CommandTest>,
}

#[derive(Debug, Deserialize)]
pub struct CommandTest {
    pub command: String,
    pub expect: String,
}
