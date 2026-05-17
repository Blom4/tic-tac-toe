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
        let base = "w-16 h-16 xs:w-20 xs:h-20 sm:w-24 sm:h-24 text-3xl xs:text-4xl sm:text-5xl font-bold border-2 transition-all duration-200";
        let cell_style = match player {
            Some(Player::X) => " border-blue-300 dark:border-blue-700",
            Some(Player::O) => " border-red-300 dark:border-red-700",
            None => " hover:bg-gray-50 dark:hover:bg-gray-700",
        };
        let win = if winning { " bg-green-200 dark:bg-green-800 border-green-500 dark:border-green-400 animate-flash" } else { "" };
        format!("{}{}{}", base, cell_style, win)
    };

    let symbol_class = move || {
        match player {
            Some(Player::X) => "text-blue-500 dark:text-blue-400 animate-scale-in",
            Some(Player::O) => "text-red-500 dark:text-red-400 animate-scale-in",
            None => "text-transparent",
        }
    };

    view! {
        <button
            class=cell_class
            disabled=player.is_some() || winning
            on:click=move |_| on_click(index)
        >
            <span class=symbol_class>
                {player.map(|p| p.symbol())}
            </span>
        </button>
    }
}