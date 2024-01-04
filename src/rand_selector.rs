use crate::unrailed_rng::UnrailedRng;

pub struct RandSelector<T>{
    pool: Vec<T>,
    prob: Vec<f32>,
    rng: UnrailedRng,
}

impl<T> RandSelector<T> {
    fn add(&mut self, item: T) -> &Self{
        self.pool.push(item);
        self.prob.push(1.0);
        self
    }
}
