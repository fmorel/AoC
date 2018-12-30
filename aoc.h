#include <stdio.h>
#include <string.h>
#include <stdlib.h>


void __attribute__((noreturn)) error(char *s) {
    printf("##Error : %s\n", s);
    exit(1);
}


