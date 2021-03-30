use crate::components::*;
use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Profile {
  props: Props,
  link: ComponentLink<Self>,
}

impl Component for Profile {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props: props,
      link: link,
    }
  }
  fn update(&mut self, _: Self::Message) -> bool {
    false
  }
  fn change(&mut self, props: Self::Properties) -> bool {
    self.props != props
  }

  fn view(&self) -> Html {
    html! {
      <div class="container" id="profile">
        <div class="columns is-centered">
            <div class="column is-full">
                <avatar::Avatar img_src="https://avatars.githubusercontent.com/u/16137578" />
            </div>
        </div>
        <div class="columns is-centered">
            <div class="column is-full">
                <title::Title text="Hironori Yamamoto" />
                <title::Title text="GitHub" title_class="is-5" />
                <subtitle::SubTitle text="https://github.com/hiro-o918" subtitle_class="is-6" url="https://github.com/hiro-o918" />
                <title::Title text="This Page" title_class="is-5" />
                <subtitle::SubTitle text="https://github.com/hiro-o918/portfolio" subtitle_class="is-6" url="https://github.com/hiro-o918/portfolio" />
            </div>
        </div>
      </div>
    }
  }
}
