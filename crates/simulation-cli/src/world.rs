use crate::*;

#[derive(Debug, Clone)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(value: &sim::World) -> Self {
        Self {
            animals: value.animals().iter().map(Animal::from).collect(),
            foods: value.foods().iter().map(Food::from).collect()
        }
    }
}
