use std::collections::HashMap;


#[derive(Default)]
pub struct World { 
    components: HashMap<String,Vec<Option<Box<dyn core::any::Any>>>>,
}

impl World { 
    pub fn new()-> Self { 
        Self {
            components: HashMap::new(),        }
    }

    pub fn register_component(&mut self,name: &str) { 
        self.components.insert(name.to_owned(), vec![]);
    }

    /// When creating a new entity we will want to add a new element to the component vectors in
    /// preparation for that entity.
    pub fn create_entity(&mut self) -> &mut Self {
        self.components.iter_mut().for_each(|(_name,components)|{ 
            components.push(None);
        });
        self
    }

    /// With is always called for create entity, so we can be sure that the last element of t he
    /// vector is what we are updating.
    pub fn with(&mut self, name: &str, data: Box<dyn core::any::Any>) -> &mut Self {
        if let Some(components) = self.components.get_mut(name){ 
            let index = components.len()-1;
            components[index] = Some(data);
        }        
        self
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

        world.create_entity().with("location",Box::new(location));
    }
}
