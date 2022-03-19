#[derive(Debug)]
#[repr(C)]
pub struct Inventory {
    total: u32,
    current: u32,
}

impl Inventory {
    pub fn new(total: u32) -> Self {
        Inventory {
            total,
            current: total,
        }
    }

    pub fn reset(&mut self, total: u32) {
        self.total = total;
        self.current = total;
    }

    pub fn increase(&mut self, by: u32) -> (u32, u32) {
        self.total += by;
        self.current += by;
        (self.total, self.current)
    }

    pub fn r#return(&mut self, r#return: u32) -> Option<u32> {
        let current = self.current + r#return;
        if current > self.total {
            return None;
        }
        self.current = current;
        Some(self.current)
    }

    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn take(&mut self, count: u32) -> Option<u32> {
        if self.current < count {
            return None;
        }

        self.current -= count;
        Some(self.current)
    }
}

#[cfg(test)]
mod test {
    use super::Inventory;
    use std::mem::size_of;

    #[test]
    fn size() {
        assert_eq!(size_of::<Inventory>(), size_of::<u64>());
    }
}
