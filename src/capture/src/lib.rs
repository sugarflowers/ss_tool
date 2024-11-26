use screenshots::Screen;
use anyhow::Result;
use std::{
    path::PathBuf,
    fs,
};

pub fn check_paths(dest: &mut PathBuf) -> Result<()> {
    let screens = Screen::all()?;
    for idx in 0..screens.len() {
        dest.push(format!("{idx}"));
        fs::create_dir_all(&mut *dest).unwrap();
    }
    Ok(())
}

pub fn screenshot(dest: &str) -> Result<()> {
    let screens = Screen::all()?;
    for (idx, screen) in screens.into_iter().enumerate() {
        //ここでファイル名とかディレクトリとかに細工が必要だし
        //なんならファイル名とか別に固定で決めちゃっていいのでは
        println!("idx: {}", idx);
        screen.capture()?.save(dest)?;
    }
    Ok(())
}

pub fn sstest(dest: &PathBuf) {
    println!("{dest:?}");
}

/*
fn main() {
    let sss = checkss().unwrap();
    println!("{sss}");
    screenshot("test.png").unwrap();
}
*/
