use interoptopus::{ffi_function, ffi_type};

#[ffi_type()]
#[repr(C)]
pub struct Args {
    init: u32,
    by: u32,
}

#[ffi_type(opaque)]
#[repr(C)]
#[derive(Clone)]
pub struct Counter {
    val: u32,
    by: u32,
    positions: Vec<Vector2>,
}

// Transparent copyable struct
#[ffi_type]
#[repr(C)]
pub struct CounterData {
    pub val: u32,
    pub by: u32,
}

#[ffi_type]
#[derive(Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Counter {
    pub fn new(args: Args) -> Counter {
        let mut poses = Vec::new();
        poses.push(Vector2 { x: 0.0, y: 0.0 });
        poses.push(Vector2 { x: 1.0, y: 1.0 });
        poses.push(Vector2 { x: 2.0, y: 2.0 });

        Counter {
            val: args.init,
            by: args.by,
            positions: poses,
        }
    }

    pub fn get(&self) -> u32 {
        self.val
    }

    pub fn get_by(&self) -> u32 {
        self.by
    }

    pub fn incr(&mut self) -> u32 {
        self.val += self.by;
        self.val
    }

    pub fn decr(&mut self) -> u32 {
        self.val -= self.by;
        self.val
    }

    pub fn incr_by(&mut self, by: u32) -> u32 {
        self.val += by;
        self.val
    }

    pub fn decr_by(&mut self, by: u32) -> u32 {
        self.val -= by;
        self.val
    }

    pub fn incr_by_many(&mut self, bys: &[u32]) -> u32 {
        for by in bys {
            self.val += *by;
        }
        self.val
    }

    pub fn decr_by_many(&mut self, bys: &[u32]) -> u32 {
        for by in bys {
            self.val -= *by;
        }
        self.val
    }
    
    pub fn get_positions(&self) -> &Vec<Vector2> {
        &self.positions
    }
}
