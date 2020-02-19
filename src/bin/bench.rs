#![feature( test )]

extern crate test;

#[macro_use]
extern crate log;

extern crate one_seek_one_read;

#[cfg( test )]
mod tests {
    use std::fmt;
    use std::io::Result;
    use test::Bencher;
    use env_logger;
    use rand::Rng;

    use one_seek_one_read::archive::Archive;
    use one_seek_one_read::normal::Normal;
    use one_seek_one_read::index;
    use one_seek_one_read::util;

    const ARCHIVE: &'static str = "archive.data";
    const INDEX: &'static str = "resource.index";

    #[bench]
    fn run_normal( b: &mut Bencher ) {
        let records = index::read_records( INDEX )?;
        let normal = Normal::new( &records );
        let mut rng = rand::thread_rng( );

        let size = records.len( );

        b.iter( | |
                {
                    let target = rng.gen_range( 0, size );
                    debug!( "target: {}", target );
                    let record = &records[ target ];
                    debug!( "record: {:?}", record );
                    let data = normal.read( target )?;

                    debug_assert_eq!( util::get_hash( &data ), record.hash );
                } );
    }

    #[bench]
    fn run_archive( b: &mut Bencher ) {
        let records = index::read_records( INDEX )?;
        let mut archive = Archive::new( ARCHIVE, &records )?;
        let mut rng = rand::thread_rng( );

        let size = records.len( );

        b.iter( | |
                {
                    let target = rng.gen_range( 0, size );
                    debug!( "target: {}", target );
                    let record = &records[ target ];
                    debug!( "record: {:?}", record );
                    let data = archive.read( target )?;

                    assert_eq!( util::get_hash( &data ), record.hash );
                } );
    }
}
