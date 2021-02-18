#!/usr/bin/perl

my @array = <>;
foreach (@array) {
    my $cur = $_;
    my $to_find = 2020 - $cur;
    foreach (@array) {
        next if ($_ >= $to_find);
        my $cur2 = $_;
        my $to_find2 = $to_find - $cur2;
        foreach (@array) {
            if ($_ == $to_find2) {
                print $to_find2*$cur*$cur2, "\n";
                exit 0;
            }
        }
    }
}


    
