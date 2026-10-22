use fs::{FakeFs, Fs};
use gpui::{
    Context, Entity, IntoElement, Render, TestAppContext, VisualTestContext, WeakEntity, Window,
    px, size,
};
use project::Project;
use project::ProjectPath;
use project::image_store::*;
use serde_json::json;
use settings::SettingsStore;
use std::path::Path;
use std::time::Duration;
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
    let calls_before_first_check = fs.is_file_call_count();
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
    let calls_after_first_check = fs.is_file_call_count();
    let calls_per_check = calls_after_first_check - calls_before_first_check;
    assert!(calls_per_check > 0);

    for _ in 0..3 {
        assert_eq!(draw(cx), None);
        cx.run_until_parked();
    }
    assert_eq!(fs.is_file_call_count(), calls_after_first_check);

    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    assert_eq!(draw(cx), None);
    let renders_before_recheck = renders(cx);
    cx.run_until_parked();
    let calls_after_recheck = fs.is_file_call_count();
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
        fs.is_file_call_count(),
        calls_after_recheck,
        "the second re-check waits longer than the first"
    );
    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    assert_eq!(draw(cx), None);
    cx.run_until_parked();
    let calls_after_second_recheck = fs.is_file_call_count();
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
    assert_eq!(fs.is_file_call_count(), calls_after_second_recheck);

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
    let first = Some(ImageLocation::Local(
        Path::new(path!("/a/work/../shot.ppm")).into(),
    ));
    let second = Some(ImageLocation::Local(
        Path::new(path!("/b/work/../shot.ppm")).into(),
    ));

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

#[gpui::test]
async fn test_use_image_location_recheck_notifies_every_view(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({})).await;
    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let new_view = || ImageLocationTestView {
        project: project.downgrade(),
        paths: vec![path!("/outside/shared.ppm").to_string()],
        location: None,
        renders: 0,
    };
    let first_window = cx.add_window(|_, _| new_view());
    let second_window = cx.add_window(|_, _| new_view());
    let first_view = first_window.root(cx).unwrap();
    let second_view = second_window.root(cx).unwrap();
    let first_cx = &mut VisualTestContext::from_window(*first_window, cx);
    cx.run_until_parked();
    let location = |view: &Entity<ImageLocationTestView>, cx: &mut TestAppContext| {
        view.read_with(cx, |view, _| view.location.clone())
    };
    let renders = |view: &Entity<ImageLocationTestView>, cx: &mut TestAppContext| {
        view.read_with(cx, |view, _| view.renders)
    };
    assert_eq!(location(&first_view, cx), None);
    assert_eq!(location(&second_view, cx), None);

    fs.insert_tree(
        path!("/outside"),
        json!({"shared.ppm": "P3\n1 1\n255\n255 0 0\n"}),
    )
    .await;
    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    let second_renders_before_recheck = renders(&second_view, cx);
    first_cx.draw(Default::default(), size(px(10.), px(10.)), {
        let view = first_view.clone();
        move |_, _| view.into_any_element()
    });
    let first_renders_before_hit = renders(&first_view, cx);
    assert_eq!(
        renders(&second_view, cx),
        second_renders_before_recheck,
        "only the first view draws once the recheck is due"
    );
    cx.run_until_parked();

    let expected = Some(ImageLocation::Local(
        Path::new(path!("/outside/shared.ppm")).into(),
    ));
    assert!(renders(&first_view, cx) > first_renders_before_hit);
    assert!(renders(&second_view, cx) > second_renders_before_recheck);
    assert_eq!(location(&first_view, cx), expected);
    assert_eq!(location(&second_view, cx), expected);
}

#[gpui::test]
async fn test_image_locations_are_released_with_their_project(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({"shot.ppm": "P3\n1 1\n255\n255 0 0\n"}),
    )
    .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let paths = vec![path!("/project/shot.ppm").to_string()];
    let (view, cx) = cx.add_window_view(|_, _| ImageLocationTestView {
        project: project.downgrade(),
        paths: paths.clone(),
        location: None,
        renders: 0,
    });
    cx.run_until_parked();
    assert!(view.read_with(cx, |view, _| view.location.is_some()));
    let image_store = project.read_with(cx, |project, _| project.image_store().downgrade());
    assert_eq!(
        image_store
            .read_with(cx, |image_store, _| image_store
                .image_location_window_count(&paths))
            .unwrap(),
        Some(1)
    );

    cx.update(|_, _| drop(project));
    assert!(image_store.upgrade().is_none());
}

#[gpui::test]
async fn test_use_image_location_drops_unused_entries(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({})).await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let first = vec![path!("/outside/first.ppm").to_string()];
    let second = vec![path!("/outside/second.ppm").to_string()];
    let (view, cx) = cx.add_window_view(|_, _| ImageLocationTestView {
        project: project.downgrade(),
        paths: first.clone(),
        location: None,
        renders: 0,
    });
    let draw = |cx: &mut VisualTestContext| {
        cx.draw(Default::default(), size(px(10.), px(10.)), {
            let view = view.clone();
            move |_, _| view.into_any_element()
        });
    };
    let window_count = |paths: &[String], cx: &mut VisualTestContext| {
        project.read_with(cx, |project, cx| {
            project
                .image_store()
                .read(cx)
                .image_location_window_count(paths)
        })
    };
    cx.run_until_parked();

    view.update(cx, |view, _| view.paths = second.clone());
    draw(cx);
    cx.run_until_parked();
    assert_eq!(window_count(&first, cx), Some(1));
    assert_eq!(window_count(&second, cx), Some(1));

    cx.executor()
        .advance_clock(UNUSED_IMAGE_LOCATION_RETENTION + Duration::from_secs(1));
    draw(cx);
    cx.run_until_parked();
    assert_eq!(window_count(&first, cx), None);
    assert_eq!(window_count(&second, cx), Some(1));
}

#[gpui::test]
async fn test_use_image_location_recheck_skips_closed_windows(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({})).await;
    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let paths = vec![path!("/outside/shared.ppm").to_string()];
    let new_view = || ImageLocationTestView {
        project: project.downgrade(),
        paths: paths.clone(),
        location: None,
        renders: 0,
    };
    let closed_window = cx.add_window(|_, _| new_view());
    let open_window = cx.add_window(|_, _| new_view());
    let open_view = open_window.root(cx).unwrap();
    let open_cx = &mut VisualTestContext::from_window(*open_window, cx);
    let window_count = |cx: &mut TestAppContext| {
        project.read_with(cx, |project, cx| {
            project
                .image_store()
                .read(cx)
                .image_location_window_count(&paths)
        })
    };
    cx.run_until_parked();
    assert_eq!(window_count(cx), Some(2));

    closed_window
        .update(cx, |_, window, _| window.remove_window())
        .unwrap();
    fs.insert_tree(
        path!("/outside"),
        json!({"shared.ppm": "P3\n1 1\n255\n255 0 0\n"}),
    )
    .await;
    cx.executor().advance_clock(IMAGE_RECHECK_DELAY);
    open_cx.draw(Default::default(), size(px(10.), px(10.)), {
        let view = open_view.clone();
        move |_, _| view.into_any_element()
    });
    cx.run_until_parked();

    assert_eq!(
        open_view.read_with(cx, |view, _| view.location.clone()),
        Some(ImageLocation::Local(
            Path::new(path!("/outside/shared.ppm")).into()
        ))
    );
    assert_eq!(window_count(cx), Some(1));
}
