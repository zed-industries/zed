import UIKit

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        zed_ios_main()
        return true
    }

    // MARK: UISceneSession Lifecycle

    func application(
        _ application: UIApplication,
        configurationForConnecting connectingSceneSession: UISceneSession,
        options: UIScene.ConnectionOptions
    ) -> UISceneConfiguration {
        UISceneConfiguration(name: "Default Configuration", sessionRole: connectingSceneSession.role)
    }

    // MARK: iPadOS Menu Bar (Stage Manager / external keyboard)

    /// Populate the iPadOS menu bar with Zed's menu structure.
    /// UIKit calls this at launch and whenever `UIMenuSystem.main.setNeedsRebuildMenu()`
    /// is invoked (which happens after `cx.set_menus()` completes in Rust).
    override func buildMenu(with builder: UIMenuBuilder) {
        super.buildMenu(with: builder)
        // Only the main menu system hosts the menu bar; ignore the context menu system.
        guard builder.system == .main else { return }
        let ptr = Unmanaged.passUnretained(builder as AnyObject).toOpaque()
        zed_ios_build_menus(ptr)
    }
}
