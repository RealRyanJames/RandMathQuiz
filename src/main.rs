mod ui;
mod utils;

fn main() {
    dioxus::launch(_ui::app);
}

struct ClientData {
    title: String,
}

trait GetInfo {
    fn get(&self) -> String;
}

impl GetInfo for ClientData {
    fn get(&self) -> String {
        format!("{}", self.title)
    }
}

mod _ui {

    use dioxus::prelude::*;

    use crate::{
        ClientData, GetInfo,
        ui::{Positions, UI},
        utils::{self, do_something},
    };

    pub fn app() -> Element {
        rsx! {
            h2 {
                "Welcome to Simple Rand Math UI \n"
            }

            Button_com {}
        }
    }

    #[component]
    fn Button_com() -> Element {
        let mut count = use_signal(|| 0);

        let name = ClientData {
            title: format!("{}", count),
        };

        let padX = (5, 5);
        let do_some = utils::do_something(padX);

        let data = name.get();
        let col = do_something(padX);

        let font_sizes = UI { x: 10, y: 1, z: 0 };

        rsx! {

            h2 {
                background_color: "#101010",
                color: "white",
                font_size: format!("{}px", (font_sizes.get_x() + 40) / 2 - 4),
                padding: do_some,
                text_align: "center",
                "Hello | Your Count is:  {data}",


            }

            button {

                background_color: "#202020",
                color: "white",
                padding: col.clone(),
                width: "120px",
                text_align: "center",
                onclick: move |_| count += 1,

                "ADD"
            }

            button {
                text_align: "center",
                background_color: "#202020",
                color: "white",
                width: "120px",
                padding: col.clone(),
                onclick: move |_| count -= 1,
                "SUB"
            }
        }
    }
}
