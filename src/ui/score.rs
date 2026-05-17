use leptos::prelude::*;

#[component]
pub fn Score(
    x_wins: u32,
    o_wins: u32,
    draws: u32,
    current_player: crate::game::Player,
    mode: crate::game::GameMode,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-2">
            <div class="flex gap-6 text-lg">
                <div class="flex items-center gap-2">
                    <span class="text-blue-500 font-bold text-xl">X</span>
                    <span class="font-semibold">{x_wins}</span>
                </div>
                <div class="flex items-center gap-2">
                    <span class="text-red-500 font-bold text-xl">O</span>
                    <span class="font-semibold">{o_wins}</span>
                </div>
                <div class="flex items-center gap-2">
                    <span class="text-gray-500 font-bold text-xl">=</span>
                    <span class="font-semibold">{draws}</span>
                </div>
            </div>
            <div class="text-gray-600 font-medium">
                {move || {
                    let player = match mode {
                        crate::game::GameMode::TwoPlayer => {
                            format!("Current: {}", current_player.symbol())
                        }
                        crate::game::GameMode::VsAI => {
                            if current_player == crate::game::Player::X {
                                "Your turn".to_string()
                            } else {
                                "AI thinking...".to_string()
                            }
                        }
                    };
                    player
                }}
            </div>
        </div>
    }
}