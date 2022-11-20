use yew::prelude::*;

enum Message {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div>
                <button onclick={link.callback(|_| Message::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Message::AddOne => {
                self.value += 1;
                true
            }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
