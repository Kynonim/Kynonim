use gloo::timers::callback::Timeout;
use web_sys::HtmlElement;
use yew::prelude::*;
use crate::{components::section::LatarBelakang, utils::statis::*};

#[function_component(GamePages)]
pub fn game_pages() -> Html {
  let giliran = use_state(|| 'X');
  let papan = use_state(|| vec![' '; 9]);
  let pemenang = use_state(|| None::<char>);
  let is_seri = use_state(|| false);

  let konten_reference = use_node_ref();
  let konten_index = use_state(|| 0);
  let karakter_index = use_state(|| 0);
  let is_hapus = use_state(|| false);

  {
    let konten_reference = konten_reference.clone();
    let konten_index = konten_index.clone();
    let karakter_index = karakter_index.clone();
    let is_hapus = is_hapus.clone();

    use_effect_with((*konten_index, *karakter_index, *is_hapus), move |_| {
      let mut timer = Timeout::new(0, || {});
      if let Some(konten) = konten_reference.cast::<HtmlElement>() {
        let current_teks = GAME_KONTEN[*konten_index];

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
            konten_index.set((*konten_index + 1) % GAME_KONTEN.len());
          }
        }
      }
      move || {
        timer.cancel();
      }
    });
  }

  let onclick = {
    let giliran = giliran.clone();
    let papan = papan.clone();
    let pemenang = pemenang.clone();
    let is_seri = is_seri.clone();

    Callback::from(move |index: usize| {
      if pemenang.is_some() || papan[index] != ' ' || *is_seri || *giliran == 'O' { return; }

      let mut board = (*papan).clone();
      board[index] = 'X';
      papan.set(board.clone());

      if let Some(win) = check_winner(&board) {
        pemenang.set(Some(win));
        return;
      }

      if !board.contains(&' ') {
        is_seri.set(true);
        return;
      }

      let mut bot = board.clone();
      let best = best_move(&mut bot);
      bot[best] = 'O';
      papan.set(bot.clone());

      if let Some(win) = check_winner(&bot) {
        pemenang.set(Some(win));
        return;
      }

      if !bot.contains(&' ') {
        is_seri.set(true);
        return;
      }
    })
  };

  html! {
    <div class="flex flex-col items-center justify-center min-h-screen bg-[#0b0b0f] text-white">
      <LatarBelakang/>
      <div class="relative h-[80px]">
        <h1 class="text-3xl font-bold mb-6" style={format!("color: {}", GAME_WARNA[*konten_index])} ref={konten_reference}>{"Tic Tac Toe"}</h1>
      </div>
      <div class="grid grid-cols-3 gap-2">
      {
        for (0..9).map(|index| {
          let onclick = {
            let onclick = onclick.clone();
            Callback::from(move |_e: MouseEvent| onclick.emit(index))
          };
          html! {
            <button {onclick} class="w-30 h-30 flex items-center justify-center text-3xl font-bold rounded-lg bg-white/10 hover:bg-white/15 border border-white/10 hover:shadow-[0_0_25px_white] duration-300">{papan[index]}</button>
          }
        })
      }
      </div>
      <div class="mt-8 text-2xl font-bold hover:scale-[1.5] duration-500">
      {
        if let Some(win) = *pemenang {
          html! { <p class={if win == 'X' {"text-green-400"} else {"text-red-400"}}>{format!("{} Menang", if win == 'X' {"Kamu"} else {"Bot"})}</p> }
        } else if *is_seri {
          html! { <p class="text-yellow-400">{"Game Seri!"}</p> }
        } else {
          html! { <p>{format!("Giliran {}", if *giliran == 'X' {"Kamu"} else {"Bot"})}</p>}
        }
      }
      </div>
    </div>
  }
}

fn check_winner(board: &Vec<char>) -> Option<char> {
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