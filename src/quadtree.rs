use nalgebra::Vector3;
use std::{cell::RefCell, rc::Rc, vec::Vec};

const PER_QUAD_CAPACITY: usize = 1;

use super::Boid;

#[derive(PartialEq, Clone)]
pub struct Quadtree {
    capacity: usize,
    pub boundary: AABB,
    points: Vec<Rc<RefCell<Boid>>>,
    pub children: Vec<Quadtree>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl AABB {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> AABB {
        AABB { min, max }
    }

    pub fn intersects(&self, other: &Vector3<f64>) -> bool {
        self.min.x <= other.x
            && self.min.y <= other.y
            && self.max.x >= other.x
            && self.max.y >= other.y
    }

    pub fn intersects_aabb(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && self.max.x >= other.min.x
            && self.max.y >= other.min.y
    }
}

impl<'a> Quadtree {
    pub fn new(boundary: AABB) -> Quadtree {
        Quadtree {
            capacity: PER_QUAD_CAPACITY,
            boundary,
            points: Vec::with_capacity(PER_QUAD_CAPACITY),
            children: Vec::with_capacity(4),
        }
    }

    pub fn insert(&mut self, boid: Rc<RefCell<Boid>>) -> bool {
        if self.boundary.intersects(&boid.borrow().pos) {

            if self.children.len() != 0 {
                // Then try to add the point to one of our children
                for child in &mut self.children {
                    // Try to add the point to the child
                    if child.insert(boid.clone()) {
                        // If we were successful, return true
                        return true;
                    }
                }
            } else {
                // If we have space to contain it
                if self.points.len() < self.capacity {
                    // Add the point to our list
                    self.points.push(boid);

                    return true;
                } else {
                    //Otherwise, if we currently have no children, subdivide
                    if self.children.len() == 0 {
                        self.subdivide();
                    }

                    // Then try to add the point to one of our children
                    for child in &mut self.children {
                        // Try to add the point to the child
                        if child.insert(boid.clone()) {
                            // If we were successful, return true
                            return true;
                        }
                    }
                }
            }

            return true;
        } else {
            return false;
        }
    }

    fn subdivide(&mut self) {
        let per_new_part = (self.boundary.max - self.boundary.min) / 2.0;
        let per_quadrant = Vector3::new(per_new_part.x, per_new_part.y, 1.0);

        for x in 0..2 {
            for y in 0..2 {
                let offset =
                    Vector3::new(x as f64 * per_new_part.x, y as f64 * per_new_part.y, 0.0);

                let min = self.boundary.min + offset;
                let max = min + per_quadrant;

                let branch = Quadtree::new(AABB { min, max });

                self.children.push(branch);
            }
        }

        // Move the points into the children
        for point in &mut self.points {
            for child in &mut self.children {
                if child.insert(point.clone()) {
                    break;
                }
            }
        }

        self.points.clear();
    }

    pub fn query_range(&self, bx: AABB) -> Vec<Rc<RefCell<Boid>>> {
        let mut ret_points: Vec<Rc<RefCell<Boid>>> = Vec::new();

        self.query_range_internal(&mut ret_points, bx);

        return ret_points;
    }

    pub fn query_range_quads(&self, bx: AABB) -> Vec<AABB> {
        let mut ret_points: Vec<AABB> = Vec::new();

        self.query_range_internal_quads(&mut ret_points, bx);

        return ret_points;
    }

    fn query_range_internal_quads(&self, ret_points: &mut Vec<AABB>, bx: AABB) {
        // If the point is within our boundary
        if self.boundary.intersects_aabb(&bx) {
            ret_points.push(self.boundary);

            for child in &self.children {
                child.query_range_internal_quads(ret_points, bx);
            }
        }
    }

    fn query_range_internal(&self, ret_points: &mut Vec<Rc<RefCell<Boid>>>, bx: AABB) {
        // If the point is within our boundary
        if self.boundary.intersects_aabb(&bx) {
            // If we are a leaf
            if self.children.len() == 0 {
                // Loop over every point we know of
                for point in &self.points {
                    ret_points.push(point.clone());
                }
            } else {
                for child in &self.children {
                    child.query_range_internal(ret_points, bx);
                }
            }
        }
    }

    // pub fn remove(&mut self, boid: &Rc<RefCell<Boid>>) {
    //     if let Some(position) = self.points.iter().position(|p| p == boid) {
    //         self.points.remove(position);
    //     }
    // }

    // pub fn true_parent(&self) -> &Quadtree {
    //     if let Some(parent) = &self.parent {
    //         parent.true_parent()
    //     } else {
    //         self
    //     }
    // }

    // pub fn update(&mut self) {
    //     for child in &mut self.children {
    //         child.update();
    //     }
    // }

    pub fn clear(&mut self) {
        self.points.clear();
        self.children.clear();
    }
}
