#!/usr/bin/perl
use strict;
use warnings;

sub parse_bags_content
{
    shift;
    my $content = "";
    while(/(\d) (\w+ \w+) bags?(.|,)(.*)/) {
        $content = $content . $2;
        $_ = $4;
    }
    return $content;
}

my %bags = ();
while (<>) {
    if (/^(\w+ \w+) bags contain (.*)$/) {
        $bags{$1} = parse_bags_content($2)
    }
}

my @shiny_content = ("shiny gold");
my $work = 1;

WORK:while (1) {
    foreach my $keys (keys %bags) {
        #Already present in shiny_content
        if ($keys ~~ @shiny_content) {
            next;
        }
        foreach (@shiny_content) {
            if (index($bags{$keys}, $_) >= 0) {
                push @shiny_content, $keys;
                next WORK;
            }

        }
    }
    last;
}

#print @shiny_content;

my $ret = scalar @shiny_content;
#Remove the 'shiny_gold' itself
$ret--;

print "$ret\n";
