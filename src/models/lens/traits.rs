use super::catalogue::{Category, Criteria};

pub trait CriteriaConverter {
    fn get_criteria() -> Vec<Criteria>;
}

pub trait CategoryConverter {
    fn get_category() -> Category;
}
