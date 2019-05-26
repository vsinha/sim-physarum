use cgmath::*;
use rand::Rng;

use crate::trail::TrailLayer;

fn random_vec2(w: f32, h: f32) -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    Vector2::new(rng.gen_range(0., w), rng.gen_range(0., h))
}

fn random_unit_vec() -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    Vector2::normalize(Vector2::new(rng.gen_range(0., 1.), rng.gen_range(0., 1.)))
}

type Radians = f32;

#[derive(Debug)]
pub struct Agent {
    pub position: Vector2<f32>,
    heading: Vector2<f32>,
    sense_angle: Radians,
    sense_distance: f32,
    turn_angle: Radians,
}

fn rotate(vec: &Vector2<f32>, theta: Radians) -> Vector2<f32> {
    let cos = theta.cos();
    let sin = theta.sin();

    Vector2 {
        x: vec.x * cos - vec.y * sin,
        y: vec.x * cos + vec.y * sin,
    }
}

#[test]
fn test_turn_vector() {
    let vec = Vector2::new(1.0, 1.0);
    assert_eq!(
        rotate(&vec, (45.0 as f32).to_radians() as Radians),
        Vector2 {
            x: 0.0,
            y: 1.4142135,
        }
    );
    assert_eq!(
        rotate(&vec, (-45.0 as f32).to_radians() as Radians),
        Vector2 {
            x: 1.4142135,
            y: 0.0,
        }
    );
}

impl Agent {
    pub fn new(width: u32, height: u32) -> Agent {
        Agent {
            position: random_vec2(width as f32, height as f32),
            heading: random_unit_vec(),
            sense_angle: (45.0 as f32).to_radians(),
            sense_distance: 1.0,
            turn_angle: 0.25,
        }
    }

    pub fn update(&mut self, trail: &TrailLayer) {
        println!("before: {:?}", self);

        let sense_heading = Vector2::normalize(self.heading) * self.sense_distance;
        let sense_forward = self.position + sense_heading;
        let sense_left = self.position + rotate(&sense_heading, self.sense_angle);
        let sense_right = self.position + rotate(&sense_heading, -self.sense_angle);
        println!("forward: {:?}", sense_forward);
        println!("left: {:?}", sense_left);
        println!("right: {:?}", sense_right);

        let trail_forward = trail.get(sense_forward.x, sense_forward.y);
        let trail_left = trail.get(sense_left.x, sense_left.y);
        let trail_right = trail.get(sense_right.x, sense_right.y);

        if trail_forward >= trail_left && trail_forward >= trail_right {
            println!("no change");
        } else if trail_left == trail_right {
            println!("turn randomly");
            // TODO this just turns left
            self.heading = rotate(&self.heading, self.turn_angle);
        } else if trail_left >= trail_right {
            println!("left");
            self.heading = rotate(&self.heading, self.turn_angle);
        } else {
            println!("right");
            self.heading = rotate(&self.heading, -self.turn_angle);
        }

        self.position += Vector2::normalize(self.heading);
        println!("after: {:?}", self);
    }
}
