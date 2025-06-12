rm -rf test2
mkdir test2
cp ../target/debug/rust-git test2/ 
cd test2
# 执⾏ rust-git init 
./rust-git init 
# 创建文件 test.txt 并添加内容
echo "1" > test.txt 
echo "Hello, Rust!" > test.txt
if [ -f "test.txt" ]; then
 echo "test.txt exists"
 else
 echo "test.txt does not exist"
 exit 1
 fi
 ./rust-git init
 ./rust-git add test.txt
  commit_hash=$(./rust-git commit -m "Initial commit" 2>&1)
  echo "Commit hash: $commit_hash"
if [ "$(ls -A .git/objects)" ]; then
 echo ".git/objects directory is not empty"
 else
 echo ".git/objects directory is empty"
 exit 1
 fi
 # 验证 .git/refs/heads/master 文件是否存在且不为空
if [ -s ".git/refs/heads/master" ]; then
 echo ".git/refs/heads/master exists and is not empty"
 else
 echo ".git/refs/heads/master does not exist or is empty"
 exit 1
 fi
 echo "Test 2 passed: git add and git commit succeeded"