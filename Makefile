RUSTC := rustc      # rust compiler
RUSTDOC := rustdoc  # rust doc generator

# compiler flags
RUSTCFLAGS := -O --color auto
RUSTDOCFLAGS := -v

# default target
RUSTTARGET := x86_64-unknown-linux-gnu
RUSTCFLAGS := $(RUSTCFLAGS) --target $(RUSTTARGET)

ifeq ($(DEBUG), 1) # use debug info
	RUSTCFLAGS := $(RUSTCFLAGS) -g -v
endif

# outputs
OUTPUTS := libsnappy.a libsnappy.rlib test docs snappy

# begin rules
all: snappy

snappy: main.c libsnappy.a
	$(CC) $(CFLAGS) $? -o $@

test: test.rs
	$(RUSTC) $(RUSTCFLAGS) $? --test -o $@

run-test: test
	@ ./test

clean:
	$(RM) -r $(OUTPUTS)

docs: main.rs
	@ mkdir docs
	$(RUSTDOC) $(RUSTDOCFLAGS) $? -o $@

libsnappy.a: main.rs
	$(RUSTC) $(RUSTCFLAGS) $? --crate-type staticlib --crate-name snappy

libsnappy.rlib: snappy.rs
	$(RUSTC) $(RUSTCFLAGS) $? --crate-type rlib --crate-name snappy

.PHONY: all run-test clean
