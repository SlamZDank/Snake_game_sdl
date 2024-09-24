// this will spawn an apple in a location that is not taken
pub struct Apple {
    pub position: (u32, u32),
}

impl Apple {
    pub fn add_to_field_at_start(field_dimension_x: u32, field_dimension_y: u32) -> Self {
        Self {
            position: (field_dimension_x, field_dimension_y),
        }
    }

    pub fn place(&mut self, new_place_x: u32, new_place_y: u32) {
        self.position.0 = new_place_x;
        self.position.1 = new_place_y;
    }
}
