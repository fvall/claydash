use rand::{Rng, SeedableRng};

pub const EPS: f32 = 0.000001;

#[inline]
pub fn clamp(x: f32, low: f32, high: f32) -> f32 {
    x.min(high).max(low)
}

#[inline]
pub fn lerp(x: f32, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
    if x <= x0 {
        return y0;
    }

    if x >= x1 {
        return y1;
    }

    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}

pub fn cut<T: Into<f32> + Copy>(input: &[T], bins: u16, output: &mut Vec<u32>) {
    output.clear();
    if input.is_empty() || bins == 0 {
        return;
    }

    for _ in 0..bins {
        output.push(0);
    }

    let mut max = f32::MIN;
    let mut min = f32::MAX;

    for val in input {
        if (*val).into() < min {
            min = (*val).into();
        }

        if (*val).into() > max {
            max = (*val).into();
        }
    }

    let step = (max - min) / (bins as f32);
    min -= EPS;
    max += EPS;

    for val in input {
        let mut idx = 0;
        let mut lhs = min;
        let mut rhs = lhs + step;
        while ((*val).into() < lhs || (*val).into() > rhs) && (rhs <= max) {
            lhs = rhs;
            rhs += step;
            idx += 1;
        }

        if idx == output.len() && (*val).into() < max {
            idx -= 1;
        }

        if idx >= output.len() {
            eprintln!(
                "ERROR: Index is not supposed to lie outside of output range: {} - {} - {} - {} - {min} - {max}",
                idx,
                (*val).into(),
                lhs,
                rhs
            );
            return;
        }

        output[idx] += 1;
    }
}

fn runif_single<T: Rng>(rng: &mut T) -> f32 {
    let val = rng.random::<u32>();
    (val as f32) / (u32::MAX as f32)
}

fn runif<T: Rng>(rng: &mut T, size: u32, output: &mut Vec<f32>) {
    output.clear();
    for _ in 0..size {
        output.push(runif_single(rng));
    }
}

fn rexp_single<T: Rng>(rng: &mut T, beta: f32) -> f32 {
    let val = runif_single(rng);
    let arg = (1.0 - val).max(EPS);
    arg.ln() / (-beta)
}
fn rexp<T: Rng>(rng: &mut T, size: u32, beta: f32, output: &mut Vec<f32>) {
    runif(rng, size, output);
    for val in output.iter_mut() {
        let arg = (1.0 - *val).max(EPS);
        *val = arg.ln() / (-beta);
    }
}

pub trait Distribution {
    type Value;

    fn random(&mut self, size: u32, output: &mut Vec<Self::Value>);
    fn random_owned(&mut self, size: u32) -> Vec<Self::Value> {
        let mut output = Vec::with_capacity(size as usize);
        self.random(size, &mut output);
        output
    }
    fn pdf(&self, x: &mut Vec<f32>, y: &mut Vec<f32>);
    fn reseed(&mut self, seed: u64);
    fn get_seed(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct Uniform {
    seed: u64,
    rng: rand::rngs::SmallRng,
}

#[derive(Debug, Clone)]
pub struct Normal {
    seed: u64,
    rng: rand::rngs::SmallRng,
}

#[derive(Debug, Clone)]
pub struct Gamma {
    pub alpha: u8, // using an integer to make the math simpler
    pub beta: f32,
    seed: u64,
    rng: rand::rngs::SmallRng,
}

#[derive(Debug, Clone)]
pub struct Exponential {
    pub beta: f32,
    seed: u64,
    rng: rand::rngs::SmallRng,
}

impl Default for Uniform {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Uniform {
    pub fn new(seed: u64) -> Self {
        let rng = rand::rngs::SmallRng::seed_from_u64(seed);
        Self { seed, rng }
    }
}

impl Distribution for Uniform {
    type Value = f32;
    fn random(&mut self, size: u32, output: &mut Vec<f32>) {
        runif(&mut self.rng, size, output);
    }

    fn reseed(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = rand::rngs::SmallRng::seed_from_u64(self.seed);
    }

    fn pdf(&self, x: &mut Vec<f32>, y: &mut Vec<f32>) {
        const N: usize = 100;
        const STEP: f32 = 1.0 / (N as f32);

        x.clear();
        y.clear();
        let mut val = 0.0;
        while x.len() < N {
            x.push(val);
            y.push(1.0);
            val += STEP;
        }
    }

    fn get_seed(&self) -> u64 {
        self.seed
    }
}

impl Default for Normal {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Normal {
    pub fn new(seed: u64) -> Self {
        let rng = rand::rngs::SmallRng::seed_from_u64(seed);
        Self { seed, rng }
    }
}

impl Distribution for Normal {
    type Value = f32;
    fn random(&mut self, size: u32, output: &mut Vec<Self::Value>) {
        // Ideally we would like something more accurate and faster...

        let mut s = size;
        if s % 2 == 1 {
            s += 1;
        }

        runif(&mut self.rng, s, output);

        let mut idx = 0;
        while idx < s {
            let u1 = output[idx as usize];
            let u2 = output[idx as usize + 1];

            let scale = (-2.0 * u1.ln()).sqrt();
            let x1 = scale * (2.0 * std::f32::consts::PI * u2).cos();
            let x2 = scale * (2.0 * std::f32::consts::PI * u2).sin();

            output[idx as usize] = x1;
            output[idx as usize + 1] = x2;
            idx += 2;
        }

        while output.len() > size as usize {
            output.pop();
        }
    }

    fn reseed(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = rand::rngs::SmallRng::seed_from_u64(self.seed);
    }

    fn pdf(&self, x: &mut Vec<f32>, y: &mut Vec<f32>) {
        const N: usize = 100;
        let min_ = -4.0 - EPS;
        let max_ = 4.0 + EPS;
        let step = (max_ - min_) / (N as f32);

        x.clear();
        y.clear();

        let mut val = min_;
        while x.len() < N {
            x.push(val);
            y.push(gaussian(val));
            val += step;
        }
    }

    fn get_seed(&self) -> u64 {
        self.seed
    }
}

impl Default for Exponential {
    fn default() -> Self {
        Self::new(1, 1.0)
    }
}

impl Exponential {
    pub fn new(seed: u64, beta: f32) -> Self {
        let rng = rand::rngs::SmallRng::seed_from_u64(seed);
        let mut g = Self { seed, rng, beta };

        if g.beta <= 0.0 {
            g.beta = 1.0;
        }

        g
    }
}

impl Distribution for Exponential {
    type Value = f32;

    fn reseed(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = rand::rngs::SmallRng::seed_from_u64(self.seed);
    }

    fn get_seed(&self) -> u64 {
        self.seed
    }

    fn random(&mut self, size: u32, output: &mut Vec<Self::Value>) {
        rexp(&mut self.rng, size, self.beta, output);
    }

    fn pdf(&self, x: &mut Vec<f32>, y: &mut Vec<f32>) {
        const N: usize = 200;
        let min_ = 0.0;
        let max_ = 5.0 + EPS;
        let step = (max_ - min_) / (N as f32);

        x.clear();
        y.clear();

        let mut val = min_;
        while x.len() < x.capacity() {
            x.push(val);
            y.push(self.beta * (-self.beta * val).exp());
            val += step;
        }
    }
}

impl Default for Gamma {
    fn default() -> Self {
        Self::new(1, 1, 1.0)
    }
}

impl Gamma {
    pub fn new(seed: u64, alpha: u8, beta: f32) -> Self {
        let rng = rand::rngs::SmallRng::seed_from_u64(seed);
        let mut g = Self { seed, rng, alpha, beta };
        if g.alpha == 0 {
            g.alpha = 1;
        }

        if g.beta <= 0.0 {
            g.beta = 1.0;
        }

        g
    }
}

impl Distribution for Gamma {
    type Value = f32;

    fn random(&mut self, size: u32, output: &mut Vec<Self::Value>) {
        output.clear();
        while output.len() < size as usize {
            output.push(0.0);
        }

        if (self.alpha == 0) || (self.beta <= 0.0) {
            return;
        }

        // let mut temp = output.clone();
        for _ in 0..self.alpha {
            for out in output.iter_mut() {
                *out += rexp_single(&mut self.rng, self.beta);
            }
            // rexp(&mut self.rng, size, self.beta, &mut temp);
            // for (out, exp) in output.iter_mut().zip(temp.iter()) {
            //     *out += *exp;
            // }
        }

        let mut max = 0.0;
        let mut min = 0.0;
        for val in output.iter() {
            if *val > max {
                max = *val;
            }
            if *val < min {
                min = *val;
            }
        }
    }

    fn reseed(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = rand::rngs::SmallRng::seed_from_u64(self.seed);
    }

    fn get_seed(&self) -> u64 {
        self.seed
    }

    fn pdf(&self, x: &mut Vec<f32>, y: &mut Vec<f32>) {
        const N: usize = 300;
        let min_ = 0.0;
        let max_ = (self.alpha as f32) / self.beta + (8.0 * (self.alpha as f32).sqrt() / self.beta) + EPS;
        let step = (max_ - min_) / (N as f32);

        x.clear();
        y.clear();

        let coeff = (self.beta).powi(self.alpha as i32) / (fat(self.alpha as u32) as f32);
        let mut val = min_;
        while x.len() < N {
            x.push(val);
            y.push(coeff * val.powi(self.alpha as i32 - 1) * (-val * self.beta).exp());
            val += step;
        }
    }
}

fn gaussian(x: f32) -> f32 {
    (-x * x * 0.5).exp() / (2.0 * std::f32::consts::PI).sqrt()
}

fn fat(mut x: u32) -> u32 {
    if x < 2 {
        return 1;
    }

    let mut out = 1;
    while x > 1 {
        out *= x;
        x -= 1;
    }

    out
}
