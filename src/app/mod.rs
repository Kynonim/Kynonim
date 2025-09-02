use gloo::events::EventListener;
use web_sys::window;
use yew::prelude::*;

use crate::components::section::*;

pub mod game;
pub mod motivasi;
pub mod cryptarithm;

#[derive(Properties, PartialEq)]
struct AnimasiProps {
	#[prop_or(100.0)]
	ambang: f64, //threshold scroll default 100px
	#[prop_or(700)]
	durasi: u32, //duration transition default 700ms
	#[prop_or("up".to_string())] //arah muncul up/left/right
	arah: String,
	children: Children,
}

#[function_component(PortofolioPages)]
pub fn portofolio_pages() -> Html {
  html! {
		<div id="home" class="min-h-screen bg-[#0b0b0f] text-white antialized">
		  <Navbar/>
			<div class="relative overflow-hidden">
				<LatarBelakang/>
				<main class="mx-auto max-w-6xl px-4 pt-20 pb-36 space-y-20">
				  <MainContent/>
					<Animasi ambang={500.0} durasi={1000} arah={"left".to_string()}><MainProyek/></Animasi>
					<Animasi ambang={900.0} durasi={800} arah={"right".to_string()}><MainSkill/></Animasi>
					<Animasi ambang={1000.0} arah={"up".to_string()}><MainKontak/></Animasi>
				</main>
			</div>
		</div>
	}
}

#[function_component(Animasi)]
fn animasi(props: &AnimasiProps) -> Html {
	let visible = use_state(|| false);
	{
		let visible = visible.clone();
		let ambang = props.ambang;

		use_effect_with((), move |_| {
			let callback = EventListener::new(&window().unwrap(), "scroll", move |_| {
				if window().unwrap().scroll_y().unwrap() > ambang {
					visible.set(true);
				}
			});
			move || drop(callback)
		});
	}

	html! {
		<div class={if *visible {
			match props.arah.as_str() {
				"left" => format!("opacity-100 translate-x-0 transition-all duration-{} ease-out", props.durasi),
				"right" => format!("opacity-100 translate-x-0 transition-all duration-{} ease-out", props.durasi),
				_ => format!("opacity-100 translate-y-0 transition-all duration-{} ease-out", props.durasi),
			}
		} else {
			match props.arah.as_str() {
				"left" => format!("opacity-0 -translate-x-10 transition-all duration-{} ease-out", props.durasi),
				"right" => format!("opacity-0 translate-x-10 transition-all duration-{} ease-out", props.durasi),
				_ => format!("opacity-0 translate-y-10 transition-all duration-{} ease-out", props.durasi),
			}
		}}>
		  { for props.children.iter() }
		</div>
	}
}