// Что ставит `yami-play install`: файлы, вшитые в бинарь, и вопросы к человеку (yami_px::install).

use yami_px::install::{Asset, Choice, Spec};

pub const SPEC: Spec = Spec {
    name: "yami-play",
    version: env!("CARGO_PKG_VERSION"),
    about: ["плеер видео и музыки на libmpv", "video and music player on libmpv", "player de vídeo e música com libmpv"], // i18n-ok
    deps: "wayland, libxkbcommon, libmpv",
    assets: &[Asset { path: "applications/yami-play.desktop", body: include_str!("../config/yami-play.desktop") }],
    choices: &[Choice { ask: ["Открывать видео и музыку в yami-play по умолчанию?", "Open video and music in yami-play by default?", "Abrir vídeo e música no yami-play por padrão?"], assets: &[], default_app: true, note: None }], // i18n-ok
};
