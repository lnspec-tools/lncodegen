BOLTDIR=bolts

all: bolt1.csv bolt2.csv

bolt1.csv: $(BOLTDIR)/01-messaging.md
bolt2.csv: $(BOLTDIR)/02-peer-protocol.md

bolt%.csv:
	$(BOLTDIR)/tools/extract-formats.py $< > $@