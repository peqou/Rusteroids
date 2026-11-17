# Rusteroids ☄️

**An ASCII-ish space shooter, an onboard computer having a questionable day, and a vibe-coded experiment that got slightly out of hand.**

[Русский](#русский) · [English](#english)

> [!WARNING]
> **Very early work in progress.** Developed and tested **on Linux only**. **Windows has not been tested** (and neither has macOS). Expect bugs, rough edges, questionable balancing, and occasional decisions made at 3 a.m. This is not a finished game.

---

## Русский

### Что это вообще такое?

**Rusteroids** — экспериментальный космический аркадный шутер на **Rust + Bevy** с псевдо-ASCII-визуалом и интерфейсом бортового компьютера.

Проект родился из идеи «а что, если просто попробовать сделать игру?» и довольно быстро превратился в «почему я уже настраиваю осциллограф внутри вымышленного космического терминала?». Это **vibecoding ради фана, экспериментов и изучения Rust**, а не серьёзный коммерческий продукт с дорожной картой на 40 кварталов.

**Но.** Есть ненулевая вероятность, что когда-нибудь из этого вырастет полноценная, немного странная инди-игра — с собственным характером, историей и вещами, которые лучше не объяснять в README.

### Уже есть ✅

- 3D-полёт корабля с управлением на **WASD** и камерой позади него.
- Постобработка в **ASCII-подобном стиле**: космос выглядит так, будто его выводит старый бортовой компьютер.
- Кокпит-интерфейс с пиксельной рамкой, HUD и живым **осциллографом**.
- Волны астероидов, счёт, жизни, пауза, Game Over и перезапуск.
- Разрушаемые и неразрушаемые астероиды с неровной low-poly-геометрией.
- Разрушаемым астероидам выдаётся **1–3 HP**; сегментированный индикатор появляется **после попадания**, а не висит над каждым камнем постоянно.
- Обычная стрельба по одному нажатию, **магазин на 12 зарядов** и перезарядка.
- Временный **Rapid Fire** power-up с автоматическим огнём.
- Взрывы, простые эффекты и постепенно усложняющийся аркадный цикл.
- Экспериментальная система предупреждения о захвате цели — местная космическая служба против стояния на месте.

Это список **реализованных механик**, а не обещание, что каждая уже идеально работает.

### Что хочется сделать 🚧

| Направление | Идея |
| --- | --- |
| **Меню и UX** | Главное меню, настройки, пауза без сюрпризов, нормальные экраны старта и поражения. |
| **Графика и спрайты** | Собственный пиксель-арт / ASCII-арт, более выразительные корабли, астероиды, враги и эффекты. |
| **Геймплей** | Новые типы противников, поведение врагов, боссы, дополнительные power-up'ы и более честный баланс. |
| **Сюжет и лор** | Постепенно собрать мир, в котором странности бортового компьютера имеют причины. Сюжет пока не раскрываю. |
| **Пасхалки и мемы** | Секреты, абсурдные сообщения терминала, внутренние шутки и отсылки, которые кто-то обязательно найдёт раньше времени. |
| **ARG** | Возможно, скрытые подсказки, шифры, фальшивые системные записи и история за пределами основного прохождения. **Пока только идея.** |
| **Звук** | Музыка, звуковые эффекты, сигналы и атмосфера старой электроники. |
| **Техническая часть** | Рефакторинг, производительность, настройки графики, сохранения, более удобная структура кода и тестирование на других ОС. |
| **Смысл / мораль** | Попробовать сделать за всей иронией что-то человечное — но без обязательной десятиминутной лекции в конце. |

**Ничего из запланированного не гарантировано.** Это хобби-проект, а не Kickstarter с дедлайнами.

### Управление

| Клавиша | Действие |
| --- | --- |
| `W A S D` | Перемещение корабля |
| `Space` | Один выстрел; при Rapid Fire — удерживать для автоогня |
| `R` | Перезарядка; после Game Over — новая попытка |
| `P` | Пауза / продолжить |

### Запуск (Linux)

Нужны установленный **Rust toolchain** и **Cargo**, а также системные зависимости Bevy для вашей Linux-системы.

```bash
git clone https://github.com/peqou/Rusteroids.git
cd Rusteroids
cargo run
```

> **Платформы:** разработка и тестирование ведутся на **Linux**. Windows **не тестировалась**; работоспособность там не подтверждена. macOS также не проверялась. Это не означает, что игра принципиально не запустится — просто никто пока не обещает.

### Немного о подходе

Здесь есть **vibecoding**, эксперименты, помощь ИИ, ручные правки, переделки и ошибки уровня «почему астероиды внезапно стали невидимыми». Код не претендует на статус эталонной архитектуры. Идеи, решения, тестирование и финальное направление проекта — процесс проб и ошибок, а не производственный конвейер.

Если однажды это станет серьёзной инди-игрой, этот README будет забавным археологическим артефактом. Если нет — зато было весело.

---

## English

### What even is this?

**Rusteroids** is an experimental space arcade shooter built with **Rust + Bevy**, featuring an **ASCII-inspired visual style** and a fictional spacecraft computer interface.

It started as a simple “what if I tried making a game?” experiment and somehow escalated into “why am I calibrating an oscilloscope inside a fake spaceship dashboard?”. This is **vibe coding for fun, experimentation, and learning Rust** — not a polished commercial production with a five-year business strategy.

**However.** There's a non-zero chance it might eventually evolve into a proper, slightly unhinged indie game with its own personality, story, and secrets best left unexplained in a README.

### What's already here ✅

- A flyable 3D spaceship with **WASD** controls and a third-person camera.
- **ASCII-inspired post-processing** that makes space feel like it is rendered by an old onboard terminal.
- A pixel-art cockpit overlay, HUD, and an animated **oscilloscope**.
- Asteroid waves, score, lives, pause, Game Over, and restart.
- Destructible and indestructible asteroids with irregular low-poly shapes.
- Destructible asteroids have **1–3 HP**; their segmented health bars become visible **only after taking damage**.
- Semi-automatic shooting, a **12-shot magazine**, and reloading.
- A temporary **Rapid Fire** pickup that unlocks automatic shooting.
- Explosions, basic effects, and an increasingly challenging arcade loop.
- An experimental target-lock warning system that strongly disapproves of camping in one spot.

These are **implemented features**, not a claim that everything is bug-free or balanced.

### The questionable master plan 🚧

| Area | Plans (subject to vibes) |
| --- | --- |
| **Menus & UX** | Main menu, settings, cleaner pause and game-over screens. |
| **Art & sprites** | Original pixel / ASCII art, more distinctive ships, asteroids, enemies, and effects. |
| **Gameplay** | More enemy types, smarter behavior, bosses, extra power-ups, and less chaotic balancing. |
| **Story & lore** | Build a world where the onboard computer's weirdness actually means something. No story spoilers yet. |
| **Easter eggs & memes** | Hidden jokes, weird terminal messages, references, and secrets nobody was supposed to notice that fast. |
| **ARG** | Maybe encrypted clues, fake logs, hidden connections, and an alternate-reality mystery. **Concept only, not implemented.** |
| **Audio** | Music, SFX, system alerts, and retro-electronic ambience. |
| **Tech** | Refactoring, performance work, graphics settings, saving, cleaner architecture, and cross-platform testing. |
| **Meaning / moral** | Perhaps hide something genuinely human underneath all the nonsense, without making it a lecture. |

**None of these are promises.** This is a hobby project, not a funded production schedule.

### Controls

| Key | Action |
| --- | --- |
| `W A S D` | Move the ship |
| `Space` | Single shot; hold during Rapid Fire for automatic fire |
| `R` | Reload; restart after Game Over |
| `P` | Pause / resume |

### Running it (Linux)

You'll need the **Rust toolchain**, **Cargo**, and the Linux system dependencies required by Bevy.

```bash
git clone https://github.com/peqou/Rusteroids.git
cd Rusteroids
cargo run
```

> **Platform status:** Built and tested on **Linux only** so far. **Windows is untested**, and compatibility is not guaranteed. macOS is also untested. It might work; it might summon a compiler error. Science requires experiments.

### About the development process

This project involves **vibe coding**, AI-assisted experimentation, manual edits, rewrites, and bugs of the “why did the asteroids become invisible?” variety. The code is **not** presented as a best-practices showcase. The design and direction are evolving through experimentation.

If Rusteroids ever becomes a serious indie release, this README will be a hilarious historical document. If it doesn't, at least we had fun.

---

**Status:** `very early WIP` · **Engine:** `Bevy` · **Language:** `Rust` · **Tested on:** `Linux` · **Stability:** `¯\_(ツ)_/¯`

*Made for the plot. The plot is still being written.*
