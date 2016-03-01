.PHONE = all clean

SOURCES = $(shell echo *.md)
PDFS = $(addsuffix .pdf,$(basename $(SOURCES)))

all: $(PDFS)
	@echo "Finished"

clean:
	rm *.pdf

%.pdf: %.md
	pandoc $< -o $@
