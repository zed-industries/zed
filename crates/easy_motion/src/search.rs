use futures::channel::oneshot;
use std::sync::Arc;

use editor::Editor;
use project::search::SearchQuery;
use ui::ViewContext;

struct Search {}

impl Search {
    pub fn search(query: String, cx: &mut ViewContext<Editor>) -> oneshot::Receiver<()> {
        let (done_tx, done_rx) = oneshot::channel();
        // let query: Arc<_> =
        //     match SearchQuery::text(query, true, false, false, Vec::new(), Vec::new()) {
        //         Ok(query) => query.with_replacement(self.replacement(cx)),
        //         Err(_) => {
        //             self.query_contains_error = true;
        //             self.clear_active_searchable_item_matches(cx);
        //             cx.notify();
        //             return done_rx;
        //         }
        //     }
        //     .into();
        // self.active_search = Some(query.clone());
        // let query_text = query.as_str().to_string();

        // let matches = active_searchable_item.find_matches(query, cx);

        // let active_searchable_item = active_searchable_item.downgrade();
        // self.pending_search = Some(cx.spawn(|this, mut cx| async move {
        //     let matches = matches.await;

        //     this.update(&mut cx, |this, cx| {
        //         let Some(active_searchable_item) =
        //             WeakSearchableItemHandle::upgrade(active_searchable_item.as_ref(), cx)
        //         else {
        //             return;
        //         };

        //         this.searchable_items_with_matches
        //             .insert(active_searchable_item.downgrade(), matches);

        //         this.update_match_index(cx);
        //         this.search_history
        //             .add(&mut this.search_history_cursor, query_text);
        //         if !this.dismissed {
        //             let matches = this
        //                 .searchable_items_with_matches
        //                 .get(&active_searchable_item.downgrade())
        //                 .unwrap();
        //             active_searchable_item.update_matches(matches, cx);
        //             let _ = done_tx.send(());
        //         }
        //         cx.notify();
        //     })
        //     .log_err();
        // }));
        done_rx
    }
}
