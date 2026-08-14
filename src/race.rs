use crate::player::Player;

#[derive(Clone, Copy)]
pub struct Checkpoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub struct Race {
    checkpoints: Vec<Checkpoint>,
    current_checkpoint: usize,
    current_lap: usize,
    total_laps: usize,
    finished: bool,
    was_inside: bool,
}

impl Race {
    pub fn new() -> Self {
        Self {
            checkpoints: vec![
                Checkpoint { x: 8.0,  y: 34.0, radius: 2.2 },
                Checkpoint { x: 57.0, y: 34.0, radius: 2.5 },
                Checkpoint { x: 57.0, y: 7.0,  radius: 2.5 },
                Checkpoint { x: 32.0, y: 6.0,  radius: 2.8 },
                Checkpoint { x: 7.0,  y: 7.0,  radius: 2.5 },
                Checkpoint { x: 7.0,  y: 21.0, radius: 2.5 },
            ],
            current_checkpoint: 0,
            current_lap: 1,
            total_laps: 3,
            finished: false,
            was_inside: false,
        }
    }

    pub fn update(&mut self, player: &Player) {
        if self.finished || self.checkpoints.is_empty() {
            return;
        }

        let cp = self.checkpoints[self.current_checkpoint];

        let dx = player.x - cp.x;
        let dy = player.y - cp.y;

        let inside =
            dx * dx + dy * dy
            <= cp.radius * cp.radius;

        if inside && !self.was_inside {
            self.current_checkpoint += 1;

            if self.current_checkpoint >= self.checkpoints.len() {
                self.current_checkpoint = 0;

                if self.current_lap >= self.total_laps {
                    self.finished = true;
                } else {
                    self.current_lap += 1;
                }
            }
        }

        self.was_inside = inside;
    }

    pub fn current_lap(&self) -> usize {
        self.current_lap
    }

    pub fn total_laps(&self) -> usize {
        self.total_laps
    }

    pub fn current_checkpoint(&self) -> usize {
        self.current_checkpoint
    }

    pub fn checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn active_checkpoint(&self) -> Option<Checkpoint> {
        if self.finished {
            None
        } else {
            self.checkpoints
                .get(self.current_checkpoint)
                .copied()
        }
    }
}
