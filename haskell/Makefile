GHCFLAGS = -O2

2025 := 2025-01

all := $(2025)

all: $(all)

%: %.hs
	ghc $(GHCFLAGS) -o $@ $^

run-%: %.hs
	runghc $^ < inputs/$*.txt

install: all
	for i in $(all); do\
		install -Dm744 $${i} -t $(PREFIX)/bin;\
	done

clean:
	git clean -fx

.PHONY: run-%
