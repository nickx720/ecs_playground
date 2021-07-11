use std::{any::Any};

use data::Data;

mod data;

pub struct World {
    data:Data,
}

impl World {
    pub fn new()-> Self { 
        let data = Data::new();
        Self { data }
    }
    pub fn insert_entity(&mut self, entity_data:Vec<impl Any>) { 
        for entity_part in entity_data {
            self.data.insert(entity_part);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::{data::Data, World};

    #[test]
    fn create_new_world(){
        let world = World::new();
        assert_eq!(world.data.data.len(),0);
    }
}
