#!/usr/bin/env perl
use 5.10.0;
use utf8;
use strict;
use warnings;
use open qw( :utf8 :std );
use Data::Dumper;

while ( <> ) {
    chomp( my $line = $_ );
    my @fields = split m{\t}, $line;
    my( undef, $path ) = split m{\s}, $fields[2];
    my( $status, $size ) = split m{\s}, $fields[3];
    say join "\t", ( $path, $status, $size );
}

exit;

