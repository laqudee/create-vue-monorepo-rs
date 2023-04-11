pub mod utils;
pub mod dialoguers;
pub mod render;

#[derive(Debug)]
pub struct ConfiguresSelected {
    pub eslint_config: bool,
    pub vitest_config: bool,
    pub common_toolbox: bool,
}

impl ConfiguresSelected {
    pub fn new() -> Self {
        Self {
            eslint_config: false,
            vitest_config: false,
            common_toolbox: false,
        }
    }

    pub fn set_eslint_config(&mut self, value: bool) {
        self.eslint_config = value
    }

    pub fn set_vitest_config(&mut self, value: bool) {
        self.vitest_config = value
    }

    pub fn set_common_toolbox(&mut self, value: bool) {
        self.common_toolbox = value
    }
}

impl Default for ConfiguresSelected {
    fn default() -> Self {
        Self::new()
    }
}


