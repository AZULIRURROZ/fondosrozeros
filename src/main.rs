use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
extern crate image_base64;

fn main() -> Result<(), io::Error> {
    let args_elm: Vec<String> = env::args().skip(1).collect();
    let args_num = args_elm.len();

    let html_name = file_route(args_num); 

    let html_path = Path::new(&html_name);
    let mut imag_name = "Default.png";
    if args_num>1 {
        imag_name = &args_elm[1]
    }
    let imag_code = image_base64::to_base64(&imag_name); 

    let mut html_file = File::open(&html_path)?; 
    let mut html_string = String::new(); 
    html_file.read_to_string(&mut html_string)?; 

    if !html_string.contains("<div id=fondo-global style=") { 
        panic!("Chocamooo, no hay lugar para poner el fondo");
    } 
    if html_string.contains("<div id=fondo-global style=background:") { 
        panic!("Chocamooo, ya hay una imagen");
    } 

    let word_from = "<div id=fondo-global style=";
    let word_to =  format!("<div id=fondo-global style=background:url('{imag_code}') !important;");
    let html_new = html_string.replace(&*word_from, &*word_to);
    let mut html_refile = File::create(&html_path)?;
    html_refile.write(html_new.as_bytes())?;
    println!("Lito lito tu roz ahora tiene foto de fondo! ");
    Ok(())
}

fn file_route<'life>(args_num: usize) -> String {
    let args_elm: Vec<String> = env::args().skip(1).collect();
    if args_num<1 { 
        println!("Ningún archivo badre, indica uno: ");
        let mut input_string = String::new();
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        if input_string.starts_with("'") {
            input_string = input_string[1..].to_string();
            input_string = (&input_string[0..input_string.find("'").unwrap_or(input_string.len())]).to_string()
        }
        let html_name = &input_string;
        return html_name.to_string();
    } 
    else{
        let html_name = &args_elm[0];
        return html_name.to_string();
    }
    
}