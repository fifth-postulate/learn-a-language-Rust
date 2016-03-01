.PHONE = all clean

SOURCES = $(shell echo *.md)
PDFS = $(addsuffix .pdf,$(basename $(SOURCES)))
ZIP = exercises.zip

all: $(PDFS) $(ZIP)
	@echo "Finished"

clean:
	rm *.pdf
	rm $(ZIP)

%.pdf: %.md
	pandoc $< -o $@

$(ZIP):
	zip $(ZIP) $(PDFS)
