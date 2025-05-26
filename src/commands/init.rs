use crate::core::repository::Repository;
pub fn init_command(){
	// 初始化 Git 仓库
	// 创建 .git 目录结构
	// 创建对象存储目录
	// 创建引用存储目录
	// 创建 HEAD 文件
	if !Repository::is_git_repo(".") {
		Repository::init(".");
	} else {
		println!("A Git repository already exists in this directory.");
		return;
	}
	println!("Git repository initialized.");
}