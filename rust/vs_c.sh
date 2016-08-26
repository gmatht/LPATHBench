c() {
	printf "%58s: " "$1"
	$1 c_fast.c
	./a.out
}
r() {
printf "%58s: " "`rustc --version`"
target/release/rs
}
cargo build --release
echo \
-------------------------------------------------------------------------------
cat /proc/cpuinfo  | grep model.name | head -n1
r
[ -e c_fast.c ] || wget https://gist.githubusercontent.com/bjourne/4599a387d24c80906475b26b8ac9e5b7/raw/65bfd6099432b0821dee701248b5941c7fb47a14/c_fast.c
c "gcc -O2"
c "gcc -O3"
c "clang -O2"
c "clang -O3"
c "clang -O3 -march=native -mtune=native -fomit-frame-pointer"
r
