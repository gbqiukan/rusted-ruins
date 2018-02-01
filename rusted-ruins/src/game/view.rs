//! This module processes the view of characters

use array2d::*;
use common::gamedata::map::Map;
use common::gamedata::chara::CharaId;
use game::Game;
use game::InfoGetter;

/// The cache for determining player's view
pub struct ViewMap {
    visible: Array2d<bool>,
}

impl ViewMap {
    pub fn new() -> ViewMap {
        ViewMap {
            visible: Array2d::new(128, 128, false),
        }
    }

    fn reserve_size(&mut self, w: u32, h: u32) {
        let size = self.visible.size();
        if size.0 >= w || size.1 >= h {
            use std::cmp::max;
            self.visible = Array2d::new(max(size.0, w), max(size.1, h), false);
        }
    }

    fn fill(&mut self, w: u32, h: u32, value: bool) {
        for ny in 0..h {
            for nx in 0..w {
                self.visible[(nx, ny)] = value;
            }
        }
    }

    pub fn get_tile_visible(&self, pos: Vec2d) -> bool {
        self.visible[pos]
    }
}

pub fn update_view_map(game: &mut Game) {
    let map = game.gd.get_current_map();
    let (w, h) = map.size();
    let view_map = &mut game.view_map;
    view_map.reserve_size(w, h);

    if game.gd.get_current_mapid().is_region_map() {
        view_map.fill(w, h, true); // Fill by true when region map
        return;
    }

    // Fill by false
    view_map.fill(w, h, false); // Fill by false

    let player_pos = game.gd.player_pos();
    let player_view_range = game.gd.chara.get(CharaId::Player).params.view_range;

    for (_, pos) in MDistRangeIter::new(player_pos, player_view_range) {
        if !map.is_inside(pos) { continue; }
        view_map.visible[pos] = true;
    }
}

pub fn calc_visual_distance(map: &Map, orig: Vec2d, dist: Vec2d) -> Option<i32> {
    Some(dist.mdistance(orig))
}

