#[derive(Clone, Copy, Debug)]
pub enum Item {
    Wood,
}

pub const ITEM_MAX: usize = Item::Wood as usize + 1;

#[derive(Clone, Copy, Debug)]
pub struct Stack {
    pub item: Item,
    pub count: u8,
}
