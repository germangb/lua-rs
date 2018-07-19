use Result;

use std::fs::File;
use std::path::Path;
use std::{fmt, io, str};

pub trait IntoLuaSource {
    fn into(self) -> LuaSource;
}

impl<T> IntoLuaSource for T
where
    T: AsRef<[u8]>,
{
    fn into(self) -> LuaSource {
        let slice = self.as_ref();
        let mut source = LuaSource::with_capacity(slice.len());
        source.extend(slice);
        source
    }
}

impl<'a> IntoLuaSource for &'a LuaSource {
    #[inline]
    fn into(self) -> LuaSource {
        self.clone()
    }
}

impl IntoLuaSource for LuaSource {
    #[inline]
    fn into(self) -> LuaSource {
        self
    }
}

#[derive(Clone)]
pub struct LuaSource {
    buffer: Vec<u8>,
}

impl fmt::Debug for LuaSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.len();
        write!(f, "{:?}", str::from_utf8(&self.buffer[..len]))
    }
}

impl LuaSource {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn from<S>(s: S) -> Self
    where
        S: AsRef<[u8]>,
    {
        let mut source = Self::with_capacity(s.as_ref().len());
        source.extend(s);
        source
    }

    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Self::from_reader(File::open(path)?)
    }

    pub fn from_reader<R>(mut reader: R) -> Result<Self>
    where
        R: io::Read,
    {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Self::from(data))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity + 1);
        buffer.push(0);
        Self { buffer }
    }

    pub fn extend<T: AsRef<[u8]>>(&mut self, s: T) {
        self.buffer.pop();
        self.buffer.extend_from_slice(s.as_ref());
        self.buffer.push(0);
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    pub fn len(&self) -> usize {
        self.buffer.len() - 1
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.buffer.push(0);
    }
}
