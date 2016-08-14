[ -e c_fast.c ] || wget https://gist.githubusercontent.com/bjourne/4599a387d24c80906475b26b8ac9e5b7/raw/65bfd6099432b0821dee701248b5941c7fb47a14/c_fast.c
gcc -O2 c_fast.c
./a.out
