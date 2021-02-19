#!/usr/bin/perl
use strict;
use warnings;

my $max_id = 0;
my @seats = ();
while (<>) {
    my $l = $_;
    my $k = 9;
    my $id = 0;
    foreach (split('', $l)) {
        $id += (1 << $k) if ($_ =~ /B|R/);
        $k--;
    }
    #print "$id\n";
    push @seats, $id;
    if ($id > $max_id) {
        $max_id = $id;
    }
}

@seats = sort {$a <=> $b} @seats;

#Check for a hole in @seats
my $last_id = shift @seats;
foreach (@seats) {
    print "$last_id ? $_\n" if ($_ != ($last_id + 1));
    $last_id = $_;
}
print "MAX $max_id\n";
