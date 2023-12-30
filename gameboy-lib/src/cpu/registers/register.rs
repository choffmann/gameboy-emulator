pub trait Register<T> {
    fn get(&self) -> T;
    fn set(&mut self, value: T);
}

