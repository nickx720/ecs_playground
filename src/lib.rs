use std::{any::TypeId, collections::HashMap, fmt::Debug, ops::Add};



#[derive(Default)]
pub struct World<T> 
where 
    T: Send + Sync + 'static, 
{ 
    components: HashMap<TypeId,Vec<T>>,
}

impl<T> World<T> 
where 
    T: Send + Sync + 'static + Debug, { 
        pub fn new()-> Self { 
            Self {
                components: HashMap::new(),       
            }
        }
        pub fn spawn(&mut self, data:T) {
            dbg!(data);
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

        world.spawn((Location{x:0.0, y:0.0},));
    }
}
