use std::io::Result;
use env_logger;
use rand::Rng;

#[macro_use]
extern crate log;

extern crate one_seek_one_read;

use one_seek_one_read::archive::Archive;
use one_seek_one_read::index;
use one_seek_one_read::util;

const ARCHIVE: &'static str = "archive.data";
const INDEX: &'static str = "resource.index";

fn main( ) -> Result< () > {
    env_logger::init( );

    let records = index::read_records( INDEX )?;
    let mut archive = Archive::new( ARCHIVE, &records )?;

    let mut rng = rand::thread_rng( );

    debug!( "read some" );

    for _ in 1 .. 10 {
        let target = rng.gen_range( 0, records.len( ) );
        debug!( "target: {}", target );
        let record = &records[ target ];
        debug!( "record: {:?}", record );
        let data = archive.read( target )?;

        debug_assert_eq!( util::get_hash( &data ), record.hash );
    }

    Ok( () )
}
