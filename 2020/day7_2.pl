#!/usr/bin/perl
use strict;
use warnings;

sub parse_bags_content
{
    shift;
    my @content = ();
    while(/(\d) (\w+ \w+) bags?(.|,)(.*)/) {
        push @content, $1;
        push @content, $2;
        $_ = $4;
    }
    return \@content;
}

my %bags = ();
while (<>) {
    if (/^(\w+ \w+) bags contain (.*)$/) {
        $bags{$1} = parse_bags_content($2)
    }
}

my %bags_count = ();
sub get_inner_bags
{
    my $b = shift;
    if (exists($bags_count{$b})) {
        return $bags_count{$b};
    }
    my @content = @{$bags{$b}};
    my $total = 0;
    
    if (scalar @content == 0) {
        goto RET;
    }
    while (my ($n, $bag) = splice(@content, 0, 2)) {
        $total += $n * (1 + get_inner_bags($bag));
    }
RET:
    $bags_count{$b} = $total;
    return $total;
}

my $total_bags = get_inner_bags("shiny gold");
print("Total bags $total_bags\n");

