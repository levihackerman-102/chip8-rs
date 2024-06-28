pub struct Keyboard {

}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard{}
    }
 
    // implement key handling
    pub fn key_pressed(&self, key_code: u8) -> bool {
        true
    }
}
