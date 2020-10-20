KaTeX for Yew
=============

Render math with KaTeX in Yew!

- The online preview: https://galaster.github.io/yew-katex

## How to use

1. It's only do the wasm bind, so load cdn first

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.12.0/dist/katex.min.css">
```

2. Easily used by `<KaTeX/>`

```rust
use yew_katex::KaTeX;

html! {
    <KaTeX math="\\KaTeX" inline=false/>
}
```

## Todo

- [ ] Automatically import css cdn when the first component is loaded
