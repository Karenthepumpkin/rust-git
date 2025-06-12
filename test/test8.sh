 #!/bin/bash 
# 测试 rust-git 的 merge 功能 
# 创建⼀个空⽬录 test8 
rm -rf test8
mkdir test8 
# 拷⻉ rust-git 到 test8 ⽬录 
cp ../target/debug/rust-git test8/ 
# 进入 test8 ⽬录
cd test8 
# 执⾏ rust-git init 
./rust-git init 
# 创建 main 分⽀并切换到 main 分⽀ 
./rust-git checkout -b main 
# 创建 main.txt 文件并添加内容
echo "Main" > main.txt 
# 添加并提交 main.txt 
./rust-git add main.txt 
hash=$(/bin/bash -c './rust-git commit -m "Add main.txt" 2>&1') 
# 创建 test 分⽀ 
./rust-git branch test 
# 切换到 test 分⽀ 
./rust-git checkout test 
# 创建 test.txt 文件并添加内容
echo "Test" > test.txt 
# 添加并提交 test.txt 
./rust-git add test.txt 
hash1=$(/bin/bash -c './rust-git commit -m "Add test.txt" 2>&1') 
# 切换回 main 分⽀ 
./rust-git checkout main 
# 验证 main 分⽀是否包含 main.txt 和 test.txt 文件。除了文件，还可能是代码文件。
if [ -f "main.txt" ] && [ -f "test.txt" ]; then 
echo "Success!"
 else 
echo "Files do not exist in the main branch!" 
exit 1 
fi 
content1=$(./rust-git merge temp1 1>&1) 