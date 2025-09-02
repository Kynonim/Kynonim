use yew::prelude::*;
use yew_router::prelude::*;
use crate::{components::section::LatarBelakang, utils::statis::Route};

pub mod section;

#[function_component(SiapaYangTau)]
pub fn siapa_yang_tau() -> Html {
  html! {
    <div class="flex flex-col items-center justify-center min-h-screen bg-[#0b0b0f] text-white">
      <LatarBelakang/>
      <h1 class="text-6xl font-bold">{"404"}</h1>
      <h3 class="text-3xl font-bold mt-2">{"PAGE_NOT_FOUND"}</h3>
      <p class="text-grey-500 text-sm mt-2">{"Mungkin url yang kamu masukan salah atau typo ?"}</p>
      <Link<Route> to={Route::PortofolioPages} classes="mt-3 relative inline-flex items-center gap-2 rounded-2xl px-5 py-3 bg-white/10 hover:bg-white/15 border border-white/10 hover:shadow-[0_0_25px_white]">{"Kembali"}</Link<Route>>
    </div>
  }
}