use std::cell::Cell;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy)]
pub struct BodyProperties {
    // the position in x and y coordinates
    pub position: (u32, u32),
    direction: Direction,
}

pub struct Snake {
    pub body: Vec<Cell<BodyProperties>>,
}

impl Snake {
    pub fn add_to_field_at_start(x: u32, y: u32) -> Self {
        let head = BodyProperties {
            position: (x, y),
            direction: Direction::LEFT,
        };
        let tail = BodyProperties {
            position: (head.position.0 - 1, head.position.1),
            direction: head.direction,
        };
        // One head and one tail for the starting point

        Snake {
            body: Vec::from([Cell::new(head), Cell::new(tail)]),
        }
    }

    // todo: ADD wrapping if no border is set
    #[allow(unused_assignments)]
    pub fn move_snake(&mut self, width: u32, height: u32) {
        let mut sub_1 = self.body[0].clone();
        let mut sub_2 = self.body[0].clone();
        match self.body[0].get().direction {
            Direction::UP => {
                if self.body[0].get().position.1 == 0 {
                    self.body[0].set(BodyProperties {
                        position: (self.body[0].get().position.0, height - 1),
                        direction: self.body[0].get().direction,
                    })
                } else {
                    self.body[0].set(BodyProperties {
                        position: (
                            self.body[0].get().position.0,
                            self.body[0].get().position.1 - 1,
                        ),
                        direction: self.body[0].get().direction,
                    });
                }
            }
            Direction::DOWN => {
                if self.body[0].get().position.1 == height - 1 {
                    self.body[0].set(BodyProperties {
                        position: (self.body[0].get().position.0, 0),
                        direction: self.body[0].get().direction,
                    })
                } else {
                    self.body[0].set(BodyProperties {
                        position: (
                            self.body[0].get().position.0,
                            self.body[0].get().position.1 + 1,
                        ),
                        direction: self.body[0].get().direction,
                    });
                }
            }
            Direction::LEFT => {
                if self.body[0].get().position.0 == width - 1 {
                    self.body[0].set(BodyProperties {
                        position: (0, self.body[0].get().position.1),
                        direction: self.body[0].get().direction,
                    })
                } else {
                    self.body[0].set(BodyProperties {
                        position: (
                            self.body[0].get().position.0 + 1,
                            self.body[0].get().position.1,
                        ),
                        direction: self.body[0].get().direction,
                    });
                }
            }
            Direction::RIGHT => {
                if self.body[0].get().position.0 == 0 {
                    self.body[0].set(BodyProperties {
                        position: (width - 1, self.body[0].get().position.1),
                        direction: self.body[0].get().direction,
                    })
                } else {
                    self.body[0].set(BodyProperties {
                        position: (
                            self.body[0].get().position.0 - 1,
                            self.body[0].get().position.1,
                        ),
                        direction: self.body[0].get().direction,
                    });
                }
            }
        }

        // moving the snake one block forward
        let mut first_run = true;
        for body_values in &mut self.body {
            if first_run {
                first_run = !first_run;
                continue;
            };
            sub_2 = body_values.clone();
            *body_values = sub_1.to_owned();
            sub_1 = sub_2.clone();
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if (self.body[0].get().direction == Direction::UP
            || self.body[0].get().direction == Direction::DOWN)
            && (direction == Direction::DOWN || direction == Direction::UP)
        {
            return;
        };

        if (self.body[0].get().direction == Direction::RIGHT
            || self.body[0].get().direction == Direction::LEFT)
            && (direction == Direction::LEFT || direction == Direction::RIGHT)
        {
            return;
        };

        self.body[0].set(BodyProperties {
            position: self.body[0].get().position,
            direction,
        });
    }

    // need to optimize this so it can conform to the wrapping feature as its crashing
    pub fn grow_snake(&mut self, width: u32, height: u32) {
        self.body.push(Cell::new(BodyProperties {
            position: match self.body[self.body.len() - 1].get().direction {
                Direction::LEFT => {
                    let mut new_body_position = self.body[self.body.len() - 1].get().position;
                    if new_body_position.0 == 0 {
                        new_body_position.0 = width - 1;
                    } else {
                        new_body_position.0 -= 1;
                    }

                    new_body_position
                }
                Direction::DOWN => {
                    let mut new_body_position = self.body[self.body.len() - 1].get().position;
                    if new_body_position.1 == 0 {
                        new_body_position.1 = height - 1;
                    } else {
                        new_body_position.1 -= 1;
                    }
                    new_body_position
                }
                Direction::UP => {
                    let mut new_body_position = self.body[self.body.len() - 1].get().position;
                    if new_body_position.1 == height - 1 {
                        new_body_position.1 = 0
                    } else {
                        new_body_position.1 += 1;
                    }
                    new_body_position
                }
                Direction::RIGHT => {
                    let mut new_body_position = self.body[self.body.len() - 1].get().position;
                    if new_body_position.0 == width - 1 {
                        new_body_position.0 = 0
                    } else {
                        new_body_position.0 += 1;
                    }
                    new_body_position
                }
            },
            direction: self.body[self.body.len() - 1].get().direction,
        }));
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::add_to_field_at_start(10, 10)
    }
}
