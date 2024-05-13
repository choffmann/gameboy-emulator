
#[derive(Debug)]
pub struct StackPointer {
    pub value: u16,
}

impl StackPointer {
    pub fn new() -> StackPointer {
        StackPointer {
            value: 0,
        }
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut stack_pointer = StackPointer::new();
        stack_pointer.value = 0x1234;
        assert_eq!(stack_pointer.get(), 0x1234);
    }

    #[test]
    fn test_set() {
        let mut stack_pointer = StackPointer::new();
        stack_pointer.set(0x1234);
        assert_eq!(stack_pointer.value, 0x1234);
    }
}
