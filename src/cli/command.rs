use crate::cli::args::git_parse_args;
use crate::commands::init::init_command;
use crate::debug_log;

pub fn git_execute() {
    let matches = git_parse_args();
    // TODO: 解析命令行参数
    match matches {
        // 如果是 Init 命令
        crate::cli::args::ArgType::Init(path) => {
            debug_log!("Initializing git repository at {}", path);
            init_command(path.as_str());
        }
        // 如果是其他命令，可以继续添加分支
        _ => {
            println!("Unknown command");
        }
    }
}
