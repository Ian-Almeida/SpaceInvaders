extern crate piston_window;
extern crate image;

use std::cmp::min;
use piston_window::*;
use crate::entities::{Alien, AlienSpecies, AlienVariant, Cannon, Entity, Shot};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::{HashMap, HashSet};
use rand::Rng;


pub struct Game {
    max_fps: u64,
    window: PistonWindow,
    width: u32,
    height: u32,

    game_score: u32,

    cannon: Cannon,

    is_left_pressed: bool,
    is_right_pressed: bool,

    cannon_shots: Vec<Shot>,
    alien_shots: Vec<Shot>,
    aliens_movement_speed: f64,
    aliens_height_modifier: f64,
}

impl Default for Game {
    fn default() -> Self {
        let width = 640;
        let height = 480;

        let mut window: PistonWindow = WindowSettings::new("Espace Invaders", [width, height])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

        let cannon = Cannon::new(
            &mut window,
            0.0,
            0.0 + height as f64 - 100.0,
            String::from("src/assets/cannon-32x32.png"),
            1.0
        );

        let mut game = Game {
            max_fps: 60,
            window,
            height,
            width,
            game_score: 0,
            cannon,
            is_left_pressed: false,
            is_right_pressed: false,
            cannon_shots: vec![],
            alien_shots: vec![],

            aliens_movement_speed: 2.0,
            aliens_height_modifier: 0.5,
        };
        game.window.set_max_fps(game.max_fps);
        return game;
    }
}

fn get_dummy_aliens(window: &mut PistonWindow,) -> Vec<Vec<Alien>> {
    vec![
        vec![
            Alien::new(
                window,
                64.0,
                64.0,
                1.0,
                AlienSpecies::Soldier,
                AlienVariant::Default,
                (0, 0)

            ),
            Alien::new(
                window,
                96.0,
                64.0,
                1.0,
                AlienSpecies::Soldier,
                AlienVariant::Default,
                (0, 1)
            ),
            Alien::new(
                window,
                576.0,
                64.0,
                1.0,
                AlienSpecies::Soldier,
                AlienVariant::Default,
                (0, 10)
            ),
        ],
        vec![
            Alien::new(
                window,
                64.0,
                128.0,
                1.0,
                AlienSpecies::Soldier,
                AlienVariant::Default,
                (1, 1)
            ),
            Alien::new(
                window,
                576.0,
                128.0,
                1.0,
                AlienSpecies::Soldier,
                AlienVariant::Default,
                (1, 10)
            ),
        ],

    ]
}

fn verify_colision(entity1: &mut Entity, entity2: &mut Entity) -> bool {
    let entity1_size = (entity1.texture.get_size().0 as f64, entity1.texture.get_size().1 as f64);
    let entity2_size = (entity2.texture.get_size().0 as f64, entity2.texture.get_size().1 as f64);

    //Real measurements of the textures
    let entity1_real_x = (entity1.x, entity1.x + (entity1_size.0 * entity1.scale.0));
    let entity1_real_y = (entity1.y, entity1.y + (entity1_size.1 * entity1.scale.1));

    let entity2_real_x = (entity2.x,  entity2.x + (entity2_size.0 * entity2.scale.0) as f64);
    let entity2_real_y = (entity2.y, entity2.y + (entity2_size.1 * entity2.scale.1) as f64);

    // Verifica colisao em x
    if (entity1_real_x.0 >= entity2_real_x.0 && entity1_real_x.0 <= entity2_real_x.1)
        || (entity1_real_x.1 >= entity2_real_x.0 && entity1_real_x.1 <= entity2_real_x.1) {

         // verifica colisao em y
        if (entity1_real_y.0 >= entity2_real_y.0 && entity1_real_y.0 <= entity2_real_y.1)
            || (entity1_real_y.1 >= entity2_real_y.0 && entity1_real_y.1 <= entity2_real_y.1) {
            entity1.colided = true;
            entity2.colided = true;
            return true;
        }

        return false;
    }
    false
}

impl Game {

    pub fn start_game(&mut self) {
        let mut game_over = false;
        let mut movement_value;
        let mut dummy_aliens = get_dummy_aliens(&mut self.window);
        let mut alien_shot_start_time = Instant::now();
        let mut last_aliens: HashMap<u32, (usize, usize)> = HashMap::new();
        let mut rng = rand::thread_rng();

        while let Some(_event) = self.window.next() {
            // Handle events
            self.handle_press_keyboard(&_event);
            self.handle_release_keyboard(&_event);

            // Cannon left move
            if self.is_left_pressed {
                movement_value = -self.cannon.get_movement_speed();
                if (self.cannon.entity.x - self.cannon.get_movement_speed()) < 0.0 {
                    movement_value = 0.0;
                }

                self.cannon.move_x_axis(movement_value);
            }

            // Cannon right move
            if self.is_right_pressed {
                movement_value = self.cannon.get_movement_speed();
                if (self.cannon.entity.x + movement_value) > (self.width as f64 - self.cannon.size.0) {
                    movement_value = 0.0;
                }
                self.cannon.move_x_axis(movement_value);
            }

            // Draw Cannon
            self.window.draw_2d(&_event, |context, graphics, _| {
                clear([1.0; 4], graphics);

                let mut transform = context.transform.trans(self.cannon.entity.x, self.cannon.entity.y);
                image(&self.cannon.entity.texture, transform, graphics);
            });

            // Draw Aliens
            self.window.draw_2d(&_event, |context, graphics, _| {

                for row in dummy_aliens.iter_mut().enumerate() {
                    for alien in row.1.iter_mut().enumerate() {
                        let mut pos = (alien.1.entity.x.clone(), alien.1.entity.y.clone());

                        let mut transform = context.transform.trans(pos.0, pos.1);
                        image(&alien.1.entity.texture, transform, graphics);

                        last_aliens.insert(alien.1.position.1, (row.0, alien.0));

                        alien.1.entity.x += self.aliens_movement_speed;
                    }
                }

                // Verify if the most distant alien is at the maximum x, to go to the other side
                if self.aliens_movement_speed > 0.0 {
                    let max_alien = dummy_aliens.iter()
                        .flat_map(|row| row.iter())
                        .max_by(|a, b| a.entity.x.partial_cmp(&b.entity.x).unwrap());

                    match max_alien {
                        Some(distant_alien) => {
                            if distant_alien.entity.x + distant_alien.size.0 + self.aliens_movement_speed >= self.width as f64 {
                                self.aliens_movement_speed = 0.0 - self.aliens_movement_speed;

                                for row in dummy_aliens.iter_mut().enumerate() {
                                    for alien in row.1.iter_mut().enumerate() {
                                        alien.1.entity.y = alien.1.entity.y + (self.aliens_height_modifier * alien.1.size.1);
                                    }
                                }
                            }
                        },
                        None => {  },
                    }
                }

                // Verify if the origin closest alien is at the minimum x, to go to the other side
                if self.aliens_movement_speed < 0.0 {
                    let min_alien = dummy_aliens.iter()
                        .flat_map(|row| row.iter())
                        .min_by(|a, b| a.entity.x.partial_cmp(&b.entity.x).unwrap());

                    match min_alien {
                        Some(closer_alien) => {
                            if closer_alien.entity.x - self.aliens_movement_speed <= 0.0 {
                                self.aliens_movement_speed = 0.0 - self.aliens_movement_speed;

                                for row in dummy_aliens.iter_mut().enumerate() {
                                    for alien in row.1.iter_mut().enumerate() {
                                        alien.1.entity.y = alien.1.entity.y + (self.aliens_height_modifier * alien.1.size.1);
                                    }
                                }
                            }
                        },
                        None => {  },
                    }
                }
            });

            // Create alien shoots
            let alien_shot_end_time = Instant::now();
            let elapsed_time = alien_shot_end_time.duration_since(alien_shot_start_time).as_secs_f64();
            if elapsed_time >= 1.0 && last_aliens.len() > 0{
                let mut values: Vec<&(usize, usize)> = vec![];

                for value in last_aliens.values() {
                    values.push(value);
                }
                let random_num = rng.gen_range(0..last_aliens.len());

                let alien_pos = values[random_num];
                let mut shooter_alien= dummy_aliens.get_mut(alien_pos.0).expect("no row").get_mut(alien_pos.1).expect("no col");
                self.alien_shots.push(shooter_alien.shoot(&mut self.window));
                alien_shot_start_time = Instant::now();
            }

            // Draw shots
            self.window.draw_2d(&_event, |context, graphics, _| {

                // Draw cannon shots
                for shot in self.cannon_shots.iter_mut() {
                    let mut transform = context.transform.trans(shot.entity.x, shot.entity.y).scale(shot.entity.scale.0, shot.entity.scale.1);

                    image(&shot.entity.texture, transform, graphics);
                    shot.entity.y += shot.entity.movement_speed;

                    // Colision with aliens
                    for row in dummy_aliens.iter_mut() {
                        let old_row = row.clone();
                        let before_colision_len = row.len();

                        // Remove collided
                        row.retain_mut(|alien| !verify_colision(&mut shot.entity, &mut alien.entity));

                        let after_colision_len = row.len();

                        // Filter the collided aliens
                        let collided_aliens = old_row
                            .iter()
                            .filter(|x| !row.iter().any(|y| y.position.0 == x.position.0 && y.position.1 == x.position.1))
                            .collect::<Vec<_>>();

                        // Add the score of each collided alien to the game score
                        for alien in collided_aliens {
                            self.game_score += alien.score;
                        }

                        // Increase alien speed when one is killed
                        let diff = before_colision_len - after_colision_len;

                        // TODO define constant to 0.2
                        if self.aliens_movement_speed > 0.0 {self.aliens_movement_speed = self.aliens_movement_speed + (0.2  * diff as f64);}
                        else {self.aliens_movement_speed = self.aliens_movement_speed + (-0.2 * diff as f64);}
                    }

                    // Colision with other shots
                    for alien_shot in self.alien_shots.iter_mut() {
                        verify_colision(&mut shot.entity, &mut alien_shot.entity);
                    }
                }

                // Draw alien shots
                for shot in self.alien_shots.iter_mut() {
                    let mut transform = context.transform.trans(shot.entity.x, shot.entity.y).scale(shot.entity.scale.0, shot.entity.scale.1);

                    image(&shot.entity.texture, transform, graphics);
                    shot.entity.y += shot.entity.movement_speed;

                    // Colision with the cannon
                    let colided = verify_colision(&mut shot.entity, &mut self.cannon.entity);
                    if colided {
                        self.cannon.life -= 1;
                    }
                }
            });

            // Drop shots outside the window
            self.cannon_shots.retain(|shot| (shot.entity.y + shot.size.1 > 0.0 && !shot.entity.colided));
            self.alien_shots.retain(|shot| (shot.entity.y + shot.size.1 <= self.height as f64) && !shot.entity.colided);
            last_aliens = HashMap::new();

            if self.cannon.life <= 0 {
                game_over = true
            }

            //Game over if aliens get to the height of the cannon
            for row in dummy_aliens.iter().rev() {
                let first_alien = row.get(0);

                if first_alien.is_some() {
                    if first_alien.unwrap().entity.y + first_alien.unwrap().size.1 > self.cannon.entity.y {
                        game_over = true
                    }
                    break
                }
            }

            // End application
            if game_over {
                break
            }

        }
    }

    fn handle_press_keyboard(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Right => {
                    // handle right
                    self.is_right_pressed = true;
                },
                Key::Left => {
                    // handle left
                    self.is_left_pressed = true;
                },
                Key::R => {
                    // R (restart)
                    self.start_game();
                },
                Key::Space => {
                    // handle space
                    self.cannon_shots.push(self.cannon.shoot(&mut self.window));
                },
                _ => {}
            }
        }
    }

    fn handle_release_keyboard(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Right => {
                    // handle right
                    self.is_right_pressed = false;
                },
                Key::Left => {
                    // handle left
                    self.is_left_pressed = false;
                },
                Key::Space => {
                    // handle space
                    println!("Space release")
                },
                _ => {}
            }
        }
    }
}