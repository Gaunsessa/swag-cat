#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::fs::read_dir;
use std::collections::HashMap;

fn get_random_cat() -> String {
    let image = read_dir("static/swagger catters").unwrap().choose(&mut thread_rng()).unwrap().unwrap();

    format!("/swagger catters/{}", image.file_name().into_string().unwrap())
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::<String, String>::new();
    let cat = get_random_cat();
    context.insert(String::from("cat"), cat.clone());
    // context.insert(String::from("credit"), String::from(cat.split(".").collect::<Vec<&str>>()[0]).replace("/swagger catters/", ""));
    Template::render("index", context)
}


fn main() {
    rocket::ignite().mount("/", StaticFiles::from("static")).mount("/", routes![index]).attach(Template::fairing()).launch();
}
