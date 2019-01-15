mod guessing_game;
mod dining_philosophers;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];
    match input.to_string().as_str() {
        "rand" => guessing_game::run(),
        "thread" => dining_philosophers::run(),
        _ => println!("command available: rand | thread")
    }
}
