# Changelog

## \[0.1.1]

- Added `compile_for` function to select which binaries to apply the resource to.
  - [3aa8411](https://github.com/tauri-apps/winres/commit/3aa84115f6a80d74fd28f4f8c81ef734ccb1c37e) refactor: Use embed-resource crate to compile resources ([#9](https://github.com/tauri-apps/winres/pull/9)) on 2023-05-04
- Use https://github.com/nabijaczleweli/rust-embed-resource to compile the resource for better cross-platform compilation support. Note that because of this a few methods are no-op now and marked as deprecated. Technically this was a breaking change.
  - [3aa8411](https://github.com/tauri-apps/winres/commit/3aa84115f6a80d74fd28f4f8c81ef734ccb1c37e) refactor: Use embed-resource crate to compile resources ([#9](https://github.com/tauri-apps/winres/pull/9)) on 2023-05-04

## \[0.1.0]

- Initial release.
  - [72e3fec](https://github.com/tauri-apps/winres/commit/72e3fecc69ad4fe6eaabc53a3f714d1ef6d39ad8) ci: Add covector to prepare publishing ([#5](https://github.com/tauri-apps/winres/pull/5)) on 2023-01-19
