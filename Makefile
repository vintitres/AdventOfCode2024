build: target/a.out

run: build input/2024/day1.txt
	./target/a.out <input/2024/day1.txt

target/a.out: src/cpp/day01.cpp
	clang++ src/cpp/day01.cpp -o target/a.out
