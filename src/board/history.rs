// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::{Board, Player};
use getset::CopyGetters;

/// One turn of the game: the board state, whose turn it is in this state, and
/// the position that was played to reach it (`None` for the initial state).
#[derive(Clone, Debug)]
struct Snapshot {
    board: Board,
    current_player: Player,
    last_position: Option<(usize, usize)>,
}

#[derive(Clone, Debug, CopyGetters)]
pub struct History {
    #[getset(get_copy = "pub")]
    current_turn: usize,
    snapshots: Vec<Snapshot>,
}

impl History {
    pub fn new(board: Board) -> Self {
        Self {
            current_turn: 0,
            snapshots: vec![Snapshot {
                board,
                current_player: Player::default(),
                last_position: None,
            }],
        }
    }

    pub fn init(&mut self, board: Board) {
        self.current_turn = 0;
        self.snapshots.clear();
        self.snapshots.push(Snapshot {
            board,
            current_player: Player::default(),
            last_position: None,
        });
    }

    pub fn push(&mut self, played_position: (usize, usize), board: Board, next_player: Player) {
        if self.current_turn < self.snapshots.len() - 1 {
            self.snapshots.drain(self.current_turn + 1..);
        }
        self.snapshots.push(Snapshot {
            board,
            current_player: next_player,
            last_position: Some(played_position),
        });
        self.current_turn += 1;
    }

    pub fn go_prev(&mut self) {
        if self.current_turn != 0 {
            self.current_turn -= 1;
        }
    }

    pub fn go_next(&mut self) {
        if self.current_turn != self.snapshots.len() - 1 {
            self.current_turn += 1;
        }
    }

    pub fn board(&self) -> &Board {
        &self.snapshots[self.current_turn].board
    }

    pub fn current_player(&self) -> Player {
        self.snapshots[self.current_turn].current_player
    }

    /// Update whose turn it is in the current state without adding a snapshot.
    /// Used when a player is skipped: the board is unchanged but the turn
    /// advances.
    pub fn set_current_player(&mut self, player: Player) {
        self.snapshots[self.current_turn].current_player = player;
    }

    pub fn past_position(&self) -> Option<(usize, usize)> {
        self.snapshots[self.current_turn].last_position
    }

    /// Each move as `(player who moved, position played)`, in turn order.
    pub fn moves(&self) -> impl Iterator<Item = (Player, (usize, usize))> + '_ {
        self.snapshots
            .windows(2)
            .map(|pair| (pair[0].current_player, pair[1].last_position.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_board() -> Board {
        Board::try_new(5).unwrap()
    }

    #[test]
    fn navigation_tracks_current_player() {
        let board = dummy_board();
        let mut history = History::new(board.clone());
        assert_eq!(history.current_turn(), 0);
        assert_eq!(history.current_player(), Player::Zero);
        assert_eq!(history.past_position(), None);

        // Zero plays, then it is One's turn, and so on.
        history.push((0, 0), board.clone(), Player::One);
        history.push((0, 1), board.clone(), Player::Two);
        history.push((1, 1), board.clone(), Player::Zero);
        assert_eq!(history.current_turn(), 3);
        assert_eq!(history.current_player(), Player::Zero);

        history.go_prev();
        assert_eq!(history.current_player(), Player::Two);
        history.go_prev();
        assert_eq!(history.current_player(), Player::One);
        history.go_prev();
        assert_eq!(history.current_player(), Player::Zero);
        history.go_prev(); // cannot go before the start
        assert_eq!(history.current_turn(), 0);

        // Returning to the latest turn restores the correct current player.
        // This is the regression that previously left current_player stale.
        history.go_next();
        history.go_next();
        history.go_next();
        assert_eq!(history.current_turn(), 3);
        assert_eq!(history.current_player(), Player::Zero);
        history.go_next(); // cannot go past the latest
        assert_eq!(history.current_turn(), 3);
    }

    #[test]
    fn push_truncates_future_after_going_back() {
        let board = dummy_board();
        let mut history = History::new(board.clone());
        history.push((0, 0), board.clone(), Player::One);
        history.push((0, 1), board.clone(), Player::Two);
        history.go_prev(); // back to turn 1
        history.push((1, 1), board.clone(), Player::Zero); // discards old turn 2
        assert_eq!(history.current_turn(), 2);
        assert_eq!(history.current_player(), Player::Zero);
        let moves: Vec<_> = history.moves().collect();
        assert_eq!(moves, vec![(Player::Zero, (0, 0)), (Player::One, (1, 1))]);
    }

    #[test]
    fn skip_updates_current_player_without_snapshot() {
        let board = dummy_board();
        let mut history = History::new(board.clone());
        history.push((0, 0), board.clone(), Player::One);
        history.set_current_player(Player::Two); // One is skipped
        assert_eq!(history.current_turn(), 1);
        assert_eq!(history.current_player(), Player::Two);
    }
}
