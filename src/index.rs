#[derive( Debug )]
pub struct Record {
    pub path: String,
    pub size: usize,
    pub hash: String,
}

#[derive( Debug )]
pub struct Index {
    pub offset: u64,
    pub size: usize,
}
