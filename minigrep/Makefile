#The Target
#For Binary: <name>, For Library: lib<name>.a
NAME		:=	minigrep


#Compiler, Linker, Flags
CARGO		:= cargo	


#Defauilt Make
all:
	@echo "make clean => clean files"
	@echo "make test => running tests"
	@echo "make build => builning the project"
	@echo "make run query file=> running the binary"
	@echo "make check => checking compilation"

#Clean only Objects
clean:
	$(CARGO) clean

build:
	$(CARGO) build
	
run:
	$(CARGO) run $(query) $(file)
	
test:
	$(CARGO) test

check:
	$(CARGO) check

