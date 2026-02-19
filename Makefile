.PHONY: show-man

show-man:
	man $$(ls -t target/debug/build/rep-grep-*/out/rep.1 | head -1)
