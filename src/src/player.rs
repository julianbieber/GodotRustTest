use gdnative::prelude::*;
use crate::globals::Globals;
use std::ops::{MulAssign, AddAssign, SubAssign};
use gdnative::api::*;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    up_velocity: f32,
    camera_rotation: Vector2,
}

#[methods]
impl Player {

    fn new(_owner: &KinematicBody) -> Player {
        Player {
            up_velocity: 0.0f32,
            camera_rotation: Vector2::zero(),
        }
    }

    #[export]
    fn _ready(&self, _owner: &KinematicBody) {
        Input::godot_singleton().set_mouse_mode(Input::MOUSE_MODE_CAPTURED);
    }
    #[export]
    fn _input(&mut self, owner: &KinematicBody, event: Ref<InputEvent>) {
        if let Some(event) = event.clone().cast::<InputEventMouseMotion>() {
            let event = unsafe { event.assume_safe() };
            let motion: Vector2 = event.relative();
            self.camera_rotation.x += motion.y * 0.0025f32;
            self.camera_rotation.y -= motion.x * 0.0025f32;

            if self.camera_rotation.x < -0.4 {
                self.camera_rotation.x = -0.4;
            }
            if self.camera_rotation.x > 0.4 {
                self.camera_rotation.x = 0.4;
            }
        }

    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, _delta: f32) {
        let mut movement_direction = Vector3::new(0.0f32, 0.0f32, 0.0f32);
        let basis: Basis = owner.transform().basis;
        let mut x = basis.x();
        x *= 0.1f32;
        let mut z = basis.z();
        z *= 0.1f32;

        let input = Input::godot_singleton();


        if input.is_action_pressed("left") {
            movement_direction -= x;
        }

        if input.is_action_pressed("right") {
            movement_direction += x;
        }

        if input.is_action_pressed("back") {
            movement_direction += z;
        }

        if input.is_action_pressed("forward") {
            movement_direction -= z;
        }


        owner.move_and_collide(movement_direction, false, false, false).map(|collision| {
            self.up_velocity = 0.0f32;
            if Input::godot_singleton().is_action_pressed("jump") {
                self.up_velocity = 1.0f32;
            }
            ()
        }).or_else(|| {
            self.up_velocity -= 0.1f32;
            Option::Some(())
        });
        owner.set_rotation(Vector3::new(self.camera_rotation.x, self.camera_rotation.y, 0.0f32));

        if self.up_velocity > 1.0f32 {
            self.up_velocity = 1.0f32;
        }
        if self.up_velocity < -1.0f32 {
            self.up_velocity = -1.0f32;
        }

        let gravity = Vector3::new(0.0f32, self.up_velocity, 0.0f32);
        owner.move_and_collide(gravity, false, false, false);

    }

    fn kill(&self, owner: &KinematicBody) {
        let rust_game_state = owner
            .get_tree()
            .and_then(|tree| {
                let tree = unsafe { tree.assume_safe() };

                tree.root()
            })
            .and_then(|root| {
                let root = unsafe { root.assume_safe() };
                root.get_node("./Globals")
            })
            .and_then(|node| {
                let node = unsafe { node.assume_unique() };
                Instance::<Globals, _>::try_from_base(node).ok()
            })
            .expect("Failed to get Globals");

        rust_game_state
            .map_mut(|gs, _| gs.increment_kills())
            .expect("Could not increment kills for some reason.");
    }


}