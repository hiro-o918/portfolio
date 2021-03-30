use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub img_src: String,
  #[prop_or(String::from("is-128x128"))]
  pub figure_class: String,
  #[prop_or(String::from("is-rounded"))]
  pub img_class: String,
}

pub struct Avatar {
  props: Props,
  link: ComponentLink<Self>,
}

impl Component for Avatar {
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
    html! {
      <figure class=format!("image {}", self.props.figure_class), >
          <img class=self.props.img_class.as_str(), src=self.props.img_src, />
      </figure>
    }
  }
}
