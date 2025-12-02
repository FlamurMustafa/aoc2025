#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int ivalue(char *string) {
    return atoi(string + 1);
}

static inline int positive_modulo1(int i, int n) {
    return (i % n + n) % n;
}

int main() {
    FILE *file = fopen("input", "r");

    char buf[256];
    int position = 50;
    int count = 0;
    int value = 0;

    while(fgets(buf, sizeof(buf), file)) {
        switch (buf[0]) {
            case 'L':
                value = ivalue(buf);
                for (int i = 0 ; i < value; i ++) {
                    position -= 1;
                    position = positive_modulo1(position, 100);
                    if (position  == 0) {
                        count++;
                    }
                }

                break;
            case 'R':
                value = ivalue(buf);

                position += value;

                if (position >= 100) {
                    count += position/100;
                    position %= 100;
                }
                break;
            default:
                abort();
        }
    }
    printf("The count is %d\n", count);
}
