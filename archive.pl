#!/usr/bin/env perl -s
use 5.10.0;
use utf8;
use strict;
use warnings;
use open qw( :utf8 :std );
use Data::Dumper;
use Fcntl qw( O_RDONLY O_WRONLY O_CREAT );

our $out
    or die usage( );

sysopen my $OUT, $out, O_WRONLY | O_CREAT
    or die "Could not open $out for write: $!";

binmode $OUT
    or die "Could not set mode: $!";

my @records;

while ( <> ) {
    chomp( my $line = $_ );
    my( $path, $size, $hash ) = split m{\t}, $line;
warn "### path; $path";

    sysopen my $IN, $path, O_RDONLY
        or die "Could not open $path $!";

    binmode $IN
        or die "Could not set mode: $!";

    my $data;
    my $length = sysread $IN, $data, $size;
    die "Could not read $path $!"
        unless defined $length;

    close $IN
        or die "Could not close $path $!";

    my $written = syswrite $OUT, $data, $length;
    die "Could not write data to $out: $!"
        unless defined $written;

    push @records, [ $path, $size, $hash ];
}

close $OUT
    or die "Could not close $out: $!";

for my $record_ref ( @records ) {
    # same as input, skip this process for now.
}

exit;

sub usage {
    return <<END_USAGE;
usage: $0 -out=<archive file>
END_USAGE
}
