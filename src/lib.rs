use nalgebra::Vector3;

use rand::Rng;
use std::rc::Rc;
use std::{cell::RefCell, f64};
use wasm_bindgen::prelude::*;
use web_sys::{console, Element};

use serde::{Deserialize, Serialize};

use serde_wasm_bindgen::from_value;

mod boid;
mod quadtree;

use boid::Boid;
use quadtree::{Quadtree, AABB};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A global reference to the canvas element
static BOIDS_COUNT: u32 = 50;
static SETUP_BOUNDARY_OFFSET: u32 = 10;

trait Setupable {
    fn setup(&mut self, width: u32, height: u32);
}

trait Renderable {
    fn draw(
        &mut self,
        context: &web_sys::CanvasRenderingContext2d,
        canvas: &web_sys::HtmlCanvasElement,
    );
}

// trait for mouse events
trait MouseEvents {
    fn on_input_drag(&mut self, position: Vec<Vector3<f64>>);
    fn on_input_move(&mut self, positions: Vec<Vector3<f64>>);
    fn on_input_stop(&mut self);
}

#[derive(Serialize)]
struct QuadTreeDemoStats {
    frame_rate: i32,
    average_compare_against: f64,
}

trait Updateable {
    fn update(&mut self, time_scale: f64);
}

pub struct BoidCharacteristics {
    pub boid_dampening: f64
}

pub struct World {
    boids: Vec<Rc<RefCell<Boid>>>,
    quad_tree: Quadtree,
    width: u32,
    height: u32,

    current_user_targets: Vec<Vector3<f64>>,

    average_compare_against: Vec<f64>,
    boid_characteristics: BoidCharacteristics,
    desired_boid_count: u32,

    selected_line_alpha: f64,
    unselected_lines_alpha: f64,
    selected_quads: f64,
    unselected_quads: f64,
    lookup_alpha: f64,
    lookup_radius: f64,
    boid_size: f64,
    lookup_method: String
}

static mut GLOBAL_DAMPNING: f64 = 0.0;

#[wasm_bindgen]
pub fn set_dampning(value: f64) {
    unsafe {
        GLOBAL_DAMPNING = value;
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct WasmInputEvent {
    event_type: String,
    data: String,
}


fn generate_quad_tree(width: u32, height: u32) -> Quadtree {
    return Quadtree::new(AABB::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(width as f64, height as f64, 0.0),
    ));
}

// This will be exported to JavaScript.
#[wasm_bindgen]
pub fn raise_event(event_name: &str, event_data: &str) {
    // You can perform any necessary operations here.
    // For demonstration purposes, we will just raise an event.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let event = web_sys::CustomEvent::new_with_event_init_dict(
        event_name,
        web_sys::CustomEventInit::new().detail(&JsValue::from_str(event_data)),
    )
    .unwrap();
    document
        .dispatch_event(&event)
        .expect("Failed to dispatch event");
}

impl World {
    // constructor
    pub fn new(width: u32, height: u32) -> World {
        World {
            boids: Vec::new(),
            quad_tree: generate_quad_tree(width, height),
            width: width,
            height: height,
            current_user_targets: Vec::new(),
            average_compare_against: Vec::new(),
            boid_characteristics: BoidCharacteristics {
                boid_dampening: 0.955
            },
            desired_boid_count: BOIDS_COUNT,

            selected_line_alpha: 0.0,
            unselected_lines_alpha: 0.0,
            selected_quads: 0.0,
            unselected_quads: 0.0,

            lookup_alpha: 0.0,
            lookup_radius: 0.0,
            boid_size: 0.0,
            lookup_method: String::from("quadtree")
        }
    }

    fn update_quad_tree(&mut self) {
        // clear the quad tree
        self.quad_tree.clear();

        // insert all boids into the quad tree
        for boid in &self.boids {
            self.quad_tree.insert(boid.clone());
        }
    }

    fn set_random_targets_for_all_boids(&mut self) {
        let mut rng = rand::thread_rng();

        for boid in &self.boids {

            if boid == &self.boids[0] {
                continue;
            }

            let random_x = rng.gen_range(0.0..self.width as f64);
            let random_y = rng.gen_range(0.0..self.height as f64);

            boid.borrow_mut()
                .set_target(Vector3::new(random_x, random_y, 0.0));
        }
    }

    fn add_boid(&mut self) {

        let mut rng = rand::thread_rng();

        let offset = SETUP_BOUNDARY_OFFSET;

        // get random in screen width
        let random_x = rng.gen_range(offset as f64 .. (self.width-offset) as f64);
        let random_y = rng.gen_range(offset as f64 .. (self.height-offset) as f64);

        let random_dx = rng.gen_range(offset as f64 .. (self.width-offset) as f64);
        let random_dy = rng.gen_range(offset as f64 .. (self.height-offset) as f64);

        let boid = Rc::new(RefCell::new(Boid::new(
            Vector3::new(random_x, random_y, 0.0),
            Vector3::new( random_dx, random_dy, 0.0),
        )));

        self.boids.push(boid.clone());
    }

    fn on_custom_event(&mut self, event: web_sys::Event) {
        let event = event.dyn_ref::<web_sys::CustomEvent>().unwrap();
        let detail = event.detail();

        let result: Result<WasmInputEvent, _> = from_value(detail);

        //let result:Result<WasmInputEvent, _> = detail.into_serde().unwrap_or_else(|_| panic!("Failed to deserialize"));

        match result {
            Ok(event) => {

                //console::log_1(&format!("Custom event: {:?}", event).into());

                // handle event "set_dampening"
                if event.event_type == "set_dampening" {
                    let value: f64 = event.data.parse().unwrap();

                    self.boid_characteristics.boid_dampening = value;
                }
                else if  event.event_type == "set_boid_count" {
                    let value: u32 = event.data.parse().unwrap();

                    self.desired_boid_count = value;
                }
                else if event.event_type == "set_selected_line_alpha" {

                    let value: f64 = event.data.parse().unwrap();

                    self.selected_line_alpha = value;
                }
                else if event.event_type == "set_unselected_lines_alpha" {

                    let value: f64 = event.data.parse().unwrap();

                    self.unselected_lines_alpha = value;
                }
                else if event.event_type == "set_selected_quads" {

                    let value: f64 = event.data.parse().unwrap();

                    self.selected_quads = value;
                }
                else if event.event_type == "set_unselected_quads" {

                    let value: f64 = event.data.parse().unwrap();

                    self.unselected_quads = value;
                }
                else if event.event_type == "set_lookup_radius" {
                    let value: f64 = event.data.parse().unwrap();

                    self.lookup_radius = value;
                }
                else if event.event_type == "set_lookup_alpha" {
                    let value: f64 = event.data.parse().unwrap();

                    self.lookup_alpha = value;
                }
                else if event.event_type == "set_boid_size" {
                    let value: f64 = event.data.parse().unwrap();

                    self.boid_size = value;
                }
                else if event.event_type == "set_lookup_method" {
                    let value: String = event.data.parse().unwrap();

                    self.lookup_method = value;
                }
            },
            Err(e) => {
                console::log_1(&format!("Error: {:?}", e).into());
            }
        }
    }
}

impl Renderable for Quadtree {
    fn draw(
        &mut self,
        context: &web_sys::CanvasRenderingContext2d,
        canvas: &web_sys::HtmlCanvasElement,
    ) {
        let min = self.boundary.min;
        let max = self.boundary.max;


        let offset = 1.0;
        context.stroke_rect(
            min.x,
            min.y,
            max.x - (min.x + offset),
            max.y - (min.y + offset),
        );

        for child in &mut self.children {
            child.draw(context, canvas);
        }
    }
}

impl MouseEvents for World {
    fn on_input_drag(&mut self, positions: Vec<Vector3<f64>>) {

        let boids_len = self.boids.len();

        for boid in &mut self.boids {
            let touch_position_to_use = boids_len % positions.len();
            boid.borrow_mut().set_target(positions[touch_position_to_use]);
        }
    }

    fn on_input_move(&mut self, positions: Vec<Vector3<f64>>) {
        self.boids[0].borrow_mut().set_target(positions[0]);
    }

    fn on_input_stop(&mut self) {
        self.set_random_targets_for_all_boids();
    }
}

impl Renderable for World {
    fn draw(
        &mut self,
        context: &web_sys::CanvasRenderingContext2d,
        canvas: &web_sys::HtmlCanvasElement,
    ) {
        context.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);

        // Draw the quad tree
        context.set_stroke_style(&JsValue::from_str(format!("rgba(0,0,0,{})", self.unselected_quads).as_str()));

        self.quad_tree.draw(context, canvas);

        let box_size = self.boid_size;

        let boids = &self.boids;

        let mut sum_compare_against: usize = 0;

        let boid_to_boid_distance: f64 = self.lookup_radius;
        let boid_to_boid_distance_half: f64 = boid_to_boid_distance / 2.0;

        let selected_box_outline = 3.0;

        // Draw lines between all boids
        for boid in boids {
            let our_pos = boid.borrow().pos;

            // use world.lookup_method to use either "quadtree" or "brute_force"

            // let nearby_boids: Vec<i32> = if self.lookup_method == "quadtree" {
            //     self.quad_tree.query_range(AABB::new(
            //         Vector3::new(our_pos.x - boid_to_boid_distance_half, our_pos.y - boid_to_boid_distance_half, 0.0),
            //         Vector3::new(our_pos.x + boid_to_boid_distance_half, our_pos.y + boid_to_boid_distance_half, 0.0),
            //     )).iter().filter(|x| {
            //         let other_boid_pos = x.borrow().pos;
            //         let distance = ((our_pos.x - other_boid_pos.x).powi(2) + (our_pos.y - other_boid_pos.y).powi(2)).sqrt();

            //         return distance < boid_to_boid_distance;
            //     }).cloned().collect();
            // } else {
            //     self.boids.iter().filter(|x| {
            //         let other_boid_pos = x.borrow().pos;
            //         let distance = ((our_pos.x - other_boid_pos.x).powi(2) + (our_pos.y - other_boid_pos.y).powi(2)).sqrt();

            //         return distance < boid_to_boid_distance;
            //     }).cloned().collect();
            // };


            //sum_compare_against += nearby_boids.len();
            let nearby_boids: Vec<_> = if self.lookup_method == "quadtree" {
                self.quad_tree.query_range(AABB::new(
                    Vector3::new(our_pos.x - boid_to_boid_distance, our_pos.y - boid_to_boid_distance, 0.0),
                    Vector3::new(our_pos.x + boid_to_boid_distance, our_pos.y + boid_to_boid_distance, 0.0),
                )).iter().filter(|x| {
                    let other_boid_pos = x.borrow().pos;
                    let distance = ((our_pos.x - other_boid_pos.x).powi(2) + (our_pos.y - other_boid_pos.y).powi(2)).sqrt();

                    sum_compare_against += 1;

                    distance < boid_to_boid_distance
                }).cloned().collect()
            } else {
                self.boids.iter().filter(|x| {
                    let other_boid_pos = x.borrow().pos;
                    let distance = ((our_pos.x - other_boid_pos.x).powi(2) + (our_pos.y - other_boid_pos.y).powi(2)).sqrt();

                    sum_compare_against += 1;

                    distance < boid_to_boid_distance
                }).cloned().collect()
            };


            //we need to fetch the nearby boids
            // let nearby_boids = self.quad_tree.query_range(AABB::new(
            //     Vector3::new(our_pos.x - boid_to_boid_distance_half, our_pos.y - boid_to_boid_distance_half, 0.0),
            //     Vector3::new(our_pos.x + boid_to_boid_distance_half, our_pos.y + boid_to_boid_distance_half, 0.0),
            // ));

            // let mut nearby_boids: Vec<Rc<RefCell<Boid>>> = Vec::new();

            // for other_boid in boids {
            //     let other_boid_pos = other_boid.borrow().pos;

            //     if our_pos.x != other_boid_pos.x && our_pos.y != other_boid_pos.y {
            //         let distance = ((our_pos.x - other_boid_pos.x).powi(2)
            //             + (our_pos.y - other_boid_pos.y).powi(2))
            //         .sqrt();
            //         if distance < 100.0 {
            //             nearby_boids.push(other_boid.clone());
            //         }
            //     }
            // }



            for other_boids in &nearby_boids {

                // if boid != &self.boids[0]
                // {
                //     continue;
                // }

                let other_boid_pos = other_boids.borrow().pos;

                let distance = ((our_pos.x - other_boid_pos.x).powi(2) + (our_pos.y - other_boid_pos.y).powi(2)).sqrt();

                let alpha = if boid == &self.boids[0] {
                    self.selected_line_alpha
                } else {
                    self.unselected_lines_alpha
                };

                let color = format!("rgba(0,0,0,{})", ((1.0 - (distance / boid_to_boid_distance)) * alpha));
                //let color = format!("rgba(0,0,0,{})", 1);

                context.set_stroke_style(&JsValue::from_str(color.as_str()));

                context.begin_path();

                context.move_to(our_pos.x, our_pos.y);
                context.line_to(other_boid_pos.x, other_boid_pos.y);

                context.stroke();
            }
        }

        let average_compare_against = (sum_compare_against as f64) / (boids.len() as f64);

        if self.average_compare_against.len() > 60 {
            self.average_compare_against.remove(0);
        }

        self.average_compare_against.push(average_compare_against);

        // Draw selected boid
        let selected_boid = boids[0].borrow();
        let selected_boid_pos = &selected_boid.pos;


        // Draw the selected boid
        context.set_stroke_style(&JsValue::from_str("rgba(255,0,0,0.5)"));
        context.set_fill_style(&JsValue::from_str("rgba(255,0,0,0.5)"));
        context.fill_rect(
            selected_boid_pos.x - ((box_size / 2.0) + selected_box_outline * 1.5),
            selected_boid_pos.y - ((box_size / 2.0) + selected_box_outline * 1.5),
            (box_size * 1.0) + (selected_box_outline * 2.0 * 1.5),
            (box_size * 1.0) + (selected_box_outline * 2.0 * 1.5),
        );

        // Draw all the boids nearby the selected boid
        let nearby_selected_boids = self.quad_tree.query_range(AABB::new(
            Vector3::new(selected_boid_pos.x - boid_to_boid_distance, selected_boid_pos.y - boid_to_boid_distance, 0.0),
            Vector3::new(selected_boid_pos.x + boid_to_boid_distance, selected_boid_pos.y + boid_to_boid_distance, 0.0),
        ));


        // Draw all the boids nearby the selected boid
        context.set_fill_style(&JsValue::from_str(format!("rgba(255,0,0,{})", self.lookup_alpha).as_str()));
        for other_boids in nearby_selected_boids {
            let other_boid_pos = other_boids.borrow().pos;

            context.fill_rect(
                other_boid_pos.x - ((box_size / 2.0) + selected_box_outline),
                other_boid_pos.y - ((box_size / 2.0) + selected_box_outline),
                (box_size * 1.0) + (selected_box_outline * 2.0),
                (box_size * 1.0) + (selected_box_outline * 2.0),
            );
        }

        // Draw all the boids
        for boid in boids {
            context.set_fill_style(&JsValue::from_str("rgba(1,1,1,0.75)"));

            let boid_pos = boid.borrow().pos;

            context.fill_rect(
                boid_pos.x - (box_size / 2.0),
                boid_pos.y - (box_size / 2.0),
                box_size * 1.0,
                box_size * 1.0,
            );
        }

        // Draw all the quad-tree's involed in the selected boid
        let nearby_selected_quads = self.quad_tree.query_range_quads(AABB::new(
            Vector3::new(selected_boid.pos.x - 100.0, selected_boid.pos.y - 100.0, 0.0),
            Vector3::new(selected_boid.pos.x + 100.0, selected_boid.pos.y + 100.0, 0.0),
        ));

        for quad in nearby_selected_quads {

            context.set_stroke_style(&JsValue::from_str(format!("rgba(255,0,0,{})", self.selected_quads).as_str()));

            let min = quad.min;
            let max = quad.max;

            let offset = 1.0;
            context.stroke_rect(min.x + offset, min.y + offset, max.x - (min.x + offset), max.y - (min.y + offset));
        };

        // Draw Capture Zone around selected boid
        context.set_stroke_style(&JsValue::from_str(format!("rgba(0,0,0,{})", self.lookup_alpha).as_str()));
        context.stroke_rect(
            selected_boid_pos.x - boid_to_boid_distance,
            selected_boid_pos.y - boid_to_boid_distance,
            boid_to_boid_distance * 2.0,
            boid_to_boid_distance * 2.0
        );

    }
}

impl Updateable for World {
    fn update(&mut self, time_scale: f64) {

        let offset = 5;

        let min = Vector3::new(offset as f64, offset as f64, 0.0);
        let max = Vector3::new(
            (self.width - offset) as f64,
            (self.height - offset) as f64,
            0.0,
        );

        let world_box: AABB = AABB::new(min, max);

        for boid in &mut self.boids {
            boid.borrow_mut().update(time_scale, &world_box, &self.boid_characteristics);
        }

        self.quad_tree.clear();

        for boid in &mut self.boids {
            self.quad_tree.insert(boid.clone());
        }

        if self.boids.len() < self.desired_boid_count as usize {
            self.add_boid();
        }
        else if self.boids.len() > self.desired_boid_count as usize {
            self.boids.pop();
        }
    }
}

impl Setupable for World {
    fn setup(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        self.boids.clear();
        self.quad_tree.clear();

        self.quad_tree = generate_quad_tree(width, height);

        for _ in 0..self.desired_boid_count {
            self.add_boid();
        }

        self.set_random_targets_for_all_boids();
    }
}

pub struct RenderLoop {
    world: Rc<RefCell<World>>,

    // store canvas and context
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    previous_time: Option<f64>,

    next_frame_rate: f64,
    frame_rate: i32,
}
trait AnimationCallback {
    fn render_loop(&mut self, time: f64);
}

impl AnimationCallback for RenderLoop {
    fn render_loop(&mut self, time: f64) {
        if self.previous_time.is_none() {
            self.previous_time = Some(time);
        }

        let elapsed = time - self.previous_time.unwrap();

        // convert to a magnitude based on 60fps
        let time_scale = elapsed / (1000.0 / 60.0);

        if self.next_frame_rate <= 0.0 {
            self.next_frame_rate = 1000.0;

            // sum average_compare_against
            let mut sum = 0.0;
            for i in &self.world.borrow().average_compare_against {
                sum += i;
            }

            let average = sum / (self.world.borrow().average_compare_against.len() as f64);

            let stats: QuadTreeDemoStats = QuadTreeDemoStats {
                frame_rate: self.frame_rate,
                average_compare_against: average
            };

            let json_string = serde_json::to_string(&stats).unwrap();

            raise_event("quadtree_update_stats", &json_string);

            self.frame_rate = 0;
        } else {
            self.next_frame_rate -= elapsed;
            self.frame_rate += 1;
        }

        self.world.borrow_mut().update(time_scale);
        self.world.borrow_mut().draw(&self.context, &self.canvas);

        self.previous_time = Some(time);
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas_element: Element = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let bounding_rect = canvas.get_bounding_client_rect();

    // get canvas size
    let width = bounding_rect.width() as u32;
    let height = bounding_rect.height() as u32;

    console::log_1(&format!("Canvas size: {}x{}", width, height).into());

    let world = Rc::new(RefCell::new(World::new(width, height)));

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    world.borrow_mut().setup(width, height);

    // Add resize event
    {
        let cloned_world = world.clone();
        let cloned_canvas = canvas.clone();

        let listener = Closure::wrap(Box::new(move |_: web_sys::Event| {
            console::log_1(&format!("Resize event").into());

            let bounding_rect = cloned_canvas.get_bounding_client_rect();

            // get canvas size
            let width = bounding_rect.width() as u32;
            let height = bounding_rect.height() as u32;

            cloned_canvas.set_width(width);
            cloned_canvas.set_height(height);
            cloned_world.borrow_mut().setup(width, height);
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("resize", listener.as_ref().unchecked_ref())
            .unwrap();

        listener.forget();
    }

    // {
    //     let cloned_world = world.clone();

    //     let mouse_down_listener = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
    //         evt.prevent_default();

    //         let screen_position = Vector3::new(evt.offset_x() as f64, evt.offset_y() as f64, 0.0);

    //         cloned_world.borrow_mut().on_mouse_down(screen_position);
    //     }) as Box<dyn FnMut(_)>);

    //     window
    //         .add_event_listener_with_callback(
    //             "mousedown",
    //             mouse_down_listener.as_ref().unchecked_ref(),
    //         )
    //         .unwrap();

    //     mouse_down_listener.forget();
    // }

    {
        let cloned_world = world.clone();

        let mouse_down_listener = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
            evt.prevent_default();

            let screen_position = Vector3::new(evt.offset_x() as f64, evt.offset_y() as f64, 0.0);

            cloned_world.borrow_mut().on_input_stop();
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback(
                "mouseup",
                mouse_down_listener.as_ref().unchecked_ref(),
            )
            .unwrap();

        mouse_down_listener.forget();
    }

    {
        let cloned_world = world.clone();

        let mouse_down_listener = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
            evt.prevent_default();

            let screen_position = Vector3::new(evt.offset_x() as f64, evt.offset_y() as f64, 0.0);

            let inputs = vec![screen_position];

            if (evt.buttons() == 1) {
                cloned_world.borrow_mut().on_input_drag(inputs.clone());
            }

            cloned_world.borrow_mut().on_input_move(inputs.clone())
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback(
                "mousemove",
                mouse_down_listener.as_ref().unchecked_ref(),
            )
            .unwrap();

        mouse_down_listener.forget();
    }

    {
        let cloned_world = world.clone();

        let listener = Closure::wrap(Box::new(move |evt: web_sys::TouchEvent| {
            evt.prevent_default();

            // convert touches to Vector3s
            let mut inputs: Vec<Vector3<f64>> = Vec::new();

            for i in 0..evt.touches().length() {
                let touch = evt.touches().item(i).unwrap();

                let screen_position =
                    Vector3::new(touch.client_x() as f64, touch.client_y() as f64, 0.0);

                inputs.push(screen_position);
            }

            if inputs.len() > 0 {
                cloned_world.borrow_mut().on_input_drag(inputs.clone());
            }

            cloned_world.borrow_mut().on_input_move(inputs.clone());
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("touchmove", listener.as_ref().unchecked_ref())
            .unwrap();

        listener.forget();
    }

    {
        let cloned_world = world.clone();

        let listener = Closure::wrap(Box::new(move |evt: web_sys::TouchEvent| {
            evt.prevent_default();
            cloned_world.borrow_mut().on_input_stop();
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("touchstop", listener.as_ref().unchecked_ref())
            .unwrap();

        listener.forget();
    }

    {
        let cloned_world = world.clone();

        let listener = Closure::wrap(Box::new(move |evt: web_sys::Event| {
            evt.prevent_default();
            cloned_world.borrow_mut().on_custom_event(evt)
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("wasmevent", listener.as_ref().unchecked_ref())
            .unwrap();

        listener.forget();
    }

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    //let context = canvas.get_context("2d").unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let render_loop = Rc::new(RefCell::new(RenderLoop {
        canvas,
        context,
        frame_rate: 0,
        next_frame_rate: 1000.0,
        previous_time: None,
        world,
    }));

    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();

    let window = web_sys::window().unwrap();

    *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        render_loop.borrow_mut().render_loop(time);

        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }) as Box<dyn FnMut(f64)>));

    let window = web_sys::window().unwrap();
    window
        .request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("failed requesting animation frame");

    Ok(())
}
