pub type Tpl = actix_web::web::Data<tera::Tera>

#[macro_export]
macro_rules! tmpl {
    ($path: expr) => ({
        let template_dir = concat!(env!("CARGO_MANIFEST_DIR"), $path);
        tera::Tera::new(template_dir).unwrap()
    })
}

#[macro_export]
macro_rules! tmpl_data {
    () => ({
        tera::Context::new()
    });
    [$($key: expr => $val: expr,)*] => ({
        let mut ctx = tera::Context::new();
        $(ctx.insert($key, $val);)*;
        ctx
    })
}

#[macro_export]
macro_rules! render { 
    ($tmpl: expr, $path: expr) => ({
        let view_file = if $path.ends_with(".html") { $path.to_owned() } else { 
            let paths = module_path!().rsplit("::").collect::<Vec<&str>>();
            format!("{}/{}.html", paths[0], $path)
        };
        let body = $tmpl.render(&view_file, &tera::Context::new()).unwrap();
        actix_web::HttpResponse::Ok().content_type("text/html").body(body)
    });
    ($tmpl: expr, $path: expr, &$data: expr) => ({
        let view_file = if $path.ends_with(".html") { $path.to_owned() } else { 
            let paths = module_path!().rsplit("::").collect::<Vec<&str>>();
            format!("{}/{}.html", paths[0], $path)
        };
        let body = $tmpl.render(&view_file, &$data).unwrap();
        actix_web::HttpResponse::Ok().content_type("text/html").body(body)
    })
}

