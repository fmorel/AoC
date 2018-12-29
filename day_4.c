#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include <stdint.h>


typedef struct {
    int y;
    int mo;
    int d;
    int h;
    int mi;
} Date;

typedef enum {
    BEGIN,
    ASLEEP,
    WAKE
} EventType;

typedef struct {
    Date day;
    int id;
    uint64_t asleep_bmp;
} Day;


typedef struct {
    Day *day;
    Date date;
    int id;
    EventType type;
} Event;

#define MAX_EVENTS 1040
Event events[MAX_EVENTS];

int n_days = 0;
Day days[365];


void __attribute__((noreturn)) error(char *s) {
    printf("##Error : %s\n", s);
    exit(1);
}

void date_incr_by_1(Date *d)
{
    d->d++;
    if ((d->mo == 2 && d->d == 29) ||
        (d->d == 31 && (d->mo == 4 || d->mo == 6 || d->mo == 9 || d->mo == 11)) ||
         d->d == 32) {
        d->d = 1;
        d->mo++;
    }
    if (d->mo == 13) {
        d->mo = 1;
    }
}

int days_equal(Day *d1, Day *d2)
{
    return (d1->day.mo == d2->day.mo && d1->day.d == d2->day.d);
}

Day *add_day(Day *d)
{
    int i;
    for (i = 0; i < n_days; i++) {
        if (days_equal(d, &days[i]))
            return &days[i];
    }
    days[n_days] = *d;
    days[n_days].asleep_bmp = 0;

    return &days[n_days++];
}

void print_bitmap(uint64_t *bmp) {
    int i;
    for (i=0; i < 60; i++) {
        if (*bmp & (1LL<<i))
            printf("#");
        else
            printf(".");
    }
    printf("\n");
}

void bitmap_asleep(uint64_t *bmp, int bit)
{
    uint64_t before_bmp;
    //Compute the number of bits between the previous asleep bit and bit -1
    before_bmp = *bmp & ((1ULL<<bit) -1);
    before_bmp <<= (64 - bit);
    int n_before = __builtin_clzll(~before_bmp);
    
    uint64_t n_before_mask = ((1ULL<<bit) -1) & ~((1ULL<<(bit - n_before)) -1);
    //Asleep bit :
    *bmp |= (1ULL << bit);
    //Put all bits before (and until the previous asleep bit) to 0
    *bmp &= ~(n_before_mask);
}


void bitmap_awake(uint64_t *bmp, int bit)
{
    uint64_t before_bmp;
    //Compute the number of bits between the previous asleep bit and fill them all to 1
    before_bmp = *bmp & ((1ULL<<bit) -1);
    before_bmp <<= (64 - bit);
    int n_before = __builtin_clzll(before_bmp);
    uint64_t n_before_mask = ((1ULL<<bit) -1) & ~((1ULL<<(bit - n_before)) -1);
    //Awake bit :
    *bmp &= ~(1ULL << bit);
    //Put all bits before (and until the previous asleep bit) to 1
    *bmp |= n_before_mask;
}

int main(void)
{

    FILE *f = fopen("day_4.input", "r");

    Date d_s, *d = &d_s;
    Day day_s, *day = &day_s;
    Event *ev;
    int i = 0, j, n_events;
    int n_b = 0, n_a = 0, n_w = 0;

    size_t len = 64;
    char *string = malloc(len);

    while (!feof(f)) {
        ev = &events[i];
        int id, n_args;

        // Get date
        n_args = fscanf(f, "[%d-%d-%d %d:%d] ",
            &d->y, &d->mo, &d->d, &d->h, &d->mi);
        if (n_args < 5)
            break;

        //Round up date
        if (d->h == 23) {
            d->h = 0;
            d->mi = 0;
            date_incr_by_1(d);
        }

        //Get the rest of the line
        if (getline(&string, &len, f) < 0)
            break;
        
        ev->date = *d;
        day->day = *d;
        ev->day = add_day(day);
        if (sscanf(string, "Guard #%d begins shift\n", &id)) {
            ev->id = id;
            ev->day->id = id;
            ev->type = BEGIN;
            n_b++;
        } else if (strcmp(string, "falls asleep\n") == 0) {
            ev->type = ASLEEP;
            n_a++;
        } else {
            ev->type = WAKE;
            n_w++;
        }

        i++;
        if (i > MAX_EVENTS)
            error("Too much events");
    }
    free(string);
    fclose(f);

    n_events = i;
    printf("%d events found, %d begin, %d asleep, %d wake up for a total of %d days\n",
        i, n_b, n_a, n_w, n_days);


    /* For each day, fill  the asleep minutes bitmap */
    for (i = 5; i < n_days; i++) {
        day = &days[i];
        printf("Day is %d-%d\n", day->day.mo, day->day.d);
        for (j = 0; j < n_events; j++) {
            ev = &events[j];
            if (ev->day == day) {
                int mi = ev->date.mi;
                if (ev->type == ASLEEP) {
                    bitmap_asleep(&day->asleep_bmp, mi);
                    
                } else if (ev->type == WAKE) {
                    bitmap_awake(&day->asleep_bmp, mi);
                }
            }
        }
        print_bitmap(&day->asleep_bmp);
    }
        
    return 0;
}


