use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive( Debug )]
pub struct Record {
    pub path: String,
    pub size: usize,
    pub hash: String,
}

#[derive( Debug )]
pub struct Index {
    pub offset: u64,
    pub size: usize,
}

pub fn create_mapping( records: &Vec<Record> ) -> HashMap< String, usize > {
    let mut map = HashMap::new( );

    for ( index, record ) in records.iter( ).enumerate( ) {
        map.insert( record.path.to_string( ), index );
    }

    map
}

pub fn read_records( path: &str ) -> Result< Vec<Record> > {
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
