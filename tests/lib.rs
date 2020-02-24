#![feature( test )]

extern crate test;

extern crate one_seek_one_read;

use std::io::Result;

use test::Bencher;

use one_seek_one_read::archive::Archive;
use one_seek_one_read::normal::Normal;
use one_seek_one_read::index;
use one_seek_one_read::util;

const ARCHIVE: &'static str = "archive.data";
const INDEX: &'static str = "resource.index";

#[bench]
fn run_archive( b: &mut Bencher ) -> Result< () > {
    let records = index::read_records( INDEX )?;
    let mut archive = Archive::new( ARCHIVE, &records )?;

    let mut iter = util::get_sequential_iterator( records.len( ) );

    b.iter( | | -> Result< () >
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
fn run_archive_random( b: &mut Bencher ) -> Result< () > {
    let records = index::read_records( INDEX )?;
    let mut archive = Archive::new( ARCHIVE, &records )?;

    let mut iter = util::get_random_iterator( records.len( ) );

    b.iter( | | -> Result< () >
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

    let mut iter = util::get_sequential_iterator( records.len( ) );

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

    let mut iter = util::get_random_iterator( records.len( ) );

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
