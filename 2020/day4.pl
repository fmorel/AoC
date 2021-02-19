#!/usr/bin/perl
use strict;
use warnings;

my %entry = ();

my @ref = ('byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid', 'cid');
@ref = sort @ref;
my @ref2 = ('byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid');
@ref2 = sort @ref2;




my $valid = 0;
my $n = 0;

sub check_year
{
    return 0 if ($_[0] !~ /^(\d{4})$/);
    my $y = int($_[0]);
    return 0 if ($y < $_[1] || $y > $_[2]);
    return 1;
}

sub check_hgt
{
    if ($_[0] =~ /^(\d{3})cm$/) {
        return 0 if ($1 < 150 || $1 > 193);
        return 1;
    } elsif ($_[0] =~ /^(\d\d)in$/) {
        return 0 if ($1 < 59 || $1 > 76);
        return 1;
    } else {
        return 0;
    }
}

sub check_hcl
{
    return ($_[0] =~ /^#[0-9a-f]{6}$/);
}

sub check_ecl
{
    return ($_[0] =~ /^(amb|blu|brn|gry|grn|hzl|oth)$/);
}

sub check_pid
{
    return ($_[0] =~ /^\d{9}$/);
}

sub check_validity
{
    my %e = %{$_[0]};
    return 0 if (!check_year($e{byr}, 1920, 2002));
    return 0 if (!check_year($e{iyr}, 2010, 2020));
    return 0 if (!check_year($e{eyr}, 2020, 2030));
    return 0 if (!check_hgt($e{hgt}));
    return 0 if (!check_hcl($e{hcl}));
    return 0 if (!check_ecl($e{ecl}));
    return 0 if (!check_pid($e{pid}));
    return 1;
}

while (<>) {
    if ($_ =~ /^$/) {
        #New entry
        my @k = keys %entry;
        @k = sort @k;
        if (@k ~~ @ref || @k ~~ @ref2) {
            $valid++ if (check_validity(\%entry));
        } else {
            #print "$n : No match : @{[ %entry ]}\n";
        }
        %entry = ();
        $n++;
    } else {
        my $l = $_;
        while ($l =~ /\s*(\w\w\w):([a-z0-9#]*)(.*)/) {
            $entry{$1} = $2;
            $l = $3;
        }
    }
}

print "$valid\n";
