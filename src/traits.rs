use crate::{Cli, config::TestConfig};

pub trait Reporter {
    fn on_test_start(&self, config: &TestConfig);
    fn on_boot_start(&self, msg: &str);
    fn on_boot_check(&self, msg: &str);
    fn on_step_start(&self, step_name: &str);
    fn on_step_success(&self, step_name: &str, details: Option<String>);
    fn on_step_failure(&self, step_name: &str, reason: &str);
    fn on_test_end(&self, success: bool);
}

pub trait Runner {
    fn run(
        &mut self,
        config: &TestConfig,
        opts: &Cli,
        reporter: &dyn Reporter,
    ) -> anyhow::Result<()>;
}
