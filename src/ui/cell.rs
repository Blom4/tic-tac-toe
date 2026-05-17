use crate::game::Player;
use leptos::prelude::*;

#[component]
pub fn Cell(
    player: Option<Player>,
    index: usize,
    winning: bool,
    on_click: impl Fn(usize) + 'static,
) -> impl IntoView {
    let cell_class = move || {
        let base = "w-20 h-20 sm:w-24 sm:h-24 text-4xl sm:text-5xl font-bold border-2 transition-all duration-200";
        let _cell = match player {
            Some(Player::X) => "text-blue-500",
            Some(Player::O) => "text-red-500",
            None => "hover:bg-gray-50",
        };
        let win = if winning { " bg-green-200 border-green-500" } else { "" };
        format!("{}{}", base, win)
    };

    view! {
        <button
            class=cell_class
            disabled=player.is_some() || winning
            on:click=move |_| on_click(index)
        >
            {player.map(|p| p.symbol())}
        </button>
    }
}