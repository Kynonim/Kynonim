use leptos::prelude::*;
use web_sys::{HtmlElement, js_sys::Math, wasm_bindgen::{JsCast, prelude::Closure}, window};
use crate::{components::section::LatarBelakang, utils::statis::MOTIVASI};

#[component]
pub fn MotivasiPage() -> impl IntoView {
  let kalimat = NodeRef::<leptos::html::Div>::new();

  Effect::new(move |_| {
    if let Some(win) = window() {
      let closure = Closure::wrap(Box::new(move || {
        if let Some(e) = kalimat.get() {
          let element = &e;
          let doc = window().unwrap().document().expect("tidak ada document");
          let div = doc.create_element("div").unwrap().dyn_into::<web_sys::HtmlDivElement>().unwrap();
          div.set_class_name("motivasi-acak");

          let index = (Math::random() * MOTIVASI.len() as f64).floor() as usize;
          let teks = MOTIVASI[index];
          let size = 14 + (Math::random() * (32 - 14) as f64).floor() as i32;

          div.set_inner_text(teks);
          let ds = HtmlElement::style(&div);
          ds.set_property("font-size", &format!("{}px", size)).unwrap();
          ds.set_property("position", "absolute").unwrap();
          ds.set_property("color", &format!("hsl({}, 70%, 50%)", (Math::random() * 360.0).floor())).unwrap();

          let lebar = (element.client_width() - 100).max(0);
          let tinggi = (element.client_height() - 50).max(0);
          let (pos_x, pos_y) = (
            (Math::random() * lebar as f64).floor(),
            (Math::random() * tinggi as f64).floor()
          );
          ds.set_property("left", &format!("{}px", pos_x)).unwrap();
          ds.set_property("top", &format!("{}px", pos_y)).unwrap();
          element.append_child(&div).unwrap();

          let hapus_div = Closure::once_into_js(move || div.remove());
          window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(hapus_div.as_ref().unchecked_ref(), 3000).unwrap();
        }
      }) as Box<dyn Fn()>);

      let interval = win.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 2000).unwrap();
      closure.forget();
      on_cleanup(move || {
        win.clear_interval_with_handle(interval);
      });
    }
  });

  view! {
    <div class="min-h-screen bg-[#0b0b0f] antialiased" node_ref=kalimat>
      <LatarBelakang />
    </div>
  }
}
