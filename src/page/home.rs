use ::yew::prelude::*;
use ::yew::services::fetch::FetchService;
use ::yew::services::fetch::FetchTask;
use ::yew::services::fetch::Response;
use ::yew::services::ConsoleService;
use yew::format::Json;

use crate::api::get_hero;
use crate::api::get_home;
use crate::layout::*;

pub struct Home {
    fetch_task: Option<Vec<FetchTask>>,
    data: Option<String>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

pub enum Msg {
    ReceiveResponse(Result<Vec<String>, anyhow::Error>),
    Receive(Result<String, anyhow::Error>),
}

impl Home {
    fn content(&self) -> Html {
        html! {
            <>
                {"this is body"}
            </>
        }
    }

    fn init(&mut self) {
        let req = get_hero();
        let callback =
            self.link
                .callback_once(|res: Response<Json<Result<Vec<String>, anyhow::Error>>>| {
                    let Json(data) = res.into_body();
                    Msg::ReceiveResponse(data)
                });
        let task_hero = FetchService::fetch(req, callback).expect("failed to start request");
        // self.fetch_task = Some(task);

        let req = get_home();
        let callback =
            self.link
                // .callback_once(|res: Response<Json<Result<String, anyhow::Error>>>| {
                    .callback_once(|res: Response<Result<String, anyhow::Error>>| {
                    let data = res.into_body();
                    Msg::Receive(data)
                });
        let task_home = FetchService::fetch(req, callback).expect("failed to start request");
        self.fetch_task = Some(vec![task_hero, task_home]);
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut home = Self {
            fetch_task: None,
            data: None,
            link,
            error: None,
        };
        home.init();
        home
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        let hero = data.get(0).unwrap();
                        self.data = Some(hero.to_string());
                        ConsoleService::log(hero);
                    }
                    Err(err) => self.error = Some(err.to_string()),
                };
            }
            Receive(res) => match res {
                Ok(data) => ConsoleService::info(&data),
                Err(err) => {
                    ConsoleService::info(&format!("{:?}", err));
                    self.error = Some(err.to_string());
                }
            },
        }
        // self.fetch_task = None;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { navbar() }
                { hero(self.data.clone().unwrap_or(String::from(""))) }
                { self.content() }
                { footer() }
            </>
        }
    }
}
