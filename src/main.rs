
fn say_hello(dest: &str) {
    println!("Hello {}", dest);
}

fn say_world() {
    say_hello("World");
    say_hello("Preston");
}

fn main() {
    say_world();
}


