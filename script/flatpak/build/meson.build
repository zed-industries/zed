project('zed', 'rust')

fs = import('fs')

# paths
resources_dir = meson.project_source_root() / 'crates' / 'zed' / 'resources'
flatpak_resources_dir = resources_dir / 'flatpak'
src_target_dir = meson.project_source_root() / 'target' / 'release'

share_dir = get_option('prefix') / 'share'
bin_dir = get_option('prefix') / 'bin'

# get channel data
channel_lookup = {
    'dev':     ['-Dev',     ' Devel',   '-dev',     '#99c1f1', '#1a5fb4'],
    'nightly': ['-Nightly', ' Nightly', '-nightly', '#e9aa6a', '#1a5fb4'],
    'preview': ['-Preview', ' Preview', '-preview', '#99c1f1', '#1a5fb4'],
    'stable':  ['',         '',         '',         '#99c1f1', '#1a5fb4'],
}

channel = get_option('channel')
channel_data = channel_lookup[get_option('channel')]
app_id = 'dev.zed.Zed@0@'.format(channel_data[0])
src_icon = resources_dir / 'app-icon@0@.png'.format(channel_data[2])

config_data = configuration_data()
config_data.set('app_id',         app_id)
config_data.set('app_name',       'Zed@0@'.format(channel_data[1]))
config_data.set('light_branding', channel_data[3])
config_data.set('dark_branding',  channel_data[4])
config_data.set('release_info',   fs.read(flatpak_resources_dir / 'release-info' / channel).strip())

# desktop entry
configure_file(
    input: flatpak_resources_dir / 'zed.desktop.in',
    output: f'@app_id@.desktop',
    configuration: config_data,
    install: true,
    install_dir: share_dir / 'applications'
)

# metainfo
configure_file(
    input: flatpak_resources_dir / 'zed.metainfo.xml.in',
    output: f'@app_id@.metainfo.xml',
    configuration: config_data,
    install: true,
    install_dir: share_dir / 'metainfo'
)

# icon
install_data(
    src_icon,
    rename: [f'@app_id@.png'],
    install_dir: share_dir / 'icons' / 'hicolor' / '512x512' / 'apps'
)

# binaries
if get_option('copy_build')
    warning('You are using copy-build mode. Ensure you do the following:')
    warning(f'1. Ensure that you have built both the main binary and the cli binary in release mode since your last change.')
    warning(f'2. Ensure that you built with `zed/crates/RELEASE_CHANNEL` containing `@channel@`.')
else
    cargo = find_program('cargo')

    custom_target(
        'zed',
        build_always_stale: true,
        build_by_default: true,
        console: true,
        output: 'cli',
        command: [
            cargo, '--offline', 'fetch', '--manifest-path', meson.project_source_root() / 'Cargo.toml', '&&',
            cargo, '--offline', 'build', '--release', '--package', 'cli'
        ]
    )

    custom_target(
        'zed-app',
        build_always_stale: true,
        build_by_default: true,
        console: true,
        output: 'Zed',
        command: [
            cargo, '--offline', 'build', '--release'
        ]
    )
endif

install_data(
    [src_target_dir / 'cli', src_target_dir / 'Zed'],
    rename: ['zed', 'zed-app'],
    install_mode: 'rwxr-xr-x',
    install_dir: bin_dir
)