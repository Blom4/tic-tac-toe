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
    is_dark: bool,
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

    let mode_btn_class = move |active: bool| {
        if active {
            if is_dark { "px-4 py-2 rounded-lg font-medium transition-all bg-blue-600 text-white shadow-lg hover:shadow-xl active:scale-95" }
            else { "px-4 py-2 rounded-lg font-medium transition-all bg-blue-500 text-white shadow-lg hover:shadow-xl active:scale-95" }
        } else {
            if is_dark { "px-4 py-2 rounded-lg font-medium transition-all bg-gray-700 text-gray-200 hover:bg-gray-600 active:scale-95" }
            else { "px-4 py-2 rounded-lg font-medium transition-all bg-gray-200 text-gray-700 hover:bg-gray-300 active:scale-95" }
        }
    };

    let diff_btn_class = move |active: bool| {
        if active {
            if is_dark { "px-3 py-1 rounded text-sm font-medium transition-all bg-green-600 text-white shadow hover:shadow-lg active:scale-95" }
            else { "px-3 py-1 rounded text-sm font-medium transition-all bg-green-500 text-white shadow hover:shadow-lg active:scale-95" }
        } else {
            if is_dark { "px-3 py-1 rounded text-sm font-medium transition-all bg-gray-700 text-gray-300 hover:bg-gray-600 active:scale-95" }
            else { "px-3 py-1 rounded text-sm font-medium transition-all bg-gray-200 text-gray-700 hover:bg-gray-300 active:scale-95" }
        }
    };

    let action_btn_class = move |primary: bool, disabled: bool| {
        let base = if primary {
            if is_dark { "px-4 py-2 rounded-lg font-medium transition-all bg-orange-600 text-white shadow-lg hover:shadow-xl active:scale-95" }
            else { "px-4 py-2 rounded-lg font-medium transition-all bg-orange-500 text-white shadow-lg hover:shadow-xl active:scale-95" }
        } else {
            if is_dark { "px-4 py-2 rounded-lg font-medium transition-all bg-gray-700 text-gray-200 hover:bg-gray-600 active:scale-95" }
            else { "px-4 py-2 rounded-lg font-medium transition-all bg-gray-200 text-gray-700 hover:bg-gray-300 active:scale-95" }
        };
        if disabled {
            format!("{} opacity-40 cursor-not-allowed", base)
        } else {
            base.to_string()
        }
    };

    view! {
        <div class="flex flex-col gap-4 items-center">
            <div class="flex gap-2">
                <button
                    class=mode_btn_class(mode == GameMode::TwoPlayer)
                    on:click=move |_| omc(GameMode::TwoPlayer)
                >
                    2 Players
                </button>
                <button
                    class=mode_btn_class(mode == GameMode::VsAI)
                    on:click=move |_| omc2(GameMode::VsAI)
                >
                    vs AI
                </button>
            </div>

            <div class=difficulty_class>
                <button
                    class=diff_btn_class(difficulty == Difficulty::Easy)
                    on:click=move |_| odc(Difficulty::Easy)
                >
                    Easy
                </button>
                <button
                    class=diff_btn_class(difficulty == Difficulty::Medium)
                    on:click=move |_| odc2(Difficulty::Medium)
                >
                    Medium
                </button>
                <button
                    class=diff_btn_class(difficulty == Difficulty::Hard)
                    on:click=move |_| odc3(Difficulty::Hard)
                >
                    Hard
                </button>
            </div>

            <div class="flex flex-wrap justify-center gap-2">
                <button
                    class=action_btn_class(false, !can_undo)
                    disabled=!can_undo
                    on:click=move |_| onu()
                >
                    Undo
                </button>
                <button
                    class=action_btn_class(false, !can_redo)
                    disabled=!can_redo
                    on:click=move |_| onre()
                >
                    Redo
                </button>
                <button
                    class=action_btn_class(true, false)
                    on:click=move |_| onng()
                >
                    New Game
                </button>
            </div>

            <button
                class=move || if is_dark { "px-4 py-2 text-red-400 hover:text-red-300 text-sm transition-all" } else { "px-4 py-2 text-red-500 hover:text-red-700 text-sm transition-all" }
                on:click=move |_| onrs()
            >
                Reset Scores
            </button>
        </div>
    }
}