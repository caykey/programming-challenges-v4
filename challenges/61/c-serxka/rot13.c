void rot13(char *msg) {
    while (*msg) {
        if (*msg >= 'a' && *msg <= 'z') {
            *msg = (*msg - 'a' + 13) % 26 + 'a';
        } else if (*msg >= 'A' && *msg <= 'Z') {
            *msg = (*msg - 'A' + 13) % 26 + 'A';
        }
        ++msg;
    }
}

#include <stdio.h>
#include <string.h>

int main(void) {
    char *msg = strdup("Hello world!");
    rot13(msg);
    printf("%s\n", msg);
}
