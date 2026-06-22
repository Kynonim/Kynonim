use gloo_timers::future::TimeoutFuture;
use leptos::{ev, html, prelude::*, task::spawn_local};
use web_sys::{MouseEvent, js_sys::{Date, Math}};

use crate::utils::{icons::*, statis::*};

#[component]
pub fn MainContent() -> impl IntoView {
  let card_reference = NodeRef::<html::Div>::new();
  let konten_reference = NodeRef::<html::H1>::new();
  let karakter_index = RwSignal::new(0);
  let teks_index = RwSignal::new(0);
  let is_hapus = RwSignal::new(false);

  Effect::new(move || {
    let idx_teks = teks_index.get();
    let idx_char = karakter_index.get();
    let hapus = is_hapus.get();

    if let Some(konten) = konten_reference.get() {
      let current_teks = KONTEN[idx_teks];

      if !hapus {
        if idx_char <= current_teks.len() {
          let sub = current_teks.get(0..idx_char).unwrap_or("");
          konten.set_inner_text(sub);
          if idx_char == current_teks.len() {
            spawn_local(async move {
              TimeoutFuture::new(2000).await;
              is_hapus.set(true);
            });
          } else {
            spawn_local(async move {
              TimeoutFuture::new(100).await;
              karakter_index.set(idx_char + 1);
            });
          }
        }
      } else {
        if idx_char > 0 {
          let sub = current_teks.get(0..idx_char).unwrap_or("");
          konten.set_inner_text(sub);
          spawn_local(async move {
            TimeoutFuture::new(50).await;
            karakter_index.set(idx_char - 1);
          });
        } else {
          is_hapus.set(false);
          teks_index.set((idx_teks + 1) % KONTEN.len());
        }
      }
    }
  });

  let onmousemove = move |e: MouseEvent| {
    if let Some(element) = card_reference.get() {
      let rect = element.get_bounding_client_rect();
      let mx = e.client_x() as f64 - (rect.left() + rect.width() / 2.0);
      let my = e.client_y() as f64 - (rect.top() + rect.height() / 2.0);

      let style = (*element).style();
      let _ = style.set_property("--mx", &format!("{}px", mx * 0.03));
      let _ = style.set_property("--my", &format!("{}px", my * 0.03));
      let _ = style.set_property("--rx", &format!("{}deg", my * -0.02));
      let _ = style.set_property("--ry", &format!("{}deg", mx * 0.02));
    }
  };

  let onmouseleave = move |_| {
    if let Some(element) = card_reference.get() {
      let style = (*element).style();
      let _ = style.set_property("--mx", "0px");
      let _ = style.set_property("--my", "0px");
      let _ = style.set_property("--rx", "0deg");
      let _ = style.set_property("--ry", "0deg");
    }
  };

  view! {
    <div class="grid md:grid-cols-2 gap-10 items-center">
      // nama dan info
      <div>
        <div class="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-white/80 backdrop-blur">"Lagi Nganggur 😂"</div>
        <div class="relative h-[80px]">
          <h1 class="absolute top-0 left-0 w-full" style={format!("{}; -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: 900; font-size: clamp(34px, 5vw, 60px);", WARNA[teks_index.get()])} node_ref=konten_reference >"Riky Ripaldo"</h1>
        </div>
        <span class="text-white text-3xl">"Programmer Malas | Enggan Ngoding"</span>
        <p class="mt-4 text-white/70 max-w-xl">"AI terus belajar, Kamu terus rebahan — Mimpi jadi Programmer cuma jadi status doang"</p>

        <div class="mt-8 flex items-center gap-3">
          <a class="relative inline-flex items-center gap-2 rounded-2xl px-5 py-3 bg-white/10 hover:bg-white/15 border border-white/10 hover:shadow-[0_0_25px_white]" href="https://github.com/Kynonim?tab=repositories" target="_blank">"Lihat Proyek"
            <IconProject />
          </a>
          <a class="relative inline-flex items-center gap-2 rounded-2xl px-5 py-3 bg-white/5 border border-white/10" href="/motivasi">"Motivasi"</a>
        </div>
      </div>

      // gambar profile
      <div>
        <div class="p-2">
          <div node_ref=card_reference on:mouseleave=onmouseleave on:mousemove=onmousemove class="relative overflow-hidden rounded-2xl border border-white/10 bg-white/5 backdrop-blur transform-gpu transition-transform duration-300 hover:shadow-[0_0_25px_white]" style="transform: perspective(1000px) rotateX(var(--rx,0deg)) rotateY(var(--ry,0deg));">
            <img src="https://avatars.githubusercontent.com/u/64513539?v=4" alt="My Profile" class="h-[520px] w-[520px] object-cover"/>
            <div class="absolute inset-0 bg-gradient-to-t from-[#0b0b0f] via-transparent to-transparent"></div>
            <div class="absolute bottom-3 left-3 right-3 flex items-center justify-between">
              <div class="text-sm">
                <div class="font-semibold">"Live Preview"</div>
                <div class="text-white/70">"Github Avatar"</div>
              </div>
              <a href="/cryptarithm" class="inline-flex items-center gap-1 text-sm bg-white/10 px-3 py-1 rounded-full border border-white/10 hover:bg-white/15">"Kynonim"</a>
            </div>
          </div>
        </div>
      </div>
    </div>
  }
}

#[component]
pub fn MainProjects() -> impl IntoView {
  view! {
    <section id="projects" class="mt-20">
      <h2 class="text-3xl font-bold">"My Projects"</h2>
      <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-6">
      {
        PROYEK.iter().map(|value| view! {
          <div class="rounded-2xl overflow-hidden border border-white/10 bg-white/5 p-0">
            <img src=value.image alt=value.title class="h-48 w-full object-cover"/>
            <div class="p-4">
              <h3 class="font-semibold">{value.title}</h3>
              <p class="text-sm text-white/80 mt-1">{value.desc}</p>
              <div class="flex gap-2 flex-wrap mt-3">
              {
                value.tags.iter().map(|tag| view! {
                  <span class="text-xs rounded-full border border-white/10 bg-white/5 px-2 py-0.5">{*tag}</span>
                }).collect::<Vec<_>>()
              }
              </div>
            </div>
          </div>
        }).collect::<Vec<_>>()
      }
      </div>
    </section>
  }
}

#[component]
pub fn MainSkills() -> impl IntoView {
  view! {
    <section id="skill" class="mt-20">
      <h2 class="text-3xl font-bold">"My Skills"</h2>
      <div class="grid md:grid-cols-2 gap-6 mt-6">
      {
        SKILL.iter().map(|value| view! {
          <div class="rounded-2xl border border-white/10 bg-white/5 p-5">
            <div class="flex items-center justify-between mb-2">
              <div class="flex items-center gap-2 text-sm">
                <span class="opacity-80">{value.icon}</span>
                <span class="font-medium">{value.name}</span>
              </div>
              <span class="text-white/60 text-sm">{format!("{}%", value.level)}</span>
            </div>
            <div class="h-2 rounded-full bg-white/10 overflow-hidden">
              <div class="h-full bg-gradient-to-r from-cyan-400 to-violet-400" style={format!("width: {}%", value.level)}></div>
            </div>
          </div>
        }).collect::<Vec<_>>()
      }
      </div>
    </section>
  }
}

#[component]
pub fn MainContacts() -> impl IntoView {
  view! {
    <section id="kontak" class="mt-20 mb-28">
      <h2 class="text-3xl font-bold">"Contacts - About me"</h2>
      <div class="rounded-2xl border border-white/10 bg-white/5 p-8 mt-6 text-center">
        <p class="text-white/80 max-w-2xl mx-auto">"Kamu terlalu rajin ngoding dan ingin jadi malas? Yuk ngobrol sini."</p>
        <div class="mt-6 flex items-center justify-center gap-3">
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="mailto:rikyripaldo@icloud.com"><IconEmail/></a>
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="https://github.com/Kynonim"><IconGithub/></a>
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="https://www.facebook.com/rikyxdz"><IconFacebook/></a>
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="https://www.instagram.com/rikyxdz"><IconInstagram/></a>
          <a class="inline-flex items-center gap-2 rounded-2xl px-4 py-2 bg-white/10 border border-white/10 hover:shadow-[0_0_25px_white] hover:scale-110 duration-500" href="https://wa.me/817093921367&text=Assalamualaikum"><IconWhatsapp/></a>
        </div>
      </div>
      <footer class="text-center text-white/50 text-sm mt-10">{format!("© {} Riky Ripaldo. Dibuat dengan Leptos + Tailwind CSS", Date::new_0().get_full_year())}</footer>
    </section>
  }
}

#[component]
pub fn Navbar() -> impl IntoView {
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
    <nav class=move || {
      let myclass = "fixed bottom-6 left-1/2 -translate-x-1/2 z-50 flex gap-6 px-6 py-3 rounded-full border border-white/10 bg-black/40 backdrop-blur-md shadow-[0_8px_32px_0_rgba(0,0,0,0.37)] transition-all duration-300 ease-in-out";
      if is_show_navbar.get() {
        format!("{} translate-y-0 opacity-100", myclass)
      } else {
        format!("{} translate-y-[200%] opacity-0", myclass)
      }
    }>
			<a href="/" class="text-white/80 hover:text-white transition-colors duration-200">"Home"</a>
			<a href="/cryptarithm" class="text-white/80 hover:text-white transition-colors duration-200">"Crypt"</a>
			<a href="/game" class="text-white/80 hover:text-white transition-colors duration-200">"Game"</a>
		</nav>
  }
}

#[component]
pub fn LatarBelakang() -> impl IntoView {
  let node = NodeRef::<html::Div>::new();

  Effect::new(move || {
    if let Some(element) = node.get() {
      for i in 0..32 {
        let doc = document().create_element("i").unwrap();
        let left = (i * 137) % 100;
        let top = (i * 97) % 100;

        let size = (Math::random() * 6.0 + 4.0).round();
        let brightness = Math::random() * 0.5 + 0.5;
        let delay = i as f32 * 0.2;

        doc.set_attribute("style", &format!("left: {}%; top: {}%; animation-delay: {}s; width: {}px; height: {}px; opacity: {};", left, top, delay, size, size, brightness)).ok();
        element.append_child(&doc).ok();
      }
    }
  });

  view! {
    <div node_ref=node class="latar-belakang"></div>
  }
}
