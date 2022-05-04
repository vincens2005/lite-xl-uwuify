#include "uwu.h"
#include <stdio.h>

int main() {
	const char* uwu = uwuify("I like to eat cheese becase it is delicious");
	printf("\r\n\t\tTHIS IS FROM C: %s\r\n\r\n", uwu);
	printf("%zu   %zu\r\n", sizeof("i wike to eat cheese becase it is d-dewicious"), sizeof(uwu));
	return 0;
}
