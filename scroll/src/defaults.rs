lazy_static! {
    pub static ref CONF: &'static str = r#"
    default_template = ""
"#;
    pub static ref TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<link rel="stylesheet" type="text/css" href="/style/index.css">
</head>
<body>
{page}
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
}
