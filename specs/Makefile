BOLTDIR=bolts

all: bolt1.csv bolt2.csv bolt3.csv bolt4.csv bolt5.csv bolt7.csv bolt8.csv bolt9.csv bolt10.csv bolt11.csv

bolt1.csv: 	$(BOLTDIR)/01-messaging.md
bolt2.csv: 	$(BOLTDIR)/02-peer-protocol.md
bolt3.csv: 	$(BOLTDIR)/03-transactions.md
bolt4.csv: 	$(BOLTDIR)/04-onion-routing.md
bolt5.csv: 	$(BOLTDIR)/05-onchain.md
bolt7.csv: 	$(BOLTDIR)/07-routing-gossip.md
bolt8.csv: 	$(BOLTDIR)/08-transport.md
bolt9.csv: 	$(BOLTDIR)/09-features.md
bolt10.csv: $(BOLTDIR)/10-dns-bootstrap.md
bolt11.csv: $(BOLTDIR)/11-payment-encoding.md

bolt%.csv:
	$(BOLTDIR)/tools/extract-formats.py $< > $@
