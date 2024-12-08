day%: day%.rs day%.txt
	rustfmt day$*.rs
	rustc -C opt-level=3 day$*.rs -o day$*
	./day$* < day$*.txt
