use std::fs::File;
use std::io::{ Read, Result };
use crate::index::Record;

pub struct Normal {
    paths: Vec< String >,
}

impl Normal {
    pub fn new( records: &Vec< Record > ) -> Self {
        let mut paths = vec![ ];

        for record in records {
            paths.push( record.path.to_string( ) );
        }

        Normal {
            paths,
        }
    }

    pub fn read( &self, at: usize ) -> Result< Vec< u8 > > {
        let path = &self.paths[ at ];

        let mut file = File::open( path )?;
        let mut data = vec![ ];

        file.read_to_end( &mut data )?;

        Ok( data )
    }
}

#[test]
fn test_reading_by_hash( ) -> Result< () > {
    use crate::util;
    use crate::index;

    let records = index::read_records( util::INDEX_PATH )?;
    let normal = Normal::new( &records );

    for ( index, record ) in records.iter( ).enumerate( ) {
        let data = normal.read( index )?;

        assert_eq!( util::get_hash( &data ), record.hash );
    }

    Ok( () )
}
