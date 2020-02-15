#!/usr/bin/env perl
use 5.10.0;
use utf8;
use strict;
use warnings;
use open qw( :utf8 :std );
use Data::Dumper;
use HTTP::Status qw( HTTP_OK );

my %resource_size;

while ( <> ) {
    chomp( my $line = $_ );
    my( $path, $status, $size ) = split m{\t}, $line;

    next
        unless $status == HTTP_OK;

    $resource_size{ $path } = $size;
}

for my $path ( sort keys %resource_size ) {
    say join "\t", $path, $resource_size{ $path };
}

exit;

