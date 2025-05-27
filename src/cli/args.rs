use crate::utils::error::ErrorType;
pub enum ArgType {
    Init(String),
    Error(ErrorType),
    Add(String),
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
            "add" =>{
                match args.len(){
                    2 => ArgType::Error(ErrorType::InvalidArgument), // 如果只有 add 命令则报错
                    3 => ArgType::Add(args[2].clone()), // 如果有额外参数则使用该参数作为路径
                    _ => ArgType::Error(ErrorType::InvalidArgument), // 如果有额外参数则报错
                }
            }
            _ => ArgType::Error(ErrorType::UnknownCommand),
        }
    } else {
        ArgType::Error(ErrorType::UnknownCommand)
    }
}
