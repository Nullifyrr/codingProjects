#include <stdio.h>

int main() {
    int x = 0;
    while (x < 101) {
        if (x % 15 == 0) {
            printf("FizzBuzz!\n");
        } else if (x % 5 == 0) {
            printf("Buzz!\n");
        } else if (x % 3 == 0) {
            printf("Fizz!\n");
        } else {
            printf("%d\n", x);
        }
        x += 1;
    }
    return 0;
}