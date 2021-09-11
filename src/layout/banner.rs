use ::ybc::TileCtx::{Ancestor, Child, Parent};
use ::ybc::{Container, Hero, Tile};

use ::yew::prelude::*;

pub fn hero() -> Html {
    html! {
        <>
            <Hero classes=classes!("is-light")
            size=ybc::HeroSize::FullheightWithNavbar
            body=html!{
                <Container classes=classes!("is-fullhd")>
                    <img src="asset/home-banner-bg.jpg" />
                // <Tile ctx=Ancestor>
                //     <Tile ctx=Parent size=ybc::TileSize::Twelve>
                //         <Tile ctx=Parent>
                //             <Tile ctx=Child classes=classes!("notification", "is-success")>
                //                 <ybc::Subtitle size=ybc::HeaderSize::Is3 classes=classes!("has-text-white")>{"Trunk"}</ybc::Subtitle>
                //                 <p>{"Trunk is a WASM web application bundler for Rust."}</p>
                //             </Tile>
                //         </Tile>
                //         <Tile ctx=Parent>
                //             <Tile ctx=Child classes=classes!("notification", "is-success")>
                //                 <ybc::Icon size=ybc::Size::Large classes=classes!("is-pulled-right")><img src="asset/yew.svg"/></ybc::Icon>
                //                 <ybc::Subtitle size=ybc::HeaderSize::Is3 classes=classes!("has-text-white")>
                //                     {"Yew"}
                //                 </ybc::Subtitle>
                //                 <p>{"Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."}</p>
                //             </Tile>
                //         </Tile>
                //         <Tile ctx=Parent>
                //             <Tile ctx=Child classes=classes!("notification", "is-success")>
                //                 <ybc::Subtitle size=ybc::HeaderSize::Is3 classes=classes!("has-text-white")>{"YBC"}</ybc::Subtitle>
                //                 <p>{"A Yew component library based on the Bulma CSS framework."}</p>
                //             </Tile>
                //         </Tile>
                //     </Tile>
                // </Tile>
                </Container>
            }
            >
            </Hero>
        </>
    }
}
