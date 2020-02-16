use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use sha1::{Sha1, Digest};

#[derive(Debug)]
struct Record {
    path: String,
    size: usize,
    hash: String,
}

const ARCHIVE: &'static str = "archive.data";

fn main( ) -> Result<(), io::Error> {
    let mut records: Vec<Record> = vec![ ];

    for result in BufReader::new( File::open( "resource.index" )? ).lines( ) {
        let line = result?;
        let fields: Vec<&str> = line.split( '\t' ).collect( );
        let size: usize = fields[1].parse( ).unwrap( );
        let record = Record {
            path: fields[0].to_string( ),
            size: size,
            hash: fields[2].to_string( ),
        };

        println!( "{:?}", record );

        records.push( record );
    }

    for record in &records {
        println!( "{:?}", record.path );
        let mut file = File::open( &record.path )?;
        let mut data = Vec::new( );
        file.read_to_end( &mut data )?;

        let mut hasher = Sha1::new( );
        hasher.input( &data );
        let result = hex::encode( hasher.result( ).to_vec( ) );

        assert_eq!( result, record.hash );
    }

    {
        let mut file = File::open( ARCHIVE )?;
        let mut offset: usize = 0;

        for record in &records {
            let mut buffer = vec![ 0; record.size ];

            let seek = SeekFrom::Start( offset as u64 );
            file.seek( seek ).unwrap( );
            let length = file.read( &mut buffer ).unwrap( );
            assert_eq!( length, record.size );

            offset = offset + length;

            let mut hasher = Sha1::new( );
            hasher.input( &buffer );
            let hash = hex::encode( hasher.result( ).to_vec( ) );

            assert_eq!( hash, record.hash );
        }
    }

    Ok( () )
}
