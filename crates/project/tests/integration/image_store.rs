use fs::{FakeFs, Fs};
use gpui::{
    Context, IntoElement, Render, TestAppContext, VisualTestContext, WeakEntity, Window, px, size,
};
use project::Project;
use project::ProjectPath;
use project::image_store::*;
use serde_json::json;
use settings::SettingsStore;
use std::path::Path;
use util::path;
use util::rel_path::rel_path;

pub fn init_test(cx: &mut TestAppContext) {
    zlog::init_test();

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });
}

#[gpui::test]
async fn test_image_not_loaded_twice(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree("/root", json!({})).await;
    // Create a png file that consists of a single white pixel
    fs.insert_file(
        "/root/image_1.png",
        vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
            0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ],
    )
    .await;

    let project = Project::test(fs, ["/root".as_ref()], cx).await;

    let worktree_id = cx.update(|cx| project.read(cx).worktrees(cx).next().unwrap().read(cx).id());

    let project_path = ProjectPath {
        worktree_id,
        path: rel_path("image_1.png").into(),
    };

    let (task1, task2) = project.update(cx, |project, cx| {
        (
            project.open_image(project_path.clone(), cx),
            project.open_image(project_path.clone(), cx),
        )
    });

    let image1 = task1.await.unwrap();
    let image2 = task2.await.unwrap();

    assert_eq!(image1, image2);
}

#[gpui::test]
fn test_compute_metadata_from_bytes() {
    // Single white pixel PNG
    let png_bytes = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    let metadata = ImageItem::compute_metadata_from_bytes(&png_bytes).unwrap();

    assert_eq!(metadata.width, 1);
    assert_eq!(metadata.height, 1);
    assert_eq!(metadata.file_size, png_bytes.len() as u64);
    assert_eq!(metadata.format, image::ImageFormat::Png);
    assert!(metadata.colors.is_some());
}

struct ImageLocationTestView {
    project: WeakEntity<Project>,
    paths: Vec<String>,
    location: Option<ImageLocation>,
    renders: usize,
}

impl Render for ImageLocationTestView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.renders += 1;
        self.location = use_image_location(self.project.clone(), self.paths.clone(), window, cx);
        gpui::Empty
    }
}

#[gpui::test]
async fn test_use_image_location_retries_missing_file_after_delay(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({})).await;
    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    cx.run_until_parked();
    // The window's initial draw in `add_window_view` starts the first check.
    let calls_before_first_check = fs.metadata_call_count();
    let (view, cx) = cx.add_window_view(|_, _| ImageLocationTestView {
        project: project.downgrade(),
        paths: vec![path!("/outside/later.ppm").to_string()],
        location: None,
        renders: 0,
    });
    let draw = |cx: &mut VisualTestContext| {
        cx.draw(Default::default(), size(px(10.), px(10.)), {
            let view = view.clone();
            move |_, _| view.into_any_element()
        });
        view.read_with(cx, |view, _| view.location.clone())
    };
    let renders = |cx: &mut VisualTestContext| view.read_with(cx, |view, _| view.renders);

    cx.run_until_parked();
    assert_eq!(draw(cx), None);
    let calls_after_first_check = fs.metadata_call_count();
    let calls_per_check = calls_after_first_check - calls_before_first_check;
    assert!(calls_per_check > 0);

    for _ in 0..3 {
        assert_eq!(draw(cx), None);
        cx.run_until_parked();
    }
    assert_eq!(fs.metadata_call_count(), calls_after_first_check);

    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    assert_eq!(draw(cx), None);
    let renders_before_recheck = renders(cx);
    cx.run_until_parked();
    let calls_after_recheck = fs.metadata_call_count();
    assert_eq!(
        calls_after_recheck,
        calls_after_first_check + calls_per_check
    );
    assert_eq!(
        renders(cx),
        renders_before_recheck,
        "a miss must not redraw"
    );

    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    assert_eq!(draw(cx), None);
    cx.run_until_parked();
    assert_eq!(
        fs.metadata_call_count(),
        calls_after_recheck,
        "the second re-check waits longer than the first"
    );
    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    assert_eq!(draw(cx), None);
    cx.run_until_parked();
    let calls_after_second_recheck = fs.metadata_call_count();
    assert_eq!(
        calls_after_second_recheck,
        calls_after_recheck + calls_per_check
    );

    fs.insert_tree(
        path!("/outside"),
        json!({"later.ppm": "P3\n1 1\n255\n255 0 0\n"}),
    )
    .await;
    for _ in 0..3 {
        assert_eq!(draw(cx), None);
        cx.run_until_parked();
    }
    assert_eq!(fs.metadata_call_count(), calls_after_second_recheck);

    let expected = Some(ImageLocation::Local(
        Path::new(path!("/outside/later.ppm")).into(),
    ));
    cx.executor().advance_clock(IMAGE_RECHECK_DELAY * 4);
    assert_eq!(draw(cx), None);
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.location.clone()),
        expected
    );
    assert_eq!(draw(cx), expected);
}

#[gpui::test]
async fn test_use_image_location_revalidates_found_location(cx: &mut TestAppContext) {
    init_test(cx);
    let red_pixel = "P3\n1 1\n255\n255 0 0\n";
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/a"), json!({"work": {}, "shot.ppm": red_pixel}))
        .await;
    fs.insert_tree(path!("/b"), json!({"work": {}, "shot.ppm": red_pixel}))
        .await;
    let project = Project::test(
        fs.clone(),
        [path!("/a/work").as_ref(), path!("/b/work").as_ref()],
        cx,
    )
    .await;
    let (view, cx) = cx.add_window_view(|_, _| ImageLocationTestView {
        project: project.downgrade(),
        paths: vec![
            path!("/a/work/../shot.ppm").to_string(),
            path!("/b/work/../shot.ppm").to_string(),
        ],
        location: None,
        renders: 0,
    });
    let draw = |cx: &mut VisualTestContext| {
        cx.draw(Default::default(), size(px(10.), px(10.)), {
            let view = view.clone();
            move |_, _| view.into_any_element()
        });
        view.read_with(cx, |view, _| view.location.clone())
    };
    let renders = |cx: &mut VisualTestContext| view.read_with(cx, |view, _| view.renders);
    let first = Some(ImageLocation::Local(Path::new(path!("/a/shot.ppm")).into()));
    let second = Some(ImageLocation::Local(Path::new(path!("/b/shot.ppm")).into()));

    cx.run_until_parked();
    assert_eq!(draw(cx), first);

    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    assert_eq!(draw(cx), first);
    let renders_before_recheck = renders(cx);
    cx.run_until_parked();
    assert_eq!(
        renders(cx),
        renders_before_recheck,
        "an unchanged location must not redraw"
    );

    fs.remove_file(Path::new(path!("/a/shot.ppm")), Default::default())
        .await
        .unwrap();
    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    assert_eq!(draw(cx), first);
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.location.clone()),
        first,
        "the second recheck waits longer than the first"
    );

    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    assert_eq!(draw(cx), first);
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.location.clone()), second);
    assert_eq!(draw(cx), second);
}
