use super::math::Vector2d;
use sdl2::rect::Rect;
use serde_derive::{Serialize, Deserialize};

type FloatVector = Vector2d<f32>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BoxShape {
    // dimension
    pub w: u32,
    pub h: u32,
    // mass
    m: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BodyType {
    Static,
    Dynamic
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RigidBody {
    pub position: FloatVector,
    pub velocity: FloatVector,
    pub shape: BoxShape,

    pub body_type: BodyType,
    // Forces applied to the component.
    pub force: FloatVector,
}

impl RigidBody {
    pub fn new(x: f32, y: f32, w: u32, h: u32, body_type: BodyType) -> RigidBody {
        let position = FloatVector::new(x, y);
        let shape = BoxShape { w, h, m: 2 };
        let velocity = FloatVector::new(0.0, 0.0);
        let force = FloatVector::new(0.0, 0.0);

        RigidBody {
            position,
            velocity,
            body_type,
            shape,
            force,
        }
    }
}

pub struct PhysicSystem { 
    
}

impl PhysicSystem {

    pub fn update(&self, mut bodies: Vec<&mut RigidBody>, dt: u32) {

        for b in &mut bodies {
            // ma = f
            if b.body_type == BodyType::Dynamic {
                let ay = 9.81 / (b.shape.m as f32);
                (*b).velocity.y += ay * (dt as f32)/1000.0;
                (*b).position.y += (*b).velocity.y * (dt as f32)/1000.0;
            }
        }


        let mut to_resolve = Vec::new();
        // quick and dirty collision.
        if bodies.len() > 1 {
        for i in 0..bodies.len()-1 {
            for j in (i+1)..bodies.len() {
                let rect1 = Rect::new(bodies[i].position.x as i32,
                                      bodies[i].position.y as i32,
                                      bodies[i].shape.w,
                                      bodies[i].shape.h);
                let rect2 = Rect::new(bodies[j].position.x as i32,
                                      bodies[j].position.y as i32,
                                      bodies[j].shape.w,
                                      bodies[j].shape.h);

                if rect1.has_intersection(rect2) {
                    println!("Intersect!");
                    to_resolve.push((i, j));
                }
            }
        }
        }

        for (i, j) in to_resolve {
            
        }
    }
}


