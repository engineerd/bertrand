---
title: "Bertrand: a client-only WebAssembly blogging engine"
description:
  "Bertrand is a very simple blogging engine that dynamically renders markdown
  content to HTML in your browser using WebAssembly. This article explains the
  goals of Bertrand, and offers an introduction to using and contributing to the
  project."
date: "September 9th 2021"
author: "Radu Matei"
template: "main"
extra:
  some-user-defined-key: "some-value"
---

Static site generators are great! They take markdown content and HTML templates
and generate the entire website as plain HTML files that can be deployed to any
static site service.

Bertrand is very similar -- it takes markdown content and HTML templates, but it
dynamically renders web pages just in time in the user's browser using
WebAssembly. The main benefit is not being required to manage two sets of files,
as is the case for static site generators -- your markdown content and templates
(and optionally scripts) _are_ the website, and when a request for your page
comes in, Bertrand uses the markdown and templates to output an HTML
representation of your page.

This started mostly as a case of "huh, it would be interesting to do this
client-side only", inspired by
[Bartholomew](https://github.com/technosophos/bartholomew), but it quickly
turned into wanting to build an engine that could power a real world website
with minimal trade-offs (more on this later). It is intended for people who want
a very simple blogging engine that just renders some content, with no fancy
features.

## The basics

There are a few things required to serve a website using Bertrand:

- `bertrand.yaml` -- this file contains general information that is used
  throughout the website, user-defined metadata, and a list of all scripts and
  templates required to render a page. For example, the current `bertrand.yaml`
  file for this website is below:

```yaml
title: Bertrand
base_url: http://localhost:8080
scripts:
  - echo
templates:
  - header
  - main
  - footer
  - index
  - 404

extra:
  copyright: "Engineerd, 2021"
  github: engineerd
```

- next, you need a bit of JavaScript that instantiates the WebAssembly module
  containing the actual application. Because this is using
  [`wasm-pack`](https://github.com/rustwasm/wasm-pack), we already have the
  required JS glue code necessary to start our application, so we only have to
  reference this from our `index.html`:

```js
import init, { run_app } from "./pkg/bertrand.js";

async function main() {
  await init("/pkg/bertrand_bg.wasm");
  run_app();
}
main();
```

- templates -- the templates use [Handlebars](https://handlebarsjs.com/), a
  Mustache-compatible templating syntax, and at rendering time, Bertrand reads
  the metadata from the markdown files and renders the web page accordingly.
  Here is the relevant part for a page that renders an article body:

```html
<article class="blog-post">
  <div class="subtitle is-6">
    {{ page.frontmatter.date }}, by {{ page.frontmatter.author }}
  </div>
  <h1 class="title">{{ title }}</h1>
  {{{ page.body }}}
</article>
```

- scripts -- you can dynamically manipulate the contents of your before
  rendering using [Rhai](https://rhai.rs/book/), _an embedded scripting language
  and evaluation engine_. Rhai scripts are placed in the `scripts/` directory,
  and they can be called from any template -- for example: `{{ echo "world" }}`

```rust
// scripts/echo.rhai
let msg = params[0];

"hello " + msg;
```

- content -- write your page contents using plain markdown, store them in the
  `content/` directory. As we saw in the template, we need a bit of metadata at
  the beginning that can be accessed through `page.frontmatter` variables:

```yaml
title: "Bertrand: a client-only WebAssembly blogging engine"
description: "Some description"
date: "September 9th 2021"
author: "Radu Matei"
template: "main"
extra:
  some-user-defined-key: "some-value"
```

## How about a page list?

When rendering a page using a static site generator, it can access all pages
from the disk, iterate through them, then create a list of pages that can be
used as the main page for a blog. Bertrand could take the same approach, but it
would have to fetch the entire content of the website just to generate a front
page with a preview for the articles. This is not ideal, particularly because it
would be fetching the content through the network -- so instead, Bertrand has
the concept of an article list that can be used together with a template that
can display a list:

```yaml
template: "index"

articles:
  # this is an array
  - title: "What is markdown?"
    description: "Some description"
    date: "September 8th 2021"
    author: "Radu Matei"
    route: "what-is-markdown"
```

An example for a template that renders multiple objects from the articles array:

```html
{{ #if page.frontmatter.articles }} {{ #each page.frontmatter.articles as |a| }}

<article class="blog-post">
  <div class="subtitle is-6">{{ a.date }}, by {{ a.author }}</div>
  <a href="{{ site.base_url }}/{{ a.route }}">
    <h2 class="title">{{ a.title }}</h2>
  </a>
  <p>{{{ a.description }}}</p>
</article>

{{/each}} {{/if}}
```

This requires keeping track of an additional file, which when using static site
generators is automatically handled, but it is a trade-off that is needed to
avoid fetching the entire website contents just to render a single page.

## FAQ

Q: How big is the WebAssembly module?

A: The current size of the Bertrand Wasm module is around 2 MB. It is relatively
large compared to a single static page but it can be compressed over the
network, and depending on the settings used it _can_ be cached by the browser.
We hope to be able to shrink it in the future.

Q: Are the templates and content fetched on each request?

A: Depending on how the server is configured, the browser can use cached
versions of the fetched files (note that the current development setup does
not). We also have plans to store the templates, scripts, and content in the
local storage, and use it to cache content.

Q: How can I host this?

A: Normally, you can host it in the same way as a static site -- copy the entire
`example` (in the case of this repository) directory, and make it the root of
the website. The only additional requirement is to be able to redirect to
`index.html` when a page is not found -- this is because routes are dynamic, and
this essentially acts as a SPA (single page application). Instructions on how to
do this on GitHub Pages will follow.

Q: Should I use Bertrand right now?

A: Probably not. It is a very early and experimental project, and should not be
used for anything serious, as it has a lot of limitations and bugs (see the code
base or issue list).

Q: What about images?

A: Because we want both Bertrand and all other markdown renderers to display
your content in the same way, images are always loaded from the same relative
path, so just include them in the same way you would in a regular markdown file.

Q: What about users who disabled JavaScript, or don't have WebAssembly enabled?

A: They should still be able to read your content, as it is written in plain
markdown, just redirect them to the `.md` file.
