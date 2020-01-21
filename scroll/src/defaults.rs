lazy_static! {
    pub static ref CONF: &'static str = r#"
    default_template = "default_template.html"
"#;
    pub static ref TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="/theme.css">
    <link rel="stylesheet" type="text/css" href="/scroll_style.css">
  </head>
  <body>
    <div class="container">
      {page}
    </div>
    <footer>
      Built with ✨ by <a href="https://github.com/metinUr/scroll">scroll</a>
    </footer>
  </body>
</html> "#;
    pub static ref CSS_DEFAULT: &'static str = r#"
[responsive]
[[responsive.query]]
name = "sm"
min_width = "640px"

[[responsive.query]]
name = "md"
min_width = "768px"

[[responsive.query]]
name = "lg"
min_width = "1024px"

[[responsive.query]]
name = "xl"
min_width = "1280px"

[colors]
range = 10
transparency_range = 10

[[colors.color]]
color = "ff0000"
color_alias = "red"

[[colors.color]]
color = "00ff00"
color_alias = "gre"

[[colors.color]]
color = "0000ff"
color_alias = "blu"

[lengths]
units = [ "cm", "mm", "in", "px", "pt", "pc", "em", "ex", "ch", "rem", "vw", "vh", "vmin", "vmax", "%" ]
default = "px"

[properties]
[[properties.property]]
# css syntax = background-color: $color|transparent|initial|inherit;
property_name = "background-color"
property_name_alias = "bc"

keywords = [ "transparent", "intial", "inherit" ]
keyword_aliases = [ "zero" ]

data_types = ["keyword", "color"]

[[properties.property]]
# css syntax = font-size: medium|xx-small|x-small|small|large|x-large|xx-large|smaller|larger|$length|initial|inherit;
property_name = "font-size"
property_name_alias = "fs"

keywords = [ "medium", "xx-small", "x-small", "small", "large", "x-large", "xx-large", "smaller", "larger", "initial", "inherit" ]
keyword_aliases = [ "md", "xxsm", "xsm", "sm", "lg", "xlg", "xxlg", "smr", "lgr" ]

data_types = ["keyword", "length"]
"#;
    pub static ref THEME: &'static str = r#"
/*************************************************************************/
/* Copyright (c) 2020 Metin Ur                                           */
/*                                                                       */
/* Permission is hereby granted, free of charge, to any person obtaining */
/* a copy of this software and associated documentation files (the       */
/* "Software"), to deal in the Software without restriction, including   */
/* without limitation the rights to use, copy, modify, merge, publish,   */
/* distribute, sublicense, and/or sell copies of the Software, and to    */
/* permit persons to whom the Software is furnished to do so, subject to */
/* the following conditions:                                             */
/*                                                                       */
/* The above copyright notice and this permission notice shall be        */
/* included in all copies or substantial portions of the Software.       */
/*                                                                       */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,       */
/* EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF    */
/* MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.*/
/* IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY  */
/* CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,  */
/* TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE     */
/* SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.                */
/*************************************************************************/

*,
*:after,
*:before {
  box-sizing: inherit;
}

html {
  box-sizing: border-box;
  font-size: 62.5%;
}

body {
  color: #212121;
  background-color: #FAFAFA;
  font-family: Merriweather, Georgia, serif;
  font-size: 1.6em;
  font-weight: 300;
  line-height: 1.8em;
}
@media only screen and (max-width: 768px) {
  body {
    font-size: 1.6em;
    line-height: 1.6em;
  }
}

a {
  font-weight: 300;
  color: #1565c0;
  text-decoration: none;
}
a:focus, a:hover {
  text-decoration: underline;
}

p {
  margin: 2rem 0 2rem 0;
}

h1,
h2,
h3,
h4,
h5,
h6 {
  font-family: Lato, Helvetica, sans-serif;
  font-weight: 700;
  color: #000;
  margin: 6.4rem 0 3.2rem 0;
}

h1 {
  font-size: 3.2rem;
  line-height: 3.6rem;
}
@media only screen and (max-width: 768px) {
  h1 {
    font-size: 3rem;
    line-height: 3.4rem;
  }
}

h2 {
  font-size: 2.8rem;
  line-height: 3.2rem;
}
@media only screen and (max-width: 768px) {
  h2 {
    font-size: 2.6rem;
    line-height: 3rem;
  }
}

h3 {
  font-size: 2.4rem;
  line-height: 2.8rem;
}
@media only screen and (max-width: 768px) {
  h3 {
    font-size: 2.2rem;
    line-height: 2.6rem;
  }
}

h4 {
  font-size: 2.2rem;
  line-height: 2.6rem;
}
@media only screen and (max-width: 768px) {
  h4 {
    font-size: 2rem;
    line-height: 2.4rem;
  }
}

h5 {
  font-size: 2rem;
  line-height: 2.4rem;
}
@media only screen and (max-width: 768px) {
  h5 {
    font-size: 1.8rem;
    line-height: 2.2rem;
  }
}

h6 {
  font-size: 1.8rem;
  line-height: 2.2rem;
}
@media only screen and (max-width: 768px) {
  h6 {
    font-size: 1.6rem;
    line-height: 2rem;
  }
}

b, strong {
  font-weight: 700;
}

pre {
  display: block;
  font-family: "Source Code Pro", "Lucida Console", monospace;
  font-size: 1.6rem;
  font-weight: 400;
  line-height: 2.6rem;
  margin: 2rem 0 2rem 0;
  padding: 2rem;
  overflow-x: auto;
}
pre code {
  display: inline-block;
  background-color: inherit;
  color: inherit;
}

code {
  font-family: "Source Code Pro", "Lucida Console", monospace;
  font-size: 1.6rem;
  font-weight: 400;
  background-color: #E0E0E0;
  color: #212121;
  padding: 0.2rem 0.4rem 0.2rem 0.4rem;
}

blockquote {
  border-left: 2px solid #E0E0E0;
  padding-left: 2rem;
  line-height: 2.2rem;
  font-weight: 400;
  font-style: italic;
}

th, td {
  padding: 1.6rem;
}

table {
  border-collapse: collapse;
}

table td, table th {
  border: 2px solid #000;
}

table tr:first-child th {
  border-top: 0;
}

table tr:last-child td {
  border-bottom: 0;
}

table tr td:first-child,
table tr th:first-child {
  border-left: 0;
}

table tr td:last-child,
table tr th:last-child {
  border-right: 0;
}

img {
  max-width: 100%;
}

footer {
  width: 100%;
  text-align: center;
  line-height: 2rem;
  margin-bottom: 1rem;
}

footer a {
  color: #1565c0;
}

.container {
  margin: 0 auto;
  max-width: 90.0rem;
  width: 100%;
  padding-left: 2.0rem;
  padding-right: 2.0rem;
}
"#;
}
