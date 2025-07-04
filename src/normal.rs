use std::fs::File;
use std::io::{ Read, Result };
use crate::index::Record;

pub struct Normal {
    paths: Vec< String >,
    sizes: Vec< usize >,
    enable_one_read: bool,
}

impl Normal {
    pub fn new( records: &Vec< Record >, enable_one_read: bool ) -> Self {
        let mut paths = vec![ ];
        let mut sizes = vec![ ];

        for record in records {
            paths.push( record.path.to_string( ) );
            sizes.push( record.size );
        }

        Normal {
            paths,
            sizes,
            enable_one_read,
        }
    }

    pub fn read( &self, at: usize ) -> Result< Vec< u8 > > {
        let path = &self.paths[ at ];

        let mut file = File::open( path )?;
        let mut data;

        if self.enable_one_read {
            let size = self.sizes[ at ];
            unsafe {
                data = Vec::<u8>::with_capacity( size );
                data.set_len( size );
            }
            file.read( &mut data )?;
        }
        else {
            data = vec![ ];
            file.read_to_end( &mut data )?;
        }

        Ok( data )
    }
}

#[test]
fn test_reading_by_hash( ) -> Result< () > {
    use crate::util;
    use crate::index;
    use crate::test_utils::test_utils;

    test_utils::setup_test_data()?;

    let records = index::read_records( util::INDEX_PATH )?;
    let normal = Normal::new( &records, false );

    for ( index, record ) in records.iter( ).enumerate( ) {
        let data = normal.read( index )?;

        debug_assert_eq!( util::get_hash( &data ), record.hash );
    }

    let one_read = Normal::new( &records, true );

    for ( index, record ) in records.iter( ).enumerate( ) {
        let data = one_read.read( index )?;

        debug_assert_eq!( util::get_hash( &data ), record.hash );
    }

    test_utils::cleanup_test_data()?;

    Ok( () )
}
