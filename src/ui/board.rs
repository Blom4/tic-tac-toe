use crate::game::{Player, Board};
use leptos::prelude::*;
use super::cell::Cell;

#[component]
pub fn Board(
    board: Board,
    winner: Option<Player>,
    winning_line: Option<[usize; 3]>,
    on_click: impl Fn(usize) + Clone + 'static,
) -> impl IntoView {
    let cells = board.cells();
    let line = winning_line.unwrap_or([0, 0, 0]);

    view! {
        <div class="grid grid-cols-3 gap-1 bg-gray-800 p-1 rounded-lg">
            {cells.iter().enumerate().map(|(i, &cell)| {
                let is_winning = winner.is_some() && line.contains(&i);
                view! {
                    <Cell
                        player=cell
                        index=i
                        winning=is_winning
                        on_click=on_click.clone()
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}