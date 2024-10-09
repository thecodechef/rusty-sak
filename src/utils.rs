use fatfs::FormatVolumeOptions;
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;

use open;

pub fn format_to_fat32(path: &PathBuf) -> Result<(), io::Error> {
  println!("Formatting to FAT32");
  // TODO: Implement Formatting to FAT32
  let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)?;


  fatfs::format_volume(&file,FormatVolumeOptions::new())?;

  Ok(())

}

pub fn extract_from_emmc() {
  println!("Extracting Save from eMMC");
  // TODO: Implement Save Extraction from eMMC
}

pub fn nro_forwarder() {
  println!("Forwarding NRO");
  // TODO: Implement NRO Forwarding
}

pub fn get_info() {
  println!("Getting Information");
  // TODO: Implement Getting the About.md
}

pub fn check_for_updates() {
  println!("Checking for Updates");
  // TODO: Implement Checking for Updates
}

pub fn go_to_homepage() {
  println!("Go to https://rusty-sak.github.io/");
  open::that("https://rusty-sak.github.io/").unwrap();
}