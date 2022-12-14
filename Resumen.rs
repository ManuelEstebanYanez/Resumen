use std::io::stdin;
use rand::Rng;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::prelude::*;

// []

// Determinar las variables

fn variables() {
    let n = 10;
    let mut m = 5;
    let m = 6;  //Al determinarlo como mut es posible cambiarlo
    println!("Escrivir el numero: {} y {}", n, m);
}

// Determinar las funciones
// Se especifica en que tipo de dato se quiere el resultado

fn suma (a: i32 , b: i32 ) -> () {
    let c = a + b;
    println!("La suma de {} + {} es: {}", a, b, c);
}

//Texto

fn get_string () -> String {
    let s = String::from("Hello , World !");
    return s;
}

fn print_string ( my_string : &str ) {
    println!("{}", my_string );
}

//Concatenar Strings y str

fn concatenar_String() {

    let string_a : String = " Hola ".to_string();
    let string_b : String = " Mundo ".to_string();

    let string_c = format!("{} {}", string_a, string_b );
    println!("{}", string_c );
}

fn concatenar_str() {

    let txt_a : &str = " Hola ";
    let txt_b : &str = " Mundo ";

    let concat = format!("{} {}", txt_a , txt_b );
    println!("{}", concat );
}

fn concatenar_string_str() {

    let mut string_a : String = " Hola ".to_string();
    let txt_b : &str = " Mundo ";

    string_a.push_str( txt_b );
    println!("{}", string_a );
}

//Input

fn inputt_numero() {
    println!(" Ingrese un numero :");
    let mut numero: String = String::new();
    stdin().read_line(&mut numero ).unwrap();
    let numero_int: u8 = numero.trim().parse().unwrap();

    println!("Su numero ingresado es: {}", numero_int);
}

fn inputt_texto() {
    println!("Ingrese un texto:");
    let mut word: String = String::new();
    stdin().read_line(&mut word).unwrap();
    let word_string: String = word.trim().parse().unwrap();
    
    println!("la palabra es: {}", word_string);
}

//Control de errores y excepciones

fn control_numero() {  //Va a pedir que se ingrese un numero hast que este sea realmente un numero 
//Tambien podria usarse el i32 o i 64 para que tome el dato como un numero negativo
    println!(" Ingrese un numero ");
    loop {
        let mut entrada : String = String::new();
        stdin().read_line(& mut entrada ).unwrap();
        let numero : u32 = match entrada.trim().parse(){
            Ok( numero ) => numero ,
            Err(_) => continue ,
        };
        println!("El numero es: {}", numero );
        break;
    }
    // Para el texto debe de ser igual solo que con Str o string
}

//CONDICIONALES
// if 
fn if_funcion() {
    let num = 7;
    if num =< 5 {
        println("El numero es menor que 5");
    } else if num < 10 {
        printl!("El numero esta entre 5 y 10");
    } else {
        println!("El numero es mayor o igual a 10");
    }
}

// for
fn for_funcion() {
    let matriz = [1, 2, 3, 4, 5, 6, 7, 8];

    for elemento in matriz.iter() { //.iter va a leer uno por uno los elementos dentro de la matriz
        println!("{}", numero); //escribira uno por uno los elementos de matriz
    }

    for x in 1..10 { // tomara del 1 al 9, si se quiere igualar se escribe como 1..=10
        println!("uwu");
    }

    for x in 1..10.rev() { // .rev hara que todo lo que este dictado se escriba desde atras hacia delante
        println!("uwu");
    }
}
// while
fn while_funcion() {
    let mut contador = 0;

    while contador != 5 {
        
        println!("Hace algo")
        contador += 1;
    }

    let matriz = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut i = 0;
    while i < 7 {
        println!("El valor del puesto {}, en la matriz es {}", i, matriz[i]);
        i += 1;
    }
}

// loop
fn loop_funcion() {
    let mut cont = 0;

    loop {
        println("cumple algo");
        cont += 1;
        if cont == 3 {
            break;  // es nesesario para salir del loop sino este sera infinito
        }
    }
}

// Arreglo
fn recorrer_array() {  // Recorrer un array
    let mut array :[ i32 ;5] = [1 ,2 ,3 ,4 ,5];
    println!(" array is {:?} ",array ); // Escribe todos los valores del array
    println!(" array size is :{}",array.len()); // Te da el largo del array

    for index in 0..5 {
        println!(" index is: {} & value is : {}",index , array [index]);
    }

    array[1] = 0; // va a cambiar el elemento que este en el indice 1 por el valor 0
    println!(" {:?} ",array);

    // Array de String
    let mut my_keywords :[&str; 2] = [" Hola Mundo "; 2];
    for my_keyword in my_keywords.iter() {
        println!("{}", my_keyword );
    }
}

// Matriz
// Estructura de la matriz es =      Matriz :[[ TIPO(str,string o u32) ; COLUMNA ]; FILAS ]
fn recorrer_matriz(){
    let matriz :[[ i32 ;2];3] = [[8;2];3];
    for (i, row) in my_int_matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            println!("[row ={}][ col ={}] fila ={} ", i, j, col);
        }
    }
    // accede al valor de forma directa
    println!("{}", my_int_matrix [0][0]);
}

// ESTRUCTURA

struct Persona {
    nombre :String ,
    apellido :String ,
    edad :u32,
}
fn crea_persona ( nombre : String , apellido : String , edad : u32) -> Persona {
    let nueva = Persona {
        nombre : nombre ,
        apellido : apellido ,
        edad : edad ,
    };

    return nueva ;
}

fn main () {
    let mut pepito = Persona {
        nombre : String::from(" Pepito "), 
        apellido : String::from(" Perez "), 
        edad : 30 ,
    };
    println!(" nombre : {} {} edad {}",pepito.nombre , pepito.apellido , pepito.edad );
    
    // mut de datos sobre a persona
    let mut pepito = crea_persona (" Carlos".to_string() , " Mesa".to_string() , 30);
    muestra_datos ( pepito );
}

// ENUM

enum GenderCategory {
    Male , Female
}

fn main() {
    let male = GenderCategory :: Male ;
    let female = GenderCategory :: Female ;

    println!(" {:?} ",male );
    println!(" {:?} ",female );
}


// Ejemplo struct y enum

#[ derive ( Debug )]
enum CategoriaGenero {
    Male , Female
}

#[ derive ( Debug )]
struct Person {
    nombre : String ,
    genero : CategoriaGenero
}

fn main () {
    let p1 = Person {
        nombre : String::from (" Pepito "),
        genero : CategoriaGenero::Male
    };
    let p2 = Person {
        nombre : String::from (" Maria "),
        genero : CategoriaGenero::Female
    };
    println!(" {:?} ", p1);
    println!(" {:?} ", p2);
}



// EJEMPLOS

//Contar palabras
fn contar_palabras() {

    let palabra = "Hola Mundo";

    let cantidad = palabra.chars().count();
    println!("{}", cantidad);
}

//Buscar palabra en una sentencia
fn buscar_palabras() {

    let sentencia: String = "Lorem ipsum dolor sit amet".to_string();
    if sentencia.contains("ipsum"){
        println!("Encontrada");
    } else {
        println!("No encontrada");
    }
}

fn buscar_palabra_2() {

    println!("Ingrese una oración: ");
    let mut oracion: String = String::new();
    stdin().read_line(&mut oracion).unwrap();

    println!("Ingrese una palabra: ");
    let mut palabra: String = String::new();
    stdin().read_line(&mut palabra).unwrap();

    if oracion.contains(&palabra.trim()){

        println!("La palabra existe en la oracion");
    } else {

        println!("La palabra no existe en la oracion");
    }

}

//Separar por .split()

fn separar() {
    let mut split = "some string 123 ffd".split("123");

    for s in split {
        println!("{}", s);
    }

}


//Numero aleatorio
//use rand::Rng;
fn aleatorio() {
    let mut rng = rand::thread_rng();
    for _x in 0..5{
    let num: u8 = rng.gen_range(0..10);
    println!("Un numero random entre 0 y 9: {}", num);
    }
}

// MANIPULACION DE TEXTO Y ARCHIVOS


fn abrir_archivo() {
    // crear
    let path = Path::new("archivo.txt");
    let display = path.display();

    // abrir y verificar
    let mut file = match File::open(&path) {
        Err(why) => panic!("no se puede abrir {}: {}", display, why),
        Ok(file) => file,
    };

    // leer y retornar el archivo
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("no se puede leer {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}

fn añadir_texto_archivo() {
    let mut file = OpenOptions::new()
        .write(true)  
        .append(true)
        .create(true)
        .open("archivo.txt")
        .unwrap();
    if let Err(e) = writeln!(file, "\nA new line!") {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn escribir_un_archivo() -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create("foo.txt")?);

    buffer.write_all(b"some bytes\n")?;
    buffer.write_all("Hello World".as_bytes())?;
    buffer.flush()?;
    Ok(())
}

fn lectura_archivo() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./archivo.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}\n", ip);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn lectura<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// EJEMPLO DE CREAR Y MODIFICAR ARCHIVO

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

fn read_file(mut f: &File){
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    println!("{}", &text);
}

fn create_blank_file(p: &Path){
    let file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


fn add_new_content(mut f: &File){

    f.write_all(b"Nuevo texto\n");
}

fn open_file_to_append(p: &Path) -> File{

    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    //add_new_content(&file);

    return file

}

fn open_file_to_read(p: &Path){
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        read_file(&file)
    } else {
        create_blank_file(p);
    }
}


fn main(){
    let path = Path::new("./data/ejemplo.txt");
    open_file_to_read(path);
    let file = open_file_to_append(path);
    add_new_content(&file);
    open_file_to_read(path);
}

//TERMINAAA EJEMPLO

//  EJEMPLO MEDICAMENTOS    

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Default)]
struct Persona{
    nombre: String,
    edad: u8,
    rut: String,
    mascota: [Mascota; 5]
}

#[derive(Default)]
struct Mascota{
    nombre: String,
    tipo: String,
    color: String,
}

fn read_file(mut f: &File){
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    println!("{}", &text);
}

fn create_blank_file(p: &Path){
    let file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


fn add_new_content(mut f: &File){

    f.write_all(b"Nuevo texto\n");
}

fn open_file_to_append(p: &Path) -> File{

    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    //add_new_content(&file);

    return file

}

fn open_file_to_read(p: &Path){
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        read_file(&file)
    } else {
        create_blank_file(p);
    }
}


fn create_persona(p: &Path){
/*
    m_blank = Mascota{nombre:"".to_string(),
                    tipo:"".to_string(),
                    color:"".to_string()};
*/
    let mut m1 = Mascota{nombre:"Don gato".to_string(),
                         tipo:"Gato".to_string(),
                         color:"cafe".to_string()};


    let mut p1 = Persona{nombre:"Alejandro".to_string(),
                             edad: 19,
                             rut: "11111111-1".to_string(),
                             mascota: Default::default()};
    
    p1.mascota[0] = m1;

    let temp = format!("{}:{}:{}:{}\n", p1.nombre, 
                                              p1.edad, 
                                              p1.rut, 
                                              p1.mascota[0].nombre,
                                              //p1.mascota.tipo,
                                              //p1.mascota.color
                                              );
    
    let mut file = open_file_to_append(p);

    file.write_all(temp.as_bytes());
    
}

fn main(){
    let path = Path::new("./data/ejemplo.txt");
    open_file_to_read(path);
    //let file = open_file_to_append(path);
    //add_new_content(&file);
    create_persona(path);
    open_file_to_read(path);
}

// TERMINA EJEMPLO  


// FUNCION MAIN REAL
fn main() {
    // variables
    variables();
    //fn suma
    let a = 9;
    let b = 15;
    //Texto
    let nuevo_string = get_string();
    print_string(& nuevo_string);
    //Concatenando Strings y str
    concatenar_String();
    concatenar_str();
    concatenar_string_str();
    //Ingresar ( input )
    inputt_numero();
    inputt_texto();
    //control de eroor
    control_numero();
    //Condicionales
    if_funcion();
    for_funcion();
    while_funcion();
    loop_funcion();
    // Matriz y array
    recorrer_array();
    recorrer_matriz();

}
