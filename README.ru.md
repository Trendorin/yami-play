# yami-play

Плеер видео и музыки для Wayland (libmpv подгружается при запуске). Часть рабочего стола [yami](https://github.com/Trendorin/yami), которая работает и сама по себе: бери в любой Wayland-райс, шелл yami не нужен.

## Установка

```sh
chmod +x yami-play
./yami-play install
```

Один файл, без Rust и root. `install` спросит настройки, покажет, что изменит, и поставит в `~/.local` только после твоего «да» (`--yes` без вопросов, `--all` ещё и включает все опции). `./yami-play deps` — что нужно от системы, `./yami-play uninstall` — удалить. Собрать самому: Rust 1.89 или новее, `cargo build --release --locked`, затем `target/release/yami-play install`.

## Как пользоваться

`yami-play [ФАЙЛ…]`. `YAMI_PLAY_OPTS="k=v,…"` передаёт настройки mpv.

## Внешний вид

Без настроек программа выглядит нейтрально: системный шрифт, скруглённые углы, мягкие тени, светлая
или тёмная тема как в системе. Поменять можно всё:

| Файл | Что |
|---|---|
| `~/.config/yami/style.json` | вид всех модулей yami |
| `~/.config/yami/style.play.json` | чем этот модуль отличается (только эти ключи) |

| Ключ | Значения |
|---|---|
| `preset` | `plain` (нейтральный) или `yami` (пиксельный) |
| `accent` | `#rrggbb` |
| `light` | `true`, `false`; нет ключа — как в системе |
| `corners` | `notch`, `round`, `square` |
| `radius` | радиус углов панелей в пикселях |
| `shadow` | `hard`, `soft`, `none` |
| `reveal` | `fade`, `none`, `dither`, `scan`, `noise`, `wipe`, `blocks` |
| `font` | семейство шрифта |
| `animations`, `animSpeed` | вкл/выкл, скорость |
| `colors` | любой из `bg panel surface surfaceHi codeBg border dim accent deep glow bright selection text textDim textFaint onAccent textHi ok warn` как `#rrggbb` |
| `lang` | `ru`, `en`, `pt-BR`; пусто — как в системе |

Файлы перечитываются при изменении, программа перекрашивается без перезапуска.

## Лицензия

MIT
