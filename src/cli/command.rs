use crate::cli::args::git_parse_args;
use crate::debug_log;

pub fn git_execute() {
    let matches = git_parse_args();
    // TODO: 解析命令行参数
    match matches {
        // 如果是 Init 命令
        crate::cli::args::ArgType::Init(path) => {
            debug_log!("Initializing git repository at {}", path);
            crate::commands::init::init_command(path.as_str());
        }
        crate::cli::args::ArgType::Add(path) => {
            debug_log!("Adding file to git repository: {}", path);
            crate::commands::add::add_command(path.as_str());
        }
        crate::cli::args::ArgType::Rm(path) => {
            debug_log!("Removing file from git repository: {}", path);
            crate::commands::rm::rm_command(path.as_str());
        }
        crate::cli::args::ArgType::Commit(message) => {
            debug_log!("Committing changes with message: {}", message);
            crate::commands::commit::commit_command(message.as_str());
        }
        crate::cli::args::ArgType::Branch(name) => {
            debug_log!("Creating branch: {}", name);
            crate::commands::branch::branch_command(name.as_str());
        }
        crate::cli::args::ArgType::Checkout(name) => {
            debug_log!("Checking out branch: {}", name);
            crate::commands::checkout::checkout_command(name.as_str());
        }
        crate::cli::args::ArgType::Merge(name) => {
            debug_log!("Merge branch: {}", name);
            crate::commands::merge::merge_command(name.as_str());
        }
        _ => {
            println!("Unknown command");
        }
    }
}
