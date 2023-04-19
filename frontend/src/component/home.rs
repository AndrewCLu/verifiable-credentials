use yew::{function_component, html, use_state, Html};

#[function_component]
pub fn Home() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button class="bg-purple-300" {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
