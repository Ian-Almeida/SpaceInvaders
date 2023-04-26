extern crate piston_window;
extern crate image;

// use image::DynamicImage;
use piston_window::*;

#[derive(Clone, Debug)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    // drawing_path: String,
    // entity_image: DynamicImage,
    pub texture: Texture<gfx_device_gl::Resources>,
    pub movement_speed: f64,
    pub colided: bool,
    pub scale: (f64, f64),
}

impl Entity {
    pub fn new(window: &mut PistonWindow, x: f64, y: f64, drawing_path: String, movement_speed: f64, scale: Option<(f64, f64)>) -> Entity {
        let img = image::open(drawing_path.clone()).unwrap();
        Entity {
            x,
            y,
            // drawing_path: drawing_path.clone(),
            // entity_image: img.clone(),
            texture: Texture::from_image(
                &mut window.create_texture_context(),
                &img.to_rgba8(),
                &TextureSettings::new()
            ).unwrap(),
            movement_speed,
            colided: false,
            scale: scale.unwrap_or((1.0, 1.0))
        }
    }
}

pub struct Cannon {
    pub entity: Entity,
    pub size: (f64, f64),
    pub life: i32,
}

impl Cannon {
    pub fn new(window: &mut PistonWindow, x: f64, y: f64, drawing_path: String, movement_speed: f64) -> Cannon {
        Cannon {
            entity: Entity::new(window, x, y, drawing_path, movement_speed, None),
            size: (32.0, 32.0),
            life: 5,
        }
    }

    pub fn move_x_axis(&mut self, value: f64) {
        self.entity.x += value;
    }

    pub fn get_movement_speed(&self) -> f64 {
        self.entity.movement_speed
    }

    pub fn shoot(&mut self, window: &mut PistonWindow) -> Shot {
        Shot::new(
            window,
            self.entity.x + (self.size.0 * 0.40),
            self.entity.y - ((self.size.1 * 0.20) * self.entity.scale.1),
            String::from("src/assets/cannon-ball-18x18.png"),
            -5.0,
            (18.0, 18.0),
            (1.0/2.0, 1.0/2.0)
        )
    }
}

#[derive(Clone, Debug)]
pub struct Shot {
    pub entity: Entity,
    pub size: (f64, f64),
}

impl Shot {
    pub fn new(window: &mut PistonWindow, x: f64, y: f64, drawing_path: String, movement_speed: f64, size: (f64, f64), scale: (f64, f64)) -> Shot {
        Shot {
            entity: Entity::new(window, x, y, drawing_path, movement_speed, Some(scale)),
            size,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Alien {
    pub entity: Entity,
    pub size: (f64, f64),
    variant: AlienVariant,
    species: AlienSpecies,
    pub position: (u32, u32), // row, col
    pub score: u32,
}
#[derive(Clone, Debug)]
pub enum AlienSpecies {
    Soldier,
    Bug,
    Squid
}
#[derive(Clone, Debug)]
pub enum AlienVariant {
    Default,
    Blue,
    Pink,
    White
}

impl Alien {

    pub fn new(window: &mut PistonWindow, x: f64, y: f64, movement_speed: f64, species: AlienSpecies, variant: AlienVariant, position: (u32, u32)) -> Alien {
        let all_species = ("bug-alien-", "soldier-alien-", "squid-alien-");
        let all_variants = ("white-", "pink-", "blue-");

        let mut drawing_path = String::from("src/assets/");
        let mut score: u32;
        match species {
            AlienSpecies::Bug => {
                drawing_path = format!("{}{}", drawing_path, all_species.0);
                score = 10;
            },
            AlienSpecies::Soldier => {
                drawing_path = format!("{}{}", drawing_path, all_species.1);
                score = 20;
            },
            AlienSpecies::Squid => {
                drawing_path = format!("{}{}", drawing_path, all_species.2);
                score = 30;
            },
        }

        match variant {
            AlienVariant::White => {
                drawing_path = format!("{}{}", drawing_path, all_variants.0);
            },
            AlienVariant::Pink => {
                drawing_path = format!("{}{}", drawing_path, all_variants.1);
                score = score * 2
            },
            AlienVariant::Blue => {
                drawing_path = format!("{}{}", drawing_path, all_variants.2);
                score = score * 3
            },
            _ => {}
        }

        drawing_path = format!("{}{}", drawing_path, "32x32.png");

        Alien {
            entity: Entity::new(window, x, y, drawing_path, movement_speed, None),
            size: (32.0, 32.0),
            variant,
            species,
            position,
            score
        }
    }

    pub fn shoot(&mut self, window: &mut PistonWindow) -> Shot {
        Shot::new(
            window,
            self.entity.x + (self.size.0 * 0.4),
            self.entity.y + self.size.1,
            String::from("src/assets/alien-shot.png"),
            3.0,
            (11.0, 15.0),
            (3.0/5.0, 3.0/5.0)
        )
    }
}

