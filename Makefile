default:
	cargo build
	gcc -Wall -g csrc/main.c -Icsrc/include -Ltarget/debug/ -luwuify_c -shared -o build/uwu.so
