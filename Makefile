PREFIX ?= ${HOME}/.config/lite-xl/plugins

all: clean build

build:
	if [ -d "build" ]; then \
		rm -rf build; \
	fi
	mkdir build
	
	cargo build --release
	gcc -Wall -g -fPIC csrc/uwu.c -Icsrc/include -Ltarget/release/ -luwuify_c -shared -o build/uwu.so
	cp csrc/init.lua build/

clean:
	@echo cleaning
	@rm -rf build
	


install: clean build
	@echo installing plugin to ${PREFIX}
	@rm -rf ${PREFIX}/uwuifier
	@cp -r build ${PREFIX}/uwuifier	
