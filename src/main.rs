use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use sha1::{Sha1, Digest};

#[derive( Debug )]
struct Record {
    path: String,
    size: usize,
    hash: String,
}

#[derive( Debug )]
struct Position {
    offset: u64,
    size: usize,
}

#[derive( Debug )]
struct Archive {
    indexes: Vec<Position>,
    file: File,
}

impl Archive {
    pub fn new(archive_path: &str,
               index_path: &str) -> Result<Self, io::Error> { // use os path instead
        let indexes = Self::read_indexes( index_path )?;

        let mut file = File::open( archive_path )?;

        Ok(
            Archive {
                indexes,
                file,
            }
        )
    }

    pub fn read(&mut self, at: usize) -> Result< Vec<u8>, io::Error > {
        let index = &self.indexes[ at ];
        let mut data = vec![ 0; 0_usize ];

        let seek = SeekFrom::Start( index.offset );
        self.file.seek( seek )?;

        self.file.read( &mut data )?;

        Ok( data )
    }

    fn read_indexes(path: &str) -> Result<Vec<Position>, io::Error> { // use os path instead
        let mut indexes = Vec::<Position>::new( );
        let mut offset = 0_u64;

        for result in BufReader::new( File::open( path )? ).lines( ) {
            let line = result?;
            let fields: Vec<&str> = line.split( '\t' ).collect( );

            let size: usize = fields[1].parse( ).unwrap( );

            let index = Position {
                offset,
                size,
            };

            println!( "{:?}", index );

            indexes.push( index );

            offset = offset + size as u64;
        }

        Ok( indexes )
    }

}

const ARCHIVE: &'static str = "archive.data";
const INDEX: &'static str = "resource.index";

fn main( ) -> Result<(), io::Error> {
    let archive = Archive::new( ARCHIVE, INDEX )?;

    Ok( () )
}

fn some( ) -> Result<(), io::Error> {
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

