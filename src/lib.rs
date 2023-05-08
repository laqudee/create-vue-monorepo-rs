pub mod utils;
pub mod dialoguers;
pub mod render;

#[derive(Debug)]
pub struct ConfiguresSelected {
    pub eslint_config: bool,
    pub vitest_config: bool,
    pub common_library: bool,
}

impl ConfiguresSelected {
    pub fn new() -> Self {
        Self {
            eslint_config: false,
            vitest_config: false,
            common_library: false,
        }
    }

    pub fn set_eslint_config(&mut self, value: bool) {
        self.eslint_config = value
    }

    pub fn set_vitest_config(&mut self, value: bool) {
        self.vitest_config = value
    }

    pub fn set_common_library(&mut self, value: bool) {
        self.common_library = value
    }
}

impl Default for ConfiguresSelected {
    fn default() -> Self {
        Self::new()
    }
}


