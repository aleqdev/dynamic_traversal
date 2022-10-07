#[derive(Debug)]
pub struct GeneralCompilationData {
    pub element_index: usize,
    pub head_index: usize,
}

impl Default for GeneralCompilationData {
    fn default() -> Self {
        Self {
            element_index: 0,
            head_index: 0,
        }
    }
}
