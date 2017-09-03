use registry::item::*;

pub struct Inventory {
    items: [u8; ITEM_MAX],
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: [0; ITEM_MAX],
        }
    }

    pub fn items(&self) -> &[u8; ITEM_MAX] {
        &self.items
    }

    pub fn add(&mut self, stack: Stack) -> bool {
        let idx = stack.item as usize;

        if self.items[idx] <= 255 - stack.count {
            self.items[idx] += stack.count;

            true
        } else {
            false
        }

        //self.items[idx] += (255 - stack.count).min(stack.count - self.items[idx]);
    }

    pub fn remove(&mut self, stack: Stack) -> bool {
        let idx = stack.item as usize;

        if self.items[idx] >= stack.count {
            self.items[idx] -= stack.count;

            true
        } else {
            false
        }
    }
}
