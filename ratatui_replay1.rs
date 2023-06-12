fn main() {
    let test_backend = ratatui::backend::TestBackend::new(58596, 58596);
    let _rect1 = if let Ok(x) = ratatui::backend::Backend::size(&(test_backend)) {
        x
    } else {
        use std::process;
        process::exit(0);
    };
    let _rect2 = ratatui::layout::Rect::new(58596, 58596, 58368, 0);
    let _local16 = ratatui::layout::Rect::intersection(_rect1, _rect2);
}
