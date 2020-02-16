use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Result};
use env_logger;
use sha1::{Sha1, Digest};
use rand::Rng;

#[macro_use]
extern crate log;

extern crate one_seek_one_read;

use one_seek_one_read::{Archive, Record};

const ARCHIVE: &'static str = "archive.data";
const INDEX: &'static str = "resource.index";

fn main( ) -> Result<()> {
    env_logger::init( );

    let records = read_records( INDEX )?;
    let mut archive = Archive::new( ARCHIVE, &records )?;

    let mut rng = rand::thread_rng( );

    debug!( "read some" );

    for _ in 1 .. 10 {
        let target = rng.gen_range( 0, records.len( ) );
        debug!( "target: {}", target );
        let record = &records[ target ];
        debug!( "record: {:?}", record );
        let data = archive.read( target )?;
        let mut hasher = Sha1::new( );
        hasher.input( data );
        let hash = hex::encode( hasher.result( ) );
        assert_eq!( hash, record.hash );
    }

    Ok( () )
}

fn create_mapping(records: &Vec<Record>) -> HashMap< String, usize > {
    let mut map = HashMap::new( );

    for ( index, record ) in records.iter( ).enumerate( ) {
        map.insert( record.path.to_string( ), index );
    }

    map
}

fn read_records(path: &str) -> Result< Vec<Record> > {
    let mut records = vec![];

    for result in BufReader::new( File::open( path )? ).lines( ) {
        let line = result?;
        let fields: Vec<&str> = line.split( '\t' ).collect( );

        let path = fields[0].to_string( );
        let size: usize = fields[1].parse( ).unwrap( );
        let hash = fields[2].to_string( );

        let record = Record {
            path,
            size,
            hash,
        };

        debug!( "{:?}", record );

        records.push( record );
    }

    Ok( records )
}

