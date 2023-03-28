SOURCES := $(wildcard cairo_programs/*.cairo)
TARGETS := $(patsubst cairo_programs/%.cairo, corpus/%.json, $(SOURCES))

all: $(TARGETS)

corpus_dir:
	mkdir corpus

corpus/%.json: cairo_programs/%.cairo corpus_dir
	cairo-compile $< --output $@

clean:
	rm -rf corpus
