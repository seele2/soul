use ::yew::format::Nothing;
use ::yew::services::fetch::Request;

const PROTOCOL: &str = "http";
const SERVE_HOST: &str = "localhost";
const SERVE_PROT: &str = "80";

const HERO: &str = "/api/hero";
const HOME: &str = "/api/home";

pub fn get_hero() -> Request<Nothing> {
    let url = assemble(HERO);
    Request::get(url)
        .body(Nothing)
        .expect("Could not build request.")
}

pub fn get_home() -> Request<Nothing> {
    let url = assemble(HOME);
    Request::get(url)
        .body(Nothing)
        .expect("Could not build request.")
}

fn assemble(uri: &str) -> String {
    format!("{}://{}:{}{}", PROTOCOL, SERVE_HOST, SERVE_PROT, uri)
}
