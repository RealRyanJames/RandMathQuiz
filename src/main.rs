mod functions;
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
        ClientData, GetInfo, functions,
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

        let math_problem_num1 = functions::rand_numbers(5);
        let math_problem_num2 = functions::rand_numbers(10);

        let padX = (5, 5);
        let do_some = utils::do_something(padX);

        let _data = name.get();
        let col = do_something(padX);

        let font_sizes = UI { x: 10, y: 1, z: 0 };

        rsx! {

            h2 {
                color: "#101010",
                font_size: format!("{}px", (font_sizes.get_x() + 40) / 2 - 4),
                padding: do_some,
                text_align: "center",

                {format!(
                    "{} + {} = {}",
                    math_problem_num1, math_problem_num2,
                    math_problem_num1 + math_problem_num2
                )},
            }


            h2 {

               color: "#101010",
                font_size: format!("{}px", (font_sizes.get_x() + 40) / 2 - 4),
                padding: col.clone(),
                text_align: "center",

                {format!(
                    "{} - {} = {}",
                    math_problem_num1, math_problem_num2,
                    math_problem_num1 - math_problem_num2
                )},
            }


            button {

                background_color: "#202020",
                color: "white",
                padding: col.clone(),
                width: "170px",
                text_align: "center",
                onclick: move |_| count += 1,

                "Get Math Problem"
            }

        }
    }
}
