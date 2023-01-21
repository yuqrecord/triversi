// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::{Player, PLAYERS};
use getset::{Getters, MutGetters};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Availables {
    #[allow(clippy::type_complexity)]
    availables: HashMap<Player, HashMap<(usize, usize), HashSet<(usize, usize)>>>,
    positions_buf: Vec<(usize, usize)>,
}

impl Default for Availables {
    fn default() -> Self {
        Self {
            availables: PLAYERS
                .iter()
                .map(|player| (*player, HashMap::new()))
                .collect::<HashMap<_, _>>(),
            positions_buf: Vec::new(),
        }
    }
}

impl Deref for Availables {
    type Target = HashMap<Player, HashMap<(usize, usize), HashSet<(usize, usize)>>>;
    fn deref(&self) -> &Self::Target {
        &self.availables
    }
}

impl DerefMut for Availables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.availables
    }
}

impl Availables {
    pub fn add_or_extend(
        &mut self,
        player: Player,
        position: (usize, usize),
        candidate_list: Vec<(usize, usize)>,
    ) {
        for candidate in candidate_list {
            self.availables
                .get_mut(&player)
                .unwrap()
                .entry(position)
                .or_insert_with(|| HashSet::from([candidate]))
                .insert(candidate);
        }
    }
}
