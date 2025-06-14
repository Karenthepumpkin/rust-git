# 当前目录位于 testcases
 # 创建一个空目录 test4
 rm -rf test6
 mkdir test6
 # 拷贝 rust-git 到 test4 目录
cp ../target/debug/rust-git test6/ 
 # 进入 test4 目录
cd test6
 # 执行 rust-git init
 ./rust-git init
 # 创建 main 分支并切换到 main 分支
./rust-git checkout -b main
 # 创建 main.rs 文件并添加内容
echo 'use std::fs::File;
 use std::io::{self, Read};
 fn main() -> io::Result<()> {
 let mut file = File::open("test.txt")?;
 let mut contents = String::new();
 file.read_to_string(&mut contents)?;
 
 println!("{}", contents);
 Ok(())}' > main.rs
 # 添加并提交 main.rs
 ./rust-git add .
 commit_hash=$(./rust-git commit -m "update main.rs" 2>&1)
  # 创建 test 分支
./rust-git branch test
 # 切换到 test 分支
./rust-git checkout test
 # 创建 test.txt 文件并添加内容
echo "测试分支合并" > test.txt
 # 添加并提交 test.txt
 ./rust-git add .
 ./rust-git commit -m "update test.txt"
 # 切换回 main 分支
./rust-git checkout main
 # 合并 test 分支
./rust-git merge test
 # 编译 main.rs
 rustc main.rs
 # 运行 main.rs 并检查输出
if ./main | grep -q "测试分支合并"; then
  echo "Test 4 passed: git merge succeeded and main.rs output is correct"
else
  echo "Test 4 failed: main.rs output is incorrect"
  exit 1
fi