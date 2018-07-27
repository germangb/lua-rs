use std::os::raw;

/// Enum to index the stack relative to the Top and Bottom
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Index {
    /// index from the top of the stack
    Top(usize),

    /// index from the bottom of the stack
    Bottom(usize),
}

impl Index {
    /// Top of the stack. Equivalent to `-1`
    pub const TOP: Index = Index::Top(1);

    /// Bottom of the stack. Equivalent to `1`
    pub const BOTTOM: Index = Index::Bottom(1);

    /// Index of the registry table. Equivalent to `LUA_REGISTRYINDEX`
    pub const REGITRY: Index = Index::Top(1001000);

    #[inline]
    pub fn from_absolute(v: raw::c_int) -> Self {
        if v < 0 {
            Index::Top((-v) as _)
        } else {
            Index::Bottom(v as _)
        }
    }

    #[inline]
    pub fn as_absolute(&self) -> raw::c_int {
        match *self {
            Index::Top(i) => {
                let idx = i as raw::c_int;
                -idx
            }
            Index::Bottom(i) => i as _,
        }
    }
}

impl From<isize> for Index {
    #[inline]
    fn from(idx: isize) -> Index {
        Index::from_absolute(idx as _)
    }
}
