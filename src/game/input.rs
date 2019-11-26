use std::collections::HashMap;
use ggez::{input, timer, Context};
use ggez::event::KeyCode;

struct KeyData
{
    key_code: KeyCode,
    down_frame: usize,
    held: bool
}
impl KeyData
{
    fn new(key_code: KeyCode) -> KeyData
    {
        KeyData
        {
            key_code: key_code,
            down_frame: 0,
            held: false
        }
    }
}
pub struct Input
{
    key_data: [KeyData; Input::NUM_KEYS],
    key_map: HashMap<KeyCode, usize>,
    latest_frame: usize
}
impl Input
{
    const NUM_KEYS: usize = 5;

    pub fn new() -> Input
    {
        let key_data =
        [
            KeyData::new(KeyCode::A),
            KeyData::new(KeyCode::D),
            KeyData::new(KeyCode::W),
            KeyData::new(KeyCode::S),
            KeyData::new(KeyCode::Space)
        ];
        let mut key_map: HashMap<KeyCode, usize> = HashMap::new();
        for i in 0..Input::NUM_KEYS
        {
            key_map.insert(key_data[i].key_code, i);
        }

        Input
        {
            key_data, 
            key_map,
            latest_frame: 0
        }
    }
    pub fn update(&mut self, context: &mut Context)
    { 
        self.latest_frame = timer::ticks(context);

        for i in 0..Input::NUM_KEYS
        {
            let pressed = input::keyboard::is_key_pressed(context, self.key_data[i].key_code);
            if pressed && !self.key_data[i].held
            {
                self.key_data[i].held = true;
                self.key_data[i].down_frame = self.latest_frame;
            }
            else if !pressed
            {
                self.key_data[i].held = false;
            }
        }       
    }
    pub fn get_key(&self, key_code: KeyCode) -> bool
    {
        self.key_data[*self.key_map.get(&key_code).unwrap()].held
    }
    pub fn get_key_down(&self, key_code: KeyCode) -> bool
    {
        self.key_data[*self.key_map.get(&key_code).unwrap()].down_frame == self.latest_frame
    }
    pub fn get_axis(&self, left: KeyCode, right: KeyCode) -> i32
    {
        let mut direction = 0;
        if self.get_key(left)
        {
            direction += -1;
        }
        if self.get_key(right)
        {
            direction += 1;
        }
        direction
    }
}
