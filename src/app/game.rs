use std::time::Duration;
use leptos::prelude::*;
use crate::{components::section::LatarBelakang, utils::statis::*};

#[component]
pub fn GamePage() -> impl IntoView {
  let giliran = RwSignal::new('X');
  let papan = RwSignal::new(vec![' '; 9]);
  let pemenang = RwSignal::new(None::<char>);
  let is_seri = RwSignal::new(false);

  let konten_index = RwSignal::new(0);
  let karakter_index = RwSignal::new(0);
  let is_hapus = RwSignal::new(false);

  Effect::new(move |_| {
    let current_teks = GAME_KONTEN[konten_index.get()];
    let char_index = karakter_index.get();
    let hapus = is_hapus.get();

    if !hapus {
      if char_index < current_teks.len() {
        set_timeout(move || karakter_index.update(|i| *i += 1), Duration::from_millis(100));
      } else {
        set_timeout(move || is_hapus.set(true), Duration::from_millis(2000));
      }
    } else {
      if char_index > 0 {
        set_timeout(move || karakter_index.update(|i| *i -= 1), Duration::from_millis(50));
      } else {
        is_hapus.set(false);
        konten_index.update(|i| *i = (*i + 1) % GAME_KONTEN.len());
      }
    }
  });

  let teks_mengetik = Memo::new(move |_| {
    let current_teks = GAME_KONTEN[konten_index.get()];
    let char_index = karakter_index.get();
    current_teks.get(0..char_index).unwrap_or("").to_string()
  });

  let main = move |index: usize| {
    if pemenang.get().is_some() || papan.with(|p| p[index] != ' ') || is_seri.get() || giliran.get() == 'O' { return; }
    papan.update(|board| {
      board[index] = 'X';
      if let Some(win) = check_winner(board) {
        pemenang.set(Some(win));
        return;
      }

      if !board.contains(&' ') {
        is_seri.set(true);
        return;
      }

      let best = best_move(board);
      board[best] = 'O';
      if let Some(win) = check_winner(board) {
        pemenang.set(Some(win));
        return;
      }

      if !board.contains(&' ') {
        is_seri.set(true);
        return;
      }
    });
  };

  let reset_game = move |_| {
    papan.set(vec![' '; 9]);
    pemenang.set(None);
    is_seri.set(false);
    giliran.set('X');
  };

  view! {
    <div class="flex flex-col items-center justify-center min-h-screen bg-[#0b0b0f] text-slate-200 font-mono relative overflow-hidden select-none">
      <LatarBelakang />

      <div class="h-[60px] mb-8 flex items-center justify-center">
        <h1 class="text-3xl font-extrabold tracking-widest transition-colors duration-500 drop-shadow-[0_0_15px_rgba(255,255,255,0.2)]"
            style={move || format!("color: {};", GAME_WARNA[konten_index.get()])}>
            {move || teks_mengetik.get()}
            <span class="animate-pulse opacity-70">"|"</span>
        </h1>
      </div>

      <div class="grid grid-cols-3 gap-3 bg-transparent p-4">
      {
        (0..9).map(|index| {
          let click = move |_| main(index);
          view! {
            <button
              on:click=click
              class=move || format!(
                "w-24 h-24 sm:w-28 sm:h-28 flex items-center justify-center text-4xl font-black rounded-xl transition-all duration-300 transform active:scale-95 bg-transparent border border-slate-500/50 hover:border-slate-500/80 text-center {} {}",
                if papan.with(|p| p[index] == 'X') {"text-green-400"} else {"text-amber-500"},
                if papan.with(|p| p[index] == ' ') && giliran.get() == 'X' {"hover:shadow-[0_0_20px_rgba(6,182,212,0.3)]"} else {""}
              )
            >
              {move || match papan.with(|p| p[index]) {
                'X' => "X",
                'O' => "O",
                _ => ""
              }}
            </button>
          }
        }).collect::<Vec<_>>()
      }
      </div>

      <div class="mt-8 flex flex-col items-center space-y-4 min-h-[100px]">
        <div class="text-xl font-bold tracking-wider transition-all duration-300">
        {move || {
          if let Some(win) = pemenang.get() {
            if win == 'X' {
                view! { <p class="text-green-400 drop-shadow-[0_0_10px_rgba(74,222,128,0.4)] animate-bounce">"Kamu Menang!"</p> }.into_any()
            } else {
                view! { <p class="text-amber-400 drop-shadow-[0_0_10px_rgba(248,113,113,0.4)]">"Bot Menang!"</p> }.into_any()
            }
          } else if is_seri.get() {
            view! { <p class="text-yellow-400 drop-shadow-[0_0_10px_rgba(250,204,21,0.4)]">"Gane Seri!"</p> }.into_any()
          } else if giliran.get() == 'X' {
            view! { <p class="text-green-400">"Giliran Kamu (X)"</p> }.into_any()
          } else {
            view! { <p class="text-amber-400 animate-pulse">"Bot Sedang Berpikir..."</p> }.into_any()
          }
        }}
        </div>
        <div class="h-10">
        {move || (pemenang.get().is_some() || is_seri.get()).then(|| view! {
          <button
            on:click=reset_game
            class="px-6 py-2 bg-transparent border border-slate-500/50 hover:border-slate-500/80 hover:text-green-400 text-sm font-bold tracking-widest rounded-full transition-all duration-300 transform hover:scale-105"
          >
            "Main Lagi"
          </button>
        })}
        </div>
      </div>
    </div>
  }
}

fn best_move(board: &mut Vec<char>) -> usize {
  let mut move_index = 0;
  let mut best_score = i32::MIN;

  for i in 0..9 {
    if board[i] == ' ' {
      board[i] = 'O';
      let score = minimax(board, 0, false);
      board[i] = ' ';
      if score > best_score {
        best_score = score;
        move_index = i;
      }
    }
  }
  return move_index;
}

fn check_winner(board: &mut Vec<char>) -> Option<char> {
  let lines = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8], // baris
    [0, 3, 6], [1, 4, 7], [2, 5, 8], // diagonal
    [0, 4, 8], [2, 4, 6], // diagonal
  ];
  for line in lines.iter() {
    let [a, b, c] = *line;
    if board[a] != ' ' && board[a] == board[b] && board[a] == board[c] {
      return Some(board[a]);
    }
  }
  return None;
}

fn minimax(board: &mut Vec<char>, depth: i32, is_max: bool) -> i32 {
  if let Some(win) = check_winner(board) {
    return if win == 'O' { 10 - depth } else { depth - 10 };
  }
  if !board.contains(&' ') { return 0; }

  if is_max {
    let mut best_score = i32::MIN;
    for i in 0..9 {
      if board[i] == ' ' {
        board[i] = 'O';
        let score = minimax(board, depth + 1, false);
        board[i] = ' ';
        best_score = best_score.max(score);
      }
    }
    return best_score;
  } else {
    let mut best_score = i32::MAX;
    for i in 0..9 {
      if board[i] == ' ' {
        board[i] = 'X';
        let score = minimax(board, depth + 1, true);
        board[i] = ' ';
        best_score = best_score.min(score);
      }
    }
    return best_score;
  }
}
