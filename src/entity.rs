
/// Entity is a unique identifier for a game object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    version: u32,
    index: u32,
}

impl Entity {
    pub const fn new(index: u32) -> Self {
        Entity {
            version: 0,
            index,
        }
    }

    pub fn with_version(index: u32, version: u32) -> Self {
        Entity {
            version,
            index,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn invalidate(&mut self) {
        self.index = u32::MAX;
        self.version += 1;
    }

    pub fn is_valid(&self) -> bool {
        self.index != u32::MAX
    }
}