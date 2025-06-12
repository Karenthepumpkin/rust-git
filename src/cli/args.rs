use crate::utils::error::ErrorType;
pub enum ArgType {
    Init(String),
    Error(ErrorType),
    Add(Vec<String>),
    Rm(String),
    Commit(String),
    Branch(Vec<String>),
    Checkout(Vec<String>),
    Merge(String),
}
pub fn git_parse_args() -> ArgType {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "init" => {
                match args.len() {
                    2 => ArgType::Init(".".to_string()), // 只有 init 命令
                    3 => ArgType::Init(args[2].clone()), // 如果有额外参数则使用该参数作为路径
                    _ => ArgType::Error(ErrorType::InvalidArgument), // 如果有额外参数则报错
                }
            }
            "add" => {
                match args.len() {
                    2 => ArgType::Error(ErrorType::InvalidArgument), // 如果只有 add 命令则报错
                    _ => ArgType::Add(args[2..].to_vec()),           // 如果有额外参数则报错
                }
            }
            "rm" => {
                match args.len() {
                    2 => ArgType::Error(ErrorType::InvalidArgument), // 如果只有 rm 命令则报错
                    3 => ArgType::Rm(args[2].clone()), // 如果有额外参数则使用该参数作为路径
                    _ => ArgType::Error(ErrorType::InvalidArgument), // 如果有额外参数则报错
                }
            }
            "commit" => {
                match args.len() {
                    2 => ArgType::Error(ErrorType::InvalidArgument), // 如果只有 commit 命令则报错
                    4 => ArgType::Commit(args[3].clone()), // 如果有额外参数则使用该参数作为路径
                    _ => ArgType::Error(ErrorType::InvalidArgument), // 如果有额外参数则报错
                }
            }
            "branch" => {
                match args.len() {
                    2 => ArgType::Error(ErrorType::InvalidArgument), // 如果只有 branch 命令则报错
                    3 => ArgType::Branch(args[2..].to_vec()), // 如果有额外参数则使用该参数作为路径
                    _ => ArgType::Error(ErrorType::InvalidArgument), // 如果有额外参数则报错
                }
            }
            "checkout" => {
                match args.len() {
                    2 => ArgType::Error(ErrorType::InvalidArgument), // 如果只有 checkout 命令则报错
                    3 => ArgType::Checkout(args[2..].to_vec()), // 如果有额外参数则使用该参数作为路径
                    4 => ArgType::Checkout(args[2..].to_vec()),
                    _ => ArgType::Error(ErrorType::InvalidArgument),
                }
            }
            "merge" => {
                match args.len() {
                    2 => ArgType::Error(ErrorType::InvalidArgument), // 如果只有 checkout 命令则报错
                    3 => ArgType::Merge(args[2].clone()), // 如果有额外参数则使用该参数作为路径
                    _ => ArgType::Error(ErrorType::InvalidArgument), // 如果有额外参数则报错
                }
            }
            _ => ArgType::Error(ErrorType::UnknownCommand),
        }
    } else {
        ArgType::Error(ErrorType::UnknownCommand)
    }
}
