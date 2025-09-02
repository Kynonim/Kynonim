use gloo::{timers::callback::Timeout, utils::document};
use web_sys::{js_sys::{Date, Math}, Element, HtmlElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::utils::{image::*, statis::*};

#[function_component(MainContent)]
pub fn main_content() -> Html {
  let card_reference = use_node_ref();
  let konten_reference = use_node_ref();
  let karakter_index = use_state(|| 0);
  let teks_index = use_state(|| 0);
  let is_hapus = use_state(|| false);

  {
    let konten_reference = konten_reference.clone();
    let karakter_index = karakter_index.clone();
    let teks_index = teks_index.clone();
    let is_hapus = is_hapus.clone();

    use_effect_with((*teks_index, *karakter_index, *is_hapus), move |_| {
      let mut timer = Timeout::new(0, || {});
      if let Some(konten) = konten_reference.cast::<HtmlElement>() {
        let current_teks = KONTEN[*teks_index];

        if !*is_hapus {
          if *karakter_index <= current_teks.len() {
            let sub = current_teks.get(0..*karakter_index).unwrap_or("");
            konten.set_inner_text(sub);

            if *karakter_index == current_teks.len() {
              timer = Timeout::new(2000, move || is_hapus.set(true));
            } else {
              timer = Timeout::new(100, move || karakter_index.set(*karakter_index + 1));
            }
          }
        } else {
          if *karakter_index > 0 {
            let sub = current_teks.get(0..*karakter_index).unwrap_or("");
            konten.set_inner_text(sub);
            timer = Timeout::new(50, move || karakter_index.set(*karakter_index - 1));
          } else {
            is_hapus.set(false);
            teks_index.set((*teks_index + 1) % KONTEN.len());
          }
        }
      }
      move || {
        timer.cancel();
      }
    });
  }

  let onmousemove = {
    let card = card_reference.clone();
    Callback::from(move |_e: MouseEvent| {
      if let Some(element) = card.cast::<HtmlElement>() {
        let rect = element.get_bounding_client_rect();
        let mx = _e.client_x() as f64 - (rect.left() + rect.width() / 2.0);
        let my = _e.client_y() as f64 - (rect.top() + rect.height() / 2.0);

        let _ = element.style().set_property("--mx", &format!("{}px", mx * 0.03));
        let _ = element.style().set_property("--my", &format!("{}px", my * 0.03));
        let _ = element.style().set_property("--rx", &format!("{}deg", my * -0.02));
        let _ = element.style().set_property("--ry", &format!("{}deg", mx * 0.02));
      }
    })
  };

  let onmouseleave = {
    let card = card_reference.clone();
    Callback::from(move |_e: MouseEvent| {
      if let Some(element) = card.cast::<HtmlElement>() {
        let _ = element.style().set_property("--mx", "0px");
        let _ = element.style().set_property("--my", "0px");
        let _ = element.style().set_property("--rx", "0deg");
        let _ = element.style().set_property("--ry", "0deg");
      }
    })
  };

  html! {
    <div class="grid md:grid-cols-2 gap-10 items-center">

      // nama dan info
      <div>
        <div class="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-white/80 backdrop-blur">{"Lagi Nganggur 😂"}</div>
        <div class="relative h-[80px]">
          <h1 class="absolute top-0 left-0 w-full" style={format!("{}; -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: 900; font-size: clamp(34px, 5vw, 60px);", WARNA[*teks_index])} ref={konten_reference}>{"Riky Ripaldo"}</h1>
        </div>
        <span class="text-white text-3xl">{"Programmer Malas | Enggan Ngoding"}</span>
        <p class="mt-4 text-white/70 max-w-xl">{"AI terus belajar, Kamu terus rebahan — Mimpi jadi Programmer cuma jadi status doang"}</p>
        
        <div class="mt-8 flex items-center gap-3">
          <a class="relative inline-flex items-center gap-2 rounded-2xl px-5 py-3 bg-white/10 hover:bg-white/15 border border-white/10 hover:shadow-[0_0_25px_white]" href="https://github.com/Kynonim?tab=repositories" target="_blank">{"Lihat Proyek"}
            <IconProyek/>
          </a>
          <Link<Route> classes="relative inline-flex items-center gap-2 rounded-2xl px-5 py-3 bg-white/5 border border-white/10" to={Route::MotivasiPages}>{"Motivasi"}</Link<Route>>
        </div>
      </div>

      // gambar profile
      <div>
        <div class="p-2">
          <div ref={card_reference} {onmouseleave} {onmousemove} class="relative overflow-hidden rounded-2xl border border-white/10 bg-white/5 backdrop-blur transform-gpu transition-transform duration-300 hover:shadow-[0_0_25px_white]" style="transform: perspective(1000px) rotateX(var(--rx,0deg)) rotateY(var(--ry,0deg));">
            <img src="https://avatars.githubusercontent.com/u/64513539?v=4" alt="My Profile" class="h-[520px] w-[520px] object-cover"/>
            <div class="absolute inset-0 bg-gradient-to-t from-[#0b0b0f] via-transparent to-transparent"></div>
            <div class="absolute bottom-3 left-3 right-3 flex items-center justify-between">
              <div class="text-sm">
                <div class="font-semibold">{"Live Preview"}</div>
                <div class="text-white/70">{"Github Avatar"}</div>
              </div>
              <Link<Route> to={Route::CryptarithmPages} classes="inline-flex items-center gap-1 text-sm bg-white/10 px-3 py-1 rounded-full border border-white/10 hover:bg-white/15">{"Kynonim"}</Link<Route>>
            </div>
          </div>
        </div>
      </div>

    </div>
  }
}

#[function_component(MainProyek)]
pub fn main_proyek() -> Html {
  html! {
    <section id="proyek" class="mt-20">
      <h2 class="text-3xl font-bold">{"My Proyek"}</h2>
      <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-6">
        {
          for PROYEK.iter().map(|x| html! {
            <div class="rounded-2xl overflow-hidden border border-white/10 bg-white/5 p-0">
              <img src={x.image} alt={x.title} class="h-48 w-full object-cover"/>
              <div class="p-4">
                <h3 class="font-semibold">{ &x.title }</h3>
                <p class="text-sm text-white/80 mt-1">{ &x.desc }</p>
                <div class="flex gap-2 flex-wrap mt-3">
                  {
                    for x.tags.iter().map(|y| html! {
                      <span class="text-xs rounded-full border border-white/10 bg-white/5 px-2 py-0.5">{ y }</span>
                    })
                  }
                </div>
              </div>
            </div>
          })
        }
      </div>
    </section>
  }
}

#[function_component(MainSkill)]
pub fn main_skill() -> Html {
  html! {
    <section id="skill" class="mt-20">
      <h2 class="text-3xl font-bold">{"My Skill"}</h2>
      <div class="grid md:grid-cols-2 gap-6 mt-6">
        {
          for SKILL.iter().map(|x| html! {
            <div class="rounded-2xl border border-white/10 bg-white/5 p-5">
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center gap-2 text-sm">
                  <span class="opacity-80">{ &x.icon }</span>
                  <span class="font-medium">{ &x.name }</span>
                </div>
                <span class="text-white/60 text-sm">{ format!("{}%", x.level) }</span>
              </div>
              <div class="h-2 rounded-full bg-white/10 overflow-hidden">
                <div class="h-full bg-gradient-to-r from-cyan-400 to-violet-400" style={ format!("width: {}%", x.level) }></div>
              </div> 
            </div>
          })
        }
      </div>
    </section>
  }
}

#[function_component(MainKontak)]
pub fn main_kontak() -> Html {
  html! {
    <section id="kontak" class="mt-20 mb-28">
      <h2 class="text-3xl font-bold">{"Contact - About me"}</h2>
      <div class="rounded-2xl border border-white/10 bg-white/5 p-8 mt-6 text-center">
        <p class="text-white/80 max-w-2xl mx-auto">{"Kamu terlalu rajin ngoding dan ingin jadi malas? Yuk ngobrol sini."}</p>
        <div class="mt-6 flex items-center justify-center gap-3">
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="mailto:rikyripaldo@icloud.com"><IconEmail/></a>
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="https://github.com/Kynonim"><IconGithub/></a>
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="https://www.facebook.com/rikyxdz"><IconFacebook/></a>
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="https://www.instagram.com/rikyxdz"><IconInstagram/></a>
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="https://wa.me/817093921367&text=Assalamualaikum"><IconWhatsapp/></a>
        </div>
      </div>
      <footer class="text-center text-white/50 text-sm mt-10">{ format!("© {} Riky Ripaldo. Dibuat dengan Yew + Tailwind CSS", Date::new_0().get_full_year())}</footer>
    </section>
  }
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
  let tampilkan = use_state(|| false);

  let klik = {
    let tampilkan = tampilkan.clone();
    Callback::from(move |_e: MouseEvent| tampilkan.set(!*tampilkan))
  };

  html! {
    <div class="fixed top-4 right-4 z-50">
			<button class="p-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10"
				onclick={klik} aria-label="Open menu">
				<IconHamburger/>
			</button>

      if *tampilkan {
        <div class="absolute top-14 right-0 w-56 rounded-2xl border border-white/10 bg-white/5 p-8 mt-6">
          <ul class="space-y-2">
            <li><a href="#home" class="flex text-white duration-300 hover:text-[#6366f1] hover:scale-110">{"Beranda"}</a></li>
            <li><a href="#proyek" class="flex text-white duration-300 hover:text-[#6366f1] hover:scale-110">{"Proyek"}</a></li>
            <li><a href="#skill" class="flex text-white duration-300 hover:text-[#6366f1] hover:scale-110">{"Skill"}</a></li>
            <li><a href="#kontak" class="flex text-white duration-300 hover:text-[#6366f1] hover:scale-110">{"Kontak"}</a></li>
            <Link<Route> to={Route::GamePages} classes="flex text-white duration-300 hover:text-[#6366f1] hover:scale-110">{"Game"}</Link<Route>>
            <Link<Route> to={Route::CryptarithmPages} classes="flex text-white duration-300 hover:text-[#6366f1] hover:scale-110">{"Cryptarithm"}</Link<Route>>
            <Link<Route> to={Route::MotivasiPages} classes="flex text-white duration-300 hover:text-[#6366f1] hover:scale-110">{"Motivasi"}</Link<Route>>
            <li>
              <button onclick={
                let show = tampilkan.clone();
                Callback::from(move |_| show.set(false))
              } class="flex text-red-500 duration-300 hover:text-[#10b981] hover:scale-110">{"Close"}</button>
            </li>
          </ul>
        </div>
      }
		</div>
  }
}

#[function_component(LatarBelakang)]
pub fn latar_belakang() -> Html {
  let node = use_node_ref();
  {
    let node = node.clone();
    use_effect(move || {
      let element = node.cast::<Element>().unwrap();
      for i in 0..32u32 {
        let doc = document().create_element("i").unwrap();

        let left = (i * 137) % 100;
        let top = (i * 97) % 100;

        let size = (Math::random() * 6.0 + 4.0).round();
        let brightness = Math::random() * 0.5 + 0.5;
        let delay = i as f32 * 0.2;

        doc.set_attribute("style", &format!("left: {}%; top: {}%; animation-delay: {}s; width: {}px; height: {}px; opacity: {};", left, top, delay, size, size, brightness)).ok();
        element.append_child(&doc).ok();
      }
      || {}
    });
  }

  html! {
    <div ref={node} class={"latarbelakang"}></div>
  }
}