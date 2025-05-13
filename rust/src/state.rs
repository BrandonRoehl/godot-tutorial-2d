use godot::classes::{Label, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct State {
    base: Base<Node>,

    #[init(val = 0)]
    score: i64,

    #[init(val = OnReady::from_node("%ScoreLabel"))]
    label: OnReady<Gd<Label>>,
}

#[godot_api]
impl State {
    pub fn add_score(&mut self, score: i64) {
        self.score += score;
        godot_print!("Score: {}", self.score);
        self.label.set_text(&format!("Score: {}", self.score));
    }
}
