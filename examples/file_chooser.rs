extern crate file_dialog;
extern crate opengl_graphics;

use file_dialog::{FileDialog, SelectType};
use opengl_graphics::OpenGL;

use std::borrow::ToOwned;

fn main() {
    let promise = FileDialog::new("File Dialog Test", font())
        .show(OpenGL::_3_2);
       
    if let Some(file) = promise.join().unwrap_or(None) {
        println!("Selected file: {}", file.display());
    }

    let promise = FileDialog::new("File Save Test", font())
        .set_select(SelectType::SaveFile(Some("filename.txt".to_owned())))
        .show(OpenGL::_3_2);

    if let Some(file) = promise.join().unwrap_or(None) {
        println!("Selected file: {}", file.display());
    }
}

fn font() -> Path {
    Path::new("./assets/Dense-Regular.otf")
}
