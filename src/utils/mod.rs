use crate::ui::{self, Positions};

pub fn do_something(d: (u32, u32)) -> String {
    let ui_styles = ui::UI {
        x: d.0 * 2,
        y: d.1 * 2,
        z: 2,
    };

    return format!(
        "{}px",
        (ui_styles.get_x() + ui_styles.get_y()) / ui_styles.get_z()
    );
}
