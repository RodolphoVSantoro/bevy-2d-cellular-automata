pub fn should_decay(neighbors: u32) -> bool {
    return neighbors > 3 || neighbors < 2;
}

pub fn should_spawn(neighbors: u32) -> bool {
    return neighbors == 2;
}

// Usually makes it looks like sound waves coming out of the center
// pub fn should_decay(neighbors: u32) -> bool {
//     return neighbors == 2 || neighbors == 3;
// }

// pub fn should_spawn(neighbors: u32) -> bool {
//     return neighbors > 1;
// }
