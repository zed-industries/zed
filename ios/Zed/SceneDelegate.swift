import UIKit

/// Root view controller that forces a layout pass on all subviews after
/// UIKit finishes laying out. This guarantees the ZedMetalView's
/// `layoutSubviews` fires with valid bounds before the first frame.
class ZedRootViewController: UIViewController {
    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        // Force all subviews to lay out immediately so the Metal layer
        // gets valid drawable dimensions before the next CADisplayLink tick.
        for subview in view.subviews {
            subview.layoutIfNeeded()
        }
    }

    // Extend the view's layout to respect safe area insets so content
    // is not rendered behind the status bar or home indicator.
    override func viewSafeAreaInsetsDidChange() {
        super.viewSafeAreaInsetsDidChange()
        for subview in view.subviews {
            subview.setNeedsLayout()
        }
    }
}

class SceneDelegate: UIResponder, UIWindowSceneDelegate {

    var window: UIWindow?

    func scene(
        _ scene: UIScene,
        willConnectTo session: UISceneSession,
        options connectionOptions: UIScene.ConnectionOptions
    ) {
        guard let windowScene = scene as? UIWindowScene else { return }
        let window = UIWindow(windowScene: windowScene)
        window.rootViewController = ZedRootViewController()
        window.makeKeyAndVisible()
        self.window = window

        let sceneId = session.persistentIdentifier
        sceneId.withCString { zed_ios_open_window($0) }
    }

    func sceneDidDisconnect(_ scene: UIScene) {
        guard let sceneId = (scene as? UIWindowScene)?.session.persistentIdentifier else { return }
        sceneId.withCString { zed_ios_close_window($0) }
    }
}
