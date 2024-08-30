struct WebViewTest {
    views: Vec<Arc<WebView>>,
}

impl WebViewTest {
    fn new(num_views: usize, handle: &dyn HasWindowHandle) -> Self {
        let views = (0..num_views)
            .map(|i| {
                Arc::new(
                    wry::WebViewBuilder::new_as_child(&handle)
                        .with_html(format!(
                            "<html><body>Hello, world! I'm webview {i}</body></html>"
                        ))
                        .build()
                        .unwrap(),
                )
            })
            .collect();

        Self { views }
    }
}

impl Render for WebViewTest {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut parent = div()
            .id("parent")
            .block()
            .overflow_y_scroll()
            .size_full()
            .bg(rgb(0xff0000))
            .justify_center()
            .items_center();

        for (i, view) in self.views.iter().enumerate() {
            parent = parent.child(
                div()
                    .size(Length::Definite(DefiniteLength::Absolute(
                        AbsoluteLength::Pixels(Pixels(100.0)),
                    )))
                    .bg(rgb(0x00ff00))
                    .child(format!("This is webview {}:", i)),
            );
            parent = parent.child(HelloWorldEl { view: view.clone() });
        }

        parent
    }
}
