#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_VAL 1000


int main(void)
{
    int freq, freq_sum = 0;
    FILE *f = fopen("day_1_input", "r");

    int n_values = 0;
    int values[MAX_VAL];

    int i, j, vi, vj, min_j, min_i, k;

    while (!feof(f)) {
        if (!fscanf(f, "%d\n", &freq))
            break;
        //printf("%d %d\n", freq, freq_sum);
        freq_sum += freq;
        values[n_values++] = freq_sum;
    }
    fclose(f);

    printf("Length=%d, sum=%d\n", n_values, freq_sum);

    min_j = n_values;
    for (i = 0; i < n_values; i++) {
        for (j = 0; j < n_values; j++) {
            if (i==j) continue;
            vi = values[i];
            vj = values[j];
            if (((vi -vj) % freq_sum) == 0)  {
                k = (vi - vj) / freq_sum;
                if (j < min_j && k > 0) {
                    printf("Found indices i=%d, j=%d vi=%d, vj=%d k=%d\n", i, j, vi, vj, k);
                    min_j = j;
                    min_i = i;
                }
            }
        }
    }

    printf("First repeating value is %d\n", values[min_i]);

    return 0;
}
