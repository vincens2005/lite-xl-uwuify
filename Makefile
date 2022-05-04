default:
	cargo build
	gcc -Wall -g csrc/main.c -Icsrc/include -Ltarget/debug/ -luwuify_c -o build/test
