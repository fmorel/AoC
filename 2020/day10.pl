#!/usr/bin/perl

use strict;
use warnings;

my @file = <>;
@file = sort {$a <=> $b} @file;

sub get_n_arrangement
{
    return 1 if scalar @_ == 1;
    my $first = shift;
    return 0 if ($_[0] - $first > 3);
    return 1 if scalar @_ == 1;
    
    #Either we include first adapter, either we don't
    my $with = get_n_arrangement(@_);
    shift;
    unshift @_, $first;
    my $without = get_n_arrangement(@_);
    return $with + $without;
}

my @jolt3_indices = ();
my $idx = 1;    #Start with 1 since we will add 0 in head of array afterwards
my $last = 0;
my $jolt1 = 0;
my $jolt3 = 0;
for (@file) {
    if ($_ - $last == 1) {
        $jolt1++;
    }
    if ($_ - $last == 3) {
        $jolt3++;
        push @jolt3_indices, $idx;
    }
    $last = $_;
    $idx++;
}

#0 and last adapter as array boundary
$last = $file[$#file] + 3;
unshift @file, 0;
push @file, $last;

#Adapter separated by jolt3 are always in all the arrangements.
#We need to multiply the gt_n_aranagement of every subsection of adapters
#delimitied byjolt3 separation
my $arrang = 1;
$idx = 0;
for (@jolt3_indices) {
    $arrang *= get_n_arrangement(@file[$idx .. ($_ - 1)]);
    $idx = $_;
}
$arrang *= get_n_arrangement(@file[$idx .. $#file]);
print "$arrang\n";
