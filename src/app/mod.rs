pub mod cryptarithm;
pub mod game;
pub use cryptarithm::CryptarithmPage;
pub use game::GamePage;

use leptos::{ev, prelude::*};
use crate::components::section::*;

#[component]
pub fn BerandaPage() -> impl IntoView {
  view! {
    <div class="min-h-screen bg-[#0b0b0f] text-white antialized">
      <div class="relative overflow-hidden">
        <LatarBelakang />
        <main class="mx-auto max-w-6xl px-4 pt-20 pb-36 space-y-20">
          <MainContent />
          <Animasi ambang={500.0} durasi="1000" arah="left"><MainProjects /></Animasi>
  				<Animasi ambang={900.0} durasi="800" arah="right"><MainSkills /></Animasi>
  				<Animasi ambang={1000.0} arah="up"><MainContacts /></Animasi>
        </main>
      </div>
    </div>
  }
}

#[component]
pub fn Animasi(
  #[prop(into, default = 100.0)]
  ambang: f64,
  #[prop(into, default = "700".to_string())]
  durasi: String,
  #[prop(into, default = "up".to_string())]
  arah: String,
  children: Children
) -> impl IntoView {
  let visible = RwSignal::new(false);

  window_event_listener(ev::scroll, move |_| {
    if let Some(win) = web_sys::window() {
      let scroll_y = win.scroll_y().unwrap_or_default();
      if scroll_y > ambang {
        visible.set(true);
      }
    }
  });

  view! {
    <div
      class=move || {
        let dur = &durasi;
        if visible.get() {
          match arah.as_str() {
            "left" => format!("opacity-100 translate-x-0 transition-all duration-{} ease-out", dur),
            "right" => format!("opacity-100 translate-x-0 transition-all duration-{} ease-out", dur),
            _ => format!("opacity-100 translate-y-0 transition-all duration-{} ease-out", dur),
          }
        } else {
          match arah.as_str() {
            "left" => format!("opacity-0 -translate-x-10 transition-all duration-{} ease-out", dur),
            "right" => format!("opacity-0 translate-x-10 transition-all duration-{} ease-out", dur),
            _ => format!("opacity-0 translate-y-10 transition-all duration-{} ease-out", dur),
          }
        }
      }
    >
      {children()}
    </div>
  }
}
