#!/usr/bin/env perl
use 5.10.0;
use utf8;
use strict;
use warnings;
use open qw( :utf8 :std );
use Data::Dumper;

while ( <> ) {
    chomp( my $line = $_ );
    my( $path, $status, $size ) = split m{\t}, $line;
    my $tidy_path = tidy_up( $path );
    say join "\t", ( $tidy_path, $status, $size );
}

exit;

sub tidy_up {
    my $path = shift;
    $path =~ s{.*?StreamingAssets/[^/]+}{}i;
    $path =~ s{[?].*}{};

    return $path;
}
