RUSTC=printf "\033[32;1mRustc:\033[33m %s\033[m\n" $@; rustc
SRC=$(wildcard */rust/*.rs)
PROG:=$(patsubst %.rs,%,$(SRC))

.SILENT:

all: exe
	# Build executables

help:
	# Show this help
	grep -A1 ^[a-z].*\: Makefile | sed -r 's/: (.*)$$/:/g' | sed ':a;N;$$!ba;s/:\n//g' | sed s,\\#,\\t,g | grep -v \\--

clean:
	# Remove executables
	rm -fr $(PROG)

exe: $(PROG)
	# Build executables

% : %.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@
