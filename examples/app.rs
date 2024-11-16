use fathom::app::{schedule, FathomApplication};

fn main() {
    let mut app = FathomApplication::new();
    app.add_systems(schedule::Startup, startup);
    app.add_systems(schedule::Update, update);
    app.run();
}

fn startup() {
    println!("Starting up...");
}

fn update() {
    println!("Running update...")
}