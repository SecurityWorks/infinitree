use super::{Result, WriteObject};
use crate::{
    backends::Backend,
    compress,
    crypto::{ChunkKey, CryptoProvider},
    ChunkPointer,
};

use std::sync::Arc;

pub trait Reader: Send {
    fn read_chunk<'target>(
        &mut self,
        pointer: &ChunkPointer,
        target: &'target mut [u8],
    ) -> Result<&'target [u8]>;
}

#[derive(Clone)]
pub struct AEADReader {
    backend: Arc<dyn Backend>,
    crypto: ChunkKey,
    buffer: WriteObject,
}

impl AEADReader {
    pub fn new(backend: Arc<dyn Backend>, crypto: ChunkKey) -> Self {
        AEADReader {
            backend,
            crypto,
            buffer: WriteObject::default(),
        }
    }
}

impl Reader for AEADReader {
    fn read_chunk<'target>(
        &mut self,
        pointer: &ChunkPointer,
        target: &'target mut [u8],
    ) -> Result<&'target [u8]> {
        let object = self.backend.read_object(pointer.object_id())?;
        let cryptbuf: &mut [u8] = self.buffer.as_inner_mut();

        let buf =
            self.crypto
                .decrypt_chunk(cryptbuf, object.as_inner(), object.id(), pointer.as_raw());
        let size = compress::decompress_into(buf, target)?;

        Ok(&target[..size])
    }
}
