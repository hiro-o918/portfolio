use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub text: String,
  #[prop_or(String::from("is-3"))]
  pub title_class: String,
  #[prop_or_default]
  pub url: Option<String>,
}

pub struct Title {
  props: Props,
  link: ComponentLink<Self>,
}

impl Component for Title {
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
    match &self.props.url {
      Some(u) => html! {
        <div class=format!("title {}", self.props.title_class), >
          <a href=u.as_str() >{ self.props.text.as_str() }</a>
        </div>
      },
      _ => html! {
        <div class=format!("title {}", self.props.title_class), >
          { self.props.text.as_str() }
        </div>
      },
    }
  }
}
