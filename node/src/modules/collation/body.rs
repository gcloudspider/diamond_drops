use modules::collation::chunk::Chunk;

/// Collation body storing arbitrary data or blobs, serialized into 32-byte chunks.
#[derive(PartialEq, Debug, Clone)]
pub struct Body;/*this doesn't work, presumably because  {
    chunk: Chunk
}

impl Body {
    pub fn new(chunk: Chunk) -> Body {
        Body {
            chunk
        }
    }
}

/// Serialize collation bodies that correspond to the same blob. In practice a blob should contain
///  a hash and this struct should then contain that hash as a field.
pub struct BlobBodies {
    body: Body
}

impl BlobBodies {
    pub fn new(body: Body) -> BlobBodies {
        BlobBodies {
            Body
        }
    }
} */