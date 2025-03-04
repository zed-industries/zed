#[derive(IntoElement, IntoComponent)]
#[component(scope = "notification")]
pub struct StatusToast {
    id: ElementId,
    // children: SmallVec<[AnyElement; 2]>,
    icon: Icon,
    label: impl Into<SharedString>,
    // action
}

// impl Modal {
//     pub fn new(id: impl Into<SharedString>, scroll_handle: Option<ScrollHandle>) -> Self {
//         let id = id.into();

//         let container_id = ElementId::Name(format!("{}_container", id.clone()).into());
//         Self {
//             id: ElementId::Name(id),
//             header: ModalHeader::new(),
//             children: SmallVec::new(),
//             footer: None,
//             container_id,
//             container_scroll_handler: scroll_handle,
//         }
//     }

//     pub fn header(mut self, header: ModalHeader) -> Self {
//         self.header = header;
//         self
//     }

//     pub fn section(mut self, section: Section) -> Self {
//         self.children.push(section.into_any_element());
//         self
//     }

//     pub fn footer(mut self, footer: ModalFooter) -> Self {
//         self.footer = Some(footer);
//         self
//     }

//     pub fn show_dismiss(mut self, show: bool) -> Self {
//         self.header.show_dismiss_button = show;
//         self
//     }

//     pub fn show_back(mut self, show: bool) -> Self {
//         self.header.show_back_button = show;
//         self
//     }
// }

// impl ParentElement for Modal {
//     fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
//         self.children.extend(elements)
//     }
// }

// impl RenderOnce for Modal {
//     fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
//         v_flex()
//             .id(self.id.clone())
//             .size_full()
//             .flex_1()
//             .overflow_hidden()
//             .child(self.header)
//             .child(
//                 v_flex()
//                     .id(self.container_id.clone())
//                     .w_full()
//                     .flex_1()
//                     .gap(DynamicSpacing::Base08.rems(cx))
//                     .when_some(
//                         self.container_scroll_handler,
//                         |this, container_scroll_handle| {
//                             this.overflow_y_scroll()
//                                 .track_scroll(&container_scroll_handle)
//                         },
//                     )
//                     .children(self.children),
//             )
//             .children(self.footer)
//     }
// }
