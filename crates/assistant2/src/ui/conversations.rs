pub struct ConversationsView {}

impl Render for ConversationsView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // header
        // toggle pinned at top
        // toggle layout - list/details
        //
        // tools: (view style), (reveal), (pin, delete), (new)

        // picker input
        // pinned conversations - if pinned=true

        // recent conversations - if pinned=false
        // today
        // this week
        // earlier

        // let header_height = Spacing::Small.rems(cx) * 2.0 + ButtonSize::Default.rems();

        // let (pinned_at_top, show_details) = (true, true);

        // let pinned_conversations = vec![];
        // let recent_conversations = vec![];

        // h_flex().size_full().child(
        //     v_flex().
        // )
    }
}
