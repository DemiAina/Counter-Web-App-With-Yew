use::yew::prelude::*;

enum Msg {
    AddOne,
    NegititiveOne,
}

#[derive(Debug)]
struct CounterComp {
    counter: i32,
}


impl Component for CounterComp {
    type Message = Msg;
    type Properties =  ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { counter: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.counter += 1;
                true
            }
            Msg::NegititiveOne =>{
                self.counter -= 1;
                    true
            }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <div class = "Container">
                <button onclick = {link.callback(|_|Msg::NegititiveOne)}> {"-1"} </button>
                <p>{self.counter} </p>
                <button onclick = {link.callback(|_|Msg::AddOne)}> {"+1"} </button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<CounterComp>::new().render();
}
