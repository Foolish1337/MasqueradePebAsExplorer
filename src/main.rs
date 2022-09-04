#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod MasqueradePebAsExplorerEx;
mod bindings;

use MasqueradePebAsExplorerEx::MasqueradePebAsExplorerEx;

fn main() {
    unsafe { MasqueradePebAsExplorerEx() };
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1_000));
    }
}
