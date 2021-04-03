#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
}

enum Msg {
    SelectionChange,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        log::info!("Create");
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        log::info!("Update");
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        log::info!("View");
        let t = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        Proin porta placerat nibh, sed pulvinar massa congue nec.
        Vestibulum rhoncus tincidunt hendrerit.
        Curabitur at enim urna.
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        Sed tortor leo, pretium fermentum posuere a, euismod eu arcu.
        Duis ultrices tincidunt purus, nec vulputate eros commodo eget.
        Sed quis hendrerit ex.
        Nulla mattis ut dui in tristique.
        Suspendisse tincidunt non purus ac laoreet.
        Maecenas malesuada dignissim dignissim.";

        let selection = yew::utils::document().get_selection().unwrap().unwrap();
        let anchor_offset = selection.anchor_offset() as usize;
        let focus_offset = selection.focus_offset() as usize;
        let beg = anchor_offset.min(focus_offset);
        let end = anchor_offset.max(focus_offset);
        let selected_t: String = t.clone().chars().skip(beg).take(end - beg).collect();

        html! {
            <div>
                <div onmousemove=self.link.callback(|_| Msg::SelectionChange)>
                    {t}
                </div>
                <hr/>
                <div>
                    {selected_t}
                </div>
            </div>
        }
    }
}

extern crate console_error_panic_hook;
use std::panic;
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Model>::new().mount_to_body();
}
