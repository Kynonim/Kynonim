use leptos::prelude::*;
use leptos_router::{components::{Route, Router, Routes}, path};
use riky::{app::{BerandaPage, CryptarithmPage, GamePage}, components::{SiapaYangTau, section::*}};

#[component]
fn App() -> impl IntoView {
	view! {
		<Router>
		  <Navbar />
		  <Routes fallback=|| view! { <SiapaYangTau /> }>
				<Route path=path!("") view=BerandaPage />
				<Route path=path!("/cryptarithm") view=CryptarithmPage />
				<Route path=path!("/game") view=GamePage />
			</Routes>
		</Router>
	}
}

fn main() {
  leptos::mount::mount_to_body(App);
}
