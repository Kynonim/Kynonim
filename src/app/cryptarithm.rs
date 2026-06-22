use leptos::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::components::section::*;

#[component]
pub fn CryptarithmPage() -> impl IntoView {
  let kata_first = RwSignal::new(String::new());
  let kata_second = RwSignal::new(String::new());
  let kata_third = RwSignal::new(String::new());
  let solusi = RwSignal::new(Vec::<String>::new());
  let total_result = RwSignal::new(0);

  let cari_solusi = move |_| {
    let x = kata_first.get().trim().to_uppercase();
    let y = kata_second.get().trim().to_uppercase();
    let z = kata_third.get().trim().to_uppercase();

    if x.is_empty() || y.is_empty() || z.is_empty() {
      solusi.set(vec!["Lengkapi input terlebih dahulu!".to_string()]);
      return;
    }

    let mut uniks = HashSet::<char>::new();
    for i in x.chars().chain(y.chars()).chain(z.chars()) {
      uniks.insert(i);
    }

    let letters = uniks.into_iter().collect::<Vec<char>>();
    if letters.len() > 10 {
      solusi.set(vec!["Terlalu banyak input, max 10!".to_string()]);
      return;
    }

    let mut hasil = Vec::<String>::new();
    let mut used = [false; 10];
    let mut mapping = HashMap::<char, u8>::new();
    backtrack(&letters, 0, &mut used,&mut mapping, &mut hasil, &x, &y, &z);

    if hasil.is_empty() {
      solusi.set(vec!["Tidak ada solusi yang ditemukan!".to_string()]);
    } else {
      solusi.set(hasil);
    }
  };

  view! {
    <div class="min-h-screen bg-[#0b0b0f] text-white antialiased flex justify-center items-start pt-10">
      <LatarBelakang />
      <div class="bg-black/20 p-6 rounded-xl shadow-xl w-[500px] sm:w-[600px] md:w-[700px] lg:w-[800px] xl:w-[900px]">
        <h1 class="text-4xl sm:text-5xl font-extrabold tracking-[0.2em] text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-green-500 mb-12 drop-shadow-[0_0_15px_rgba(6,182,212,0.4)] text-center">"Cryptarithm"</h1>
        <div class="flex flex-col space-y-2 mb-6 items-end w-full text-4xl sm:text-5xl">
          <input
            type="text"
            placeholder="ABC"
            prop:value=move || kata_first.get()
            on:input=move |e| kata_first.set(event_target_value(&e))
            class="w-full bg-transparent text-right text-amber-400 font-bold tracking-[0.3em] uppercase outline-none placeholder-amber-600 focus:text-amber-300 transition-all"
          />
          <div class="flex items-center w-full">
            <span class="text-center text-white font-bold">{"+"}</span>
            <input
              type="text"
              placeholder="DEF"
              prop:value=move || kata_second.get()
              on:input=move |e| kata_second.set(event_target_value(&e))
              class="w-full bg-transparent text-right text-amber-400 font-bold tracking-[0.3em] uppercase outline-none placeholder-amber-600 focus:text-amber-300 transition-all"
            />
          </div>
          <div class="w-full h-1 bg-gradient-to-r from-transparent via-slate-600 to-slate-600 my-2 shadow-[0_0_10px_rgba(255,255,255,0.1)]"></div>
          <input
            type="text"
            placeholder="GHI"
            prop:value=move || kata_third.get()
            on:input=move |e| kata_third.set(event_target_value(&e))
            class="w-full bg-transparent text-right text-green-400 font-bold tracking-[0.3em] uppercase outline-none placeholder-green-600 focus:text-green-300 transition-all"
          />
          <button on:click=cari_solusi class="w-full mt-12 py-4 bg-transparent border border-slate-700/50 text-white text-xl font-bold tracking-[0.2em] rounded-full hover:bg-green-600 transition-all duration-300 active:scale-95">
            {move || {
              let result = total_result.get();
              if result > 0 {
                format!("Result: {}", result)
              } else {
                "Calculate".to_string()
              }
            }}
          </button>
        </div>
        <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
        {move || {
          let list_solusi = solusi.get();
          total_result.set(list_solusi.len());
          if list_solusi.is_empty() {
            view! { <p class="text-center text-gray-400 italic animate-pulse col-span-full">"Cryptarithm Solver"</p> }.into_any()
          } else {
            list_solusi.iter().map(|value| {
              let parts = value.split('.').collect::<Vec<&str>>();
              if parts.len() != 3 {
                return view! {
                  <span class="text-center text-gray-400 italic animate-pulse col-span-full">"Solusi tidak ditemukan !"</span>
                }.into_any();
              }
              let (a, b, c) = (parts[0], parts[1], parts[2]);

              view! {
                <div class="bg-transparent border border-slate-700/50 p-6 rounded-2xl shadow-lg hover:border-slate-500/80 flex flex-col items-end font-bold">
                  <span class="text-2xl text-amber-400 tracking-[0.2em]">{a.to_string()}</span>
                  <div class="flex w-full justify-between items-center text-2xl mt-2">
                    <span class="text-white font-bold">"+"</span>
                    <span class="text-amber-400 tracking-[0.2em]">{b.to_string()}</span>
                  </div>
                  <div class="w-full h-1 bg-gradient-to-r from-transparent via-slate-600 to-slate-600 my-2 shadow-[0_0_10px_rgba(255,255,255,0.1)]"></div>
                  <span class="text-2xl text-green-400 tracking-[0.2em] drop-shadow-[0_0_8px_rgba(74,222,128,0.5)]">{c.to_string()}</span>
                </div>
              }.into_any()
            }).collect::<Vec<_>>().into_any()
          }
        }}
        </div>
      </div>
    </div>
  }
}

fn is_valid(mapping: &HashMap<char, u8>, x: &str, y: &str, z: &str) -> bool {
  let a = kata_ke_nomor(x, mapping);
  let b = kata_ke_nomor(y, mapping);
  let c = kata_ke_nomor(z, mapping);
  a + b == c
}

fn kata_ke_nomor(kata: &str, mapping: &HashMap<char, u8>) -> i64 {
  kata.chars().map(|c| mapping[&c] as i64).fold(0, |acc, d| acc * 10 + d)
}

fn backtrack(letters: &Vec<char>, index: usize, used: &mut [bool; 10], mapping: &mut HashMap<char, u8>, result: &mut Vec<String>, x: &str, y: &str, z: &str) {
  if index == letters.len() {
    if is_valid(mapping, x, y, z) {
      let a = x.chars().map(|c| mapping[&c].to_string()).collect::<String>();
      let b = y.chars().map(|c| mapping[&c].to_string()).collect::<String>();
      let c = z.chars().map(|c| mapping[&c].to_string()).collect::<String>();
      result.push(format!("{}.{}.{}", a, b, c));
    }
    return;
  }

  let letter = letters[index];
  for c in 0..10 {
    if !used[c as usize] {
      mapping.insert(letter, c);
      used[c as usize] = true;

      if (x.starts_with(letter) || y.starts_with(letter) || z.starts_with(letter)) && c == 0 {
        mapping.remove(&letter);
        used[c as usize] = false;
        continue;
      }

      backtrack(letters, index + 1, used, mapping, result, x, y, z);
      mapping.remove(&letter);
      used[c as usize] = false;
    }
  }
}
