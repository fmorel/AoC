#!/usr/bin/perl
use strict;
use warnings;


my $width;
my $l = 0;
my @data = ();
push @data, {slope_h => 1, slope_v => 1, trees => 0, x => 0};
push @data, {slope_h => 3, slope_v => 1, trees => 0, x => 0};
push @data, {slope_h => 5, slope_v => 1, trees => 0, x => 0};
push @data, {slope_h => 7, slope_v => 1, trees => 0, x => 0};
push @data, {slope_h => 1, slope_v => 2, trees => 0, x => 0};
while (<>) {
    chomp;
    $width = length($_);
    my $line = $_;
    foreach (@data) {
        if ($l % $_->{slope_v} == 0) {
            $_->{trees}++ if (substr($line, $_->{x}, 1) eq "#");
            $_->{x} += $_->{slope_h};
            $_->{x} %= $width;
        }
    }
    $l++;
}

my $res = 1;
foreach (@data) {
    $res *= $_->{trees};
    print "$_->{trees}\n";
}
print "$res\n";
