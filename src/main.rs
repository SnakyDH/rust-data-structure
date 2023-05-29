use bson::Document;
use csv::ReaderBuilder;
use mongodb::{
    bson,
    error::Result,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use std::fs;
use tokio;

const DATA_DIR: &str = "./data/MOCK_DATA.csv";
const MONGODB_URI: &str =
    "mongodb+srv://snakydev:X83SmDAkPMq6vrwa@archcluster.afqfbzk.mongodb.net/?retryWrites=true&w=majority";

#[derive(Debug)]
struct User {
    nombre: String,
    apellido: String,
    email: String,
    genero: String,
    palabra: String,
    universidad: String,
    trabajo: String,
    habilidad: String,
    idioma: String,
}
impl User {
    fn new(
        nombre: String,
        apellido: String,
        email: String,
        genero: String,
        palabra: String,
        universidad: String,
        trabajo: String,
        habilidad: String,
        idioma: String,
    ) -> User {
        User {
            nombre,
            apellido,
            email,
            genero,
            palabra,
            universidad,
            trabajo,
            habilidad,
            idioma,
        }
    }
}

/* struct Pila {
    pila: Vec<User>,
}
impl Pila {
    fn push(&mut self, user: User) {
        self.pila.insert(0, user);
    }
    fn pop(&mut self) -> Option<User> {
        if self.pila.is_empty() {
            None
        } else {
            Some(self.pila.remove(0))
        }
    }
}
struct Cola {
    cola: Vec<User>,
}
impl Cola {
    fn push(&mut self, user: User) {
        self.cola.push(user);
    }
    fn pop(&mut self) -> Option<User> {
        if self.cola.is_empty() {
            None
        } else {
            Some(self.cola.remove(0))
        }
    }
} */
struct Nodo<User> {
    valor: User,
    siguiente: Option<Box<Nodo<User>>>,
}

impl<User> Nodo<User> {
    fn new(valor: User) -> Nodo<User> {
        Nodo {
            valor,
            siguiente: None,
        }
    }
}
// cola
struct Cola<User> {
    tope: Option<Box<Nodo<User>>>,
}

impl<User> Cola<User> {
    fn new() -> Cola<User> {
        Cola { tope: None }
    }

    fn push(&mut self, valor: User) {
        let mut nuevo_nodo = Box::new(Nodo::new(valor));
        nuevo_nodo.siguiente = self.tope.take();
        self.tope = Some(nuevo_nodo);
    }

    fn pop(&mut self) -> Option<User> {
        match self.tope.take() {
            Some(nodo) => {
                self.tope = nodo.siguiente;
                Some(nodo.valor)
            }
            None => None,
        }
    }
}

impl<User> IntoIterator for Cola<User> {
    type Item = User;
    type IntoIter = ColaIterator<User>;

    fn into_iter(self) -> Self::IntoIter {
        ColaIterator { cola: self }
    }
}

struct ColaIterator<User> {
    cola: Cola<User>,
}

impl<User> Iterator for ColaIterator<User> {
    type Item = User;

    fn next(&mut self) -> Option<Self::Item> {
        self.cola.pop()
    }
}

// pila

struct Pila<User> {
    tope: Option<Box<Nodo<User>>>,
}

impl<User> Pila<User> {
    fn new() -> Pila<User> {
        Pila { tope: None }
    }

    fn push(&mut self, valor: User) {
        let mut nuevo_nodo = Box::new(Nodo::new(valor));
        nuevo_nodo.siguiente = self.tope.take();
        self.tope = Some(nuevo_nodo);
    }

    fn pop(&mut self) -> Option<User> {
        match self.tope.take() {
            Some(nodo) => {
                self.tope = nodo.siguiente;
                Some(nodo.valor)
            }
            None => None,
        }
    }
}

impl<User> IntoIterator for Pila<User> {
    type Item = User;
    type IntoIter = PilaIterator<User>;

    fn into_iter(self) -> Self::IntoIter {
        PilaIterator { pila: self }
    }
}

struct PilaIterator<User> {
    pila: Pila<User>,
}

impl<User> Iterator for PilaIterator<User> {
    type Item = User;

    fn next(&mut self) -> Option<Self::Item> {
        self.pila.pop()
    }
}

fn read_file(url: &str) -> Vec<String> {
    let content = fs::read_to_string(url).unwrap();
    let mut rbr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());
    let mut array_users: Vec<String> = Vec::new();
    for result in rbr.records() {
        let record = result.unwrap();
        let user = record.get(0).unwrap_or_default().trim().to_string();
        array_users.push(user);
    }
    return array_users;
}
fn create_users(array_users_string: Vec<String>) -> Vec<User> {
    let mut user_definitive: Vec<User> = Vec::new();
    for user in &array_users_string {
        let my_data: Vec<&str> = user.split(',').collect();
        let user = User::new(
            my_data[0].to_string(),
            my_data[1].to_string(),
            my_data[2].to_string(),
            my_data[3].to_string(),
            my_data[4].to_string(),
            my_data[5].to_string(),
            my_data[6].to_string(),
            my_data[7].to_string(),
            my_data[8].to_string(),
        );
        user_definitive.push(user);
    }
    return user_definitive;
}
fn create_document(user: User) -> Document {
    let new_doc: Document = bson::doc! {
        "nombre": user.nombre,
        "apellido": user.apellido,
        "email": user.email,
        "genero": user.genero,
        "palabra": user.palabra,
        "universidad": user.universidad,
        "trabajo": user.trabajo,
        "habilidad": user.habilidad,
        "idioma": user.idioma,
    };
    return new_doc;
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("<<<<< Iniciando >>>>>");
    print!("Leyendo archivo... \n");
    let array_users_string = read_file(DATA_DIR);
    println!("OK");
    print!("Creando usuarios... \n");
    let array_users = create_users(array_users_string);
    println!("OK");
    println!("Conectando con MongoDB...");
    let mut client_options = ClientOptions::parse(MONGODB_URI).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    print!("OK\n");
    println!("Insertando usuarios en MongoDB...");
    for user in array_users {
        let user_collection = client.database("Users").collection("AllUsers");
        let insert_result = user_collection
            .insert_one(create_document(user).clone(), None)
            .await?;
        println!("Nuevo Documento con ID: {}", insert_result.inserted_id);
    }
    println!("OK");
    let mut determinante_str = String::new();
    let mut determinante_num: i8 = 0;
    loop {
        println!("\n<<<<< Organización por generó >>>>> \n");
        print!("1. Masculino a la Pila \n");
        print!("2. Femenino a la Pila \n");
        std::io::stdin()
            .read_line(&mut determinante_str)
            .expect("Error al leer la entrada");
        determinante_num = determinante_str.trim().parse().unwrap();
        let array_users_string_new = read_file(DATA_DIR);
        println!("OK");
        print!("Creando usuarios Nuevos... \n");
        print!("Creando Pilas y colas... \n");
        let array_users_new = create_users(array_users_string_new);
        let mut pila = Pila::new();
        let mut cola = Cola::new();
        if determinante_num == 1 {
            print!("Organizando genero masculino a la Pila \n");
            for user in array_users_new {
                if user.genero == "Male" {
                    pila.push(user);
                } else if user.genero == "Female" {
                    cola.push(user);
                }
            }
            println!("Conectando con MongoDB COLA...");
            let mut client_options = ClientOptions::parse(MONGODB_URI).await?;
            let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
            client_options.server_api = Some(server_api);
            let client = Client::with_options(client_options)?;
            print!("OK\n");
            println!("Insertando usuarios en MongoDB...");
            for user in cola {
                let user_collection = client.database("Users").collection("Female-cola");
                let insert_result = user_collection
                    .insert_one(create_document(user).clone(), None)
                    .await?;
                println!("Nuevo Documento con ID: {}", insert_result.inserted_id);
            }
            println!("OK");
            for user in pila {
                let user_collection = client.database("Users").collection("Male-pila");
                let insert_result = user_collection
                    .insert_one(create_document(user).clone(), None)
                    .await?;
                println!("Nuevo Documento con ID: {}", insert_result.inserted_id);
            }
            break;
        } else if determinante_num == 2 {
            print!("Organizando genero femenino a la Pila \n");
            for user in array_users_new {
                if user.genero == "Male" {
                    cola.push(user);
                } else if user.genero == "Female" {
                    pila.push(user);
                }
            }
            for user in cola {
                let user_collection = client.database("Users").collection("Male-cola");
                let insert_result = user_collection
                    .insert_one(create_document(user).clone(), None)
                    .await?;
                println!("Nuevo Documento con ID: {}", insert_result.inserted_id);
            }
            for user in pila {
                let user_collection = client.database("Users").collection("Female-pila");
                let insert_result = user_collection
                    .insert_one(create_document(user).clone(), None)
                    .await?;
                println!("Nuevo Documento con ID: {}", insert_result.inserted_id);
            }
            break;
        } else {
            print!("Opción no valida \n");
        }
    }
    println!("<<<<< Finalizando >>>>>");
    Ok(())
}
