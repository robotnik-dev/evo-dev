use crate::genotype::Genotype;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn genotype(&self) -> &Genotype;
    fn create(genotype: Genotype) -> Self;
}
