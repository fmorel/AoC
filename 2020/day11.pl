#!/usr/bin/perl

use strict;
use warnings;

my @seats = ();
my $width;
my $height;

sub print_seat
{
    my $v = shift;
    print '.' if $v < 0;
    print '#' if $v > 0;
    print 'L' if $v == 0;
}

sub print_seats {
    for (my $row = 0; $row < $height; $row++) {
        for (my $col = 0; $col < $width; $col++) {
            print_seat $seats[$row * $width + $col];
        }
        print "\n";
    }
    print "\n";
}

sub get_seat2
{
    my $v = shift; 
    return 0 if $v < 0;
    return $v;
}

sub get_seat
{
    my $row = shift;
    my $col = shift;
    return 0 if ($row < 0 || $row >= $height);
    return 0 if ($col < 0 || $col >= $width);
    
    return get_seat2($seats[$row * $width + $col]);
}

sub get_adjacent
{
    my $row = shift;
    my $col = shift;
    my $sum = get_seat($row-1, $col-1) + get_seat($row-1, $col) + get_seat($row-1, $col+1);
    $sum += get_seat($row, $col-1) + get_seat($row, $col+1);
    $sum += get_seat($row+1, $col-1) + get_seat($row+1, $col) + get_seat($row+1, $col+1);
    return $sum;
}

sub step
{
    my @new_seats;
    my $toggle = 0;
    for (my $row = 0; $row < $height; $row++) {
        for (my $col = 0; $col < $width; $col++) {
            if ($seats[$row * $width + $col] == 0 && get_adjacent($row, $col) == 0) {
                $new_seats[$row * $width + $col] = 1;
                $toggle = 1;
            } elsif ($seats[$row * $width + $col] == 1 && get_adjacent($row, $col) >= 4) {
                $new_seats[$row * $width + $col] = 0;
                $toggle = 1;
            } else {
                $new_seats[$row * $width + $col] = $seats[$row * $width + $col];
            }
        }
    }
    @seats = @new_seats;
    return $toggle;
}

sub convert
{
    my $c = shift;
    return 0 if ($c =~ /L/);
    return -1;
}

    
while (<>) {
    chomp;
    my @row = split //;
    $width = scalar @row;
    @row = map { convert($_); } @row;
    push @seats, @row;
    $height++;
}

print_seats();

my $loop = 0;
while (step()) {
    $loop++;
    print "Loop $loop\n";
#    print_seats();
}

my $occupied = 0;
map {$occupied += get_seat2 $_;} @seats;

print "Loop $loop, Occupied $occupied\n";

