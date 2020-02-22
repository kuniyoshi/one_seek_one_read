#[macro_use]
extern crate log;

extern crate one_seek_one_read;

use std::env;
use std::fmt;
use std::io::Result;
use std::io::Write;
use std::str::FromStr;
use env_logger;
use rand::Rng;

use one_seek_one_read::archive::Archive;
use one_seek_one_read::normal::Normal;
use one_seek_one_read::index;
use one_seek_one_read::util;

const ARCHIVE: &'static str = "archive.data";
const INDEX: &'static str = "resource.index";

#[derive( Debug )]
enum Mode {
    Archive,
    Normal,
}

impl fmt::Display for Mode {
    fn fmt( &self, format: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            Mode::Archive   => write!( format, "archive" ),
            Mode::Normal    => write!( format, "normal" ),
        }
    }
}

fn main( ) -> Result< () > {
    env_logger::init( );

    let ( which, iteration_count ) = get_args( &( env::args( ).collect( ) ) );

    debug!( "which: {}", which );
    debug!( "iteration_count: {}", iteration_count );

    match which {
        Mode::Archive   => run_archive( iteration_count ),
        Mode::Normal    => run_normal( iteration_count ),
    }
}

fn run_normal( count: u64 ) -> Result< () > {
    let records = index::read_records( INDEX )?;
    let normal = Normal::new( &records );
    let mut rng = rand::thread_rng( );

    for _ in 0 .. count {
        let target = rng.gen_range( 0, records.len( ) );
        debug!( "target: {}", target );
        let record = &records[ target ];
        debug!( "record: {:?}", record );
        let data = normal.read( target )?;

        assert_eq!( util::get_hash( &data ), record.hash );
    }

    Ok( () )
}

fn run_archive( count: u64 ) -> Result< () > {
    let records = index::read_records( INDEX )?;
    let mut archive = Archive::new( ARCHIVE, &records )?;
    let mut rng = rand::thread_rng( );

    for _ in 0 .. count {
        let target = rng.gen_range( 0, records.len( ) );
        debug!( "target: {}", target );
        let record = &records[ target ];
        debug!( "record: {:?}", record );
        let data = archive.read( target )?;

        assert_eq!( util::get_hash( &data ), record.hash );
    }

    Ok( () )
}

fn get_args( args: &Vec<String> ) -> ( Mode, u64 ) {
    assert!( args.len( ) > 0 );
    let me = &args[0];
    let message = format!( "usage: {} <{} | {}> <iteration count>", me, Mode::Archive, Mode::Normal );

    if args.len( ) != 3 {
        usage( &message );
    }

    let mode = match &args[1][..] {
        "archive"   => Some( Mode::Archive ),
        "normal"    => Some( Mode::Normal ),
        _           => { usage( &message ); None },
    }.unwrap( );
    let count = match u64::from_str( &args[2][..] ) {
        Ok( value ) => Some( value ),
        _           => { usage( &message ); None },
    }.unwrap( );

    ( mode, count )
}

fn usage( message: &String ) {
    writeln!( std::io::stderr( ), "{}", message ).unwrap( );
    std::process::abort( );
}
