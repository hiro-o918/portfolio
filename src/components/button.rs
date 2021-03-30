use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub children: Children,
  #[prop_or(String::from("is-primary is-rounded"))]
  pub button_class: String,
  #[prop_or_default]
  pub on_click: Callback<()>,
}

pub enum Msg {
  OnClick,
}

pub struct Button {
  props: Props,
  link: ComponentLink<Self>,
}

impl Component for Button {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props: props,
      link: link,
    }
  }
  fn update(&mut self, msg: Self::Message) -> bool {
    match msg {
      Msg::OnClick => self.props.on_click.emit(()),
    }
    true
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
      <button class=format!("button {}", self.props.button_class) onclick=self.link.callback(|_| Msg::OnClick)>
        { self.props.children.clone() }
      </button>
    }
  }
}
