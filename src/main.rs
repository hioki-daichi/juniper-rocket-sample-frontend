use yew::prelude::*;
use yew::services::ConsoleService;

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    i: u8,
    console: ConsoleService,
}

enum Msg {
    Increment,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _component_link: ComponentLink<Self>) -> Self {
        Model {
            i: 0,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.log(self.i.to_string().as_str());

        match msg {
            Msg::Increment => self.i += 1,
        }

        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="container mx-auto">
                <button onclick=|_| Msg::Increment>{ "Increment!" }</button>
                {{ self.i }}
            </div>
        }
    }
}
