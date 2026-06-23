use leptos::prelude::*;
use leptos_router::{components::*, path};
use riky::{app::*, components::{*, section::*}};

#[component]
fn App() -> impl IntoView {
	view! {
		<Router>
		  <Navbar />
		  <Routes fallback=|| view! { <SiapaYangTau /> }>
				<Route path=path!("") view=BerandaPage />
				<Route path=path!("/cryptarithm") view=CryptarithmPage />
				<Route path=path!("/game") view=GamePage />
				<Route path=path!("/motivasi") view=MotivasiPage />
			</Routes>
		</Router>
	}
}

fn main() {
  leptos::mount::mount_to_body(App);
}
