use std::time::Duration;

fn main() {
    loop {
        std::thread::sleep(Duration::from_secs(1));
        println!("moi");
    }
}
