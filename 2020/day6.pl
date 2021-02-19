#!/usr/bin/perl
use strict;
use warnings;

#Encode each answer as a bit into a bitmap
##Group answer will be a simple |= or &=
sub parse_line
{
    my $x = 0;
    foreach (split('',$_[0])) {
        $x |= (1 << (ord($_) - ord('a')));
    }
    return $x;
}

#Found on the internet
sub popcount
{
    return unpack('%32b*', pack('Q', shift));
}

my $sum = 0;
my $ans = (1 << 27) -1;

while (<>) {
    if ($_ =~ /^$/) {
        $sum += popcount($ans);       
        $ans = (1 << 27) -1;
    } else {
        #$ans |= parse_line($_);
        $ans &= parse_line($_);
    }
}

print "$sum\n";
