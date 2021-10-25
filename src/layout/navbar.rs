use ::ybc;
use ::ybc::Size;
use ::ybc::{ButtonAnchor, HeaderSize};
use ::yew::prelude::*;

pub fn navbar() -> Html {
    html! {
        <>
            <ybc::Navbar
                classes=classes!("is-primary")
                padded=true
                navbrand=html!{
                    <ybc::NavbarItem>
                        <ybc::Icon size=Size::Large classes=classes!("is-pulled-right")><img src="asset/yew.svg"/></ybc::Icon>
                        <ybc::Title classes=classes!("has-text-white") size=HeaderSize::Is4>{"Soul"}</ybc::Title>
                    </ybc::NavbarItem>
                }
                navstart=html!{}
                navend=html!{
                    <>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/trunk">
                            {"首页"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/trunk">
                            {"关于seele2"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/trunk">
                            {"产品服务"}
                        </ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/trunk">
                            {"成功案例"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/ybc">
                            {"加入我们"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    </>
                }
            />
        </>
    }
}
