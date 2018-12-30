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

typedef struct {
    int id;
    int total_sleep;
    int total_shifts;
} Guard;

typedef struct {
    Date day;
    Guard *guard;
    uint64_t asleep_bmp;
} Day;

typedef enum {
    BEGIN,
    ASLEEP,
    WAKE
} EventType;


typedef struct {
    Day *day;
    Date date;
    Guard *guard;
    EventType type;
} Event;

#define MAX_EVENTS 1040
Event events[MAX_EVENTS];

int n_days = 0;
Day days[365];

#define MAX_GUARDS 25
int n_guards = 0;
Guard guards[MAX_GUARDS];

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

int days_equal(Day *d1, Date *date)
{
    return (d1->day.mo == date->mo && d1->day.d == date->d);
}

Day *add_day(Date *date)
{
    int i;
    for (i = 0; i < n_days; i++) {
        if (days_equal(&days[i], date))
            return &days[i];
    }
    days[n_days].day = *date;
    days[n_days].asleep_bmp = 0;

    return &days[n_days++];
}

void print_bitmap(uint64_t bmp) {
    int i;
    for (i=0; i < 60; i++) {
        if (bmp & (1LL<<i))
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

Guard *add_guard(int id) {
    int i;
    for (i = 0; i < n_guards; i++) {
        if (guards[i].id == id) {
            guards[i].total_shifts++;
            return &guards[i];
        }
    }
    if (n_guards == MAX_GUARDS)
        error("Too many guards !\n");
    guards[n_guards].id = id;
    guards[n_guards].total_sleep = 0;
    guards[n_guards].total_shifts = 1;

    return &guards[n_guards++];
}

int main(void)
{

    FILE *f = fopen("day_4.input", "r");

    Date d_s, *d = &d_s;
    Event *ev;
    Day *day;
    Guard *guard;

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
        ev->day = add_day(d);
        if (sscanf(string, "Guard #%d begins shift\n", &id)) {
            guard = add_guard(id);
            ev->guard = guard;
            ev->day->guard = guard;
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
    printf("%d events found, %d begin, %d asleep, %d wake up for a total of %d days and %d guards\n",
        n_events, n_b, n_a, n_w, n_days, n_guards);


    /* Complete asleep_bmp for each days by parsing the events */
    for (i = 0; i < n_events; i++) {
        ev = &events[i];
        day = ev->day;
        int mi = ev->date.mi;
        if (ev->type == ASLEEP) {
            bitmap_asleep(&day->asleep_bmp, mi);
        } else if (ev->type == WAKE) {
            bitmap_awake(&day->asleep_bmp, mi);
        }
    }

    /* Compute total sleep for each day */
    for (i = 0; i < n_days; i++) {
        days[i].guard->total_sleep += __builtin_popcountll(days[i].asleep_bmp);
    }

    /* Get guard with maximum sleep  (or sleep ratio ? Neither answer seem to work anyway ...) */
    float  best_sleep_ratio = 0, sleep_ratio;
    Guard *best;
    for (i = 0; i < n_guards; i++) {
        sleep_ratio = (float)guards[i].total_sleep / 1 ;/*guards[i].total_shifts;*/
        printf("Guard %d has %d total sleep (%d shifts), ratio %.2f\n", guards[i].id, guards[i].total_sleep, guards[i].total_shifts, sleep_ratio/60);
        if (sleep_ratio > best_sleep_ratio) {
            best_sleep_ratio = sleep_ratio;
            best = &guards[i];
        }
    }
    printf("Guard with most sleep is %d (ratio = %.2f)\n", best->id, best_sleep_ratio/60);

    /* Find best minute for our best guard */
    int minutes[60];
    memset(minutes, 0, sizeof(minutes));

    for (i = 0; i < n_days; i++) {
        if (days[i].guard->id != best->id)
            continue;
        printf("%02d-%02d : ", days[i].day.mo, days[i].day.d);
        print_bitmap(days[i].asleep_bmp);
        for (j = 0; j < 60 ; j++) {
            if (days[i].asleep_bmp & (1ULL<<j))
                minutes[j]++;
        }
    }

    int total_sleep = 0;
    int best_minute = 0;
    for (i = 0 ; i < 60; i++) {
        if (minutes[i] > total_sleep) {
            total_sleep = minutes[i];
            best_minute = i;
        }
    }

    printf("Best minute is %d (%d occurrence), answer is %d\n", best_minute, total_sleep, best->id*best_minute);
    return 0;
}


