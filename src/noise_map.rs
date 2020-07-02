use std::ops::Index;

use perlin_noise::PerlinNoise;

#[derive(Debug)]
pub struct NoiseMap(Vec<Vec<f64>>);

impl NoiseMap {
    pub fn new(width: usize, height: usize, mut scale: f64) -> Self {
        let perlin = PerlinNoise::new();
        let mut map = vec![vec![0.0; width]; height];

        if scale <= 0.0 {
            scale = 0.0001;
        }

        for y in 0..height {
            for x in 0..width {
                let sample_x = x as f64 / scale;
                let sample_y = y as f64 / scale;

                let value = perlin.get2d([sample_x, sample_y]);

                map[y][x] = value;
            }
        }

        NoiseMap(map)
    }
}

impl Index<(u32, u32)> for NoiseMap {
    type Output = f64;

    fn index(&self, point: (u32, u32)) -> &Self::Output {
        &self.0[point.1 as usize][point.0 as usize]
    }
}
