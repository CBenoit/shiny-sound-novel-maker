use bevy::prelude::*;
use rfd::AsyncFileDialog;

fn main() {
    App::new().add_system(hello_world_system).run();
}

async fn try_file_picker() {
    if let Some(file) = AsyncFileDialog::new().set_directory("./").pick_file().await {
        println!("{}", file.path().display())
    }
}

fn hello_world_system() {
    pollster::block_on(try_file_picker());
}
