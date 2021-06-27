use std::{any::{Any, TypeId}, collections::HashMap, fmt::Debug, ops::Add};



#[derive(Default,Debug)]
pub struct World<T> 
where 
    T: Send + Sync + 'static, 
{ 
    data: HashMap<TypeId,Vec<T>>,
}

impl<T> World<T> 
where 
    T: Send + Sync + 'static + Debug, { 
        pub fn new()-> Self { 
            Self {
               data: HashMap::new(),       
            }
        }
        pub fn spawn(&mut self, componments:Vec<T>) {
            for component in componments{
                let type_id = component.type_id();
                if let Some(data_vec) = self.data.get_mut(&type_id) {
                    data_vec.push(component);
                } else {
                    self.data.insert(type_id,vec![component]);
                }
            }
        }

    }

#[cfg(test)]
mod tests {
    use super::*;

#[derive(Debug)]
    struct Location {
        x:f32,
        y:f32,
        }

    #[test]
    fn creating_the_world() {
        let mut world = World::new();

        world.spawn(vec![Location{x:0.0, y:0.0},]);
        dbg!(world);
    }
}
