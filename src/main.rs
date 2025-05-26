mod cli;
pub mod commands;
pub mod core;
fn main() {
    cli::command::git_execute();
}
