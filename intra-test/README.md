# test-roms

Source tree of MIT-licensed Game Boy test ROMs bundled with the
project. The contents of this directory are zipped by `tool/Makefile`
into `attachments/test-roms.zip`, which is what the subject distributes
to students and what correctors use during evaluation.

## Layout

```
test-roms/
├── acid2/
│   ├── LICENSE              # MIT, Copyright (c) 2020 Matt Currie
│   ├── dmg-acid2.gb         # PPU acid test for DMG
│   └── cgb-acid2.gbc        # PPU acid test for CGB
└── mooneye/
    ├── LICENSE              # MIT, Copyright (c) 2014-2022 Joonas Javanainen
    ├── mbc1/
    │   ├── rom_512kb.gb     # bank switching, no RAM
    │   └── ram_64kb.gb      # MBC1 + 64Kb RAM + Battery
    ├── mbc2/
    │   └── ram.gb           # built-in 4-bit RAM
    ├── mbc5/
    │   └── rom_2Mb.gb       # bank switching
    └── acceptance/
        ├── div_timing.gb    # DIV register timing
        ├── intr_timing.gb   # interrupt service timing
        └── oam_dma/
            └── basic.gb     # OAM DMA basics
```

Every ROM here is licensed under MIT. No commercial / copyrighted
material may ever land in this directory.

## How to add new ROMs

1. Verify the license is MIT or equivalent permissive (BSD, ISC,
   Apache 2.0, public domain, CC0).
2. Drop the ROM in the appropriate subdirectory.
3. Make sure a `LICENSE` file accompanies it (either reuse one of the
   existing folder licenses or add a new one).
4. Run `make -C tool` to rebuild `attachments/test-roms.zip`.
