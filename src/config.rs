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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_yaml_parsing() {
        let yaml_content = r#"
name: "Test OS"
command: "qemu-system-i386 -display none"
timeout_ms: 5000
boot_sequence:
  - "Booting..."
shell_interactions:
  prompt: "os~$"
  tests:
    - command: "uptime"
      expect: "seconds"
"#;

        let config: Result<TestConfig, _> = serde_yaml::from_str(yaml_content);
        assert!(config.is_ok(), "Valid YAML should be succesfuly parsed :/");

        let config = config.unwrap();
        assert_eq!(config.name, "Test OS");
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.boot_sequence.len(), 1);
        assert_eq!(config.shell_interactions.tests[0].command, "uptime");
    }

    #[test]
    fn test_invalid_yaml_parsing() {
        let yaml_content = r#"
        name: "Test OS"
        command: "qemu-system-i386"
        "#;

        let config: Result<TestConfig, _> = serde_yaml::from_str(yaml_content);
        assert!(
            config.is_err(),
            "Incomplete YAML should trigger an error :/"
        );
    }
}
