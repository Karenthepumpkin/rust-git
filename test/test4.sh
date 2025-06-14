rm -rf test4
mkdir test4
# 拷⻉ rust-git 到 test5 ⽬录 
cp ../target/debug/rust-git test4/ 
# 进入 test5 ⽬录
cd test4
# 执⾏ rust-git init 
./rust-git init 
# 创建文件 test.txt 并添加内容
echo "1" > test.txt 
# 执⾏ rust-git add 和 rust-git commit 
./rust-git add git.png 
hash1=$(/bin/bash -c './rust-git commit -m "add png file" 2>&1') 
# 添加第⼆个文件
# 创建文件 test1.txt 并添加内容
echo "Hello, Rust!" > test1.txt 
# 执⾏ git add 和 git commit 
./rust-git add test1.txt 
hash2=$(./rust-git commit -m "Add test file" 2>&1) 
# 检查是否成功获取哈希值
if [ -z "$hash1" ] || [ -z "$hash2" ]; then 
echo "Commit hash is empty!" 
exit 1 
fi 
# 提取哈希值的前两位和后⾯的位数。请注意，⼀定要只返回hash值。 
hash_prefix1=${hash1:0:2} 
hash_suffix1=${hash1:2} 
hash_prefix2=${hash2:0:2} 
hash_suffix2=${hash2:2} 
# 检查 .git/objects ⽬录下是否存在对应的对象文件。如果文件不存在，则打印报错信息。
if [ -f ".git/objects/$hash_prefix1/$hash_suffix1" ] && [ -f ".git/objects/$hash_prefix2/$hash_suffix2" ]; then 
echo "Success!"
 else 
echo "The object file does not exist!" 
exit 1 
fi