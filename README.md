# Dynamic Panels

This crate allows creating dynamic panels in [egui](https://crates.io/crates/egui) that are displayed with different configuration depending on a given choice function.
A very simple example:
```rust
let dpanel = DynamicPanel::new("bla");
let dpanel = dpanel.with_panels(vec![
    SinglePanelCfg::left().into(),
    SinglePanelCfg::bottom().into(),
]);
let dpanel = 
    dpanel.with_choice_function(|ctx| {
        if ctx.input(|i| i.screen_rect).width() < 500. {
            1
        } else {
            0
        }
    });
dpanel.show_dynamic(ctx, |ui| {
    ui.label("This moves!");
});
```
This will show a panel to the left if the context width is >= 500., otherwise it will show a panel at the bottom.