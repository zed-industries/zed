# Changelog

## [v4.18.0](https://github.com/hoodie/notify-rust/compare/v4.17.0...v4.18.0) (2026-06-16)

### Features

* add common wait_for_response on all platforms
([b20c59c](https://github.com/hoodie/notify-rust/commit/b20c59c9dd4db0d83c99dc37ebda1f550334f27b))
* add wait_for_actions and on_close using mac-notification-sys
([2627ef7](https://github.com/hoodie/notify-rust/commit/2627ef772b792f9e1660dfc611b4ea026054329c))
* **macos:** add mac-usernotifications backend
([602cd5f](https://github.com/hoodie/notify-rust/commit/602cd5f2c22b72c85f3895ad95543b38dc966a0b))
* add common response types
([25994d5](https://github.com/hoodie/notify-rust/commit/25994d58e0ea97a34ff31de8fbcee72d927c4290))
* add Windows notification handles
([eaaa540](https://github.com/hoodie/notify-rust/commit/eaaa5404da373727c4902d31456dae005afb7348))

## [v4.17.0](https://github.com/hoodie/notify-rust/compare/v4.16.1...v4.17.0) (2026-05-05)

### Features

* return Result from update() instead of panicking on D-Bus errors
([10d049b](https://github.com/hoodie/notify-rust/commit/10d049b4dfd3ea168379c01055922af815049e59))

### [v4.16.1](https://github.com/hoodie/notify-rust/compare/v4.16.0...v4.16.1) (2026-04-29)

#### Fixes

* return Result from handle_action instead of panicking on D-Bus errors
([dbd4fd6](https://github.com/hoodie/notify-rust/commit/dbd4fd678e5aa9341f7cbc36c6610544496ef725)),
closes [#54](https://github.com/hoodie/notify-rust/issues/54)

## [v4.16.0](https://github.com/hoodie/notify-rust/compare/v4.15.0...v4.16.0) (2026-04-19)

### Features

* add `images_no_default_features` crate feature for downstream users wishing
to manually specify `image` sub-features.
([6a42dd0](https://github.com/hoodie/notify-rust/commit/6a42dd0b2b335115f7dec5fda45b0cf8f3937cf3))

## [v4.15.0](https://github.com/hoodie/notify-rust/compare/v4.14.0...v4.15.0) (2026-04-14)

### Features

* add non-blocking `wait_for_action_async`
([9f3613c](https://github.com/hoodie/notify-rust/commit/9f3613c7d1b2bad5b00203fd2d6a18c37f01584a))

## [v4.14.0](https://github.com/hoodie/notify-rust/compare/v4.13.1...v4.14.0) (2026-04-06)

### Features

* add urgency support for Windows
([bb48726](https://github.com/hoodie/notify-rust/commit/bb48726aff7a4559fac046da908cb01b3466f60f))

### [v4.13.1](https://github.com/hoodie/notify-rust/compare/v4.13.0...v4.13.1) (2026-04-04)

#### Fixes

* update maintenance state and formatting in README
([efafd19](https://github.com/hoodie/notify-rust/commit/efafd19371f4628a3b5a4232061562c24630b702))

## [v4.13.0](https://github.com/hoodie/notify-rust/compare/v4.12.0...v4.13.0) (2026-04-04)

### Features

* **macos:** forward image_path to content_image in notifications
([959bcae](https://github.com/hoodie/notify-rust/commit/959bcae8f9a7e08d531322290d1cb252452e3a07))
* **macos:** add image_path field and method
([1351bf2](https://github.com/hoodie/notify-rust/commit/1351bf27486a98bbb07236f25c7bbc94b2dbc122))

## [v4.12.0](https://github.com/hoodie/notify-rust/compare/v4.11.7...v4.12.0) (2026-02-01)

### Features

* add `z-with-tokio` feature (#245)
([895280c](https://github.com/hoodie/notify-rust/commit/895280c0209786f87bb7dbaf751561bed13eb4cd)),
closes [#245](https://github.com/hoodie/notify-rust/issues/245)

### Fixes

* **zbus:** don't unwrap result of close_fallible
([d0c0d84](https://github.com/hoodie/notify-rust/commit/d0c0d8448771afca9902bb6ddcc65b823f4cdf0c)),
closes [#253](https://github.com/hoodie/notify-rust/issues/253)

### [v4.11.7](https://github.com/hoodie/notify-rust/compare/v4.11.6...v4.11.7) (2025-04-08)

#### Fixes

* **deps:** update rust crate winrt-notification to 0.7 (#220)
([6411aec](https://github.com/hoodie/notify-rust/commit/6411aec798fab664f984ac60bd40ae6b41e19011)),
closes [#220](https://github.com/hoodie/notify-rust/issues/220)

### [v4.11.6](https://github.com/hoodie/notify-rust/compare/v4.11.5...v4.11.6) (2025-03-28)

#### Fixes

* resolve type inference issue with u32::MAX conversion (#244)
([0c6c3ff](https://github.com/hoodie/notify-rust/commit/0c6c3ff251dbb32df6d45446940820d2f5d8e375)),
closes [#244](https://github.com/hoodie/notify-rust/issues/244)

### [v4.11.5](https://github.com/hoodie/notify-rust/compare/v4.11.4...v4.11.5) (2025-02-10)

#### Fixes

* **deps:** broken zbus export
([354d5a8](https://github.com/hoodie/notify-rust/commit/354d5a8c7072e46aeba41a7e1991e2d780b6ad6a)),
closes [#236](https://github.com/hoodie/notify-rust/issues/236)

### [v4.11.4](https://github.com/hoodie/notify-rust/compare/v4.11.3...v4.11.4) (2025-01-12)

#### Fixes

* **deps:** update rust crate zbus to v5
([0834adc](https://github.com/hoodie/notify-rust/commit/0834adc4635c72420ab52cb58ef573004693cab8))

### [v4.11.3](https://github.com/hoodie/notify-rust/compare/v4.11.2...v4.11.3) (2024-09-11)

#### Fixes

* update build badge
([c0ffeed](https://github.com/hoodie/notify-rust/commit/c0ffeed9455244f616ae738d2ac009e66fc2513a))

### [v4.11.2](https://github.com/hoodie/notify-rust/compare/v4.11.1...v4.11.2) (2024-09-10)

#### Fixes

* **docs:** update links to Hints specification
([6312e9c](https://github.com/hoodie/notify-rust/commit/6312e9c2273150d8c844c4725bf9ad29b3ac6021))
* **docs:** link warning
([6edff02](https://github.com/hoodie/notify-rust/commit/6edff02d5450b9ae13f8271f32685fe9bca64404))
* more precise dependency on lazy_static
([c24169e](https://github.com/hoodie/notify-rust/commit/c24169ef76b2b0be6d2acb4e6b4b445161a19d49))

### [v4.11.1](https://github.com/hoodie/notify-rust/compare/v4.11.0...v4.11.1) (2024-08-01)

#### Fixes

* **docs:** make sure crates.io and readme are in sync
([2bd2a2c](https://github.com/hoodie/notify-rust/commit/2bd2a2ce94aa9941040a939244ac3798f3a0110d))
* update maintenance badge
([789fe44](https://github.com/hoodie/notify-rust/commit/789fe44140416837812889c5cdc0b64ac81f26b7))

## [v4.11.0](https://github.com/hoodie/notify-rust/compare/v4.10.0...v4.11.0) (2024-04-05)

### Features

* update to zbus v4
([9683d01](https://github.com/hoodie/notify-rust/commit/9683d012568e4975f4e99068c7653b844146b536))

### Fixes

* **deps:** update rust crate image to 0.25
([c453121](https://github.com/hoodie/notify-rust/commit/c4531213af4d7319c5c3b23095d48d75c14f2181))
* **deps:** update rust crate env_logger to 0.11
([656a915](https://github.com/hoodie/notify-rust/commit/656a915063484f1f96a5c3c5e0a566cf8dad71a7))
* **deps:** update rust crate winrt-notification to 0.2
([730d563](https://github.com/hoodie/notify-rust/commit/730d5632f541ed3f452c62612c7b14992923d978))

## [v4.10.0](https://github.com/hoodie/notify-rust/compare/v4.9.0...v4.10.0) (2023-11-16)

### Features

* add feature `d_vendored` to enable dbus/vendored feature
([a34239c](https://github.com/hoodie/notify-rust/commit/a34239c99e9caedf8f327226de82ad36dc6b3dcc))

## [v4.9.0](https://github.com/hoodie/notify-rust/compare/v4.8.0...v4.9.0) (2023-08-22)

### Features

* base `Urgency` on `u8` instead of `u64`
([e9e3c30](https://github.com/hoodie/notify-rust/commit/e9e3c3068478bd2133a202fb4cc7f03af8160836))
* **xdg:** make dbus bus configurable
([a514664](https://github.com/hoodie/notify-rust/commit/a514664cbe3277231106775482a87ac792f21635))

### Fixes

* remove redundant action button on macOS
([c0ffee9](https://github.com/hoodie/notify-rust/commit/c0ffee958afdf312d4c8eee5433345690686efb9))
* remove server feature
([f8e89cf](https://github.com/hoodie/notify-rust/commit/f8e89cfd1a392d9b6cff43adcb167e9f64ebbdec))
* building with both dbus and zbus backend
([83d4489](https://github.com/hoodie/notify-rust/commit/83d44898a853944b9e62b77655c6384c5e75030c))

## [v4.8.0](https://github.com/hoodie/notify-rust/compare/v4.7.1...v4.8.0) (2023-02-19)

### Features

* add .show_async()
([ae3db62](https://github.com/hoodie/notify-rust/commit/ae3db621665ce4e2c741e2507f57d805d88c25ba))

### [v4.7.1](https://github.com/hoodie/notify-rust/compare/v4.7.0...v4.7.1) (2023-02-15)

#### Fixes

* remove build issue when building with both z and d
([6f4238f](https://github.com/hoodie/notify-rust/commit/6f4238fe60c7790dd47ed4f31613eb9a1814b436))

## [v4.7.0](https://github.com/hoodie/notify-rust/compare/v4.6.0...v4.7.0) (2023-01-08)

### Features

* Accept durations as timeout arguments
([3ab191a](https://github.com/hoodie/notify-rust/commit/3ab191a9b74ad3120211719050f125de2e6f757d))
* implement FromStr for Timeout
([b87b859](https://github.com/hoodie/notify-rust/commit/b87b859e6d18481e40d90ae7b4cd1b885589b897))

### Fixes

* **deps:** update rust crate zbus to 3.6
([89779e8](https://github.com/hoodie/notify-rust/commit/89779e8f43978e3d4f72d9a66107a60bc3443d10))

## [v4.6.0](https://github.com/hoodie/notify-rust/compare/v4.5.10...v4.6.0) (2022-12-04)

### Features

* update zbus dependency to 3.x
([8425c05](https://github.com/hoodie/notify-rust/commit/8425c05b9ad945745cba9f16e288d6a8fc9887a6))

### [v4.5.10](https://github.com/hoodie/notify-rust/compare/v4.5.9...v4.5.10) (2022-09-25)

#### Fixes

* lower minimum rustc version to 1.59.0 for linux (dbus)
([c0ffeed](https://github.com/hoodie/notify-rust/commit/c0ffeede93816f601a0a8701fb3510c052ce3a0e))

### [v4.5.9](https://github.com/hoodie/notify-rust/compare/v4.5.8...v4.5.9) (2022-09-21)

#### Fixes

* raise minimum rustc version to 1.60.0 for zbus
([b3b9aba](https://github.com/hoodie/notify-rust/commit/b3b9aba88fbacfd2c2c406e58cbc020c46895879))
* update winrt-notification to a maintained fork, closes #148
([7cc6824](https://github.com/hoodie/notify-rust/commit/7cc682428d2f5f0c2335c8ff141fb71af1e01a26)),
closes [#148](https://github.com/hoodie/notify-rust/issues/148)

### [v4.5.8](https://github.com/hoodie/notify-rust/compare/v4.5.7...v4.5.8) (2022-03-26)

#### Fixes

* re-exported types from mac-notification-sys
([80daef6](https://github.com/hoodie/notify-rust/commit/80daef6b0dfda0d61697d118c824930bb00a8a35))

### [v4.5.7](https://github.com/hoodie/notify-rust/compare/v4.5.6...v4.5.7) (2022-03-20)

#### Fixes

* **deps:** update rust crate mac-notification-sys to 0.5.0
([1f3a9f5](https://github.com/hoodie/notify-rust/commit/1f3a9f5c36c17775885fd6adb80bbf0295219e45))

### [v4.5.6](https://github.com/hoodie/notify-rust/compare/v4.5.5...v4.5.6) (2022-02-04)

#### Fixes

* update crates zbus to v2, zvariant to v3, zvariant_derive to v3
([14bca58](https://github.com/hoodie/notify-rust/commit/14bca588efbc88d6a1e00b5abcb5be7db44abfcf))
* **deps:** update rust crate zbus to v2
([04901a8](https://github.com/hoodie/notify-rust/commit/04901a85fd587ed7482a0bf222d76d7c898369ef))
* **deps:** update rust crate image to 0.24
([c7fa276](https://github.com/hoodie/notify-rust/commit/c7fa276cab7ffdd2b6e825334837a3515599a3a9))

### [v4.5.5](https://github.com/hoodie/notify-rust/compare/v4.5.4...v4.5.5) (2021-11-04)

#### Fixes

* **deps:** update rust crate winrt-notification to 0.5
([6620110](https://github.com/hoodie/notify-rust/commit/6620110a5a8e37b4ac738ccd3df87cda88f57ce8))

### [v4.5.4](https://github.com/hoodie/notify-rust/compare/v4.5.3...v4.5.4) (2021-10-08)

#### Fixes

* update winrt-notification to 0.4
([c94e111](https://github.com/hoodie/notify-rust/commit/c94e111f320c75015de9d2177d12f4dce218804b))

### [v4.5.3](https://github.com/hoodie/notify-rust/compare/v4.5.2...v4.5.3) (2021-09-16)

#### Fixes

* clear up documentation and flip env switch for dual stack
([3a3b175](https://github.com/hoodie/notify-rust/commit/3a3b1750ce24d2ba54437cfe65a9c8d7dd251cf2))

### [v4.5.2](https://github.com/hoodie/notify-rust/compare/v4.5.1...v4.5.2) (2021-05-14)

#### Fixes

* clear up documentation of action handling
([ad35d34](https://github.com/hoodie/notify-rust/commit/ad35d34f93bed6c4f926403d6495972f9dcba273))

### [v4.5.1](https://github.com/hoodie/notify-rust/compare/v4.5.0...v4.5.1) (2021-05-13)

#### Fixes

* export of ActionResponse
([3bf2f37](https://github.com/hoodie/notify-rust/commit/3bf2f37836902334f556d66c609d5d52d732284a))

## [v4.5.0](https://github.com/hoodie/notify-rust/compare/v4.4.1...v4.5.0) (2021-05-05)

### Features

* **NotificationHandle:** add close reason handling
([01f9980](https://github.com/hoodie/notify-rust/commit/01f9980e20a687f37391b8c32675436167b4f435)),
closes [#104](https://github.com/hoodie/notify-rust/issues/104)

### [v4.4.1](https://github.com/hoodie/notify-rust/compare/v4.4.0...v4.4.1) (2021-05-01)

#### Fixes

* notify with __closed when notification is closed in zbus implementation
([3a9c206](https://github.com/hoodie/notify-rust/commit/3a9c2066929bace67bf88b5882f4296906a7a7dc))

## [v4.4.0](https://github.com/hoodie/notify-rust/compare/v4.3.0...v4.4.0) (2021-04-25)

### Features

* add schedule_raw() for f64 timestamps
([b8f811b](https://github.com/hoodie/notify-rust/commit/b8f811b030c94224a8b6c17cdd49f09a41a417a4))
* add schedule method to Notification
([30f1741](https://github.com/hoodie/notify-rust/commit/30f1741ca8818fd4552de0b53bf7fc19511886bc))
* create macos schedule_notification method
([72bda94](https://github.com/hoodie/notify-rust/commit/72bda94d3e0281d8c0dc88343f643099d1963520))

## [v4.3.0](https://github.com/hoodie/notify-rust/compare/v4.2.2...v4.3.0) (2021-02-27)

### Features

* Convert DynamicImage::ImageRgba8
([87e92b5](https://github.com/hoodie/notify-rust/commit/87e92b5d717edcd75f94f66b0f7aed1f04363459))
* Implement TryFrom trait for RgbImage and RgbaImage
([69c2b1e](https://github.com/hoodie/notify-rust/commit/69c2b1e2417399ea08a13463218c1ef34f64ab25))
* Implement converting image with alpha
([d25ab47](https://github.com/hoodie/notify-rust/commit/d25ab47b4bdc0d7cab2802787e3f086e32dad29d))
* make zbus backend the default
([582b87e](https://github.com/hoodie/notify-rust/commit/582b87ec85647e66058f1857bc0a169411ebeb3b))

### [v4.2.2](https://github.com/hoodie/notify-rust/compare/v4.2.1...v4.2.2) (2021-01-09)

#### Fixes

* remove another stray println
([bd6ab59](https://github.com/hoodie/notify-rust/commit/bd6ab59d04d23f25da1f333677fc89d90ff2aaf3))

### [v4.2.1](https://github.com/hoodie/notify-rust/compare/v4.2.0...v4.2.1) (2021-01-08)

#### Fixes

* **deps:** update zbus
([684d031](https://github.com/hoodie/notify-rust/commit/684d03161470ad119095c86999f169a352d2e4d3))
* **deps:** update rust crate dbus to 0.9
([96f84f4](https://github.com/hoodie/notify-rust/commit/96f84f4e1c2d025d0d42488f07c178fba012a030))

## [v4.2.0](https://github.com/hoodie/notify-rust/compare/v4.1.1...v4.2.0) (2021-01-08)

### Features

* make custom hints unique
([f6ec445](https://github.com/hoodie/notify-rust/commit/f6ec4453367db915fb1523c8ea223ed3e9cae800)),
closes [#88](https://github.com/hoodie/notify-rust/issues/88)

### Fixes

* remove stray dbug!()
([b67c1d5](https://github.com/hoodie/notify-rust/commit/b67c1d504df2c99f24a8ceef0079b4764ebc52d7))

### [v4.1.1](https://github.com/hoodie/notify-rust/compare/v4.1.0...v4.1.1) (2021-01-07)

#### Fixes

* remove stray println
([15b3ecd](https://github.com/hoodie/notify-rust/commit/15b3ecd2be6ec09a1a490187a24ed40e03526d77))

## [v4.1.0](https://github.com/hoodie/notify-rust/compare/v4.0.0...v4.1.0) (2021-01-06)

### Features

* add zbus version
([58d38ba](https://github.com/hoodie/notify-rust/commit/58d38ba98e19b6f382f577416b039bd0dcb4491c))

### Fixes

* **deps:** update rust crate image to 0.23
([1dd236d](https://github.com/hoodie/notify-rust/commit/1dd236d95b9bd47ea4cbfd75d648ba6e93d8a9a0))

## [v4.0.0](https://github.com/hoodie/notify-rust/compare/v3.6.3...v4.0.0) (2020-06-06)

### ⚠ BREAKING CHANGE

* remove `From<&str>`
* restructure modules and exports


### Features

* **windows:** additions to the API (#69)
([1d9cb0e](https://github.com/hoodie/notify-rust/commit/1d9cb0eb73fdf7f442757a63f2131cb42c4c150f)),
closes [#69](https://github.com/hoodie/notify-rust/issues/69)
* make notification non-exhaustive
([0304274](https://github.com/hoodie/notify-rust/commit/03042744a05fdab5eff1eee5023c93b2930710d2))
* make error non-exhaustive
([26f96e4](https://github.com/hoodie/notify-rust/commit/26f96e4a90916496b26a0c955a878b0fe83b9aab))
* drop redundant name prefixes
([faf3123](https://github.com/hoodie/notify-rust/commit/faf3123bb375a58b57930727da9a09df49830c3e))
* restructure modules and exports
([45be84c](https://github.com/hoodie/notify-rust/commit/45be84ce976ac3a18d674df7a412ec638861e6b0))
* .image() no longer silently fails
([8b215bd](https://github.com/hoodie/notify-rust/commit/8b215bd8dfd31655711c522553bb130552930201))

### Fixes

* reexport NotificationHandle
([00edbc9](https://github.com/hoodie/notify-rust/commit/00edbc9a2e44db642d02abfcb6931f0e3bb77563))

### [v3.6.3](https://github.com/hoodie/notify-rust/compare/v3.6.2...v3.6.3) (2019-11-02)

#### Fixes

* **deps:** update rust crate lazy_static to 1.4
([687e34d](https://github.com/hoodie/notify-rust/commit/687e34d7a73db36eb2bdeb20e27c8ed18b08f4c1))
* build again after merge conflict
([bcfc8c8](https://github.com/hoodie/notify-rust/commit/bcfc8c8d41d38bcc51a78ca5febcb4f1adb4d810))
* test-build without `--features image` 🙄
([3eead0b](https://github.com/hoodie/notify-rust/commit/3eead0b30d189315828a7e4835526e551f0e81a0))
* test-build with `--features image`
([60e963d](https://github.com/hoodie/notify-rust/commit/60e963de1ea40e29ab09599d26160b206b8acf31))
* test-build with `--features image`
([92217a2](https://github.com/hoodie/notify-rust/commit/92217a2c83b4c083076978061059175ecb9048ee))

### [v3.6.2](https://github.com/hoodie/notify-rust/compare/v3.6.1...v3.6.2) (2019-08-11)

#### Fixes

* test-build without `--features image` 🙄
([0524a5f](https://github.com/hoodie/notify-rust/commit/0524a5fba819a7470525cfdcbcf4dcb4db4b3623))

### [v3.6.1](https://github.com/hoodie/notify-rust/compare/v3.6.0...v3.6.1) (2019-08-11)

#### Fixes

* test-build with `--features image`
([8ee6998](https://github.com/hoodie/notify-rust/commit/8ee69984cb5350287581de2b453f225d1fef3de2))

## [v3.6.0](https://github.com/hoodie/notify-rust/compare/v3.5.0...v3.6.0) (2019-05-06)

## [v3.5.0](https://github.com/hoodie/notify-rust/compare/v3.4.3...v3.5.0) (2018-10-21)

### [v3.4.3](https://github.com/hoodie/notify-rust/compare/v3.4.0...v3.4.3) (2018-10-13)

## [v3.4.0](https://github.com/hoodie/notify-rust/compare/v3.2.1...v3.4.0) (2017-05-21)

### [v3.2.1](https://github.com/hoodie/notify-rust/compare/v3.1.1...v3.2.1) (2016-09-07)

### [v3.1.1](https://github.com/hoodie/notify-rust/compare/v3.1.0...v3.1.1) (2016-03-03)

## [v3.1.0](https://github.com/hoodie/notify-rust/compare/v3.0.4...v3.1.0) (2016-03-01)

### [v3.0.4](https://github.com/hoodie/notify-rust/compare/v3.0.3...v3.0.4) (2016-02-15)

### [v3.0.3](https://github.com/hoodie/notify-rust/compare/v3.0.2...v3.0.3) (2016-02-15)

### [v3.0.2](https://github.com/hoodie/notify-rust/compare/v3.0.1...v3.0.2) (2016-02-15)

### [v3.0.1](https://github.com/hoodie/notify-rust/compare/v3.0.0...v3.0.1) (2015-10-23)

## [v3.0.0](https://github.com/hoodie/notify-rust/compare/v2.1.0...v3.0.0) (2015-10-01)

## [v2.1.0](https://github.com/hoodie/notify-rust/compare/v2.0.0...v2.1.0) (2015-09-27)

## [v2.0.0](https://github.com/hoodie/notify-rust/compare/v1.1.0...v2.0.0) (2015-08-04)

## [v1.1.0](https://github.com/hoodie/notify-rust/compare/v1.0.1...v1.1.0) (2015-08-03)

### [v1.0.1](https://github.com/hoodie/notify-rust/compare/v1.0.0...v1.0.1) (2015-07-19)

## [v1.0.0](https://github.com/hoodie/notify-rust/compare/v0.9.0...v1.0.0) (2015-07-13)

## [v0.9.0](https://github.com/hoodie/notify-rust/compare/v0.8.0...v0.9.0) (2015-07-01)

## [v0.8.0](https://github.com/hoodie/notify-rust/compare/v0.0.8...v0.8.0) (2015-06-19)

### [v0.0.8](https://github.com/hoodie/notify-rust/compare/v0.0.7...v0.0.8) (2015-06-19)

### [v0.0.7](https://github.com/hoodie/notify-rust/compare/v0.0.6...v0.0.7) (2015-06-13)

### [v0.0.6](https://github.com/hoodie/notify-rust/compare/v0.0.4...v0.0.6) (2015-06-08)

### [v0.0.4](https://github.com/hoodie/notify-rust/compare/v0.0.3...v0.0.4) (2015-05-30)

### [v0.0.3](https://github.com/hoodie/notify-rust/compare/v0.0.2...v0.0.3) (2015-05-24)

### v0.0.2 (2015-05-22)
