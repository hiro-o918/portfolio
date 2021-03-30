use crate::components::{button, card, title};
use once_cell::sync::Lazy;
use yew::prelude::*;

pub enum Msg {
  Increment,
  Decrement,
}

pub struct Publishing {
  card_props: card::Props,
  card_idx: usize,
  link: ComponentLink<Self>,
}

const N_CARDS: usize = 2;
const CARD_PROPS_LIST: Lazy<[card::Props; N_CARDS]> = Lazy::new(|| {
  [
    card::Props {
      img_src: String::from("https://learn.hashicorp.com/img/terraform/providers/core-plugins-api.png"),
      img_class: String::from("is-4by3"),
      title_props: title::Props {
        text: String::from("Terraform のエラーに落ち着いて立ち向かうために"),
        title_class: String::from("is-3"),
        url: Some(String::from(
          "https://tech.gunosy.io/entry/2020/12/11/100000",
        )),
      },
    },
    card::Props {
      img_src: String::from("https://www.preferred.jp/wp-content/themes/preferred/assets/img/projects/optuna/pict01.jpg"),
      img_class: String::from("is-4by3"),
      title_props: title::Props {
        text: String::from("Optuna における最適化の流れを追ってみた"),
        title_class: String::from("is-3"),
        url: Some(String::from(
          "https://qiita.com/hiro_o918/items/96edb40756d8adeec11b",
        )),
      },
    },
  ]
});

impl Component for Publishing {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      card_props: CARD_PROPS_LIST[0].clone(),
      card_idx: 0,
      link: link,
    }
  }

  fn update(&mut self, msg: Self::Message) -> bool {
    match msg {
      Msg::Increment => {
        if self.card_idx == N_CARDS - 1 {
          self.card_idx = 0
        } else {
          self.card_idx += 1;
        }
        self.card_props = CARD_PROPS_LIST[self.card_idx].clone();
        true
      }
      Msg::Decrement => {
        if self.card_idx == 0 {
          self.card_idx = N_CARDS - 1
        } else {
          self.card_idx -= 1;
        }
        self.card_props = CARD_PROPS_LIST[self.card_idx].clone();
        true
      }
    }
  }
  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn view(&self) -> Html {
    let card_props = self.card_props.clone();
    html! {
      <div id="publishing" class="container">
        <div class="card-button">
          <button::Button on_click=self.link.callback(|_| Msg::Decrement)>
            <span class="icon">
              <i class="fas fa-chevron-left"></i>
            </span>
          </button::Button>
        </div>
        <card::Card with card_props />
        <div class="card-button">
          <button::Button on_click=self.link.callback(|_| Msg::Increment)>
            <span class="icon">
              <i class="fas fa-chevron-right"></i>
            </span>
          </button::Button>
        </div>
      </div>
    }
  }
}
