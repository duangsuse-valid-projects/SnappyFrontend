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
OUTPUTS := libsnappy.so libsnappy.rlib test docs

# begin rules
all: libsnappy.so

test: test.rs
	$(RUSTC) $(RUSTCFLAGS) $? --test -o $@

run-test: test
	@ ./test

clean:
	$(RM) -r $(OUTPUTS)

docs: main.rs
	@ mkdir docs
	$(RUSTDOC) $(RUSTDOCFLAGS) $? -o $@

libsnappy.so: main.rs
	$(RUSTC) $(RUSTCFLAGS) $? --crate-type cdylib --crate-name snappy

libsnappy.rlib: snappy.rs
	$(RUSTC) $(RUSTCFLAGS) $? --crate-type rlib --crate-name snappy

.PHONY: all run-test clean
