use std::iter;
use hex;
use rand::{ Rng, SeedableRng, rngs::StdRng };
use sha1::{ Sha1, Digest };

pub const ARCHIVE_PATH: &'static str = "data/archive.data";
pub const INDEX_PATH: &'static str = "data/resource.index";

pub fn get_hash( data: &Vec< u8 > ) -> String {
    let mut hasher = Sha1::new( );
    hasher.input( data );
    let hash = hex::encode( hasher.result( ).to_vec( ) );

    hash
}

pub fn get_random_iterator( max_index: usize ) -> impl Iterator< Item=usize > {
    let seed = [
        9, 167, 249, 169, 8,
        33, 178, 6, 107, 190,
        90, 75, 229, 24, 59,
        168, 153, 217, 43, 190,
        139, 182, 222, 137, 75,
        45, 239, 225, 64, 57,
        143, 91
    ];
    let mut rng: StdRng = SeedableRng::from_seed( seed );

    iter::repeat( max_index ).map( move | i | rng.gen_range( 0, i ) )
}

pub fn get_sequential_iterator( max_index: usize ) -> impl Iterator< Item=usize > {
    let iter = ( 0_usize .. max_index ).cycle( );

    iter
}
