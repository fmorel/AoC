#include <stdio.h>
#include <stdlib.h>
#include <string.h>


#define MAX_CHAR 256
#define MAX_LINE 250

int main (void)
{
    FILE *f = fopen("day_2.input", "r");
    int line = 0;
    int bit_signature[MAX_LINE], xor_sig;
    char content[MAX_LINE][32];
    int i, j, k, candidates = 0;
    
    while (1) {
        char letters[MAX_CHAR];
        int pos;

        memset(letters, 0, sizeof(letters));
        memset(content[line], 0, 32);
        pos = 0;
        //Parse line
        while (1) {
            int c = fgetc(f);
            if (c == EOF)
                goto end_of_file;
            if (c == '\n')
                break;
            if (c > MAX_CHAR)
                exit(1);

            letters[c]++;
            content[line][pos] = c;
            pos++;
        }
        //Store the count parity for each letter to have a quick 26-wide signature
        bit_signature[line] = 0;
        for (i = 'a'; i <= 'z'; i++) {
            bit_signature[line] |= (letters[i] & 1) << (i - 'a');
        }
        line++;
        if (line > MAX_LINE)
            exit(1);
    }

end_of_file:
    fclose(f);
    
    for (i = 0; i < line; i++) {
        for (j = i+1; j < line; j++) {
            //The lines we are looking for have exactly two bits difference (one letter swap)
            xor_sig = bit_signature[i] ^ bit_signature[j];
            if (__builtin_popcount(xor_sig) == 2) {
                candidates++;
                int error = 0;
                for (k = 0; k < 32; k++) {
                    if (content[i][k] != content[j][k]) {
                        if (error) 
                            break;
                        error = 1;
                    }
                }
                if (k == 32) {
                    printf("After %d candidates, Lines found %d and %d, content is %s and %s\n",
                        candidates, i, j, content[i], content[j]);
                    break;
                }
            }
        }
    }

    return 0;
}
