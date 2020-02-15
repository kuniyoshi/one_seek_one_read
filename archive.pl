#!/usr/bin/env perl -s
use 5.10.0;
use utf8;
use strict;
use warnings;
use open qw( :utf8 :std );
use Data::Dumper;
use Path::Class qw( dir );
use Fcntl qw( O_RDONLY O_WRONLY O_CREAT );

our $in
    or die usage( );

our $out
    or die usage( );

my $root = dir( $in );

die "No resource directory found"
    unless -e $root;

sysopen my $OUT, $out, O_WRONLY | O_CREAT
    or die "Could not open $out for write: $!";

binmode $OUT
    or die "Could not set mode: $!";

my @records;

while ( <> ) {
    chomp( my $line = $_ );
    my( $path, $size ) = split m{\t}, $line;
    my $file = $root->file( $path );
warn "### file; $file";

    sysopen my $IN, $file, O_RDONLY
        or die "Could not open $file: $!";

    binmode $IN
        or die "Could not set mode: $!";

    my $data;
    my $length = sysread $IN, $data, $size;
    die "Could not read $file: $!"
        unless defined $length;

    close $IN
        or die "Could not close $file: $!";

    my $written = syswrite $OUT, $data, $length;
    die "Could not write data to $out: $!"
        unless defined $written;

    push @records, [ $file, $size ];
}

close $OUT
    or die "Could not close $out: $!";

for my $record_ref ( @records ) {
    # same as input, skip this process for now.
}

exit;

sub usage {
    return <<END_USAGE;
usage: $0 -in=<resource directory> -out=<archive file>
END_USAGE
}
