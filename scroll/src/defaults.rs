lazy_static! {
    pub static ref CONF: &'static str = r#"
    default_template = "default_template.html"
"#;
    pub static ref TEMPLATE: &'static str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="/theme.css">
    <link rel="stylesheet" type="text/css" href="/scroll_style.css">
  </head>
  <body>
    <div style="text-align: center; padding: 15px">
      <a class="nav" href="/">HOME</a>
      <hr style="width: 90%">
    </div>
    <div class="container">
      {{ if title }}<h1 class="title">{title}</h1>{{ endif }}
      {{ if date }}<h1 class="date">{date}</h1>{{ endif }}
      {{ if title }}<hr style="margin-bottom: 10rem;">{{ endif }}
      <<page>>
    </div>
    <footer>
      Built with ✨ by <a href="https://github.com/metinUr/scroll">scroll</a>
    </footer>
  </body>
</html>"#;
    pub static ref EMPTY_TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="/theme.css">
    <link rel="stylesheet" type="text/css" href="/scroll_style.css">
  </head>
  <body>
    <<page>>
  </body>
</html>
"#;
    pub static ref CSS_DEFAULT: &'static str = r#"[responsive]
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
    color = "000000"
    color_alias = "black"

  [[colors.color]]
    color = "ffffff"
    color_alias = "white"

  [[colors.color]]
    color = "95a5a6"
    color_alias = "grey"

  [[colors.color]]
    color = "e74c3c"
    color_alias = "red"

  [[colors.color]]
    color = "2ecc71"
    color_alias = "green"

  [[colors.color]]
    color = "3498db"
    color_alias = "blue"

  [[colors.color]]
    color = "e67e22"
    color_alias = "orange"

  [[colors.color]]
    color = "f1c40f"
    color_alias = "yellow"

  [[colors.color]]
    color = "9b59b6"
    color_alias = "purple"

  [[colors.color]]
    color = "ed64a6"
    color_alias = "pink"

  [[colors.color]]
    color = "1abc9c"
    color_alias = "teal"

  [[colors.color]]
    color = "2c3e50"
    color_alias = "space"

[lengths]
  units = [ "cm", "mm", "in", "px", "pt", "pc", "em", "ex", "ch", "rem", "vw", "vh", "vmin", "vmax", "%" ]
  default = "px"

[properties]
  [[properties.property]]
    ## css syntax = align-content: stretch|center|flex-start|flex-end|space-between|space-around|initial|inherit;
    property_name = "align-content"
    property_name_alias = "ac"

    keywords = [ "stretch", "center", "flex-start", "flex-end", "space-between", "space-around", "intial", "inherit" ]
    keyword_aliases = [ "_", "_", "start", "end", "between", "around" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = align-items: stretch|center|flex-start|flex-end|baseline|initial|inherit;
    property_name = "align-items"
    property_name_alias = "ai"

    keywords = [ "stretch", "center", "flex-start", "flex-end", "baseline", "intial", "inherit" ]
    keyword_aliases = [ "_", "_", "start", "end", "base" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = align-self: auto|stretch|center|flex-start|flex-end|baseline|initial|inherit;
    property_name = "align-self"
    property_name_alias = "as"

    keywords = [ "auto", "stretch", "center", "flex-start", "flex-end", "baseline", "intial", "inherit" ]
    keyword_aliases = [ "_", "_", "_", "start", "end", "base" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = all: initial|inherit|unset;
    property_name = "all"
    property_name_alias = "all"

    keywords = [ "intial", "inherit", "unset" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = background-attachment: scroll|fixed|local|initial|inherit;
    property_name = "background-attachment"
    property_name_alias = "ba"

    keywords = [ "scroll", "fixed", "local", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax =  background-clip: border-box|padding-box|content-box|initial|inherit;
    property_name = "background-clip"
    property_name_alias = "bclip"

    keywords = [ "border-box", "padding-box", "content-box", "intial", "inherit" ]
    keyword_aliases = [ "bbox", "pbox", "cbox" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = background-color: $color|transparent|initial|inherit;
    property_name = "background-color"
    property_name_alias = "bc"

    keywords = [ "transparent", "intial", "inherit" ]
    keyword_aliases = [ "zero" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = background-image: url|none|initial|inherit;
    property_name = "background-image"
    property_name_alias = "bi"

    keywords = [ "none", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "url"]

  [[properties.property]]
    ## css syntax = background-origin: padding-box|border-box|content-box|initial|inherit;
    property_name = "background-origin"
    property_name_alias = "bcorigin"

    keywords = [ "border-box", "padding-box", "content-box", "intial", "inherit" ]
    keyword_aliases = [ "bbox", "pbox", "cbox" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = background-position: value;
    property_name = "background-position"
    property_name_alias = "bp"

    keywords = [ "center", "top", "bottom", "left", "right", "intial", "inherit" ]
    keyword_aliases = [ "c", "t", "b", "l", "r" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = background-repeat: repeat|repeat-x|repeat-y|no-repeat|initial|inherit;
    property_name = "background-repeat"
    property_name_alias = "br"

    keywords = [ "repeat", "repeat-x", "repeat-y", "no-repeat", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = background-size: auto|length|cover|contain|initial|inherit;
    property_name = "background-size"
    property_name_alias = "bs"

    keywords = [ "auto", "cover", "contain", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-width: medium|thin|thick|length|initial|inherit;
    property_name = "border-width"
    property_name_alias = "borw"

    keywords = [ "medium", "thin", "thick", "intial", "inherit" ]
    keyword_aliases = [ "md", "thn", "thc" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;
    property_name = "border-style"
    property_name_alias = "bors"

    keywords = [ "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "intial", "inherit" ]
    keyword_aliases = [ "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = border-color: color|transparent|initial|inherit;
    property_name = "border-color"
    property_name_alias = "borc"

    keywords = [ "transparent", "intial", "inherit" ]
    keyword_aliases = [ "zero" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = border: border-width border-style border-color|initial|inherit;
    property_name = "border"
    property_name_alias = "bor"

    keywords = [ "medium", "thin", "thick", "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "transparent", "intial", "inherit" ]
    keyword_aliases = [  "md", "thn", "thc", "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out", "zero" ]

    data_types = ["keyword", "length", "color" ]

  [[properties.property]]
    ## css syntax = border-bottom-width: medium|thin|thick|length|initial|inherit;
    property_name = "border-bottom-width"
    property_name_alias = "borbw"

    keywords = [ "medium", "thin", "thick", "intial", "inherit" ]
    keyword_aliases = [ "md", "thn", "thc" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-bottom-style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;
    property_name = "border-bottom-style"
    property_name_alias = "borbs"

    keywords = [ "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "intial", "inherit" ]
    keyword_aliases = [ "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = border-bottom-color: color|transparent|initial|inherit;
    property_name = "border-bottom-color"
    property_name_alias = "borbc"

    keywords = [ "transparent", "intial", "inherit" ]
    keyword_aliases = [ "zero" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = border-bottom: border-width border-style border-color|initial|inherit;
    property_name = "border-bottom"
    property_name_alias = "borb"

    keywords = [ "medium", "thin", "thick", "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "transparent", "intial", "inherit" ]
    keyword_aliases = [  "md", "thn", "thc", "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out", "zero" ]

    data_types = ["keyword", "length", "color" ]

  [[properties.property]]
    ## css syntax = border-top-width: medium|thin|thick|length|initial|inherit;
    property_name = "border-top-width"
    property_name_alias = "bortw"

    keywords = [ "medium", "thin", "thick", "intial", "inherit" ]
    keyword_aliases = [ "md", "thn", "thc" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-top-style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;
    property_name = "border-top-style"
    property_name_alias = "borts"

    keywords = [ "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "intial", "inherit" ]
    keyword_aliases = [ "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = border-top-color: color|transparent|initial|inherit;
    property_name = "border-top-color"
    property_name_alias = "bortc"

    keywords = [ "transparent", "intial", "inherit" ]
    keyword_aliases = [ "zero" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = border-top: border-width border-style border-color|initial|inherit;
    property_name = "border-top"
    property_name_alias = "bort"

    keywords = [ "medium", "thin", "thick", "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "transparent", "intial", "inherit" ]
    keyword_aliases = [  "md", "thn", "thc", "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out", "zero" ]

    data_types = ["keyword", "length", "color" ]

  [[properties.property]]
    ## css syntax = border-left-width: medium|thin|thick|length|initial|inherit;
    property_name = "border-left-width"
    property_name_alias = "borlw"

    keywords = [ "medium", "thin", "thick", "intial", "inherit" ]
    keyword_aliases = [ "md", "thn", "thc" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-left-style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;
    property_name = "border-left-style"
    property_name_alias = "borls"

    keywords = [ "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "intial", "inherit" ]
    keyword_aliases = [ "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = border-left-color: color|transparent|initial|inherit;
    property_name = "border-left-color"
    property_name_alias = "borlc"

    keywords = [ "transparent", "intial", "inherit" ]
    keyword_aliases = [ "zero" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = border-left: border-width border-style border-color|initial|inherit;
    property_name = "border-left"
    property_name_alias = "borl"

    keywords = [ "medium", "thin", "thick", "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "transparent", "intial", "inherit" ]
    keyword_aliases = [  "md", "thn", "thc", "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out", "zero" ]

    data_types = ["keyword", "length", "color" ]

  [[properties.property]]
    ## css syntax = border-right-width: medium|thin|thick|length|initial|inherit;
    property_name = "border-right-width"
    property_name_alias = "borrw"

    keywords = [ "medium", "thin", "thick", "intial", "inherit" ]
    keyword_aliases = [ "md", "thn", "thc" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-right-style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;
    property_name = "border-right-style"
    property_name_alias = "borrs"

    keywords = [ "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "intial", "inherit" ]
    keyword_aliases = [ "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = border-right-color: color|transparent|initial|inherit;
    property_name = "border-right-color"
    property_name_alias = "borrc"

    keywords = [ "transparent", "intial", "inherit" ]
    keyword_aliases = [ "zero" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = border-right: border-width border-style border-color|initial|inherit;
    property_name = "border-right"
    property_name_alias = "borr"

    keywords = [ "medium", "thin", "thick", "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "transparent", "intial", "inherit" ]
    keyword_aliases = [  "md", "thn", "thc", "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out", "zero" ]

    data_types = ["keyword", "length", "color" ]

  [[properties.property]]
    ## css syntax = border-bottom-left-radius: length|% [length|%]|initial|inherit;
    property_name = "border-bottom-left-radius"
    property_name_alias = "radbl"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-bottom-right-radius: length|% [length|%]|initial|inherit;
    property_name = "border-bottom-right-radius"
    property_name_alias = "radbr"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-top-left-radius: length|% [length|%]|initial|inherit;
    property_name = "border-top-left-radius"
    property_name_alias = "radtl"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-top-right-radius: length|% [length|%]|initial|inherit;
    property_name = "border-top-right-radius"
    property_name_alias = "radtr"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-radius: 1-4 length|% / 1-4 length|%|initial|inherit;
    property_name = "border-radius"
    property_name_alias = "rad"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-spacing: length|initial|inherit;
    property_name = "border-spacing"
    property_name_alias = "borspa"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = border-collapse: separate|collapse|initial|inherit;
    property_name = "border-collapse"
    property_name_alias = "borcol"

    keywords = [ "separate", "collapse", "intial", "inherit" ]
    keyword_aliases = [ "sp", "cl" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = bottom: auto|length|initial|inherit;
    property_name = "bottom"
    property_name_alias = "bt"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = box-shadow: none|h-offset v-offset blur spread color |inset|initial|inherit;
    property_name = "box-shadow"
    property_name_alias = "sha"

    keywords = [ "none", "inset", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length", "color" ]

  [[properties.property]]
    ## css syntax = box-sizing: content-box|border-box|initial|inherit;
    property_name = "box-sizing"
    property_name_alias = "boxs"

    keywords = [ "content-box", "border-box", "intial", "inherit" ]
    keyword_aliases = [ "cb", "bb" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = clear: none|left|right|both|initial|inherit;
    property_name = "clear"
    property_name_alias = "cl"

    keywords = [ "none", "left", "right", "both", "intial", "inherit" ]
    keyword_aliases = [ "no", "l", "r", "bo" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = color: color|initial|inherit;
    property_name = "color"
    property_name_alias = "clr"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "color" ]

  [[properties.property]]
    ## css syntax = column-count: number|auto|initial|inherit;
    property_name = "column-count"
    property_name_alias = "colc"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number"]

  [[properties.property]]
    ## css syntax = column-gap: length|normal|initial|inherit;
    property_name = "column-gap"
    property_name_alias = "colg"

    keywords = [ "normal", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = column-rule-width: medium|thin|thick|length|initial|inherit;
    property_name = "column-rule-width"
    property_name_alias = "colrw"

    keywords = [ "medium", "thin", "thick", "intial", "inherit" ]
    keyword_aliases = [ "md", "thn", "thc" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = column-rule-style: none|hidden|dotted|dashed|solid|double|groove|ridge|inset|outset|initial|inherit;
    property_name = "column-rule-style"
    property_name_alias = "colrs"

    keywords = [ "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "intial", "inherit" ]
    keyword_aliases = [ "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = column-rule-color: color|initial|inherit;
    property_name = "column-rule-color"
    property_name_alias = "colrc"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = column-rule: column-rule-width column-rule-style column-rule-color|initial|inherit;
    property_name = "column-rule"
    property_name_alias = "colr"

    keywords = [ "medium", "thin", "thick", "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge", "inset", "outset", "transparent", "intial", "inherit" ]
    keyword_aliases = [  "md", "thn", "thc", "no", "hi", "do", "da", "so", "dol", "gro", "ri", "in", "out", "zero" ]

    data_types = ["keyword", "length", "color" ]

  [[properties.property]]
    ## css syntax = column-width: auto|length|initial|inherit;
    property_name = "column-width"
    property_name_alias = "colw"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = columns: auto|column-width column-count|initial|inherit;
    property_name = "columns"
    property_name_alias = "col"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length", "number" ]

  [[properties.property]]
    ## css syntax = cursor: value;
    property_name = "cursor"
    property_name_alias = "cur"

    keywords = [ "alias", "auto", "cell", "copy", "crosshair", "default", "grab", "grabbing", "help", "move", "none", "pointer", "progress", "text", "wait", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = direction: ltr|rtl|initial|inherit;
    property_name = "direction"
    property_name_alias = "dir"

    keywords = [ "ltr", "rtl", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = display: value;
    property_name = "display"
    property_name_alias = "dis"

    keywords = [ "inline", "block", "contents", "flex", "grid", "table", "none", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = flex-grow: number|initial|inherit;
    property_name = "flex-grow"
    property_name_alias = "fg"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number"]

  [[properties.property]]
    ## css syntax = flex-shrink: number|initial|inherit;
    property_name = "flex-shrink"
    property_name_alias = "fsh"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number"]

  [[properties.property]]
    ## css syntax = flex-basis: number|auto|initial|inherit;
    property_name = "flex-basis"
    property_name_alias = "fb"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length" ]

  [[properties.property]]
    ## css syntax = flex: flex-grow flex-shrink flex-basis|auto|initial|inherit;
    property_name = "flex"
    property_name_alias = "f"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number", "length" ]

  [[properties.property]]
    ## css syntax = flex-direction: row|row-reverse|column|column-reverse|initial|inherit;
    property_name = "flex-direction"
    property_name_alias = "fdir"

    keywords = [ "row", "column", "intial", "inherit" ]
    keyword_aliases = [ "r", "c" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = flex-wrap: nowrap|wrap|wrap-reverse|initial|inherit;
    property_name = "flex-wrap"
    property_name_alias = "fwrap"

    keywords = [ "nowrap", "wrap", "intial", "inherit" ]
    keyword_aliases = [ "n", "w" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = flex-flow: flex-direction flex-wrap|initial|inherit;
    property_name = "flex-flow"
    property_name_alias = "fflow"

    keywords = [ "row", "column", "nowrap", "wrap", "intial", "inherit" ]
    keyword_aliases = [ "r", "c", "n", "w" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = float: none|left|right|initial|inherit;
    property_name = "float"
    property_name_alias = "flo"

    keywords = [ "none", "left", "right", "intial", "inherit" ]
    keyword_aliases = [ "n", "l", "r" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = font-family: family-name|generic-family|initial|inherit;
    property_name = "font-family"
    property_name_alias = "fm"

    keywords = [ "initial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "string"]

  [[properties.property]]
    ## css syntax = font-size: medium|xx-small|x-small|small|large|x-large|xx-large|smaller|larger|$length|initial|inherit;
    property_name = "font-size"
    property_name_alias = "fs"

    keywords = [ "medium", "xx-small", "x-small", "small", "large", "x-large", "xx-large", "smaller", "larger", "initial", "inherit" ]
    keyword_aliases = [ "md", "xxsm", "xsm", "sm", "lg", "xlg", "xxlg", "smr", "lgr" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = font-size-adjust: number|none|initial|inherit;
    property_name = "font-size-adjust"
    property_name_alias = "fsa"

    keywords = [ "none", "intial", "inherit" ]
    keyword_aliases = [ "n" ]

    data_types = ["keyword", "number"]

  [[properties.property]]
    ## css syntax = font-style: normal|italic|oblique|initial|inherit;
    property_name = "font-style"
    property_name_alias = "fsty"

    keywords = [ "normal", "italic", "oblique", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = font-weight: normal|bold|bolder|lighter|number|initial|inherit;
    property_name = "font-weight"
    property_name_alias = "fw"

    keywords = [ "normal", "bold", "bolder", "lighter", "intial", "inherit" ]
    keyword_aliases = [ "norm", "b", "br", "lr" ]

    data_types = ["keyword", "number"]

  [[properties.property]]
    ## css syntax = height: auto|length|initial|inherit;
    property_name = "height"
    property_name_alias = "h"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = justify-content: flex-start|flex-end|center|space-between|space-around|initial|inherit;
    property_name = "justify-content"
    property_name_alias = "jc"

    keywords = [ "flex-start", "flex-end", "center", "space-between", "space-around", "intial", "inherit" ]
    keyword_aliases = [ "fs", "fe", "c", "sb", "sa" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = left: auto|length|initial|inherit;
    property_name = "left"
    property_name_alias = "l"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = letter-spacing: normal|length|initial|inherit;
    property_name = "letter-spacing"
    property_name_alias = "letspa"

    keywords = [ "normal", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = line-height: normal|number|length|initial|inherit;
    property_name = "line-height"
    property_name_alias = "lh"

    keywords = [ "normal", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number", "length"]

  [[properties.property]]
    ## css syntax = margin: length|auto|initial|inherit;
    property_name = "margin"
    property_name_alias = "m"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = margin-bottom: length|auto|initial|inherit;
    property_name = "margin-bottom"
    property_name_alias = "mb"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = margin-top: length|auto|initial|inherit;
    property_name = "margin-top"
    property_name_alias = "mt"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = margin-left: length|auto|initial|inherit;
    property_name = "margin-left"
    property_name_alias = "ml"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = margin-right: length|auto|initial|inherit;
    property_name = "margin-right"
    property_name_alias = "mr"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = max-height: none|length|initial|inherit;
    property_name = "max-height"
    property_name_alias = "maxh"

    keywords = [ "none", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "lenght"]

  [[properties.property]]
    ## css syntax = min-height: none|length|initial|inherit;
    property_name = "min-height"
    property_name_alias = "minh"

    keywords = [ "none", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "lenght"]

  [[properties.property]]
    ## css syntax = max-width: length|initial|inherit;
    property_name = "max-width"
    property_name_alias = "maxw"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "lenght"]

  [[properties.property]]
    ## css syntax = min-width: length|initial|inherit;
    property_name = "min-width"
    property_name_alias = "minw"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "lenght"]

  [[properties.property]]
    ## css syntax = mix-blend-mode: normal|multiply|screen|overlay|darken|lighten|color-dodge|color-burn|difference|exclusion|hue|saturation|color|luminosity;
    property_name = "mix-blend-mode"
    property_name_alias = "mbm"

    keywords = [ "normal", "multiply", "screen", "overlay", "darken", "lighten", "color-dodge", "color-burn", "difference", "exclusion", "hue", "saturation", "color", "luminosity" ]
    keyword_aliases = [ "norm", "mul", "sc", "ov", "dar", "lig", "cd", "cb", "dif", "exc", "hu", "sat", "col", "lum" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = object-fit: fill|contain|cover|scale-down|none|initial|inherit;
    property_name = "object-fit"
    property_name_alias = "of"

    keywords = [ "fill", "contain", "cover", "scale-down", "none", "intial", "inherit" ]
    keyword_aliases = [ "fi", "con", "cov", "sd", "no" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = object-position: position|initial|inherit;
    property_name = "object-positon"
    property_name_alias = "op"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = opacity: number|initial|inherit;
    property_name = "opacity"
    property_name_alias = "o"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number"]

  [[properties.property]]
    ## css syntax = order: number|initial|inherit;
    property_name = "order"
    property_name_alias = "or"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number"]

  [[properties.property]]
    ## css syntax =  overflow: visible|hidden|scroll|auto|initial|inherit;
    property_name = "overflow"
    property_name_alias = "ov"

    keywords = [ "visible", "hidden", "scroll", "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax =  overflow-x: visible|hidden|scroll|auto|initial|inherit;
    property_name = "overflow-x"
    property_name_alias = "ovx"

    keywords = [ "visible", "hidden", "scroll", "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]
  [[properties.property]]
    ## css syntax =  overflow-y: visible|hidden|scroll|auto|initial|inherit;
    property_name = "overflow-y"
    property_name_alias = "ovy"

    keywords = [ "visible", "hidden", "scroll", "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = padding: length|initial|inherit;
    property_name = "padding"
    property_name_alias = "p"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = padding-bottom: length|initial|inherit;
    property_name = "padding-bottom"
    property_name_alias = "pb"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = padding-top: length|initial|inherit;
    property_name = "padding-top"
    property_name_alias = "pt"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = padding-left: length|initial|inherit;
    property_name = "padding-left"
    property_name_alias = "pl"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = padding-right: length|initial|inherit;
    property_name = "padding-right"
    property_name_alias = "pr"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = position: static|absolute|fixed|relative|sticky|initial|inherit;
    property_name = "position"
    property_name_alias = "pos"

    keywords = [ "static", "absolute", "fixed", "relative", "sticky", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = resize: none|both|horizontal|vertical|initial|inherit;
    property_name = "resize"
    property_name_alias = "res"

    keywords = [ "none", "both", "horizontal", "vertical", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = right: auto|length|initial|inherit;
    property_name = "right"
    property_name_alias = "r"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = tab-size: number|length|initial|inherit;
    property_name = "tab-size"
    property_name_alias = "ts"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number", "length"]

  [[properties.property]]
    ## css syntax = text-align: left|right|center|justify|initial|inherit;
    property_name = "text-align"
    property_name_alias = "ta"

    keywords = [ "left", "right", "center", "justify", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = text-align-last: auto|left|right|center|justify|start|end|initial|inherit;
    property_name = "text-align-last"
    property_name_alias = "tal"

    keywords = [ "auto", "left", "right", "center", "justify", "start", "end", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = text-decoration-line: none|underline|overline|line-through|initial|inherit;
    property_name = "text-decoration-line"
    property_name_alias = "tdl"

    keywords = [ "none", "underline", "overline", "line-through", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = text-decoration-color: color|initial|inherit;
    property_name = "text-decoration-color"
    property_name_alias = "tdc"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = text-decoration-style: solid|double|dotted|dashed|wavy|initial|inherit;
    property_name = "text-decoraion-style"
    property_name_alias = "tds"

    keywords = [ "solid", "double", "dotted", "dashed", "wavy", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = text-decoration: text-decoration-line text-decoration-color text-decoration-style|initial|inherit;
    property_name = "text-decoration"
    property_name_alias = "td"

    keywords = [ "none", "underline", "overline", "line-through", "solid", "double", "dotted", "dashed", "wavy", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "color"]

  [[properties.property]]
    ## css syntax = text-indent: length|initial|inherit;
    property_name = "text-indent"
    property_name_alias = "ti"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = text-shadow: h-shadow v-shadow blur-radius color|none|initial|inherit;
    property_name = "text-shadow"
    property_name_alias = "tsha"

    keywords = [ "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length", "color"]

  [[properties.property]]
    ## css syntax = text-transform: none|capitalize|uppercase|lowercase|initial|inherit;
    property_name = "text-transform"
    property_name_alias = "tt"

    keywords = [ "none", "capitalize", "uppercase", "lovercase", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = top: auto|length|initial|inherit;
    property_name = "top"
    property_name_alias = "t"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = user-select: auto|none|text|all;
    property_name = "user-select"
    property_name_alias = "us"

    keywords = [ "auto", "none", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = vertical-align: baseline|length|sub|super|top|text-top|middle|bottom|text-bottom|initial|inherit;
    property_name = "vertical-align"
    property_name_alias = "va"

    keywords = [ "text-top", "text-bottom", "baseline", "sub", "super", "top", "middle", "bottom", "intial", "inherit" ]
    keyword_aliases = [ "tt", "tb" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax =  visibility: visible|hidden|collapse|initial|inherit;
    property_name = "visibility"
    property_name_alias = "vis"

    keywords = [ "visibles", "hidden", "collapse", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = white-space: normal|nowrap|pre|pre-line|pre-wrap|initial|inherit;
    property_name = "white-space"
    property_name_alias = "ws"

    keywords = [ "pre-line", "pre-wrap", "normal", "nowrap", "pre", "intial", "inherit" ]
    keyword_aliases = [ "pl", "pw" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = width: auto|length|initial|inherit;
    property_name = "width"
    property_name_alias = "w"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = word-spacing: normal|length|initial|inherit;
    property_name = "word-spacing"
    property_name_alias = "ws"

    keywords = [ "normal", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "length"]

  [[properties.property]]
    ## css syntax = writing-mode: horizontal-tb|vertical-rl|vertical-lr;
    property_name = "writing-mode"
    property_name_alias = "wm"

    keywords = [ "horizontal-tb", "vertical-rl", "vertical-lr" ]
    keyword_aliases = [ "h", "vrl", "vlr" ]

    data_types = ["keyword"]

  [[properties.property]]
    ## css syntax = z-index: auto|number|initial|inherit;
    property_name = "z-index"
    property_name_alias = "z"

    keywords = [ "auto", "intial", "inherit" ]
    keyword_aliases = [ "" ]

    data_types = ["keyword", "number"]

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
::before,
::after {
  box-sizing: inherit;
  border-width: 0;
  border-style: solid;
  border-color: #e2e8f0;
}

html {
  box-sizing: border-box;
  font-size: 62.5%;
}

body {
  margin: 0;
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

hr {
  background-color: #191919;
  height: 2px;
}

footer {
  width: 100%;
  text-align: center;
  line-height: 2rem;
  padding: 3rem;
}

footer a {
  color: #1565c0;
}

.container {
  margin: 0 auto;
  margin-top: 10rem;
  max-width: 90.0rem;
  width: 100%;
  padding-left: 2.0rem;
  padding-right: 2.0rem;
}

.title {
  margin: 0;
  margin-bottom: 1rem;
  font-size: 5.4rem;
  line-height: 6.2rem;
  text-align: center;
}

.date {
  font-size: 1.4rem;
  line-height: 1rem;
  text-align: center;
  margin: 0;
  opacity: 0.6;
}

.nav {
  padding: 15px;
  font-size: 2rem;
  color: black;
}
"#;
}
