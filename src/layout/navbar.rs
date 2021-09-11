use ::ybc::Size::Large;
use ::ybc::{ButtonAnchor, HeaderSize, Icon, Navbar, NavbarItem, Title};
use ::yew::prelude::*;

pub fn navbar() -> Html {
    html! {
        <>
            <Navbar
                classes=classes!("is-primary")
                padded=true
                navbrand=html!{
                    <NavbarItem>
                        <Icon size=Large classes=classes!("is-pulled-right")><img src="asset/yew.svg"/></Icon>
                        <Title classes=classes!("has-text-white") size=HeaderSize::Is4>{"Soul"}</Title>
                    </NavbarItem>
                }
                navstart=html!{}
                navend=html!{
                    <>
                    <NavbarItem>
                        <ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/trunk">
                            {"首页"}
                        </ButtonAnchor>
                    </NavbarItem>
                    <NavbarItem>
                        <ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/trunk">
                            {"关于seele2"}
                        </ButtonAnchor>
                    </NavbarItem>
                    <NavbarItem>
                        <ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/trunk">
                            {"产品服务"}
                        </ButtonAnchor>
                    </NavbarItem>
                    <NavbarItem>
                        <ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/trunk">
                            {"成功案例"}
                        </ButtonAnchor>
                    </NavbarItem>
                    <NavbarItem>
                        <ButtonAnchor classes=classes!("is-primary", "is-rounded") rel=String::from("noopener noreferrer") target=String::from("_blank") href="https://github.com/thedodd/ybc">
                            {"加入我们"}
                        </ButtonAnchor>
                    </NavbarItem>
                    </>
                }
            />
        </>
    }
}
