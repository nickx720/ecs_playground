use std::collections::HashMap;

pub struct Entity<T> where T: core::any::Any, { 
    data: T,
}

pub struct World<T: 'static> { 
    components: HashMap<String,Vec<Entity<T>>>,
}

impl<T> World<T> { 
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
        let mut world = World::<f32>::new();

        world.register_component("location");

        // Create a bird
        let location:(f32,f32) = (0.0,0.0);
    }
}
