---
winres: minor
---

`winres` is now more strict about `embed_resource`'s result (using manifest_required instead of manifest_option) and therefore may panic more likely, for example if the environment is missing a resource compiler.
