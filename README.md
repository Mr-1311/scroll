
- - -
<p align="center"><img src="./scroll_logo.png" alt="Scroll" height="128" width="128"/></p>
<h1 align="center">SCROLL</h1>
<p align="center"><b>MAGICAL STATIC SITE GENERATOR</b></p>
<p align="center">
<b><a href="https://metinur.github.io/scroll-doc/">Documentation</a></b>
</p>

- - -
<p align="center"><b>Scroll is a static site genetator that aims to maximize the balance of ease of producing and freedom of customization.</b></p> 

## Getting Started
### Installation
Build from source:
```
git clone https://github.com/metinUr/scroll.git
cd scroll
cargo build --release
```

Scroll also on crate.io with scroll-ssg name, to install from crates.io:
```
cargo install scroll-ssg
```

### Quick Start
```
scroll new <site_name>
cd <site_name>
scroll build --then-serve
```
Now you can visit your page on `localhost:1919` in your browser.

## Why Magical

Static site generators creates beautiful web pages almost without writing HTML
and CSS. In regular site generators, user pick one of already created theme,
layout and template, and pass some content which usually a markup language, and
site generator generate web page according to theme and templates. 

Even though this process make site creating so easy and fast, it increases
difficulty of making components that has custom style. Scroll has built-in
utility-first CSS support that lets you add custom CSS to your elements right
inside of your document. In Scroll while you can use only theme and templates
also you can change some of your elements appearance and behavior without
leaving your document. 

So, Scroll turns your markup file to a magical scroll that creates unique web
pages. 

[learn more about inline styling](https://metinur.github.io/scroll-doc/docs/styling.html)

## Disclaimer
The project, while usable, is in early stages. Therefore, you may
not find some expected features or encounter some unexpected behavior. 

## Documents Support
Scroll was born from personal needs and desires. As an Emacs and org-mode lover
Scroll is supporting only org-mode yet. But there are plans to support markdown
natively soon.

Also, via some converters Scroll can work with markdown now but it may lose some
of its magic.

## License
This project is under MIT license.
