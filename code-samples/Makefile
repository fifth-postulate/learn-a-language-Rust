.PHONY: clean

sources=$(shell echo *.rs)
objects=$(addprefix bin/,$(basename $(sources)))

all: $(objects)
	@echo "Finished with the source files"

$(objects): bin
	rustc --out-dir bin $(addsuffix .rs,$(notdir $@))

bin:
	mkdir -p bin

clean:
	rm -rf bin
