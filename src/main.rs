use rfd::FileDialog;

slint::include_modules!();

mod xci;
mod nsp;
mod utils;

fn main() {
  let ui = MainWindow::new().unwrap();

  // XCI
  ui.on_xci_split(|| { xci::xci_split(); });
  ui.on_xci_merge(|| { xci::xci_merge(); });
  ui.on_xci_patch(|| { xci::xci_patch(); });
  ui.on_xci_update(|| { xci::xci_update(); });
  ui.on_xci_to_nsp(|| { xci::xci_to_nsp(); });
  ui.on_xci_to_xcz(|| { xci::xci_to_xcz(); });
  ui.on_xcz_to_xci(|| { xci::xcz_to_xci(); });
  ui.on_extract_fw_from_xci(|| { xci::extract_fw_from_xci(); });

  // NSP
  ui.on_nsp_split(|| { nsp::nsp_split(); });
  ui.on_nsp_merge(|| { nsp::nsp_merge(); });
  ui.on_nsp_patch(|| { nsp::nsp_patch(); });
  ui.on_nsp_update(|| { nsp::nsp_update(); });
  ui.on_nsp_to_xci(|| { nsp::nsp_to_xci(); });
  ui.on_nsp_to_nsz(|| { nsp::nsp_to_nsz(); });
  ui.on_nsz_to_nsp(|| { nsp::nsz_to_nsp(); });

  // Misc
  ui.on_format_to_fat32(move || {
    if let Some(path) = FileDialog::new().pick_file() {
      match utils::format_to_fat32(&path) {
        Ok(_) => println!("Successfully formatted {} to FAT32", path.display()),
        Err(e) => println!("Error formatting to FAT32: {}", e),
      }
    }
  });
  ui.on_extract_save_from_emmc(|| { utils::extract_from_emmc(); });
  ui.on_nro_forwarder(|| { utils::nro_forwarder(); });

  // Utils
  ui.on_get_info(|| { utils::get_info(); });
  ui.on_check_for_updates(|| { utils::check_for_updates(); });
  ui.on_go_to_homepage(|| { utils::go_to_homepage(); });

  let _ = ui.run();
}