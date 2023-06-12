fn main() {
    let mut test_backend = ratatui::backend::TestBackend::new(8224, 8224);

    let _rect1 = if let Ok(x) = ratatui::backend::Backend::size(&(test_backend)) {
        x
    } else {
        use std::process;
        process::exit(0);
    };

    let _rect2 = ratatui::layout::Rect::new(8223, 65312, 8415, 8224);
    let _ = ratatui::layout::Rect::intersection(_rect1, _rect2);
}
