#include <stdio.h>
#include <string.h>
#include <stdlib.h>


void __attribute__((noreturn)) error(char *s) {
    printf("##Error : %s\n", s);
    exit(1);
}

const char polarity_offset = 'A' - 'a';

int adjacent_polarity(const char *s, int pos)
{
    char d = s[pos] - s[pos+1];
    if (d == polarity_offset || d == -polarity_offset)
        return 1;
    return 0;
}


size_t reduce_polymer_without_unit(const char *s, size_t len, char u)
{
    char *s1 = malloc(len);
    char *s2 = malloc(len);
    char *swap;
    size_t len2 = 0, i, iter = 0;

    //Remove unit and copy to s1
    for (i = 0; i < len; i++) {
        if (s[i] == u || s[i] == u + polarity_offset)
            continue;
        s1[len2++] = s[i];
    }

    printf("Remove unit %c len %ld -> %ld\n", u, len, len2);
    len = len2;
    len2 = 0;

    //Reduce polymer
    while (1) {
        for (i = 0; i < len-1; i++) {
            if (adjacent_polarity(s1, i)) {
                i++;
                continue;
            }
            s2[len2++] = s1[i];
        }
        if (i == len-1)
            s2[len2++] = s1[len-1];
        s2[len2] = '\0';

        swap = s1;
        s1 = s2;
        s2 = swap;
        if (len == len2)
            break;
        len = len2;
        len2 = 0;
        iter++;
    }

    printf("%ld iterations, final length is %ld\n", iter, len);
    free(s2);
    free(s1);
    return len;
}



int main(void)
{

    /* Parse */
    FILE *f = fopen("day_5.input", "r");
    char *s = NULL;
    size_t len = 0, cur_len, min_len;
    char c, min_c;
    ssize_t ret;

    ret = getline(&s, &len, f);
    if (ret < 0)
        error("Can't parse input");
    printf("Parse string of length %ld char OK\n", ret);
    fclose(f);

    //Replace new_line with null char
    len = ret-1;
    s[len] ='\0';
    
    printf("Part 1 answer is %ld\n", reduce_polymer_without_unit(s, len, '0'));

    min_len = len;
    for (c = 'a'; c <= 'z'; c++) {
        cur_len = reduce_polymer_without_unit(s, len, c);
        if (cur_len < min_len) {
            min_len = cur_len;
            min_c = c;
        }
    }
    printf("Minimum length is %ld (removing unit %c)", min_len, min_c);
    free(s);

    return 0;
}
    
