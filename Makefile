CC=cargo
FMT=fmt

OPTIONS=

default: fmt
	$(CC) build

fmt:
	$(CC) fmt --all

check:
	$(CC) test --all

example:
	@echo "No example yet"

clean:
	$(CC) clean
