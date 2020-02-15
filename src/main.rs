use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug)]
struct Record {
    name: String,
    size: i32,
}

const ROOT_DIR: &'static str = "resource.d";

fn main( ) -> Result<(), io::Error> {
    let mut records: Vec<Record> = vec![];

    for result in BufReader::new( File::open( "resource.list" )? ).lines( ) {
        let line = result?;
        let fields: Vec<&str> = line.split( '\t' ).collect( );
        let size: i32 = fields[1].parse( ).unwrap();
        let record = Record {
            name: fields[0].to_string( ),
            size: size,
        };

//        println!( "{:?}", record );

        records.push( record );
    }

    let root = Path::new( ROOT_DIR );

    for record in &records {
        let path = root.join( &record.name );
        println!( "{:?}", path );
        let mut file = File::open( path )?;
        let mut data = Vec::new();
        file.read_to_end( &mut data )?;
    }

    Ok( () )
}
