use super::title;
use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub img_src: String,
  pub title_props: title::Props,
  #[prop_or(String::from("is-4by3"))]
  pub img_class: String,
}

pub struct Card {
  props: Props,
  link: ComponentLink<Self>,
}

impl Component for Card {
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
    if self.props != props {
      self.props = props;
      true
    } else {
      false
    }
  }

  fn view(&self) -> Html {
    let title_props = self.props.title_props.clone();
    let title_url = self
      .props
      .title_props
      .url
      .clone()
      .unwrap_or(String::from(""));
    html! {
      <div class="card">
        <div class="card-image">
          <figure class=format!("image {}", self.props.img_class)>
            <img src=self.props.img_src style="object-fit: cover" />
          </figure>
        </div>
        <div class="card-content">
          <title::Title with title_props />
        </div>
        <footer class="card-footer">
          <a href=title_url class="subtitle is-4 card-footer-item">{ "More" }</a>
        </footer>
      </div>
    }
  }
}
