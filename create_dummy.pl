#!/usr/bin/env perl -s
use 5.10.0;
use utf8;
use strict;
use warnings;
use open qw( :utf8 :std );
use Data::Dumper;
use Path::Class qw( dir );
use Fcntl qw( O_WRONLY O_RDONLY O_CREAT );

our $out_dir
    or die usage( );

my $root = dir( $out_dir );
$root->mkpath;

sysopen my $RANDOM, "/dev/random", O_RDONLY
    or die $!;

binmode $RANDOM
    or die "Could not set mode: $!";

while ( <> ) {
    chomp( my $line = $_ );
    my( $path, $size ) = split m{\t}, $line;
    my $file = $root->file( $path );
warn "### path: $path";
warn "--- size: $size";
    $file->dir->mkpath;

    my $length = sysread $RANDOM, my $data, $size;

    die "Could not read random: $!"
        unless defined $length;

warn "--- read: $length";

    sysopen my $OUT, $file, O_WRONLY | O_CREAT
        or die "Could not open $file: $!";

    binmode $OUT
        or die "Could not set mode to $file: $!";

    my $block_size = 8192;
    my $offset = 0;

    while ( $length ) {
warn "... writing.";
        my $written = syswrite $OUT, $data, $block_size, $offset;
warn "... wrote";

        die "Could not write to $file: $!"
            unless defined $written;

        $offset = $offset + $written;
        $length = $length - $written;
    }

    close $OUT
        or die "Could not close $file: $!";
warn "close";
}

close $RANDOM
    or die $!;

exit;

sub usage {
    return <<END_USAGE;
usage: $0 -out_dir=<output dir name>
END_USAGE
}
