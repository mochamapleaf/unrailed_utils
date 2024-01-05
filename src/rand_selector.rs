use crate::unrailed_rng::UnrailedRng;

pub struct RandSelector<T> where
    T: PartialEq + Clone{
    pool: Vec<T>,
    weight: Vec<f32>,
}

impl<T> RandSelector<T> where
   T: PartialEq + Clone{
    pub(crate) fn new() -> Self{
        Self{
            pool: Vec::new(),
            weight: Vec::new(),
        }
    }
    //add an item with default weight of 1.0
    pub(crate) fn add(&mut self, item: T) -> &Self{
        debug_assert!(!self.pool.contains(&item));
        self.pool.push(item);
        self.weight.push(1.0);
        self
    }

    pub(crate) fn select(&self, prob: f32) -> &T{
        //generate lookup list
        let total_weight = self.weight.iter().sum::<f32>();
        let prob_lookup = self.weight.iter().map(|x| x / total_weight).collect::<Vec<f32>>();
        let selected = prob_lookup.iter().position(|x| prob < *x).unwrap_or(self.weight.len() -1);
        &self.pool[selected]
    }

    //multiply weight of item by mul
    pub(crate) fn update_weight(&mut self, item: &T, mul: f32){
        if let Some(item_i) = self.pool.iter().position(|x| x == item){
            self.weight[item_i] *= mul;
        }
    }
}

impl<T> FromIterator<T> for RandSelector<T> where
    T: PartialEq + Clone
{
    fn from_iter<K: IntoIterator<Item=T>>(iter: K) -> Self {
        let mut ret = Self{
            pool: Vec::new(),
            weight: Vec::new(),
        };
        for item in iter{
            ret.add(item);
        }
        ret
    }
}
