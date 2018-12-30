#include "aoc.h"

#define GRID_SIZE 400
#define MAX_SEEDS 100
#define MAX_DIST 100

typedef enum {
    SEED_IN_PROG,
    SEED_DONE,
    SEED_INFINITE,
} SeedStatus;

typedef struct {
    int id;
    int x;
    int y;
    int area;
    SeedStatus status;
} Seed;

typedef struct {
    Seed *seed;
    char dist;
    char boundary;
} Coord;


Coord grid[GRID_SIZE][GRID_SIZE];
Seed seeds[MAX_SEEDS];


void print_grid(int size)
{
    int x, y;
    for (x = 0; x < size; x++) {
        for (y = 0; y < size; y++) {
            Coord *c = &grid[x][y];
            if (!c->seed)
                printf("*");
            else {
                if (c->boundary)
                    printf(".");
                else
                    printf("%c", 'A' + c->seed->id + 32*(c->dist>0));
            }
        }
        printf("\n");
    }
}

void handle_coord(int x, int y, Seed *s, int dist)
{
    if (x < 0 || x >= GRID_SIZE ||
        y < 0 || y >= GRID_SIZE) {
        
        if  (x < -1 || x >= GRID_SIZE+1 ||
            y < -1 || y >= GRID_SIZE+1) {
       
            s->status = SEED_INFINITE;
        }
        return;
    }
    Coord *c = &grid[x][y];
    //Cell belongs to another coordinate
    if (c->seed) {
        if (c->dist < dist) {
            return;
        } else if (c->dist == dist) {
            if (!c->boundary) {
                c->boundary = 1;
                c->seed->area--;    //Remove this coord from other seed's area
            }
            return;
        }
    }
    //Claim coordinate
    c->seed = s;
    c->dist = dist;
    s->area++;
}



int main(void)
{

    FILE *f = fopen("day_6.input", "r");
    int i, n_seeds = 0, x, y;
    int dist, start_area;

    memset(grid, 0, sizeof(grid));
    memset(seeds,0, sizeof(seeds));

    /* Parse file */
    while (!feof(f)) {
        Seed *s = &seeds[n_seeds];
        fscanf(f, "%d, %d\n", &s->y, &s->x);
        s->id = n_seeds;
        s->area = 1;
        grid[s->x][s->y].seed = s;
        grid[s->x][s->y].dist = 0;
        grid[s->x][s->y].boundary  = 0;

        n_seeds++;
        if (n_seeds > MAX_SEEDS)
            error("Too many seeds\n");
    }
    
    printf("%d seeds found \n", n_seeds);
    //print_grid(10);
    fclose(f);

    /* Grow space around each seed */
    for (dist = 1; dist < MAX_DIST; dist++) {
        printf("Step %d\n", dist);
        for (i = 0; i < n_seeds; i++) {
            Seed *s = &seeds[i];
            if (s->status == SEED_DONE)
                continue;
            start_area = s->area;
            
            //Handle coord in the Manhattan distance circle 
            for (x = s->x - dist, y = s->y; x < s->x; x++, y--) {
                handle_coord(x, y, s, dist);
            }
            for (x = s->x, y = s->y - dist; x < s->x + dist; x++, y++) {
                handle_coord(x, y, s, dist);
            }
            for (x = s->x + dist, y = s->y; x > s->x; x--, y++) {
                handle_coord(x, y, s, dist);
            }
            for (x = s->x, y = s->y + dist; x > s->x - dist; x--, y--) {
                handle_coord(x, y, s, dist);
            }

            if (s->area == start_area && s->status != SEED_INFINITE) {
                s->status = SEED_DONE;
            }
        }
        //print_grid(10);
    }



    /* Print seeds */
    int max_area = 0;
    for (i = 0; i < n_seeds; i++) {
        printf("Seed %d area %d status = %d\n", seeds[i].id, seeds[i].area, seeds[i].status);
        if (seeds[i].status == SEED_DONE && seeds[i].area > max_area)
            max_area = seeds[i].area;
    }
    printf("Maximum finite area is %d\n", max_area);

    return 0;
}
