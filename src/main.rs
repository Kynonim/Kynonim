use leptos::prelude::*;
use riky::components::section::MainContent;

#[component]
fn App() -> impl IntoView {
	view! {
		<MainContent />
	}
}

fn main() {
  leptos::mount::mount_to_body(App);
}
