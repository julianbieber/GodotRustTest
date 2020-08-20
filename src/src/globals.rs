use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Globals {
    kills: u16,
    current_stage: u16,
}

#[methods]
impl Globals {
    fn new(_owner: &Node) -> Self {
        Globals {
            kills: 0,
            current_stage: 0
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world.");
    }

    pub fn reset(&mut self) {
        self.kills = 0;
        self.current_stage = 1;
    }


    pub fn kills(&self, _owner: &Node) -> u16 {
        self.kills
    }

    pub fn increment_kills(&mut self) {
        self.kills += 1;
    }
}