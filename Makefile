day%: day%.rs
	rustfmt day$*.rs
	rustc day$*.rs -o day$*

day%-final: day% day%.txt
	./day$* < day$*.txt

day%-test: day% day%-test.txt
	./day$* < day$*-test.txt
