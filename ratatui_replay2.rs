fn main() {
    let _local4 = ratatui::layout::Rect::new(65280, 2, 29696, 640);
    let mut _local5 = ratatui::backend::TestBackend::new(37492, 37492);
    let _local12 = if let Ok(x) = ratatui::backend::Backend::size(&(_local5)) {
        x
    } else {
        use std::process;
        process::exit(0);
    };
    let _ = ratatui::layout::Rect::intersection(_local4, _local12);
}
