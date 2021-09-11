#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod page;
mod layout;

fn main() {
    yew::start_app::<page::Home>();
}
