#!/usr/bin/env perl
use 5.10.0;
use utf8;
use strict;
use warnings;
use open qw( :utf8 :std );
use Data::Dumper;
use Readonly qw( Readonly );
use Test::More;
use Digest::SHA1 qw( sha1_hex );
use Path::Class qw( file );
use Fcntl qw( O_RDONLY );

Readonly my $ARCHIVE_FILENAME   => "data/archive.data";
Readonly my $INDEX_FILENAME     => "data/resource.index";

my $file = file( $INDEX_FILENAME );
my @lines = $file->slurp( chomp => 1 );

plan tests => 2 * @lines;

for my $line ( @lines ) {
    my( $path, $size, $hash ) = split m{\t}, $line;

    open my $FH, "<:raw", $path
        or die "Could not open $path: $!";

    binmode $FH
        or die "Could not set binmode to $path handler: $!";

    my $data;
    my $length = sysread $FH, $data, $size;
    die "Could not read $path: $!"
        unless defined $length;

    my $got = sha1_hex( $data );
    is( $got, $hash );

    close $FH
        or die "Could not close $path: $!";
}

sysopen my $ARCHIVE, $ARCHIVE_FILENAME, O_RDONLY
    or die "Could not open: $ARCHIVE_FILENAME: $!";

binmode $ARCHIVE
    or die "Could not set binmode to $ARCHIVE_FILENAME: $!";

my $offset = 0;

for my $line ( @lines ) {
    my( $_path, $size, $hash ) = split m{\t}, $line;

    my $data;
    my $length = sysread $ARCHIVE, $data, $size, $offset;
    die "Could not read from $ARCHIVE_FILENAME: $!"
        unless defined $length;

    my $got = sha1_hex( $data );
    is( $got, $hash );
}

close $ARCHIVE
    or die "Could not close $ARCHIVE_FILENAME: $!";

exit;

