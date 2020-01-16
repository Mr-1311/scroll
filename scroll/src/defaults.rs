lazy_static! {
    pub static ref CONF: &'static str = r#"
    default_template = ""
"#;
    pub static ref TEMPLATE: &'static str = r#" <!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
</head>
<body>
{page}
</body>
</html> "#;
}
