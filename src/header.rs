use yew::prelude::*;
#[function_component]
pub fn Header() -> Html {
    html! { 
        <div id = "header"> 
            <a src="/">{"logo"}</a>
            <a src="/">{"about"}</a>
        </div>
    }
}
