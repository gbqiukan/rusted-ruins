
use array2d::*;
use common::maptemplate::*;
use common::objholder::*;
use common::gamedata::OverlappedTile;
use common::gobj;
use common::piece_pattern::*;

pub struct EditingMap {
    pub property: MapProperty,
    pub width: u32,
    pub height: u32,
    pub tile: Array2d<OverlappedTile>,
    pub wall: Array2d<Option<WallIdx>>,
    pub deco: Array2d<Option<DecoIdx>>,
}

impl EditingMap {
    pub fn new(id: &str, width: u32, height: u32) -> EditingMap {
        let tile = Array2d::new(width, height, OverlappedTile::default());
        let wall = Array2d::new(width, height, None);
        let deco = Array2d::new(width, height, None);
        let property = MapProperty::new(id);
        EditingMap { property, width, height, tile, wall, deco }
    }

    pub fn set_tile(&mut self, pos: Vec2d, tile: TileIdx) {
        self.tile[pos] = tile.into();
    }

    pub fn set_wall(&mut self, pos: Vec2d, wall: Option<WallIdx>) {
        self.wall[pos] = wall;
    }

    pub fn set_deco(&mut self, pos: Vec2d, deco: Option<DecoIdx>) {
        self.deco[pos] = deco;
    }

    pub fn erase(&mut self, pos: Vec2d) {
        self.wall[pos] = None;
        self.deco[pos] = None;
    }

    pub fn tile_overlap(&mut self, pos: Vec2d, new_tile_idx: TileIdx) {
        let piece_pattern_flags = { // Workaround instead of NNL
            let f = |pos: Vec2d| {
                if let Some(t) = self.tile.get(pos) {
                    t.main_tile() == new_tile_idx
                } else {
                    false
                }
            };

            let mut piece_pattern_flags = PiecePatternFlags::new();

            piece_pattern_flags.set(Direction::N, f(pos + (0, -1)));
            piece_pattern_flags.set(Direction::S, f(pos + (0, 1)));
            piece_pattern_flags.set(Direction::E, f(pos + (1, 0)));
            piece_pattern_flags.set(Direction::W, f(pos + (-1,0)));
            piece_pattern_flags.set(Direction::NE, f(pos + (1, -1)));
            piece_pattern_flags.set(Direction::NW, f(pos + (-1, -1)));
            piece_pattern_flags.set(Direction::SE, f(pos + (1, 1)));
            piece_pattern_flags.set(Direction::SW, f(pos + (-1,1)));
            piece_pattern_flags
        };

        self.tile[pos][1].idx = new_tile_idx;
        self.tile[pos][1].piece_pattern = piece_pattern_flags.to_piece_pattern();
    }

    pub fn resize(&mut self, new_w: u32, new_h: u32) {
        self.width = new_w;
        self.height = new_h;
        let tile = self.tile.clip_with_default((0, 0), (new_w, new_h), OverlappedTile::default());
        self.tile = tile;
        let wall = self.wall.clip_with_default((0, 0), (new_w, new_h), None);
        self.wall = wall;
        let deco = self.deco.clip_with_default((0, 0), (new_w, new_h), None);
        self.deco = deco;
    }

    pub fn create_mapobj(&self) -> MapTemplateObject {
        let mut tile_table: Vec<String> = Vec::new();

        // Create table for TileIdx
        for &tile in self.tile.iter() {
            for i in 0..tile.len() {
                let tile_idx = tile[i].idx;
                let tile_id = gobj::idx_to_id(tile_idx);
                if tile_table.iter().all(|a| *a != tile_id) {
                    tile_table.push(tile_id.to_owned());
                }
            }
        }

        // Create converted tile map
        let mut tile_map = Array2d::new(self.width, self.height, OverlappedTileConverted::default());
        for (pos, tile) in self.tile.iter_with_idx() {
            tile_map[pos] = tile.conv_into(&tile_table);
        }

        let mut wall_table: Vec<String> = Vec::new();
        for wall_idx in self.wall.iter() {
            if let Some(wall_idx) = *wall_idx {
                let wall_id = gobj::idx_to_id(wall_idx);
                if wall_table.iter().all(|a| *a != wall_id) {
                    wall_table.push(wall_id.to_owned());
                }
            }
        }
        let mut wall_map = Array2d::new(self.width, self.height, None);
        for (pos, wall_idx) in self.wall.iter_with_idx() {
            if let Some(wall_idx) = *wall_idx {
                let wall_id = gobj::idx_to_id(wall_idx);
                let converted_idx =
                    wall_table.iter().enumerate().find(|&(_, a)| a == wall_id).unwrap().0 as u32;
                wall_map[pos] = Some(converted_idx);
            }
        }

        let mut deco_table: Vec<String> = Vec::new();
        for deco_idx in self.deco.iter() {
            if let Some(deco_idx) = *deco_idx {
                let deco_id = gobj::idx_to_id(deco_idx);
                if deco_table.iter().all(|a| *a != deco_id) {
                    deco_table.push(deco_id.to_owned());
                }
            }
        }
        let mut deco_map = Array2d::new(self.width, self.height, None);
        for (pos, deco_idx) in self.deco.iter_with_idx() {
            if let Some(deco_idx) = *deco_idx {
                let deco_id = gobj::idx_to_id(deco_idx);
                let converted_idx =
                    deco_table.iter().enumerate().find(|&(_, a)| a == deco_id).unwrap().0 as u32;
                deco_map[pos] = Some(converted_idx);
            }
        }

        MapTemplateObject {
            id: self.property.id.to_owned(),
            w: self.width,
            h: self.height,
            tile_table: tile_table,
            tile: tile_map,
            wall_table: wall_table,
            wall: wall_map,
            deco_table: deco_table,
            deco: deco_map,
            boundary: self.property.boundary,
        }
    }
}

#[derive(Debug)]
pub struct MapProperty {
    pub id: String,
    pub is_region_map: bool,
    pub boundary: MapTemplateBoundary,
}

impl MapProperty {
    fn new(id: &str) -> MapProperty {
        MapProperty {
            id: id.to_owned(),
            is_region_map: false,
            boundary: MapTemplateBoundary::default(),
        }
    }
}

impl From<MapTemplateObject> for EditingMap {
    fn from(obj: MapTemplateObject) -> EditingMap {
        let mut map = EditingMap::new(&obj.id, obj.w, obj.h);

        for (pos, c) in obj.tile.iter_with_idx() {
            map.tile[pos] = OverlappedTile::conv_from(*c, &obj.tile_table);
        }

        for (pos, i) in obj.wall.iter_with_idx() {
            if let Some(i) = *i {
                let wall_id = &obj.wall_table[i as usize];
                map.wall[pos] = Some(gobj::id_to_idx(wall_id));
            }
        }

        for (pos, i) in obj.deco.iter_with_idx() {
            if let Some(i) = *i {
                let deco_id = &obj.deco_table[i as usize];
                map.deco[pos] = Some(gobj::id_to_idx(deco_id));
            }
        }

        map.property.boundary = obj.boundary;
        
        map
    }
}

