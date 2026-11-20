# Upstream sources

GTRS is built from two upstream projects. Both are licensed under GPL-3.0,
and so is GTRS. Their sources were imported verbatim in the first two commits
of this repository; every later commit is a GTRS change.

| Upstream | Location here | Imported revision |
| --- | --- | --- |
| [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) | repository root | tag `0.2.2+26.3-26.60`, commit `eda69440e6369c5f8a05c3f69fc809c40dab93a6` |
| [PatchBukkit](https://github.com/Pumpkin-MC/PatchBukkit) | `bridge/` | commit `ffacbd0f71fff7d3f3c6c8c79acb62b92ab800d9` (0.2.2 plus three commits) |

To see what GTRS changed in either project, diff against the import commits:

```sh
git diff 0567e92 -- . ':!bridge'   # changes to Pumpkin
git diff 5f3c548 -- bridge         # changes to PatchBukkit
```

## Picked upstream changes

None yet. When an upstream fix is applied, record it here with the upstream
commit it came from.
