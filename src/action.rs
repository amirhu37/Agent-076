use std::fmt::Display;
use crate::predicate::VecPredicate;


#[derive(Debug , Default)]
pub struct Action{
    pub action_set : VecPredicate
}

impl Action{
    pub fn new() -> Self{
        Self{
            action_set : VecPredicate::default(),
            }   }
    }
    


impl Display for Action{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std
    ::fmt::Result {
        write!(f, "actions {{ {} }}", self.action_set)
        }
}
