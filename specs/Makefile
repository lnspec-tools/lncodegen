BOLTDIR=bolts

all: bolt1.csv bolt2.csv bolt7.csv

bolt1.csv: $(BOLTDIR)/01-messaging.md
bolt2.csv: $(BOLTDIR)/02-peer-protocol.md
bolt7.csv: $(BOLTDIR)/07-routing-gossip.md

bolt%.csv:
	$(BOLTDIR)/tools/extract-formats.py $< > $@
