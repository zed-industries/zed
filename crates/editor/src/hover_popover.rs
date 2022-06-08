/// Keeps track of the state of the [`HoverPopover`].
/// Times out the initial delay and the grace period.
pub struct HoverState {
    popover: Option<HoverPopover>,
    last_hover: std::time::Instant,
    start_grace: std::time::Instant,
}

impl HoverState {
    /// Takes whether the cursor is currently hovering over a symbol,
    /// and returns a tuple containing whether there was a recent hover,
    /// and whether the hover is still in the grace period.
    pub fn determine_state(&mut self, hovering: bool) -> (bool, bool) {
        // NOTE: We use some sane defaults, but it might be
        //       nice to make these values configurable.
        let recent_hover = self.last_hover.elapsed() < std::time::Duration::from_millis(500);
        if !hovering {
            self.last_hover = std::time::Instant::now();
        }

        let in_grace = self.start_grace.elapsed() < std::time::Duration::from_millis(250);
        if hovering && !recent_hover {
            self.start_grace = std::time::Instant::now();
        }

        return (recent_hover, in_grace);
    }

    pub fn close(&mut self) {
        self.popover.take();
    }
}

#[derive(Clone)]
pub(crate) struct HoverPopover {
    pub project: ModelHandle<Project>,
    pub hover_point: DisplayPoint,
    pub range: Range<DisplayPoint>,
    pub contents: Vec<HoverBlock>,
    pub task: Option<Task<()>>,
}

impl HoverPopover {
    fn render(
        &self,
        style: EditorStyle,
        cx: &mut RenderContext<Editor>,
    ) -> (DisplayPoint, ElementBox) {
        let element = MouseEventHandler::new::<HoverPopover, _, _>(0, cx, |_, cx| {
            let mut flex = Flex::new(Axis::Vertical).scrollable::<HoverBlock, _>(1, None, cx);
            flex.extend(self.contents.iter().map(|content| {
                let project = self.project.read(cx);
                if let Some(language) = content
                    .language
                    .clone()
                    .and_then(|language| project.languages().get_language(&language))
                {
                    let runs = language
                        .highlight_text(&content.text.as_str().into(), 0..content.text.len());

                    Text::new(content.text.clone(), style.text.clone())
                        .with_soft_wrap(true)
                        .with_highlights(
                            runs.iter()
                                .filter_map(|(range, id)| {
                                    id.style(style.theme.syntax.as_ref())
                                        .map(|style| (range.clone(), style))
                                })
                                .collect(),
                        )
                        .boxed()
                } else {
                    Text::new(content.text.clone(), style.hover_popover.prose.clone())
                        .with_soft_wrap(true)
                        .contained()
                        .with_style(style.hover_popover.block_style)
                        .boxed()
                }
            }));
            flex.contained()
                .with_style(style.hover_popover.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::Arrow)
        .with_padding(Padding {
            bottom: 5.,
            top: 5.,
            ..Default::default()
        })
        .boxed();

        (self.range.start, element)
    }
}
