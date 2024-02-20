use std::hash::{Hash, Hasher};

use collections::{hash_map::Entry, HashMap};

use crate::{BorrowWindow, ElementContext, ElementId, GlobalElementId, WindowContext};

pub type AccessKitState = HashMap<accesskit::NodeId, accesskit::NodeBuilder>;

impl From<&GlobalElementId> for accesskit::NodeId {
    fn from(value: &GlobalElementId) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        value.0.hash(&mut hasher);
        accesskit::NodeId(hasher.finish())
    }
}

impl<'a> ElementContext<'a> {

    // TODO: What's a good, useful signature for this? Need to expose this from the div as well.
    fn accesskit_action(&mut self, id: impl Into<ElementId>, action: accesskit::Action, f: impl FnOnce(accesskit::ActionRequest)) {
        self.with_element_id(Some(id), |cx| {
            // Get the access kit actions from somewhere
            // call f with the action request and cx
            // egui impl:
            //     let accesskit_id = id.accesskit_id();
            //     self.events.iter().filter_map(move |event| {
            //         if let Event::AccessKitActionRequest(request) = event {
            //             if request.target == accesskit_id && request.action == action {
            //                 return Some(request);
            //             }
            //         }
            //         None
            //     })

        })
    }

    // TODO: Expose this through the div API
    fn with_accesskit_node(&mut self, id: impl Into<ElementId>, f: impl FnOnce(&mut accesskit::NodeBuilder)) {
        let id = id.into();
        let window = self.window_mut();
        let parent_id: accesskit::NodeId = (&window.element_id_stack).into();
        self.with_element_id(Some(id), |cx| {
            let window = cx.window_mut();
            let this_id: accesskit::NodeId = (&window.element_id_stack).into();

            window.next_frame.accesskit.as_mut().map(|nodes| {
                if let Entry::Vacant(entry) = nodes.entry(this_id) {
                    entry.insert(Default::default());
                    let parent = nodes.get_mut(&parent_id).unwrap();
                    parent.push_child(this_id);
                }

                f(nodes.get_mut(&this_id).unwrap());
            })
        });
    }
}
