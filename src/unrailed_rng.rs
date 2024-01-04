pub struct UnrailedRng{
    pub state1: u64,
    pub state2: u64,
}

impl UnrailedRng{
    pub fn new(val1: u64, val2: u64) -> Self{
        let mut ret = Self{
            state1: 0,
            state2: 0,
        };
        ret.state2 = (val2 << 1) | 0b1;
        ret.update_state();
        ret.state1 = ret.state1.wrapping_add(val1);
        ret.update_state();
        ret
    }
    pub fn from_states(state1: u64, state2: u64) -> Self{
        Self{
            state1,
            state2,
        }
    }
    pub fn from_seed_str(seed: &str) -> Self {
        todo!();
    }
    fn update_state(&mut self){
        self.state1 = self.state1.wrapping_mul(6364136223846793005);
        self.state1 = self.state1.wrapping_add(self.state2);
    }

    pub fn next_u32(&mut self) -> u32{
        let mut ret: u64 = self.state1;
        self.update_state();
        let tmp1: u32 = ( (ret >> 18 ^ ret) >> 27 ) as u32;
        let tmp2 = ret >> 59;
        let neg_tmp2 =  (!tmp2).wrapping_add(1);
        return (tmp1 >> (tmp2 % 32)) | tmp1 << ( neg_tmp2 % 32);
    }

    pub fn gen_range(&mut self, range: core::ops::Range<u32>) -> u32{
        if range.start >= range.end { return range.start; }
        let range_size = range.end - range.start;
        let neg_range_size = (!range_size).wrapping_add(1);
        let threshold = neg_range_size % range_size;
        let mut ret = self.next_u32();
        while ret < threshold {  ret = self.next_u32();  }
        (ret % range_size) + range.start
    }

    pub fn gen_f64(&mut self)-> f64{ self.gen_range(0..1000000) as f64 / 1000000.0 }

    pub fn gen_bool(&mut self) -> bool{ self.gen_range(0..2) == 1 }

    pub fn gen_prob(&mut self) -> f32{
        let val = self.next_u32();
        let denominator = f32::from_bits( (127 << 23) | (0x7FFFFF) );
        let numerator = f32::from_bits( (127 << 23) | (0x7FFFFF & val) );
        let tmp = (numerator / denominator).to_bits() - 1;
        return 2.0 * (f32::from_bits(tmp) - 0.5);
    }
}
