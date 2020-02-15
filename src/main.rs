use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug)]
struct Record {
    path: String,
    size: i32,
    hash: String,
}

const ARCHIVE: &'static str = "archive.data";

fn main( ) -> Result<(), io::Error> {
    let mut records: Vec<Record> = vec![];

    for result in BufReader::new( File::open( "resource.index" )? ).lines( ) {
        let line = result?;
        let fields: Vec<&str> = line.split( '\t' ).collect( );
        let size: i32 = fields[1].parse( ).unwrap();
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
        let mut data = Vec::new();
        file.read_to_end( &mut data )?;
    }

    Ok( () )
}
