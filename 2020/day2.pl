#!/usr/bin/perl
use strict;
use warnings;

my $valid = 0;
while (<>) {
    if ($_ =~ m/(\d+)-(\d+) (\w): (\w+)$/) {
        my $min = $1;
        my $max = $2;
        my $char = $3;
        my $pwd = $4;
        
        my $pwd1 = substr($pwd, $min-1, 1);
        my $pwd2 = substr($pwd, $max-1, 1);
        if (($pwd1 eq $char) != ($pwd2 eq $char)) {
            $valid++;
        }

        #my $count = length( $pwd =~ s/[^\Q$char\E]//rg );
        #if ($count <= $max && $count >= $min) {
        #    $valid++;
        #}
    }
}

print $valid, "\n";
