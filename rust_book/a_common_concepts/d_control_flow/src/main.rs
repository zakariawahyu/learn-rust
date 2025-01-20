mod repetition;
mod condition;

fn main() {
    println!("======== Condition ========");
    condition::condition();
    println!("======== Repetition ========");
    println!("=== While ===");
    repetition::while_repetition();
    println!("=== Loop ===");
    repetition::loop_repetition();
    println!("=== For in ===");
    repetition::for_repetition();
}
