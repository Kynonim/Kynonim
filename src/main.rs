use riky::{app::{cryptarithm::CryptarithmPages, game::GamePages, motivasi::MotivasiPages, PortofolioPages}, components::SiapaYangTau, utils::statis::Route};
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: Route) -> Html {
	match routes {
		Route::PortofolioPages => html! { <PortofolioPages/> },
		Route::GamePages => html! { <GamePages/> },
		Route::BucinPages => html! { <h1>{"belum dibuat karena malas"}</h1> },
		Route::CryptarithmPages => html! { <CryptarithmPages/> },
		Route::MotivasiPages => html! { <MotivasiPages/> },
		Route::SiapaYangTau => html! { <SiapaYangTau/> },
	}
}

#[function_component(App)]
fn app() -> Html {
	html! {
		<BrowserRouter>
		  <Switch<Route> render={switch} />
		</BrowserRouter>
	}
}

fn main() {
  yew::Renderer::<App>::new().render();
}
