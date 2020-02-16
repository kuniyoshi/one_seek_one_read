use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Result};
use sha1::{Sha1, Digest};

#[derive( Debug )]
struct Record {
    path: String,
    size: usize,
    hash: String,
}

#[derive( Debug )]
struct Index {
    offset: u64,
    size: usize,
}

#[derive( Debug )]
struct Archive {
    indexes: Vec<Index>,
    file: File,
}

impl Archive {
    pub fn new(archive_path: &str,
               records: &Vec< Record >) -> Result< Self > { // use os path instead
        let indexes = Self::indexes_from_records( records );

        let mut file = File::open( archive_path )?;

        Ok(
            Archive {
                indexes,
                file,
            }
        )
    }

    pub fn read(&mut self, at: usize) -> Result< Vec<u8> > {
        let index = &self.indexes[ at ];
        let mut data = vec![ 0; 0_usize ];

        let seek = SeekFrom::Start( index.offset );
        self.file.seek( seek )?;

        self.file.read( &mut data )?;

        Ok( data )
    }

    fn indexes_from_records(records: &Vec< Record >) -> Vec<Index> { // use os path instead
        let mut indexes = vec![ ];
        let mut offset = 0_u64;

        for record in records {
            let size = record.size;

            let index = Index {
                offset,
                size,
            };

            println!( "{:?}", index );

            indexes.push( index );

            offset = offset + size as u64;
        }

        indexes
    }

}

const ARCHIVE: &'static str = "archive.data";
const INDEX: &'static str = "resource.index";

fn main( ) -> Result<()> {
    let records = read_records( INDEX )?;
    let archive = Archive::new( ARCHIVE, &records )?;

    Ok( () )
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

        println!( "{:?}", record );

        records.push( record );
    }

    Ok( records )
}

fn some( ) -> Result< () > {
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

