#[macro_use]
extern crate log;

#[macro_use]
extern crate auto_enums;

extern crate one_seek_one_read;

use std::env;
use std::collections::VecDeque;
use std::fmt;
use std::io::Result;
use std::io::Write;
use std::str::FromStr;
use env_logger;

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

#[derive( Debug )]
enum IterationType {
    Sequential,
    Random,
}

impl fmt::Display for Mode {
    fn fmt( &self, format: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            Mode::Archive   => write!( format, "archive" ),
            Mode::Normal    => write!( format, "normal" ),
        }
    }
}

impl fmt::Display for IterationType {
    fn fmt( &self, format: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            IterationType::Sequential   => write!( format, "sequential" ),
            IterationType::Random       => write!( format, "random" ),
        }
    }
}

fn main( ) -> Result< () > {
    env_logger::init( );

    let ( which, iteration_count, iteration_type ) = parse_args( env::args( ).collect( ) );

    debug!( "which: {}", which );
    debug!( "iteration_count: {}", iteration_count );
    debug!( "iteration_type: {}", iteration_type );

    let records = index::read_records( INDEX )?;

    let iter = get_iterator( records.len( ), iteration_count as usize, iteration_type );

    match which {
        Mode::Archive   => run_archive( &records, iter ),
        Mode::Normal    => run_normal( &records, iter ),
    }
}

fn run_normal<I>( records: &Vec<index::Record>, iter: I ) -> Result< () >
    where I: IntoIterator<Item=usize>
{
    let normal = Normal::new( &records, true );

    for target in iter {
        debug!( "target: {}", target );
        let record = &records[ target ];
        debug!( "record: {:?}", record );
        let data = normal.read( target )?;

        debug_assert_eq!( util::get_hash( &data ), record.hash );
    }

    Ok( () )
}

fn run_archive<I>( records: &Vec<index::Record>, iter: I ) -> Result< () >
    where I: IntoIterator<Item=usize>
{
    let mut archive = Archive::new( ARCHIVE, &records )?;

    for target in iter {
        debug!( "target: {}", target );
        let record = &records[ target ];
        debug!( "record: {:?}", record );
        let data = archive.read( target )?;

        debug_assert_eq!( util::get_hash( &data ), record.hash );
    }

    Ok( () )
}

#[ auto_enum( Iterator ) ]
fn get_iterator( max_index: usize,
                 iteration_count: usize,
                 iteration_type: IterationType ) -> impl Iterator< Item = usize > {
    match iteration_type {
        IterationType::Sequential   => util::get_sequential_iterator( max_index ).take( iteration_count ),
        IterationType::Random       => util::get_random_iterator( max_index ).take( iteration_count ),
    }
}

fn parse_args( mut args: VecDeque<String> ) -> ( Mode, u64, IterationType ) {
    debug_assert!( args.len( ) > 0 );
    let me = args.pop_front( ).unwrap( );
    let message = format!( "usage: {} <reader mode> <iteration count> [iteration type]\n\
                            \treader mode: {}, {}\n\
                            \titeration type: {}, {}",
                           me,
                           Mode::Archive,
                           Mode::Normal,
                           IterationType::Sequential,
                           IterationType::Random);

    if args.len( ) < 2 {
        usage( &message );
    }

    let mode = match &args.pop_front( ).unwrap( )[..] {
        "archive"   => Some( Mode::Archive ),
        "normal"    => Some( Mode::Normal ),
        _           => { usage( &message ); None },
    }.unwrap( );
    let count = match u64::from_str( &args.pop_front( ).unwrap( ) ) {
        Ok( value ) => Some( value ),
        _           => { usage( &message ); None },
    }.unwrap( );

    let iteration_type = match args.len( ) > 0 {
        true    => match &( args.pop_front( ).unwrap( ) )[..] {
            "sequential"    => IterationType::Sequential,
            "random"        => IterationType::Random,
            _               => IterationType::Random,
        },
        false   => IterationType::Random,
    };

    ( mode, count, iteration_type )
}

fn usage( message: &String ) {
    writeln!( std::io::stderr( ), "{}", message ).unwrap( );
    std::process::abort( );
}
