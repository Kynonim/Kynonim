use std::{collections::{HashMap, HashSet}};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::section::LatarBelakang;

#[function_component(CryptarithmPages)]
pub fn cryptarithm_pages() -> Html {
  let kata_1 = use_state(|| String::new());
  let kata_2 = use_state(|| String::new());
  let kata_3 = use_state(|| String::new());
  let solusi: UseStateHandle<Vec<String>> = use_state(|| vec![]);

  let onclick = {
    let kata_1 = kata_1.clone();
    let kata_2 = kata_2.clone();
    let kata_3 = kata_3.clone();
    let solusi = solusi.clone();
    
    Callback::from(move |_: MouseEvent| {
      let x = &kata_1.trim().to_uppercase();
      let y = &kata_2.trim().to_uppercase();
      let z = &kata_3.trim().to_uppercase();

      if x.is_empty() || y.is_empty() || z.is_empty() {
        solusi.set(vec!["Lengkapi huruf terlebih dahulu !".to_string()]);
        return;
      }

      let mut unik: HashSet<char> = HashSet::new();
      for i in x.chars().chain(y.chars()).chain(z.chars()) {
        unik.insert(i);
      }
      let letters: Vec<char> = unik.into_iter().collect();

      if letters.len() > 10 {
        solusi.set(vec!["Terlalu banyak huruf unik".to_string()]);
        return;
      }
      let mut hasil = vec![];

      fn backtrack(letters: &[char], index: usize, used: &mut [bool; 10], mapping: &mut HashMap<char, u8>, result: &mut Vec<String>, x: &str, y: &str, z: &str) {
        if index == letters.len() {
          if valid(mapping, x, y, z) {
            let a: String = x.chars().map(|c| mapping[&c].to_string()).collect();
            let b: String = y.chars().map(|c| mapping[&c].to_string()).collect();
            let c: String = z.chars().map(|c| mapping[&c].to_string()).collect();

            result.push(format!("{}.{}.{}", a, b, c));
          }
          return;
        }

        let lets = letters[index];
        for c in 0..10 {
          if !used[c as usize] {
            mapping.insert(lets, c);
            used[c as usize] = true;

            if (x.starts_with(lets) || y.starts_with(lets) || z.starts_with(lets)) && c == 0 {
              mapping.remove(&lets);
              used[c as usize] = false;
              continue;
            }
            backtrack(letters, index + 1, used, mapping, result, x, y, z);
            mapping.remove(&lets);
            used[c as usize] = false;
          }
        }
      }

      fn valid(mapping: &HashMap<char, u8>, x: &str, y: &str, z: &str) -> bool {
        let a = kata_ke_nomor(x, mapping);
        let b = kata_ke_nomor(y, mapping);
        let c = kata_ke_nomor(z, mapping);
        a + b == c
      }

      fn kata_ke_nomor(kata: &str, mapping: &HashMap<char, u8>) -> i64 {
        kata.chars().map(|c| mapping[&c] as i64).fold(0, |acc, d| acc * 10 + d)
      }

      let mut used = [false; 10];
      let mut mapping = HashMap::new();
      backtrack(&letters, 0, &mut used, &mut mapping, &mut hasil, x, y, z);

      if hasil.is_empty() {
        solusi.set(vec!["Tidak ada solusi".to_string()]);
      } else {
        solusi.set(hasil);
      }
    })
  };

  html! {
    <div class="min-h-screen bg-[#0b0b0f] text-white antialiased flex justify-center items-start pt-10">
      <LatarBelakang/>
      <div class="bg-black/20 p-6 rounded-xl shadow-xl w-[500px]">
        <h1 class="text-4xl text-center font-bold mb-6">{"Cryptarithm"}</h1>
        <div class="flex flex-col space-y-2 mb-6">
          <input
            class="bg-transparent border-none text-yellow-400 font-bold text-3xl outline-none uppercase"
            placeholder="ABC"
            value={(*kata_1).clone()}
            oninput={Callback::from(move |e: InputEvent| {
              let input: HtmlInputElement = e.target_unchecked_into();
              kata_1.set(input.value());
            })}
          />
          <input
            class="bg-transparent border-none text-yellow-400 font-bold text-3xl outline-none uppercase"
            placeholder="DEF"
            value={(*kata_2).clone()}
            oninput={Callback::from(move |e: InputEvent| {
              let input: HtmlInputElement = e.target_unchecked_into();
              kata_2.set(input.value());
            })}
          />

          <div class="flex items-center space-x-2">
            <div class="w-full h-1 bg-white rounded"></div>
            <button {onclick} class="px-3 py-1 bg-red-500 rounded text-white font-bold hover:bg-blue-500 hover:scale-110 transition-all duration-300">{"+"}</button>
          </div>

          <input
            class="bg-transparent border-none text-green-400 font-bold text-3xl outline-none uppercase"
            placeholder="GHI"
            value={(*kata_3).clone()}
            oninput={Callback::from(move |e: InputEvent| {
              let input: HtmlInputElement = e.target_unchecked_into();
              kata_3.set(input.value());
            })}
          />
        </div>
        <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {
            if solusi.is_empty() {
              html! { <p class="text-center text-gray-400 italic animate-pulse col-span-full">{"Cryptarithm Solver"}</p> }
            } else {
              html! {
                for solusi.iter().map(|s| {
                  let parts: Vec<&str> = s.split('.').collect();
                  if parts.len() != 3 {
                    return html! { <span class="text-center text-gray-400 italic animate-pulse col-span-full">{"Solusi tidak ditemukan !"}</span> }
                  }
                  let (a, b, c) = (parts[0], parts[1], parts[2]);

                  html! {
                    <div class="bg-black/40 p-3 rounded-lg font-bold text-xl text-right space-y-1">
                      <span class="text-2xl text-yellow-400">{a}</span>
                      <div class="flex items-center justify-end space-x-1">
                        <span class="inline-block text-3xl text-red-400">{"+"}</span>
                        <span class="text-2xl text-yellow-400">{b}</span>
                      </div>
                      <div class="border-t-2 border-white w-full"></div>
                      <span class="text-2xl text-green-400">{c}</span>
                    </div>
                  }
                })
              }
            }
          }
        </div>
      </div>
    </div>
  }
}