# Si uso mas de un thread, puede llegar a fallar porque estan todos intentando
# leer del mismo archivo y capaz se pisan.
THREADS = --test-threads 1
FILE = 
STACK = 

run:
	cargo run -- $(FILE)

run-stack:
	cargo run -- $(FILE) stack-size=$(STACK)

test:
	cargo test -- $(THREADS)	

clean:
	cargo clean