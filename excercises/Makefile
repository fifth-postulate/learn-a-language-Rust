.PHONE = all

SOURCES = $(shell echo *.md)
PDFS = $(addsuffix .pdf,$(basename $(SOURCES)))

all: $(PDFS)
	@echo "Finished"

$(PDFS): $(SOURCES)
	pandoc $< -o $@
