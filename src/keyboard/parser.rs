// parser.rs

pub enum Layer {
    Base,
    Keypad,
    Fn1,
    Fn2,
    Fn3,
}
impl Layer {
    pub fn from_string(tag: &str) -> Option<Layer> {
        match tag {
            "base" => Some(Layer::Base),
            "kp" => Some(Layer::Keypad), // The Kinesis tag for keypad is usually <kp>
            "fn1" => Some(Layer::Fn1),
            "fn2" => Some(Layer::Fn2),
            "fn3" => Some(Layer::Fn3),
            _ => None, // If it reads a layer that doesn't exist, it returns nothing
        }
    }
}

pub fn parse_layout(raw_file: &str) {
    for line in raw_file.lines() {
        let trimmed_line = line.trim();
        if &line[0..] == "<" {}
    }
}
