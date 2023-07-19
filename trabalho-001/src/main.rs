//**********************************************************************
// Rust Imports
//**********************************************************************
use core::time;
use std::{env, thread};
use std::f32::consts::PI;
use std::path;

//**********************************************************************
// GG:EZ Imports
//**********************************************************************
use ggez::audio::SoundSource;
use ggez::{
    Context, 
    ContextBuilder, 
    GameResult, 
    audio
};
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{
    Canvas, 
    Color, 
    DrawMode, 
    DrawParam, 
    Image, 
    ImageFormat, 
    Mesh, 
    //PxScale, 
    Rect, 
    ScreenImage, 
    Text
};
use ggez::input::keyboard::{KeyCode, KeyInput};



// **********************************************************************
// Utility functions
// **********************************************************************
// Create a unit vector representing the given angle (in radians)
fn vec_from_angle(angle: f32) -> Vec2 {
    let x = angle.sin();
    let y = angle.cos();
    Vec2::new(x, -y)
}



// **********************************************************************
// Player Consts
// **********************************************************************
// Acceleration in pixels per second.
const ROCKET_THRUST: f32 = 30.0;
// Rotation in radians per second.
const ROCKET_TURN_RATE: f32 = 1.2;
// Player Box size
const ROCKET_BBOX: Vec2 = Vec2::new(37.0, 64.0);
const ROCKET_FUEL: f32 = 100.0;
const ROCKET_FUEL_CONSUPTION: f32 = 0.12;

// **********************************************************************
// Game Generic Consts
// **********************************************************************
const DESIRED_FPS: u32 = 60;
const SCREEN_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const MAX_IMPACT_VELOCITY: f32 = 30.0;
const GRAVITY_ACCELERATION: f32 = 3.0;



// **********************************
// Keeps track of the user's input state 
// Turn keyboard events into state-based commands
// **********************************
#[derive(Debug)]
struct InputState {
    xaxis: f32,
    yaxis: f32,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
        }
    }
}

// **********************************************************************
// Player functions
// **********************************************************************
#[derive(Debug)]
struct Player {
    pos: Vec2,
    facing: f32,
    velocity: Vec2,
    fuel: f32,
    rect: Rect,
    //rect_base: Rect
}

// **********************************
// Create PLayer
// **********************************
fn create_player() -> Player {
    Player {
        pos: Vec2::new(100.0, 530.0),
        facing: 0.0,
        velocity: Vec2::ZERO,
        fuel: ROCKET_FUEL,
        // Rect object stays "inside" player sprite to check collisions
        rect: Rect::new(0.0, 0.0, ROCKET_BBOX.x, ROCKET_BBOX.y)
    }
}


// **********************************************************************
// Draw Functions
// **********************************************************************
fn draw_rocket(assets: &mut Assets, canvas: &mut Canvas, actor: &Player) {
    let image = assets.rocket_sprite();

    let drawparams = DrawParam::new()
        .dest(actor.pos)
        .rotation(actor.facing)
        .offset(Vec2::new(0.5, 0.5));

    canvas.draw(image, drawparams);
}


// **********************************
// Rocket Physics
// **********************************
fn player_handle_input(rocket: &mut Player, input: &InputState, dt: f32) {
    // Rocket rotation
    rocket.facing += dt * ROCKET_TURN_RATE * input.xaxis;
    rocket.facing = rocket.facing % (2.0 * PI);
    
    // Rocket thrust
    if (input.yaxis > 0.0) && (rocket.fuel > 0.0) {
        rocket_thrust(rocket, dt);
    }
}

fn rocket_thrust(rocket: &mut Player, dt: f32) {
    let direction_vector = vec_from_angle(rocket.facing);
    let thrust_vector = direction_vector * (ROCKET_THRUST);

    rocket.velocity += thrust_vector * (dt);

    if rocket.fuel > 0.0 {
        rocket.fuel -= ROCKET_FUEL_CONSUPTION;
    }
}

fn update_player_position(rocket: &mut Player, dt: f32) {
    rocket.velocity.y += GRAVITY_ACCELERATION * dt;

    rocket.pos += rocket.velocity * dt;
    
    //Update rect position that stays "inside" the rocket
    rocket.rect.x = rocket.pos.x - (rocket.rect.w / 2.0);
    rocket.rect.y = rocket.pos.y - (rocket.rect.h / 2.0);
}



// **********************************************************************
// Assets Creation
// Structure contain the images, sounds, etc.
// All the file names are hard-coded
// **********************************************************************
struct Assets {
    rocket_sprite: Image,
    hit_sound: audio::Source
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let rocket_sprite = Image::from_path(ctx, "/rocket.png")?;
        let hit_sound = audio::Source::new(ctx, "/boom.ogg")?;

        Ok(Assets {rocket_sprite, hit_sound})
    }

    fn rocket_sprite(&self) -> &Image {
        &self.rocket_sprite
    }
}



struct MyGame {
    screen: ScreenImage,
    player: Player,
    assets: Assets,
    input: InputState,
    rocket_velocity_text: Text,
    rocket_fuel_text: Text,
    ground_rect: Rect,
    game_end: bool,
}

impl MyGame {
    fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let screen = ScreenImage::new(
            ctx, 
            ImageFormat::Rgba8UnormSrgb, 
            1.0, 
            1.0,
            1);
        let player = create_player();
        let assets = Assets::new(ctx)?;
        let rocket_velocity_text = Text::new(format!("{}", 0));
        let rocket_fuel_text= Text::new(format!("{}", ROCKET_FUEL));
        let ground_rect = Rect::new(350.0, 300.0, 100.0, 20.0);
        let game_end: bool = false;

        let s = MyGame {
            screen,
            player,
            assets,
            input: InputState::default(),
            rocket_velocity_text,
            rocket_fuel_text,
            ground_rect,
            game_end,
        };

        Ok(s)
    }


    fn check_collision(&mut self, ctx: &mut ggez::Context) {
        if self.ground_rect.overlaps(&self.player.rect) {
            // Checks impact velocity and rocket facing
            if (self.player.velocity.length() >= MAX_IMPACT_VELOCITY) ||
            ((self.player.facing.abs() > 1.0) && (self.player.facing.abs() < 5.0))
            { 
                let _ = self.assets.hit_sound.play(ctx);
                self.game_end = true;
            }

            // Update physics
            self.player.velocity.y *= -0.15;
            self.player.velocity.x *= 0.99;
            self.player.pos.y = self.ground_rect.y - self.player.rect.h / 2.0;
        }

    }

}

// **********************************************************************
// EventHandler (ggez::event) 
// responsable for updating, drawing game objects,and handling input events.
// **********************************************************************
impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // PRINT PLAYER POSITION
        // println!("PLAYER POS X: {}", self.player.pos.x);
        // println!("PLAYER POS Y: {}", self.player.pos.y);
        // println!("PLAYER FACING ANG: {}", self.player.facing);

        // Deciding when to update the game, and how many times.
        // Run once for each frame fitting in the time since the last update.
        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = GRAVITY_ACCELERATION / (DESIRED_FPS as f32);
            
            // Update the player state based on the user input.
            player_handle_input(&mut self.player, &self.input, seconds);

            // Update the physics for player
            update_player_position(&mut self.player, seconds);

            // Check rocket collision with objects
            self.check_collision(ctx);
            if self.game_end {
                let duration = time::Duration::from_secs(1);
                thread::sleep(duration);
                ctx.request_quit();
            }

            // Update rocket fuel text
            self.rocket_fuel_text = Text::new(format!("{:.2?}", self.player.fuel));

            // Update player velocity
            let mut mag = (self.player.velocity.x.powi(2)) + (self.player.velocity.y.powi(2));
            mag = mag.sqrt();
            self.rocket_velocity_text = Text::new(format!("{:.2}", mag));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Draw Canvas
        let mut canvas = Canvas::from_screen_image(ctx, &mut self.screen, Color::BLUE);

        // **********************************************************************
        // Draw Player
        // **********************************************************************
        draw_rocket(&mut self.assets, &mut canvas, &self.player);



        let object_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.ground_rect,
            Color::WHITE,
        )?;

        // Drawing ground
        let draw_param = DrawParam::default();
        canvas.draw(&object_mesh, draw_param);





        // **********************************************************************
        // Finish Drawing
        // **********************************************************************
        canvas.finish(ctx)?;
        ctx.gfx.present(&self.screen.image(ctx))?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool, ) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.input.yaxis = 1.0;
            }
            Some(KeyCode::Left) => {
                self.input.xaxis = -1.0;
            }
            Some(KeyCode::Right) => {
                self.input.xaxis = 1.0;
            }
            Some(KeyCode::Escape) => ctx.request_quit(),
            _ => (), // Do nothing
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.input.yaxis = 0.0;
            }
            Some(KeyCode::Left) => {
                self.input.xaxis = 0.0;
            }
            Some(KeyCode::Right) => {
                self.input.xaxis = 0.0;
            }
            _ => (), // Do nothing
        }
        Ok(())
    }

}

pub fn main() -> GameResult {
    // Access resource folder
    let resource_dir = 
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");

            path
        } else {
            path::PathBuf::from("./resources")
        };

    // Setup metadata about our game
    let cb = ContextBuilder::new("rocket-game", "Thiago")
        .window_setup(conf::WindowSetup::default()
            .title("Rocket Game"))
        .window_mode(conf::WindowMode::default()
            .dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y))
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let game_state = MyGame::new(&mut ctx)?;

    // Run our game, passing in our context and state.
    event::run(ctx, events_loop, game_state)
}