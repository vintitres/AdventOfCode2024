day := 1

build: target/a.out

run: input/2024/day$(day).txt build
	./target/a.out <$<

target/a.out: src/cpp/day0$(day).cpp
	clang++ -std=c++17 $^ -o target/a.out
