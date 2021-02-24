#!/usr/bin/perl

use strict;
use warnings;

my $preamble_size = 25;
my @last = ();
my $n = 0;
my $k = 0;
my $s = 0;
my @file = <>;

sub check_sum
{
    my $n = shift;
    my @plast = @last;
    while (@plast) {
        my $p = shift @plast;
        if ($p > $n) {
            next;
        }
        foreach my $p2 (@plast) {
            if (($p != $p2) && ($p + $p2) == $n) {
                return 1;
            }
        }
    }
    return 0;
}
        
#Step 1 : look for number where check_sum failed
for (@file) {
    $n = int($_);
    if (scalar @last < $preamble_size) {
        push @last, $n;
    } else {
        #print "last: @last, to check $n\n";
        if (!check_sum($n)) {
            last;
        }
        push @last, $n;
        shift @last;
    }
}

print "Weakness found:\n$n\n";

#Step 2 : look for adjacent sum equaling $n
@last = ();
$s = 0;
for (@file) {
    $k = int($_);
    print "sum: $s, next: $k, to find $n\n";
    if ($s < $n) {
        push @last, $k;
        $s += $k;
    }
    while ($s > $n) {
        $s -= shift @last;
    }
    if ($s == $n) {
        last;
    }
}

@last = sort {$a <=> $b} @last;
my $res = $last[0] + $last[$#last];
print "Sum found:\n$res\n@last\n";
