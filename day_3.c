#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define GRID_SIZE 1024

typedef struct {
    int id;
    int x;
    int y;
    int w;
    int h;
    int intact;
} Square;

int grid[GRID_SIZE][GRID_SIZE];
Square squares[1331];

Square* get_square(int id)
{
    return &squares[id-1];
}

int fill_square(Square *s) {
    int i,j;
    int overlap = 0;
    s->intact = 1;

    for (i = 0; i < s->w; i++) {
        for (j = 0; j < s->h; j++) {
            int *g = &grid[s->x + i][s->y + j];
            if (*g == 0)
                *g = s->id;
            else {
                s->intact = 0;
                if (*g > 0) {
                    get_square(*g)->intact = 0;
                    *g = -1;
                    overlap++;
                }
            }
        }
    }

    return overlap;
}

int main(void)
{
    FILE *f = fopen("day_3.input", "r");
    int overlap = 0, elves = 0;
    int i;

    while (!feof(f)) {
        Square *s = &squares[elves];
        fscanf(f, "#%d @ %d,%d: %dx%d\n",
            &s->id, &s->x, &s->y, &s->w, &s->h);
        overlap += fill_square(s);
        elves++;
    }

    printf("%d elves; %d square inches overlap\n", elves, overlap);
    
    for (i = 0; i < elves; i++) {
        if (squares[i].intact) {
            printf("Intact square is %d (ID %d)\n", i, squares[i].id);
        }
    }

    return 0;
}
