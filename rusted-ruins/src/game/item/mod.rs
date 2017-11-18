
use rand::{Rng, thread_rng};
use common::gamedata::item::*;
use common::gobj;
use text::obj_txt;

/// Generate new item on dungeon floor
pub fn gen_dungeon_item(floor_level: u32) -> Box<Item> {
    let idx: ::common::objholder::ItemIdx = gobj::id_to_idx("!plank");

    let itemcontent = ItemContent::Object;
    let item = Item {
        idx: idx,
        content: itemcontent,
    };
    Box::new(item)
}

