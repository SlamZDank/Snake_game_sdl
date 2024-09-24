// this is used to generate the field with some has_obstacles
use rand::Rng;

pub struct Field {
    pub size: (u32, u32),
    has_border: bool,
    has_obstacles: bool,
    num_obstacles: u32,
    pub obstacles: Option<Vec<(u32, u32)>>,
}

pub fn generate_obstacles(
    size: (u32, u32),
    num_obstacles: u32,
    has_border: bool,
) -> Vec<(u32, u32)> {
    // generate obstacles
    let mut random_obstacles: Vec<(u32, u32)> = vec![];
    let mut rng = rand::thread_rng(); // seed the random number generator
    let (mut x, mut y);
    for _ in 0..num_obstacles {
        loop {
            (x, y) = (rng.gen_range(1..size.0), rng.gen_range(1..size.1));
            if x != size.0 / 2 && y != size.1 / 2 {
                break;
            }
        }
        random_obstacles.push((x, y));
    }

    if has_border {
        for idx_x in 0..size.0 {
            random_obstacles.push((idx_x, 0));
            random_obstacles.push((idx_x, size.1 - 1));
        }
        for idx_y in 0..size.1 {
            random_obstacles.push((0, idx_y));
            random_obstacles.push((size.0 - 1, idx_y));
        }
    }

    random_obstacles
}

impl Field {
    // initialilze the field with the default settings
    pub fn new() -> Self {
        Self {
            size: (21, 21),
            has_border: true,
            has_obstacles: false,
            num_obstacles: 0,
            obstacles: None,
        }
    }

    pub fn from(
        size: (u32, u32),
        has_border: bool,
        has_obstacles: bool,
        num_obstacles: u32,
    ) -> Self {
        Self {
            size,
            has_border,
            has_obstacles,
            num_obstacles,
            obstacles: if has_obstacles {
                Some(generate_obstacles(size, num_obstacles, has_border))
            } else if has_border {
                Some(generate_obstacles(size, 0, has_border))
            } else {
                None
            },
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
