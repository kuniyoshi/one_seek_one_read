use std::fs::File;
use std::io::{ Read, Seek, SeekFrom, Result };
use crate::index::{ Index, Record };

#[derive( Debug )]
pub struct Archive {
    indexes: Vec<Index>,
    file: File,
    last_index: usize,
    optimization: bool,
}

impl Archive {
    pub fn new( archive_path: &str, // use os path instead
                records: &Vec< Record >,
                optimization: bool ) -> Result< Self > {
        let indexes = Self::indexes_from_records( records );

        let file = File::open( archive_path )?;

        Ok(
            Archive {
                indexes,
                file,
                last_index: 0,
                optimization,
            }
        )
    }

    pub fn read( &mut self, at: usize ) -> Result< Vec<u8> > {
        let index = &self.indexes[ at ];
        let mut data;

        unsafe {
            data = Vec::<u8>::with_capacity( index.size );
            data.set_len( index.size );
        }

        if !self.optimization || at != self.last_index + 1 {
            let seek = SeekFrom::Start( index.offset );
            self.file.seek( seek )?;
        }

        self.file.read( &mut data )?;
        debug_assert!( data.len( ) > 0 );

        self.last_index = at;

        Ok( data )
    }

    fn indexes_from_records( records: &Vec< Record > ) -> Vec<Index> {
        let mut indexes = vec![ ];
        let mut offset = 0_u64;

        for record in records {
            let size = record.size;

            let index = Index {
                offset,
                size,
            };

            debug!( "{:?}", index );

            indexes.push( index );

            offset = offset + size as u64;
        }

        indexes
    }

}

#[test]
fn test_reading_by_hash( ) -> Result< () > {
    use crate::util;
    use crate::index;

    let records = index::read_records( util::INDEX_PATH )?;
    let mut archive = Archive::new( util::ARCHIVE_PATH, &records )?;

    for ( index, record ) in records.iter( ).enumerate( ) {
        let data = archive.read( index )?;

        debug_assert_eq!( util::get_hash( &data ), record.hash );
    }

    Ok( () )
}

