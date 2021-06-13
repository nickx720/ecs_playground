use std::{collections::HashMap, fmt::Debug, ops::Add};


pub trait Component <T>
where 
    T:(core::any::Any)+ Debug{ 
        fn get(&self) -> Box<T>;
        fn get_mut(&mut self)-> Box<&mut T>;
    }

pub type ComponentData<T> = Vec<Option<Box<dyn Component<T>>>>;

#[derive(Default)]
pub struct World<T> { 
    components: HashMap<String,ComponentData<T>>,
}

impl<T> World<T> { 
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
    pub fn with(&mut self, name: &str, data: Box<dyn Component<T>>) -> &mut Self {
        if let Some(components) = self.components.get_mut(name){ 
            let index = components.len()-1;
            components[index] = Some(data);
        }        
        self
    }

    pub fn get(&self,name:&str)-> Option<&Vec<Option<Box<dyn Component<T>>>>> { 
        self.components.get(name)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Point(f32,f32);

    impl Component<(f32,f32)> for Point { 
        fn get(&self)->Box<(f32,f32)> { 
            Box::new((self.0,self.1))
        }

        fn get_mut(&mut self)-> Box<&mut (f32,f32 )> { 
            Box::new(&mut (self.0,self.1))
        }

    }

struct UpdateLocationsSystemg;

impl<T: Debug> UpdateLocationsSystem { 
    pub fn update_locations_system(&mut self, world:&mut World<T>) { 

    }
}

  //  fn update_locations_system<'a, T:Debug>(world:&mut World<T>) { 
  //      let mut locations = world.get("location").unwrap();
  //      let mut velocities = world.get("velocity").unwrap();

  //      let length = locations.len();

  //      locations.iter_mut().enumerate().for_each(|(index,location)|{
  //          if let Some(location) = location { 
  //              let mut location = location.get_mut();
  //              dbg!(location);
  //          }
  //      })
  //  }

    #[test]
    fn creating_the_world() {
        let mut world = World::new();

        world.register_component("location");
        world.register_component("velocity");        

        world
            .create_entity()
            .with("location",Box::new(Point(0.0,0.0)))
            .with("velcity", Box::new(Point(15.0,15.0)));

        let locations = world.get("location").unwrap();
        if let Some(location) = &locations[0]{
            assert_eq!(location.get(),Box::new((0.0,0.0)))
        }
    }
}
