#!/usr/bin/perl

use strict;
use warnings;

my @code = <>;
our $pc = 0;
our $acc = 0;

sub parse_arg
{
    $_ = shift;
    if (/\+(\d+)/) {
        return int($1);
    }
    if (/-(\d+)/) {
        return -int($1);
    }
    return 0;
}

sub exec_inst
{
    $_ = shift;
    if (/nop/) {
        $pc += 1;
    } elsif (/acc (.*)/) {
        $acc += parse_arg($1);
        $pc += 1;
    } elsif (/jmp (.*)/) {
        $pc += parse_arg($1);
    }
}

sub switch_inst
{
    $_ = shift;
    if (/nop/) {
        $_ =~ s/nop/jmp/;
    } elsif (/jmp/) {
        $_ =~ s/jmp/nop/;
    }
    return $_;
}

sub run_program
{
    my $ret = -1;
    local $pc = 0;
    local $acc = 0;
    my @visited = ();

    while(1) {
        #Infinite loop
        if ($pc ~~ @visited) {
            last;
        }
        #Normal termination
        if ($pc == $#code) {
            $ret = $acc;
            last;
        }
        push @visited, $pc;
        exec_inst($code[$pc]);
    }
    return $ret;
}

my $ret = -1;
for my $i (0 .. $#code) {
    my $inst = $code[$i];
    $code[$i] = switch_inst($inst);
    $ret = run_program();
    if ($ret >= 0) {
        last;
    }
    $code[$i] = $inst;
}

printf("acc after debug $ret\n");
