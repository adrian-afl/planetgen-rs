use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    #[arg(default_value = "test-input.json")]
    pub input: String,
}
