use leptos::{ev, prelude::*};
use leptos_router::{components::{Route, Router, Routes}, path};
use riky::components::{SiapaYangTau, section::*};

#[component]
fn App() -> impl IntoView {
  let is_show_navbar = RwSignal::new(true);
  let last_scroll_pos = RwSignal::new(0.0);

  window_event_listener(ev::scroll, move |_| {
    if let Some(win) = web_sys::window() {
      let pos_now = win.scroll_y().unwrap_or_default();
      let pos_old = last_scroll_pos.get();

      if pos_now > pos_old && pos_now > 50.0 {
        is_show_navbar.set(false);
      } else {
        is_show_navbar.set(true);
      }
      last_scroll_pos.set(pos_now);
    }
  });

	view! {
		<Router>
		  <nav
				class=format!("fixed bottom-6 left-1/2 -translate-x-1/2 z-50 flex gap-6 px-6 py-3 rounded-full border border-white/10 bg-black/40 backdrop-blur-md shadow-[0_8px_32px_0_rgba(0,0,0,0.37)] transition-all duration-300 ease-in-out {}",
				if is_show_navbar.get() {"translate-y-0 opacity-100"} else {"translate-y-[200%] opacity-0"})
			>
				<a href="/" class="text-white/80 hover:text-white transition-colors duration-200">"Beranda"</a>
				<a href="/projects" class="text-white/80 hover:text-white transition-colors duration-200">"Projects"</a>
			</nav>
			<main class="p-8 pb-24">
			  <Routes fallback=|| view! { <SiapaYangTau /> }>
					<Route path=path!("") view=MainContent />
					<Route path=path!("/projects") view=MainProjects />
				</Routes>
			</main>
		</Router>
	}
}

fn main() {
  leptos::mount::mount_to_body(App);
}
