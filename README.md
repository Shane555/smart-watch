# smart-watch

> A Rust project for a smart watch using an ESP32-C3 module.

<!-- Add a hero photo once the first board is assembled: ![Watch](docs/img/hero.jpg) -->

## Overview

A wrist-worn smart watch built around the **ESP32-C3**, covering custom PCB design, a 3D-printed enclosure, and firmware written in **Rust**.

| | |
|---|---|
| **MCU** | ESP32-C3 |
| **Firmware** | Rust (ESP-IDF `std`, `esp-idf-svc`) |
| **Display** | _TBD_ |
| **Battery** | _TBD_ |
| **Battery life (target / measured)** | _TBD_ / _TBD_ |
| **Sensors** | _TBD_ |
| **Enclosure** | _TBD_ |
| **Current hardware revision** | Rev A (not yet built) |

## Status

- [ ] Requirements agreed
- [ ] Architecture and power budget agreed
- [ ] Schematic Rev A
- [ ] Layout Rev A
- [ ] Boards fabbed and assembled
- [ ] Bring-up complete
- [ ] Firmware MVP (time, display, BLE)
- [ ] Enclosure fit-checked and printed
- [ ] Test report Rev A
- [ ] Rev B

## Repository layout

| Folder | Contents |
|---|---|
| [`docs/`](docs/) | Requirements, architecture, power budget, decisions, notebook, bring-up and test reports |
| [`hardware/`](hardware/) | KiCad project, schematic PDFs, BOM, fab outputs per revision |
| [`firmware/`](firmware/) | Rust firmware — see [`firmware/README.md`](firmware/README.md) to build and flash |
| [`mechanical/`](mechanical/) | Enclosure CAD (STEP, STL, source) |

## Key documents

- [Requirements](docs/requirements.md)
- [Architecture](docs/architecture.md)
- [Power budget](docs/power-budget.md)
- [Design decisions](docs/decisions/)
- [Engineering notebook](docs/notebook/)
- [Bring-up log](docs/bring-up-log.md)
- [Test reports](docs/test-reports/)
- [Errata](docs/errata.md)

## Building the docs site

The `docs/` folder is an [mdBook](https://rust-lang.github.io/mdBook/):

```sh
cargo install mdbook
cd docs
mdbook serve --open
```

## Licence

_TBD_
