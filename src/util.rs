use hex;
use sha1::{ Sha1, Digest };

pub const ARCHIVE_PATH: &'static str = "archive.data";
pub const INDEX_PATH: &'static str = "resource.index";

pub fn get_hash( data: &Vec<u8> ) -> String {
    let mut hasher = Sha1::new( );
    hasher.input( &data );
    let hash = hex::encode( hasher.result( ).to_vec( ) );

    hash
}
