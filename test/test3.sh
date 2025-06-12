rm -rf test3
mkdir test3
cp ../target/debug/rust-git test3/ 
cd test3
echo "1" > test.txt 
 ./rust-git init
 # 创建文件 test.txt 并添加内容
echo "Hello, Rust!" > test.txt
 # 执行 git add 和 git commit
 ./rust-git add test.txt
 commit_hash=$(./rust-git commit -m "Initial commit" 2>&1)
# 执行 git branch test
 ./rust-git branch test
 # 执行 git checkout test
 ./rust-git checkout test
 # 验证 .git/refs/heads/test 文件是否存在
if [ -f ".git/refs/heads/test" ]; then
 echo ".git/refs/heads/test exists"
 else
 echo ".git/refs/heads/test does not exist"
 exit 1
 fi
 # 验证 .git/HEAD 文件是否指向 refs/heads/test
 if grep -q "ref: refs/heads/test" ".git/HEAD"; then
 echo ".git/HEAD points to refs/heads/test"
 else
 echo ".git/HEAD does not point to refs/heads/test"
 exit 1
 fi
 echo "Test 3 passed: git branch and git checkout succeeded"