day%: day%.rs day%.txt
	rustfmt day$*.rs
	rustc day$*.rs -o day$*
	./day$* < day$*.txt
