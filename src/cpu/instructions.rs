pub enum Instruction {
    // Move sum from a and value to a
    ADD(RegisterTarget),
}

pub enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
