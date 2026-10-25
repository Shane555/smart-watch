# Mechanical

Enclosure CAD, one folder per revision, matched to the hardware revision it fits.

```
mechanical/
├── rev-a/
│   ├── source/   native CAD files
│   ├── step/     STEP exports (for others to open, and for the KiCad 3D fit-check)
│   └── stl/      print files, with print settings noted in the README or notebook
└── rev-b/ ...
```

KiCad files are text and diff well in Git; STEP and STL files are large binaries. If they start to
bloat the repo, move `*.step` and `*.stl` to Git LFS.
