pub enum ArgType {
    Init,
	Error
}
pub fn git_parse_args() -> ArgType{
	use std::env;
	let args: Vec<String> = env::args().collect();
	
	if args.len() > 1 {
		match args[1].as_str() {
			"init" => ArgType::Init,
			_ => ArgType::Error,
		}
	} else {
		ArgType::Error
	}
}