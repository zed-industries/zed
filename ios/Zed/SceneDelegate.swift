import UIKit

class SceneDelegate: UIResponder, UIWindowSceneDelegate {

    var window: UIWindow?

    func scene(
        _ scene: UIScene,
        willConnectTo session: UISceneSession,
        options connectionOptions: UIScene.ConnectionOptions
    ) {
        guard let windowScene = scene as? UIWindowScene else { return }
        let window = UIWindow(windowScene: windowScene)
        // GPUI will take over the window in Phase 1. For now keep it black.
        window.backgroundColor = .black
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
