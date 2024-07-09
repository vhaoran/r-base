



rm -rf ./target

ssh whr@w2 "cd /D d:\rust\; rmdir r-base /D /Q"



scp -r ../r-base  whr@w2:/d:/rust/


