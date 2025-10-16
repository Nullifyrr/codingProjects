#include <stdio.h>
#include <math.h>
#include <string.h>

int calc(float x, float y, const char o[]);

int main() {
    float x = 0;
    float y = 0;
    char o[2] = "";
    printf("Calculator !\n");
    
    printf("What would you like the first number to be?\n");
    scanf("%f", &x);

    printf("What about the operation?\n");
    scanf("%s", o);

    printf("Finally, the second number?\n");
    scanf("%f", &y);

    calc(x, y, o);

    return 0;
}

int calc(float x, float y, const char o[]) {
    if (strcmp(o, "+") == 0) {
        printf("%f + %f = %f\n", x, y, x + y);
    } else if (strcmp(o, "-") == 0) {
        printf("%f - %f = %f\n", x, y, x - y);
    } else if (strcmp(o, "*") == 0) {
        printf("%f * %f = %f\n", x, y, x * y);
    } else if (strcmp(o, "/") == 0) {
        printf("%f / %f = %f\n", x, y, x / y);
    } else if (strcmp(o, "^") == 0) {
        printf("%f ^ %f = %f\n", x, y, powf(x, y));
    } else {
        printf("ERR: Invalid operation!\n");
    }

    return 0;
}