#![recursion_limit = "1024"]
use yew::prelude::*;

mod components;
mod containers;
use containers::{profile, publishing};

struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="hero is-fullheight is-fullwidth">
                <div class="hero-body">
                    <div class="container">
                        <div id="top" class="columns is-vcentered">
                            <div class="column is-one-third">
                                <profile::Profile />
                            </div>
                            <div class="column is-two-third is-center">
                                <publishing::Publishing />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
