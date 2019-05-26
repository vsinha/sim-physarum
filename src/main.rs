use cgmath::*;
use ggez::conf::*;
use ggez::event::{self, EventHandler, EventsLoop, KeyCode, KeyMods};
use ggez::*;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

mod trail;
use crate::trail::TrailLayer;

mod agent;
use crate::agent::*;

static HEIGHT: u32 = 400;
static WIDTH: u32 = 600;

fn window_setup() -> (Context, EventsLoop) {
    ContextBuilder::new("simthing", "Ty Overby | Viraj Sinha")
        .window_mode(WindowMode::default().dimensions(WIDTH as f32, HEIGHT as f32))
        .window_setup(WindowSetup::default().title("simthing"))
        .build()
        .unwrap()
}

fn main() {
    let (mut ctx, mut event_loop) = window_setup();
    let mut my_game = MyGame::new(&mut ctx).unwrap();

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    circle_mesh: ggez::graphics::Mesh,
    target_mesh: ggez::graphics::Mesh,
    trail_mesh: ggez::graphics::Mesh,
    camera_pos: Vector3<f32>,
    agents: Vec<Agent>,
    trail: TrailLayer,
}

impl MyGame {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        use ggez::graphics::{Color, DrawMode, FillOptions, Mesh};
        let circle_mesh = Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            [0.0, 0.0],
            5.0,
            0.1,
            Color::from_rgb(200, 100, 0),
        )?;

        let trail_mesh = Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            [0.0, 0.0],
            2.0,
            0.1,
            Color::from_rgb(80, 80, 80),
        )?;

        let target_mesh = Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            [0.0, 0.0],
            1.0,
            0.1,
            Color::from_rgb(40, 40, 40),
        )?;

        let mut trail = TrailLayer::new(WIDTH as usize, HEIGHT as usize);
        trail.randomize();

        let agents = (0..1).map(|_i| Agent::new(WIDTH, HEIGHT));

        Ok(MyGame {
            circle_mesh,
            target_mesh,
            trail_mesh,
            camera_pos: Vector3::new(0.0, 0.0, 0.0),
            agents: agents.collect(),
            trail,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for agent in &mut self.agents {
            // update movement
            agent.update(&self.trail);

            // deposit
            self.trail
                .set(agent.position.y as usize, agent.position.x as usize, 1.0);

            // diffuse
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 0.00].into());

        let transform = Matrix4::from_translation(self.camera_pos);

        graphics::push_transform(ctx, Some(transform));
        graphics::apply_transformations(ctx)?;

        for agent in &self.agents {
            graphics::draw(
                ctx,
                &self.circle_mesh,
                graphics::DrawParam::default().dest(Point2::from_vec(agent.position)),
            )?;
        }

        let trail_image = graphics::Image::from_rgba8(
            ctx,
            self.trail.num_columns as u16,
            self.trail.num_rows as u16,
            &self.trail.as_rgba8(),
        )?;

        println!("{:?}", trail_image);

        use ggez::graphics::Drawable;
        trail_image.draw(ctx, graphics::DrawParam::default())?;

        println!("FPS: {}", timer::fps(ctx));
        graphics::present(ctx)?;
        graphics::pop_transform(ctx);
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        let vector = match keycode {
            KeyCode::W => Vector3::new(0.0, 1.0, 0.0),
            KeyCode::S => Vector3::new(0.0, -1.0, 0.0),
            KeyCode::A => Vector3::new(1.0, 0.0, 0.0),
            KeyCode::D => Vector3::new(-1.0, 0.0, 0.0),
            _ => Vector3::new(0.0, 0.0, 0.0),
        };
        let vector = vector * 5.0;
        self.camera_pos += vector;
    }
}
