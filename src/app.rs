use crate::game::{Difficulty, GameMode, Player, GameState};
use leptos::prelude::*;
use crate::ui::{Board, Controls, Score};

fn get_winning_line(cells: &[Option<Player>; 9]) -> Option<[usize; 3]> {
    let lines = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6],
    ];
    for line in lines {
        let a = cells[line[0]];
        let b = cells[line[1]];
        let c = cells[line[2]];
        if a.is_some() && a == b && b == c {
            return Some(line);
        }
    }
    None
}

#[component]
pub fn App() -> impl IntoView {
    let game_state = RwSignal::new(GameState::new());

    let winner = move || game_state.get().board.winner();
    let winning_line = move || get_winning_line(game_state.get().board.cells());

    let status = move || {
        if let Some(w) = winner() {
            match w {
                Player::X => "X Wins!",
                Player::O => "O Wins!",
            }
        } else if game_state.get().board.is_full() {
            "Draw!"
        } else {
            ""
        }
    };

    let handle_click = move |index: usize| {
        if winner().is_none() && !game_state.get().board.is_full() {
            let mut new_state = game_state.get();
            let _ = new_state.make_move(index);
            game_state.set(new_state);
        }
    };

    let handle_undo = move || {
        let mut new_state = game_state.get();
        let _ = new_state.undo();
        game_state.set(new_state);
    };

    let handle_redo = move || {
        let mut new_state = game_state.get();
        let _ = new_state.redo();
        game_state.set(new_state);
    };

    let handle_new_game = move || {
        let mut new_state = game_state.get();
        new_state.reset();
        game_state.set(new_state);
    };

    let handle_mode_change = move |mode: GameMode| {
        let mut new_state = game_state.get();
        new_state.set_mode(mode);
        game_state.set(new_state);
    };

    let handle_difficulty_change = move |difficulty: Difficulty| {
        let mut new_state = game_state.get();
        new_state.set_difficulty(difficulty);
        game_state.set(new_state);
    };

    let handle_reset_scores = move || {
        let mut new_state = game_state.get();
        new_state.reset_scores();
        game_state.set(new_state);
    };

    view! {
        <div class="min-h-screen bg-gray-100 flex flex-col items-center justify-center p-4">
            <h1 class="text-4xl font-bold text-gray-800 mb-8">Tic Tac Toe</h1>

            <Score
                x_wins=game_state.get().x_wins
                o_wins=game_state.get().o_wins
                draws=game_state.get().draws
                current_player=game_state.get().current_player
                mode=game_state.get().mode
            />

            <div class="my-6 h-8">
                <span class="text-2xl font-bold text-green-600">
                    {status()}
                </span>
            </div>

            <Board
                board=game_state.get().board
                winner=winner()
                winning_line=winning_line()
                on_click=handle_click
            />

            <div class="mt-6">
                <Controls
                    mode=game_state.get().mode
                    difficulty=game_state.get().difficulty
                    can_undo=game_state.get().board.can_undo()
                    can_redo=game_state.get().board.can_redo()
                    _is_game_over=winner().is_some() || game_state.get().board.is_full()
                    on_mode_change=handle_mode_change
                    on_difficulty_change=handle_difficulty_change
                    on_new_game=handle_new_game
                    on_undo=handle_undo
                    on_redo=handle_redo
                    on_reset_scores=handle_reset_scores
                />
            </div>
        </div>
    }
}