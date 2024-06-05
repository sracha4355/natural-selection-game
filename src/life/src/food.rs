use crate::utils::traits::Food;
pub struct SimpleFood {}
impl Food for SimpleFood {}
impl Default for SimpleFood {
    fn default() -> Self {
        SimpleFood {}
    }
}
