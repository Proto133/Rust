use yew::prelude::*;

// Declare the nested modules
mod components; // This makes `components/footer.rs` available for us

struct App {
    count: i32,
}

enum Msg {
    Increment,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <main id="content">
            <div class="counter_container">
                <div class="counter">
                    <p>{ self.count }</p>
                    <button onclick={ctx.link().callback(|_| Msg::Increment)}>{ "Increment" }</button>
                </div>
            </div>
            <components::footer::AppFooter />
        </main>
        }
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
