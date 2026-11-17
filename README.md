# Rusteroids ☄️

**Псевдо-ASCII космический шутер на Rust + Bevy, бортовой компьютер, у которого плохой день, и вайбкодинг-эксперимент, который слегка вышел из-под контроля.**

[Русский](#русский) · [English](#english)

> [!WARNING]
> **Очень ранний WIP.** Разрабатывается и тестируется **только на Linux**. Windows и macOS не проверялись. Ждите багов, кривого баланса и решений, принятых в 3 часа ночи.

<!-- TODO: добавить gif/скриншот геймплея, например docs/preview.gif -->
<!-- ![preview](docs/preview.gif) -->

---

## Русский

### Что это

**Rusteroids** — личный экспериментальный проект: аркадный шутер от третьего лица с ASCII-постобработкой и кокпитом бортового компьютера (рамка, HUD, осциллограф).

Сделан **для себя**: попробовать оффлайн-геймдев, учить Rust с помощью ИИ и учиться рисовать — проще учиться, когда есть своя идея. Никакой дорожной карты и дедлайнов. Если когда-нибудь это вырастет в маленькую визуальную новеллу со своим сюжетом, спрайтами и пасхалками — отлично. Если нет — всё равно было весело.

### Уже есть ✅

- Полёт корабля (**WASD**), камера позади него.
- ASCII-подобная постобработка и пиксельный кокпит с HUD и осциллографом.
- Волны астероидов с разными формациями, счёт, жизни, пауза, Game Over, рестарт.
- Разрушаемые (1–3 HP, полоска появляется после попадания) и неразрушаемые астероиды.
- Магазин на 12 зарядов, перезарядка, временный **Rapid Fire**.
- Взрывы и экспериментальный **target lock**: стоять на месте нельзя.

### Идеи на потом 🚧

Меню и настройки · свои спрайты и ASCII-арт · новые враги и боссы · сюжет и лор · пасхалки · звук · рефакторинг и тесты на других ОС. Всё это идеи, не обещания.

### Управление

| Клавиша | Действие |
| --- | --- |
| `W A S D` | Движение корабля |
| `Space` | Выстрел; при Rapid Fire — удерживать для автоогня |
| `R` | Перезарядка; после Game Over — новая попытка |
| `P` | Пауза |

### Запуск (Linux)

Нужны [Rust](https://rustup.rs) и системные зависимости Bevy. Для Debian/Ubuntu примерно так:

```bash
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev
```

Остальные дистрибутивы: [Bevy: Linux dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).

```bash
git clone https://github.com/peqou/Rusteroids.git
cd Rusteroids
cargo run --release
```

- Запускай из корня репозитория, чтобы игра нашла папку `assets/` (шейдер и рамка кокпита).
- `--release` важен: в debug Bevy заметно тормозит.
- Окно сейчас жёстко 1920×1080 и рассчитано на 16:9. На других разрешениях HUD может съехать.
- Версия Bevy указана в `Cargo.toml`.

### Про разработку

Это **вайбкодинг**: помощь ИИ, ручные правки, переделки и баги вида «почему астероиды стали невидимыми». Код — не образец архитектуры, всё лежит в `main.rs` и `cockpit.rs` и будет разбираться по мере надобности.

### Лицензия

[MIT](LICENSE). Берите, форкайте, ломайте, переделывайте под себя — разрешение не нужно, только оставьте упоминание автора.

---

## English

### What is this

**Rusteroids** is a personal experiment: a third-person arcade space shooter with ASCII-style post-processing and a fake spaceship-computer cockpit (frame, HUD, oscilloscope).

Made **for myself**: to try offline gamedev, learn Rust with AI help, and learn to draw — it's easier to learn when you have an idea of your own. No roadmap, no deadlines. If it ever grows into a small visual novel with its own story, sprites and easter eggs, great. If not, it was still fun.

### What's in

- Ship flight (**WASD**), camera behind the ship.
- ASCII-style post-processing, pixel-art cockpit with HUD and oscilloscope.
- Asteroid waves with different formations, score, lives, pause, game over, restart.
- Destructible (1–3 HP, bar shows after the first hit) and indestructible asteroids.
- 12-round magazine, reload, temporary **Rapid Fire** pickup.
- Explosions and an experimental **target lock** that punishes standing still.

### Ideas for later 🚧

Menus and settings · own sprites and ASCII art · new enemies and bosses · story and lore · easter eggs · audio · refactoring and testing on other OSes. Ideas only, not promises.

### Controls

| Key | Action |
| --- | --- |
| `W A S D` | Move the ship |
| `Space` | Shoot; hold during Rapid Fire for auto-fire |
| `R` | Reload; restart after Game Over |
| `P` | Pause |

### Running (Linux)

You need [Rust](https://rustup.rs) and Bevy's system dependencies. On Debian/Ubuntu roughly:

```bash
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev
```

Other distros: [Bevy: Linux dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).

```bash
git clone https://github.com/peqou/Rusteroids.git
cd Rusteroids
cargo run --release
```

- Run from the repo root so the game finds the `assets/` folder (shader and cockpit frame).
- Use `--release`: Bevy is noticeably slow in debug.
- The window is currently hardcoded to 1920×1080 and designed for 16:9; the HUD may misalign at other resolutions.
- The Bevy version is pinned in `Cargo.toml`.

### About development

This is **vibe coding**: AI assistance, manual edits, rewrites, and bugs of the "why did the asteroids go invisible" kind. The code is not a best-practices showcase; everything lives in `main.rs` and `cockpit.rs` and will be cleaned up when needed.

### License

[MIT](LICENSE). Take it, fork it, break it, remake it — no permission needed, just keep the copyright notice.

---

**Status:** very early WIP · **Engine:** Bevy · **Language:** Rust · **Tested on:** Linux
