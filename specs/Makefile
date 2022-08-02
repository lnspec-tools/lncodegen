BOLTDIR=bolts

all: bolt1.csv

bolt1.csv: $(BOLTDIR)/01-messaging.md

bolt%.csv:
	$(BOLTDIR)/tools/extract-formats.py $< > $@