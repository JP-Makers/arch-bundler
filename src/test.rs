use std::str::FromStr;
use alpm_types::InstalledSize;
fn main() {
    let _ = InstalledSize::from_str("1");
}
