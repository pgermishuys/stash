use std::{
    fs::File,
    io::prelude::*
};

pub struct Chunk {
    location: String
}

impl Chunk {
    pub fn open(location: String) -> Chunk {
        return Chunk {
            location
        }
    }
}
