use leptos::prelude::*;

#[component]
pub fn Score(
    x_wins: u32,
    o_wins: u32,
    draws: u32,
    current_player: crate::game::Player,
    mode: crate::game::GameMode,
    is_dark: bool,
) -> impl IntoView {
    let thinking_text = move || {
        if mode == crate::game::GameMode::VsAI && current_player == crate::game::Player::O {
            "AI thinking...".to_string()
        } else {
            String::new()
        }
    };

    let container_class = "flex flex-col items-center gap-2";

    let x_class = move || {
        if is_dark { "text-blue-400 font-bold text-xl" } else { "text-blue-500 font-bold text-xl" }
    };

    let o_class = move || {
        if is_dark { "text-red-400 font-bold text-xl" } else { "text-red-500 font-bold text-xl" }
    };

    let draw_class = move || {
        if is_dark { "text-gray-400 font-bold text-xl" } else { "text-gray-500 font-bold text-xl" }
    };

    let status_class = move || {
        if is_dark {
            match mode {
                crate::game::GameMode::TwoPlayer => "text-gray-300 font-medium",
                crate::game::GameMode::VsAI => {
                    if current_player == crate::game::Player::X {
                        "text-green-400 font-medium"
                    } else {
                        "text-yellow-400 font-medium animate-pulse-slow"
                    }
                }
            }
        } else {
            match mode {
                crate::game::GameMode::TwoPlayer => "text-gray-600 font-medium",
                crate::game::GameMode::VsAI => {
                    if current_player == crate::game::Player::X {
                        "text-green-600 font-medium"
                    } else {
                        "text-yellow-600 font-medium animate-pulse-slow"
                    }
                }
            }
        }
    };

    view! {
        <div class=container_class>
            <div class="flex gap-6 text-lg">
                <div class="flex items-center gap-2">
                    <span class=x_class>X</span>
                    <span class=move || if is_dark { "font-semibold text-white" } else { "font-semibold" }>{x_wins}</span>
                </div>
                <div class="flex items-center gap-2">
                    <span class=o_class>O</span>
                    <span class=move || if is_dark { "font-semibold text-white" } else { "font-semibold" }>{o_wins}</span>
                </div>
                <div class="flex items-center gap-2">
                    <span class=draw_class>=</span>
                    <span class=move || if is_dark { "font-semibold text-white" } else { "font-semibold" }>{draws}</span>
                </div>
            </div>
            <div class=status_class>
                {move || {
                    match mode {
                        crate::game::GameMode::TwoPlayer => {
                            format!("Current: {}", current_player.symbol())
                        }
                        crate::game::GameMode::VsAI => {
                            if current_player == crate::game::Player::X {
                                "Your turn".to_string()
                            } else {
                                thinking_text()
                            }
                        }
                    }
                }}
            </div>
        </div>
    }
}