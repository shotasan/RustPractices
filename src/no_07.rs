// Create a combat function that takes the player's current health and the amount of damage recieved, and returns the player's new health. Health can't be less than 0.

fn combat(health: f32, damage: f32) -> f32 {
    let result = health - damage;
    if result > 0.0 {
        result
    } else {
        0.0
    }
}

fn combat(health: f32, damage: f32) -> f32 {
    (health - damage).max(0.0)
}
