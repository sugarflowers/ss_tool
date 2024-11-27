use screenshots::Screen;
use anyhow::{Result, anyhow};
use glob::glob;
use std::{
    path::PathBuf,
    fs,
};

fn count_files(dir: &str) -> Result<usize> {
    let pattern = format!("{dir}/*.png");
    let mut cnt = 0;
    for entry in glob(&pattern)? {
        match entry {
            Ok(_) => cnt += 1,
            Err(e) => return Err(anyhow!(e)),
        }
    }
    Ok(cnt)
}

pub fn check_paths(dest: &mut PathBuf) -> Result<()> {
    let screens = Screen::all()?;
    for idx in 0..screens.len() {
        let mut d = dest.clone();
        d.push(format!("{idx}"));
        fs::create_dir_all(&mut *d)?;
    }
    Ok(())
}

pub fn screen_shot(dest: &PathBuf) -> Result<()> {
    let screens = Screen::all()?;
    for (idx, screen) in screens.into_iter().enumerate() {
        let mut d = dest.clone();
        d.push(format!("{idx}"));

        let cnt = match count_files(&d.to_string_lossy().to_string()){
            Ok(c) => c,
            Err(e) => return Err(anyhow!(e)),
        };
        d.push(format!("{cnt:06}.png"));

        let ds = d.to_string_lossy().to_string();
        screen.capture()?.save(&ds)?;
    }
    Ok(())
}

