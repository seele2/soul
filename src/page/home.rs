use ::yew::prelude::*;

use crate::layout::*;

pub struct Home {}

pub enum Msg {}

impl Home {
    fn content(&self) -> Html {
        html! {
            <>
                {"this is body"}
            </>
        }
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { navbar() }
                { hero() }
                { self.content() }
                { footer() }
            </>
        }
    }
}
