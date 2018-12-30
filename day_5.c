#include <stdio.h>
#include <string.h>
#include <stdlib.h>


void __attribute__((noreturn)) error(char *s) {
    printf("##Error : %s\n", s);
    exit(1);
}

const char polarity_offset = 'A' - 'a';

int adjacent_polarity(char *s, int pos)
{
    char d = s[pos] - s[pos+1];
    if (d == polarity_offset || d == -polarity_offset)
        return 1;
    return 0;
}

int main(void)
{

    /* Parse */
    FILE *f = fopen("day_5.input", "r");
    char *s = NULL, *s2, *swap;
    size_t len = 0, len2 = 0, i;
    int iter = 0;
    ssize_t ret;

    ret = getline(&s, &len, f);
    if (ret < 0)
        error("Can't parse input");
    printf("Parse string of length %ld char OK\n", ret);
    fclose(f);

    //Replace new_line with null char
    len = ret-1;
    s[len] ='\0';
    s2 = malloc(len);

    while (1) {
        for (i = 0; i < len-1; i++) {
            if (adjacent_polarity(s, i)) {
                i++;
                continue;
            }
            s2[len2++] = s[i];
        }
        if (i == len-1)
            s2[len2++] = s[len-1];
        s2[len2] = '\0';

        swap = s;
        s = s2;
        s2 = swap;
        if (len == len2)
            break;
        len = len2;
        len2 = 0;
        iter++;
        printf("Pass %d, new length is %ld\n", iter, len);
    }

    printf("Result is length %ld\nvalue = %s\n", strlen(s), s);
    free(s);
    free(s2);

    return 0;
}
    
