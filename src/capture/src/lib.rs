use screenshots::Screen;
use anyhow::Result;
use std::path::PathBuf;

pub fn checkss() -> Result<usize> {
    let screens = Screen::all()?;
    Ok(screens.len())
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
