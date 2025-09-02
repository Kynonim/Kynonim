use yew_router::Routable;

pub struct Proyek {
  pub title: &'static str,
  pub desc: &'static str,
  pub tags: &'static [&'static str],
  pub image: &'static str,
}

pub struct Skill {
  pub name: &'static str,
  pub icon: &'static str,
  pub level: u8,
}

pub static PROYEK: &[Proyek] = &[
  Proyek {
    title: "Rehat - Sosial Media Mangkrak",
    desc: "Fullstack Actix Web + Yew, realtime like via WebSocket, UI bersih dengan animasi halus.",
    tags: &["Rust", "Actix", "Yew", "WebSocket"],
    image: "https://images.unsplash.com/photo-1555066931-4365d14bab8c?q=80&w=1600&auto=format&fit=crop",
  },
  Proyek {
    title: "MiniServe - Simple Backend dengan Rust",
    desc: "Backend dengan Actix Web dan token validasi JWT",
    tags: &["Rust", "Actix", "SQLite", "JWT"],
    image: "https://mblogthumb-phinf.pstatic.net/MjAyMzA0MTZfMjUg/MDAxNjgxNjQzODgwMzEx.YNOK_iGtn0SRu7JrL2QCFwmJBR5oTlTuBcsRs9k1pf8g.Ep25NsdmNEhdIvR8MomJIg3uhyWOY43ehsHV1cNmuYsg.PNG.shino1025/image.png?type=w800",
  },
  Proyek {
    title: "Scroll Fesnuk - Edisi Malas",
    desc: "Fullstack kemalasan with Facebook",
    tags: &["Reels", "Marketplace", "Grup", "Postingan"],
    image: "https://media.istockphoto.com/id/1631217382/video/female-fingers-scroll-through-the-news-feed-in-social-networks-on-a-mobile-phone.jpg?s=640x640&k=20&c=5oH9qBlZUecxAH57BzEySbRe-bPG91qBbLnS-YHB3aA=",
  },
];

pub static SKILL: &[Skill] = &[
  Skill { name: "React", level: 12, icon: "JS/TS"},
  Skill { name: "Yew", level: 15, icon: "Rust"},
  Skill { name: "Actix", level: 2, icon: "Rust"},
  Skill { name: "Scroll Fesnuk", level: 99, icon: "Imphnen"},
];

pub const KONTEN: [&str; 4] = ["Riky Ripaldo", "Malas Ngoding", "Member Imphnen", "Kynonim"];
pub const WARNA: [&str; 4] = [
  "background:linear-gradient(90deg, #6366f1, #38bdf8, #10b981)",
  "background:linear-gradient(90deg, #38bdf8, #10b981, #6366f1)",
  "background:linear-gradient(90deg, #38bdf8, #6366f1, #10b981)",
  "background:linear-gradient(90deg, #10b981, #38bdf8, #6366f1)"
];

pub const MOTIVASI: [&str; 14] = [
    "Hallo Enggan Ngoding", "Jangan Ngoding", "Mending Turu", "Scroll Facebook Aja",
    "Mending Main Game", "Mending Nonton Anime", "Mending Nonton Film", "Mending Nonton TV",
    "Coding Hanya Buang Waktu", "Hapus VSCodenya", "Coding Hanyalah Tipuan", "Tetap Malas Ya",
    "Jangan Semanggat", "Jangan Pikirkan Masa Depan",
];

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	PortofolioPages,
	#[at("/game")]
	GamePages,
	#[at("/bucin")]
	BucinPages,
	#[at("/cryptarithm")]
	CryptarithmPages,
	#[at("/motivasi")]
	MotivasiPages,
	#[not_found]
	#[at("/404")]
	SiapaYangTau,
}

pub const GAME_KONTEN: [&str; 6] = ["Hallo Apa Kabar ?", "Baik Ya", "Selamat Datang Di..", "Tic Tac Toe", "Versus Bot", "Ayo Kalahkan Bot"];
pub const GAME_WARNA: [&str; 6] = ["#FF5733", "#33FF57", "#f5ff33", "#FF3357", "#7E33FFFF", "#33C9FFFF"];