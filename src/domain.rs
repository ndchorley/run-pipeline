pub struct Pipeline {
    pub stages: Vec<Stage>
}

pub struct Stage {
    pub name: String,
    pub command: String
}
