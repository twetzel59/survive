#[derive(Clone, Copy, Debug)]
pub enum Item {
    Wood,
}

#[derive(Clone, Copy, Debug)]
pub struct Stack {
    pub item: Item,
    pub count: u8,
}
