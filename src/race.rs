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

    // Tiempo total desde que empieza la carrera.
    race_time: f32,

    // Tiempo de la vuelta actual.
    lap_time: f32,

    // Última vuelta completada.
    last_lap_time: Option<f32>,

    // Mejor vuelta registrada.
    best_lap_time: Option<f32>,
}

impl Race {
    pub fn new() -> Self {
        Self {
            checkpoints: vec![
                Checkpoint {
                    x: 6.0,
                    y: 34.0,
                    radius: 1.8,
                },
                Checkpoint {
                    x: 18.0,
                    y: 34.0,
                    radius: 2.3,
                },
                Checkpoint {
                    x: 32.0,
                    y: 34.0,
                    radius: 2.3,
                },
                Checkpoint {
                    x: 48.0,
                    y: 34.0,
                    radius: 2.3,
                },
                Checkpoint {
                    x: 57.0,
                    y: 31.0,
                    radius: 2.3,
                },
                Checkpoint {
                    x: 57.0,
                    y: 21.0,
                    radius: 2.4,
                },
                Checkpoint {
                    x: 57.0,
                    y: 7.0,
                    radius: 2.4,
                },
                Checkpoint {
                    x: 44.0,
                    y: 6.0,
                    radius: 2.3,
                },
                Checkpoint {
                    x: 32.0,
                    y: 6.0,
                    radius: 2.5,
                },
                Checkpoint {
                    x: 18.0,
                    y: 7.0,
                    radius: 2.3,
                },
                Checkpoint {
                    x: 7.0,
                    y: 10.0,
                    radius: 2.4,
                },
                Checkpoint {
                    x: 7.0,
                    y: 21.0,
                    radius: 2.4,
                },
                Checkpoint {
                    x: 7.0,
                    y: 30.0,
                    radius: 2.4,
                },
            ],

            current_checkpoint: 0,

            current_lap: 1,
            total_laps: 3,

            finished: false,
            was_inside: false,

            race_time: 0.0,
            lap_time: 0.0,
            last_lap_time: None,
            best_lap_time: None,
        }
    }

    pub fn update(
        &mut self,
        player: &Player,
        dt: f32,
    ) {
        if self.finished
            || self.checkpoints.is_empty()
        {
            return;
        }

        self.race_time += dt;
        self.lap_time += dt;

        let checkpoint =
            self.checkpoints[
                self.current_checkpoint
            ];

        let dx =
            player.x
            - checkpoint.x;

        let dy =
            player.y
            - checkpoint.y;

        let inside =
            dx * dx + dy * dy
            <= checkpoint.radius
                * checkpoint.radius;

        if inside && !self.was_inside {
            self.advance_checkpoint();
        }

        self.was_inside = inside;
    }

    fn advance_checkpoint(
        &mut self,
    ) {
        self.current_checkpoint += 1;

        if self.current_checkpoint
            < self.checkpoints.len()
        {
            return;
        }

        // Terminó una vuelta.
        self.current_checkpoint = 0;

        let completed_lap_time =
            self.lap_time;

        self.last_lap_time =
            Some(completed_lap_time);

        match self.best_lap_time {
            Some(best) => {
                if completed_lap_time < best {
                    self.best_lap_time =
                        Some(completed_lap_time);
                }
            }

            None => {
                self.best_lap_time =
                    Some(completed_lap_time);
            }
        }

        self.lap_time = 0.0;

        if self.current_lap
            >= self.total_laps
        {
            self.finished = true;
        } else {
            self.current_lap += 1;
        }
    }

    pub fn current_lap(
        &self,
    ) -> usize {
        self.current_lap
    }

    pub fn total_laps(
        &self,
    ) -> usize {
        self.total_laps
    }

    pub fn current_checkpoint(
        &self,
    ) -> usize {
        self.current_checkpoint
    }

    pub fn checkpoint_count(
        &self,
    ) -> usize {
        self.checkpoints.len()
    }

    pub fn finished(
        &self,
    ) -> bool {
        self.finished
    }

    pub fn active_checkpoint(
        &self,
    ) -> Option<Checkpoint> {
        if self.finished {
            None
        } else {
            self.checkpoints
                .get(
                    self.current_checkpoint
                )
                .copied()
        }
    }

    pub fn race_time(
        &self,
    ) -> f32 {
        self.race_time
    }

    pub fn lap_time(
        &self,
    ) -> f32 {
        self.lap_time
    }

    pub fn last_lap_time(
        &self,
    ) -> Option<f32> {
        self.last_lap_time
    }

    pub fn best_lap_time(
        &self,
    ) -> Option<f32> {
        self.best_lap_time
    }
}
