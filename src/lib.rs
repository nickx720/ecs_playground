use std::collections::HashMap;

pub trait Component {}

pub struct World { 
    components: HashMap<String,Vec<Box<dyn Component>>>,
}

impl World { 
    pub fn new()-> Self { 
        Self {
            components: HashMap::new(),        }
    }

    pub fn register_component(&mut self,name: &str) { 
        self.components.insert(name.to_owned(), vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut world = World::new();

        world.register_component("location");

        // Create a bird
        let location:(f32,f32) = (0.0,0.0);
    }
}
