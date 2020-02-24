#![feature( test )]

extern crate test;

#[macro_use]
extern crate log;

extern crate one_seek_one_read;

use std::fmt;
use std::io::Result;
use std::iter::Cycle;
use std::ops::Range;

use test::Bencher;
use env_logger;
use rand::{Rng, SeedableRng, rngs::StdRng};

use one_seek_one_read::archive::Archive;
use one_seek_one_read::normal::Normal;
use one_seek_one_read::index;
use one_seek_one_read::util;

const ARCHIVE: &'static str = "archive.data";
const INDEX: &'static str = "resource.index";

#[bench]
fn run_archive( b: &mut Bencher ) -> Result<()> {
    let records = index::read_records( INDEX )?;
    let mut archive = Archive::new( ARCHIVE, &records )?;

    let mut iter = get_sequential_iterator( records.len( ) );

    b.iter( | | -> Result<()>
            {
                let target = iter.next( ).unwrap( );
                let record = &records[ target ];
                let data = archive.read( target )?;

                debug_assert_eq!( util::get_hash( &data ), record.hash );

                Ok( () )
            } );

    Ok( () )
}

#[bench]
fn run_archive_random( b: &mut Bencher ) -> Result<()> {
    let records = index::read_records( INDEX )?;
    let mut archive = Archive::new( ARCHIVE, &records )?;

    let mut iter = get_random_iterator( records.len( ) );

    b.iter( | | -> Result<()>
            {
                let target = iter.next( ).unwrap( );
                let record = &records[ target ];
                let data = archive.read( target )?;

                debug_assert_eq!( util::get_hash( &data ), record.hash );

                Ok( () )
            } );

    Ok( () )
}

#[bench]
fn run_normal( b: &mut Bencher ) -> Result<()> {
    let records = index::read_records( INDEX )?;
    let normal = Normal::new( &records, false );

    let mut iter = get_sequential_iterator( records.len( ) );

    b.iter( | | -> Result<()>
            {
                let target = iter.next( ).unwrap( );
                let record = &records[ target ];
                let data = normal.read( target )?;

                debug_assert_eq!( util::get_hash( &data ), record.hash );

                Ok( () )
            } );

    Ok( () )
}

#[bench]
fn run_normal_random( b: &mut Bencher ) -> Result<()> {
    let records = index::read_records( INDEX )?;
    let normal = Normal::new( &records, false );

    let mut iter = get_random_iterator( records.len( ) );

    b.iter( | | -> Result<()>
            {
                let target = iter.next( ).unwrap( );
                let record = &records[ target ];
                let data = normal.read( target )?;

                debug_assert_eq!( util::get_hash( &data ), record.hash );

                Ok( () )
            } );

    Ok( () )
}

fn get_random_iterator( max_index: usize ) -> impl Iterator<Item=usize> + 'static {
    let seed = [
        9, 167, 249, 169, 8,
        33, 178, 6, 107, 190,
        90, 75, 229, 24, 59,
        168, 153, 217, 43, 190,
        139, 182, 222, 137, 75,
        45, 239, 225, 64, 57,
        143, 91
    ];
    let mut rng: StdRng = SeedableRng::from_seed( seed );

    std::iter::repeat( max_index ).map( move | i | rng.gen_range( 0, i ) )
}

fn get_sequential_iterator( max_index: usize ) -> Cycle< Range<usize> >
{
    let iter = ( 0_usize .. max_index ).cycle( ).into_iter( );

    iter
}
