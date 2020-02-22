// #![feature( test )]
#![feature(test)]

extern crate test;

#[macro_use]
extern crate log;

extern crate one_seek_one_read;

use std::fmt;
use std::io::Result;
use test::Bencher;
use env_logger;
use rand::{Rng, SeedableRng, rngs::StdRng};

use one_seek_one_read::archive::Archive;
use one_seek_one_read::normal::Normal;
use one_seek_one_read::index;
use one_seek_one_read::util;

const ARCHIVE: &'static str = "archive.data";
const INDEX: &'static str = "resource.index";

#[test]
fn it_works( ) {
}

#[bench]
fn run_normal( b: &mut Bencher ) -> Result<()> {
    let records = index::read_records( INDEX )?;
    let normal = Normal::new( &records, true );
    let seed = [9, 167, 249, 169, 8, 33, 178, 6, 107, 190, 90, 75, 229, 24, 59, 168, 153, 217, 43, 190, 139, 182, 222, 137, 75, 45, 239, 225, 64, 57, 143, 91];
    let mut rng: StdRng = SeedableRng::from_seed( seed );

    let size = records.len( );
    let mut iter = ( 0_usize .. size ).cycle( );

    b.iter( | | -> Result<()>
            {
//                    let target = rng.gen_range( 0, size );
                let target = iter.next( ).unwrap( );
                debug!( "target: {}", target );
                let record = &records[ target ];
                debug!( "record: {:?}", record );
                let data = normal.read( target )?;

                debug_assert_eq!( util::get_hash( &data ), record.hash );

                Ok( () )
            } );

    Ok( () )
}

#[bench]
fn run_archive( b: &mut Bencher ) -> Result<()> {
    let records = index::read_records( INDEX )?;
    let mut archive = Archive::new( ARCHIVE, &records )?;
    let seed = [9, 167, 249, 169, 8, 33, 178, 6, 107, 190, 90, 75, 229, 24, 59, 168, 153, 217, 43, 190, 139, 182, 222, 137, 75, 45, 239, 225, 64, 57, 143, 91];
    let mut rng: StdRng = SeedableRng::from_seed( seed );

    let size = records.len( );
    let mut iter = ( 0_usize .. size ).cycle( );

    b.iter( | | -> Result<()>
            {
//                    let target = rng.gen_range( 0, size );
                let target = iter.next( ).unwrap( );
                debug!( "target: {}", target );
                let record = &records[ target ];
                debug!( "record: {:?}", record );
                let data = archive.read( target )?;

                debug_assert_eq!( util::get_hash( &data ), record.hash );

                Ok( () )
            } );

    Ok( () )
}
