# Hardware

KiCad project for the watch PCB. One folder per board revision, so each revision's design and
fab outputs stay together and can be compared later.

```
hardware/
├── rev-a/
│   ├── kicad/        KiCad project (.kicad_pro, .kicad_sch, .kicad_pcb)
│   ├── schematic.pdf exported schematic for quick reading
│   ├── bom.csv
│   └── fab/          Gerbers, drill files, pick-and-place, as sent to the fab
└── rev-b/ ...
```

Tag the commit that was sent to the fab, e.g. `git tag hw-rev-a`.
What went wrong on each revision goes in [`docs/errata.md`](../docs/errata.md).
