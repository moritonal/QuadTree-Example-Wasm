use nalgebra::{Vector3 };
use bbox::BoundingBox;
use crate::AABB;
use crate::BoidCharacteristics;

#[derive(Debug, PartialEq)]
pub struct Boid {
    pub pos: Vector3<f64>,
    pub target: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Boid {
    pub fn new(pos: Vector3<f64>, target: Vector3<f64>) -> Boid {
        Boid {
            pos,
            target,
            velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, time_scale: f64, world_box: &AABB, boid_characteristics: &BoidCharacteristics) {

        let target_force = self.target - self.pos;
        let target_force = target_force.normalize() * 0.1;

        self.velocity += target_force;
        self.velocity *= boid_characteristics.boid_dampening;

        let next_pos = self.pos + (self.velocity * time_scale);

        if !world_box.intersects(&next_pos) {
            self.velocity *= -1.0;
        } else {
            self.pos = next_pos;
        }
    }

    pub fn set_target(&mut self, target: Vector3<f64>) {
        self.target = target;
    }
}
