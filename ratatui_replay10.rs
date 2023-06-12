fn main() {
let _local1 = ratatui::layout::Rect::new(44, 11, 0, 65535);
let mut _local2 = ratatui::backend::TestBackend::new(65535, 14385);
let _local3 = if let Ok(x) = ratatui::backend::Backend::size(&(_local2)) {
x
} else {
use std::process;
process::exit(0);
};
let _ = ratatui::layout::Rect::intersects(_local1, _local3);
}
