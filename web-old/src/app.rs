use yew::prelude::*;
use bounce::BounceRoot;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
			<BounceRoot>
				<Route />
			</BounceRoot>
            
        }
    }
}

use crate::layout::Layout;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum MyRoute {
	#[at("/")]
	Home,
	#[at("/about")]
	About,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub struct Route;

impl Component for Route {
	type Message = ();
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
		false
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
			<BrowserRouter>
				// <Layout>
					<Switch<MyRoute> render={Switch::render(switch)} />
				// </Layout>
			</BrowserRouter>
		}
	}
}

fn switch(routes: &MyRoute) -> Html {
    use crate::pages;
	match routes {
		MyRoute::Home => {
			html! {<pages::TaskPage />}
		}
		MyRoute::About => {
			html! {<div>{"About"}</div>}
		}
		_ => {
			html! {<div>{"404 Not Found"}</div>}
		}
	}
}
