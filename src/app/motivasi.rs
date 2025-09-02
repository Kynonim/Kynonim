use web_sys::{js_sys::Math, wasm_bindgen::{prelude::Closure, JsCast}, window, HtmlDivElement, HtmlElement};
use yew::prelude::*;

use crate::{components::section::LatarBelakang, utils::statis::MOTIVASI};

#[function_component(MotivasiPages)]
pub fn motivasi_pages() -> Html {
  let kata = use_node_ref();

  {
    let kata = kata.clone();
    use_effect_with((), move |_| {
      let jendela = window().expect("tidak ada window");
      
      let closure = Closure::wrap(Box::new(move || {
        if let Some(element) = kata.cast::<HtmlElement>() {
          let dokumen = window().unwrap().document().expect("tidak ada dokumen");
          let div: HtmlDivElement = dokumen.create_element("div").unwrap().dyn_into().unwrap();
          div.set_class_name("motivasi-acak");

          let index = (Math::random() * MOTIVASI.len() as f64).floor() as usize;
          let teks = MOTIVASI[index];
          let size = 14 + (Math::random() * (32 - 14) as f64).floor() as i32;
          div.set_inner_text(teks);
          div.style().set_property("font-size", &format!("{}px", size)).unwrap();
          div.style().set_property("position", "absolute").unwrap();
          div.style().set_property("color", &format!("hsl({}, 70%, 50%)", (Math::random() * 360.0).floor())).unwrap();

          let lebar = (element.client_width() - 100).max(0);
          let tinggi = (element.client_height() - 50).max(0);
          let posx = (Math::random() * lebar as f64).floor();
          let posy = (Math::random() * tinggi as f64).floor();
          div.style().set_property("left", &format!("{}px", posx)).unwrap();
          div.style().set_property("top", &format!("{}px", posy)).unwrap();
          element.append_child(&div).unwrap();

          let remdiv = div.clone();
          let hapus = Closure::once_into_js(move || remdiv.remove());
          window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(hapus.as_ref().unchecked_ref(), 3000).unwrap();
        }
      }) as Box<dyn Fn()>);
      let interval = jendela.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 2000).unwrap();
      closure.forget();

      move || {
        jendela.clear_interval_with_handle(interval);
      }
    });
  }

  html! {
    <div class="min-h-screen bg-[#0b0b0f] antialized" ref={kata}>
      <LatarBelakang/>
    </div>
  }
}