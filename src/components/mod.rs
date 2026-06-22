use leptos::prelude::*;

use crate::components::section::LatarBelakang;

pub mod section;

#[component]
pub fn SiapaYangTau() -> impl IntoView {
  view! {
    <div class="flex flex-col items-center justify-center min-h-screen bg-[#0b0b0f] text-white">
      <LatarBelakang/>
      <h1 class="text-6xl font-bold">{"404"}</h1>
      <h3 class="text-3xl font-bold mt-2">{"PAGE_NOT_FOUND"}</h3>
      <p class="text-grey-500 text-sm mt-2">{"Mungkin url yang kamu masukan salah atau typo ?"}</p>
      <a href="/" class="mt-4 relative inline-flex items-center gap-2 rounded-full px-5 py-3 bg-transparent border border-slate-700/60 hover:border-slate-500/80 hover:font-bold hover:shadow-[0_0_25px_white]">{"Kembali"}</a>
    </div>
  }
}
