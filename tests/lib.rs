#![feature( test )]

extern crate test;

extern crate one_seek_one_read;

use std::io::Result;

use test::Bencher;

use one_seek_one_read::archive::Archive;
use one_seek_one_read::normal::Normal;
use one_seek_one_read::index;
use one_seek_one_read::util;

#[bench]
fn run_archive_optimized( b: &mut Bencher ) -> Result< () > {
    let records = index::read_records( util::INDEX_PATH )?;
    let mut archive = Archive::new( util::ARCHIVE_PATH, &records, true )?;

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
fn run_archive_random_optimized( b: &mut Bencher ) -> Result< () > {
    let records = index::read_records( util::INDEX_PATH )?;
    let mut archive = Archive::new( util::ARCHIVE_PATH, &records, true )?;

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
fn run_archive( b: &mut Bencher ) -> Result< () > {
    let records = index::read_records( util::INDEX_PATH )?;
    let mut archive = Archive::new( util::ARCHIVE_PATH, &records, false )?;

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
    let records = index::read_records( util::INDEX_PATH )?;
    let mut archive = Archive::new( util::ARCHIVE_PATH, &records, false )?;

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
    let records = index::read_records( util::INDEX_PATH )?;
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
    let records = index::read_records( util::INDEX_PATH )?;
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
#[bench]
fn run_one_read( b: &mut Bencher ) -> Result<()> {
    let records = index::read_records( util::INDEX_PATH )?;
    let normal = Normal::new( &records, true );

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
fn run_one_read_random( b: &mut Bencher ) -> Result<()> {
    let records = index::read_records( util::INDEX_PATH )?;
    let normal = Normal::new( &records, true );

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
