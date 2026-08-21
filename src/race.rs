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

    event_text: Option<&'static str>,
    event_timer: f32,
}

impl Race {
    pub fn new(track_index: usize) -> Self {
        Self {
            checkpoints: checkpoints(track_index),

            current_checkpoint: 0,

            current_lap: 1,
            total_laps: 3,

            finished: false,
            was_inside: false,

            race_time: 0.0,
            lap_time: 0.0,
            last_lap_time: None,
            best_lap_time: None,

            event_text: None,
            event_timer: 0.0,
        }
    }

    pub fn current_lap(&self) -> usize {
        self.current_lap
    }
}

fn checkpoints(track_index: usize) -> Vec<Checkpoint> {
    match track_index % 2 {
        1 => vec![
            Checkpoint {
                x: 25.0,
                y: 34.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 39.0,
                y: 34.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 55.5,
                y: 30.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 55.5,
                y: 20.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 50.0,
                y: 12.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 41.0,
                y: 12.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 29.0,
                y: 19.5,
                radius: 3.0,
            },
            Checkpoint {
                x: 18.0,
                y: 15.5,
                radius: 3.0,
            },
            Checkpoint {
                x: 8.0,
                y: 12.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 8.0,
                y: 25.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 22.0,
                y: 28.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 22.5,
                y: 33.9,
                radius: 1.15,
            },
        ],

        _ => vec![
            Checkpoint {
                x: 29.0,
                y: 34.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 34.0,
                y: 34.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 49.0,
                y: 34.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 57.0,
                y: 31.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 57.0,
                y: 22.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 57.0,
                y: 13.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 52.0,
                y: 7.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 39.0,
                y: 7.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 26.0,
                y: 7.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 12.0,
                y: 7.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 8.0,
                y: 14.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 8.0,
                y: 24.0,
                radius: 3.2,
            },
            Checkpoint {
                x: 8.0,
                y: 31.0,
                radius: 3.0,
            },
            Checkpoint {
                x: 22.5,
                y: 33.9,
                radius: 1.15,
            },
        ],
    }
}

impl Race {
    pub fn update(&mut self, player: &Player, dt: f32) {
        if self.finished || self.checkpoints.is_empty() {
            return;
        }

        self.race_time += dt;
        self.lap_time += dt;

        if self.event_timer > 0.0 {
            self.event_timer = (self.event_timer - dt).max(0.0);

            if self.event_timer == 0.0 {
                self.event_text = None;
            }
        }

        let checkpoint = self.checkpoints[self.current_checkpoint];

        let dx = player.x - checkpoint.x;

        let dy = player.y - checkpoint.y;

        let inside = dx * dx + dy * dy <= checkpoint.radius * checkpoint.radius;

        if inside && !self.was_inside {
            self.advance_checkpoint();
        }

        self.was_inside = inside;
    }

    fn advance_checkpoint(&mut self) {
        self.current_checkpoint += 1;

        if self.current_checkpoint < self.checkpoints.len() {
            self.set_event("CHECKPOINT!", 0.75);

            return;
        }

        // Terminó una vuelta.
        self.current_checkpoint = 0;

        let completed_lap_time = self.lap_time;

        self.last_lap_time = Some(completed_lap_time);

        match self.best_lap_time {
            Some(best) => {
                if completed_lap_time < best {
                    self.best_lap_time = Some(completed_lap_time);
                }
            }

            None => {
                self.best_lap_time = Some(completed_lap_time);
            }
        }

        self.lap_time = 0.0;

        if self.current_lap >= self.total_laps {
            self.finished = true;

            self.set_event("FINISH!", 2.5);
        } else {
            self.current_lap += 1;

            if self.current_lap == self.total_laps {
                self.set_event("ULTIMA VUELTA!", 1.4);
            } else {
                self.set_event("VUELTA COMPLETA!", 1.2);
            }
        }
    }

    fn set_event(&mut self, text: &'static str, duration: f32) {
        self.event_text = Some(text);

        self.event_timer = duration;
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

    pub fn checkpoints(&self) -> &[Checkpoint] {
        &self.checkpoints
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn active_checkpoint(&self) -> Option<Checkpoint> {
        if self.finished {
            None
        } else {
            self.checkpoints.get(self.current_checkpoint).copied()
        }
    }

    pub fn active_checkpoint_label(&self) -> &'static str {
        if self.current_checkpoint + 1 == self.checkpoints.len() {
            "META"
        } else {
            "CHECKPOINT"
        }
    }

    pub fn active_checkpoint_color(&self) -> raylib::prelude::Color {
        if self.current_checkpoint + 1 == self.checkpoints.len() {
            raylib::prelude::Color::new(255, 80, 60, 255)
        } else {
            raylib::prelude::Color::new(255, 230, 70, 255)
        }
    }

    pub fn race_time(&self) -> f32 {
        self.race_time
    }

    pub fn lap_time(&self) -> f32 {
        self.lap_time
    }

    pub fn last_lap_time(&self) -> Option<f32> {
        self.last_lap_time
    }

    pub fn best_lap_time(&self) -> Option<f32> {
        self.best_lap_time
    }

    pub fn event_text(&self) -> Option<&'static str> {
        self.event_text
    }

    pub fn event_timer(&self) -> f32 {
        self.event_timer
    }
}
