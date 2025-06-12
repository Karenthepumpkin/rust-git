use std::path;

use crate::cli::args::{self, git_parse_args};
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
        crate::cli::args::ArgType::Add(path_vec) => {
            for path in path_vec {
                debug_log!("Adding file to git repository: {}", path);
                crate::commands::add::add_command(&path);
            }
        }
        crate::cli::args::ArgType::Rm(path) => {
            debug_log!("Removing file from git repository: {}", path);
            crate::commands::rm::rm_command(path.as_str());
        }
        crate::cli::args::ArgType::Commit(message) => {
            debug_log!("Committing changes with message: {}", message);
            crate::commands::commit::commit_command(message.as_str());
        }
        crate::cli::args::ArgType::Branch(args) => match args[0].as_str() {
            "-d" => {
                crate::commands::branch::branch_command(
                    &args[1],
                    crate::commands::branch::BranchCommandType::Delete,
                );
                debug_log!("Checking out branch: {}", args[1]);
            }
            _ => {
                crate::commands::branch::branch_command(
                    &args[0],
                    crate::commands::branch::BranchCommandType::New,
                );
                debug_log!("Checking out branch: {}", args[0]);
            }
        },
        crate::cli::args::ArgType::Checkout(args) => match args.len() {
            1 => {
                crate::commands::checkout::checkout_command(&args[0], false);
                debug_log!("Checking out branch: {}", args[0]);
            }
            2 => {
                crate::commands::checkout::checkout_command(&args[1], true);
                debug_log!("Checking out branch: {}", args[1]);
            }
            _ => {}
        },
        crate::cli::args::ArgType::Merge(name) => {
            debug_log!("Merge branch: {}", name);
            crate::commands::merge::merge_command(name.as_str());
        }
        _ => {
            println!("Unknown command");
        }
    }
}
