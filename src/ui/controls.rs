use crate::game::{Difficulty, GameMode};
use leptos::prelude::*;

#[component]
pub fn Controls(
    mode: GameMode,
    difficulty: Difficulty,
    can_undo: bool,
    can_redo: bool,
    _is_game_over: bool,
    on_mode_change: impl Fn(GameMode) + Clone + 'static,
    on_difficulty_change: impl Fn(Difficulty) + Clone + 'static,
    on_new_game: impl Fn() + Clone + 'static,
    on_undo: impl Fn() + Clone + 'static,
    on_redo: impl Fn() + Clone + 'static,
    on_reset_scores: impl Fn() + Clone + 'static,
) -> impl IntoView {
    let difficulty_class = move || {
        if mode == GameMode::VsAI { "flex gap-2" } else { "hidden" }
    };

    let omc = on_mode_change.clone();
    let omc2 = on_mode_change.clone();
    let odc = on_difficulty_change.clone();
    let odc2 = on_difficulty_change.clone();
    let odc3 = on_difficulty_change.clone();
    let onu = on_undo.clone();
    let onre = on_redo.clone();
    let onng = on_new_game.clone();
    let onrs = on_reset_scores.clone();

    view! {
        <div class="flex flex-col gap-4 items-center">
            <div class="flex gap-2">
                <button
                    class=move || format!(
                        "px-4 py-2 rounded-lg font-medium transition-all {}",
                        if mode == GameMode::TwoPlayer {
                            "bg-blue-500 text-white"
                        } else {
                            "bg-gray-200 hover:bg-gray-300"
                        }
                    )
                    on:click=move |_| omc(GameMode::TwoPlayer)
                >
                    2 Players
                </button>
                <button
                    class=move || format!(
                        "px-4 py-2 rounded-lg font-medium transition-all {}",
                        if mode == GameMode::VsAI {
                            "bg-blue-500 text-white"
                        } else {
                            "bg-gray-200 hover:bg-gray-300"
                        }
                    )
                    on:click=move |_| omc2(GameMode::VsAI)
                >
                    vs AI
                </button>
            </div>

            <div class=difficulty_class>
                <button
                    class=move || format!(
                        "px-3 py-1 rounded text-sm font-medium transition-all {}",
                        if difficulty == Difficulty::Easy {
                            "bg-green-500 text-white"
                        } else {
                            "bg-gray-200 hover:bg-gray-300"
                        }
                    )
                    on:click=move |_| odc(Difficulty::Easy)
                >
                    Easy
                </button>
                <button
                    class=move || format!(
                        "px-3 py-1 rounded text-sm font-medium transition-all {}",
                        if difficulty == Difficulty::Medium {
                            "bg-green-500 text-white"
                        } else {
                            "bg-gray-200 hover:bg-gray-300"
                        }
                    )
                    on:click=move |_| odc2(Difficulty::Medium)
                >
                    Medium
                </button>
                <button
                    class=move || format!(
                        "px-3 py-1 rounded text-sm font-medium transition-all {}",
                        if difficulty == Difficulty::Hard {
                            "bg-green-500 text-white"
                        } else {
                            "bg-gray-200 hover:bg-gray-300"
                        }
                    )
                    on:click=move |_| odc3(Difficulty::Hard)
                >
                    Hard
                </button>
            </div>

            <div class="flex gap-2">
                <button
                    class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-lg font-medium transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                    disabled=!can_undo
                    on:click=move |_| onu()
                >
                    Undo
                </button>
                <button
                    class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-lg font-medium transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                    disabled=!can_redo
                    on:click=move |_| onre()
                >
                    Redo
                </button>
                <button
                    class="px-4 py-2 bg-orange-500 hover:bg-orange-600 text-white rounded-lg font-medium transition-all"
                    on:click=move |_| onng()
                >
                    New Game
                </button>
            </div>

            <button
                class="px-4 py-2 text-red-500 hover:text-red-700 text-sm transition-all"
                on:click=move |_| onrs()
            >
                Reset Scores
            </button>
        </div>
    }
}