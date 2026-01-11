use crate::astrobox::psys_host::{self, ui};

pub fn build_main_ui() -> ui::Element {
    let example_img = ui::Element::new(
        ui::ElementType::Image,
        Some(crate::resources::UI_EXAMPLE_IMAGE_B64),
    )
    .width(200)
    .height(200);

    let example_text =
        ui::Element::new(ui::ElementType::P, Some("何意味。这只是一个示例插件。")).size(26);

    ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .justify_center()
        .align_center()
        .child(example_img)
        .child(example_text)
}

pub fn render_main_ui(element_id: &str) {
    psys_host::ui::render(element_id, build_main_ui());
}
