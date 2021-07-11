use std::{any::{Any, TypeId}, collections::HashMap};



#[derive(Debug)]
pub struct Data 
{ 
    data: HashMap<TypeId,Vec<Box <dyn Any>>>,
}

impl Data
{ 
        pub fn new()-> Self { 
            let data = HashMap::new();
            Self {
               data       
            }
        }
 pub fn insert(&mut self, new_data: impl Any) { 
     self.data.insert(new_data.type_id(), vec![Box::new(new_data)]);
 }

    }

#[cfg(test)]
mod tests {
use std::any::Any;
use super::*;
    #[test]
    fn creating_new_data() {
        let data = Data::new();
        assert_eq!(data.data.len(),0);
    }
}
