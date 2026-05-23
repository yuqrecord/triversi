// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::{Player, PlayerMap};
use std::collections::{HashMap, HashSet};

/// For a settable position, the set of positions whose stones get flipped.
type PositionMap = HashMap<(usize, usize), HashSet<(usize, usize)>>;

#[derive(Clone, Debug)]
pub struct Availables {
    availables: PlayerMap<PositionMap>,
    positions_buf: Vec<(usize, usize)>,
}

impl Default for Availables {
    fn default() -> Self {
        Self {
            availables: PlayerMap::new(HashMap::new(), HashMap::new(), HashMap::new()),
            positions_buf: Vec::new(),
        }
    }
}

impl Availables {
    pub fn get(&self, player: Player) -> &PositionMap {
        self.availables.get(player)
    }

    pub fn get_mut(&mut self, player: Player) -> &mut PositionMap {
        self.availables.get_mut(player)
    }

    pub fn values(&self) -> impl Iterator<Item = &PositionMap> {
        self.availables.values()
    }

    pub fn positions_buf(&self) -> &Vec<(usize, usize)> {
        &self.positions_buf
    }

    pub fn positions_buf_mut(&mut self) -> &mut Vec<(usize, usize)> {
        &mut self.positions_buf
    }

    pub fn add_or_extend(
        &mut self,
        player: Player,
        position: (usize, usize),
        candidate_list: Vec<(usize, usize)>,
    ) {
        for candidate in candidate_list {
            self.availables
                .get_mut(player)
                .entry(position)
                .or_insert_with(|| HashSet::from([candidate]))
                .insert(candidate);
        }
    }
}
