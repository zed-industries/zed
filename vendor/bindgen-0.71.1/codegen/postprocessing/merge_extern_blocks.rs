use syn::{
    visit_mut::{visit_file_mut, visit_item_mod_mut, VisitMut},
    File, Item, ItemForeignMod, ItemMod,
};

pub(super) fn merge_extern_blocks(file: &mut File) {
    Visitor.visit_file_mut(file);
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_file_mut(&mut self, file: &mut File) {
        visit_items(&mut file.items);
        visit_file_mut(self, file);
    }

    fn visit_item_mod_mut(&mut self, item_mod: &mut ItemMod) {
        if let Some((_, ref mut items)) = item_mod.content {
            visit_items(items);
        }
        visit_item_mod_mut(self, item_mod);
    }
}

fn visit_items(items: &mut Vec<Item>) {
    // Keep all the extern blocks in a different `Vec` for faster search.
    let mut extern_blocks = Vec::<ItemForeignMod>::new();

    for item in std::mem::take(items) {
        if let Item::ForeignMod(ItemForeignMod {
            attrs,
            abi,
            brace_token,
            unsafety,
            items: extern_block_items,
        }) = item
        {
            let mut exists = false;
            for extern_block in &mut extern_blocks {
                // Check if there is a extern block with the same ABI and
                // attributes.
                if extern_block.attrs == attrs && extern_block.abi == abi {
                    // Merge the items of the two blocks.
                    extern_block.items.extend_from_slice(&extern_block_items);
                    exists = true;
                    break;
                }
            }
            // If no existing extern block had the same ABI and attributes, store
            // it.
            if !exists {
                extern_blocks.push(ItemForeignMod {
                    attrs,
                    abi,
                    brace_token,
                    unsafety,
                    items: extern_block_items,
                });
            }
        } else {
            // If the item is not an extern block, we don't have to do anything and just
            // push it back.
            items.push(item);
        }
    }

    // Move all the extern blocks alongside the rest of the items.
    for extern_block in extern_blocks {
        items.push(Item::ForeignMod(extern_block));
    }
}
