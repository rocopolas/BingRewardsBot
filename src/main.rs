use rdev::{simulate, EventType, Key};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

fn main() {

    let palabras = vec![
        "de", "la", "que", "el", "en", "y", "a", "los", "se", "del", "las", "un", "una", "su", "para", "es",
        "al", "lo", "como", "más", "o", "pero", "sus", "le", "ha", "me", "si", "sin", "sobre", "este", "ya",
        "entre", "cuando", "todo", "esta", "ser", "son", "dos", "también", "fue", "había", "era", "muy", "años",
        "hasta", "desde", "por", "qué", "uno", "les", "nos", "así", "puede", "ahora", "porque", "durante", "sólo",
        "han", "hay", "vez", "donde", "quien", "están", "estado", "bien", "poco", "esa", "eso", "hace", "otra",
        "gobierno", "tan", "día", "caso", "parte", "mismo", "según", "después", "primero", "grande", "mientras",
        "menos", "mundo", "año", "antes", "forma", "hecho", "estos", "mayor", "nombre", "país", "persona",
        "momento", "punto", "tres", "vida", "lugar", "manera", "tiempo", "trabajo", "nueva", "cada", "tipo",
        "todos", "varios", "algunos", "señor", "nueva", "información", "hombres", "mujeres", "niños", "casa",
        "grupo", "desarrollo", "social", "nacional", "sistema", "interés", "actividad", "comunidad", "empresa",
        "número", "nivel", "campo", "área", "proceso", "orden", "problema", "final", "política", "cultura",
        "seguridad", "organización", "recursos", "historia", "arte", "naturaleza", "ciencia", "tecnología",
        "educación", "medicina", "derecho", "economía", "ambiente", "relación", "comunicación", "transporte",
        "energía", "agua", "tierra", "aire", "animal", "planta", "mineral", "cuerpo", "mente", "espíritu",
        "palabra", "imagen", "sonido", "luz", "color", "forma", "movimiento", "espacio", "materia", "vida",
        "muerte", "amor", "odio", "miedo", "alegría", "tristeza", "ira", "paz", "guerra", "libertad",
        "justicia", "verdad", "mentira", "belleza", "fealdad", "bueno", "malo", "correcto", "incorrecto",
        "importante", "necesario", "posible", "imposible", "creer", "saber", "pensar", "sentir", "hacer",
        "tener", "decir", "ir", "ver", "dar", "venir", "querer", "poder", "deber", "saber", "conocer",
        "entender", "aprender", "enseñar", "trabajar", "jugar", "vivir", "morir", "amar", "odiar", "temer",
        "reír", "llorar", "gritar", "callar", "hablar", "escuchar", "leer", "escribir", "cantar", "bailar",
        "comer", "beber", "dormir", "despertar", "soñar", "recordar", "olvidar", "encontrar", "perder",
        "ganar", "perder", "subir", "bajar", "entrar", "salir", "empezar", "terminar", "continuar", "parar",
        "cambiar", "permanecer", "crecer", "disminuir", "aumentar", "reducir", "mejorar", "empeorar",
        "construir", "destruir", "crear", "conservar", "proteger", "atacar", "defender", "ayudar", "dañar",
        "curar", "enfermar", "nacer", "vivir", "morir", "amar", "odiar", "temer", "reír", "llorar", "gritar",
        "callar", "hablar", "escuchar", "leer", "escribir", "cantar", "bailar", "comer", "beber", "dormir",
        "despertar", "soñar", "recordar", "olvidar"
    ];

    sleep(Duration::from_secs(4));
    let mut rng = thread_rng();

    println!("¿Qué quieres hacer despues de que termine?");
    println!("1. No apagar pc");
    println!("2. Apagar pc");

    let mut opcion = String::new();

    io::stdin()
        .read_line(&mut opcion)
        .expect("Error al leer la entrada");

    let opcion: u32 = match opcion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada inválida. Por favor, ingresa 1 o 2.");
            return; // Salir del programa si la entrada no es válida
        }
    };

    for _ in 0..100 {
        sleep(Duration::from_secs(1));

        // Simular Ctrl+T
        simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
        simulate(&EventType::KeyPress(Key::KeyT)).unwrap();
        simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
        simulate(&EventType::KeyRelease(Key::KeyT)).unwrap();

        sleep(Duration::from_millis(500));

        let palabra = palabras.choose(&mut rng).unwrap();
        for letra in palabra.chars() {
            let key = match letra.to_ascii_lowercase() {
                'a' => Key::KeyA,
                'b' => Key::KeyB,
                'c' => Key::KeyC,
                'd' => Key::KeyD,
                'e' => Key::KeyE,
                'f' => Key::KeyF,
                'g' => Key::KeyG,
                'h' => Key::KeyH,
                'i' => Key::KeyI,
                'j' => Key::KeyJ,
                'k' => Key::KeyK,
                'l' => Key::KeyL,
                'm' => Key::KeyM,
                'n' => Key::KeyN,
                'o' => Key::KeyO,
                'p' => Key::KeyP,
                'q' => Key::KeyQ,
                'r' => Key::KeyR,
                's' => Key::KeyS,
                't' => Key::KeyT,
                'u' => Key::KeyU,
                'v' => Key::KeyV,
                'w' => Key::KeyW,
                'x' => Key::KeyX,
                'y' => Key::KeyY,
                'z' => Key::KeyZ,
                _ => continue,
            };

            simulate(&EventType::KeyPress(key)).unwrap();
            simulate(&EventType::KeyRelease(key)).unwrap();

            sleep(Duration::from_millis(100));
        }

        simulate(&EventType::KeyPress(Key::Return)).unwrap(); 
        simulate(&EventType::KeyRelease(Key::Return)).unwrap();

        let numero_aleatorio = rng.gen_range(5000..=7000);
        sleep(Duration::from_millis(numero_aleatorio));w
    }

    match opcion {
        1 => {
            
        }
        2 => {
            Command::new("shutdown")
                .arg("/s") // Apagar
                .arg("/t") // Tiempo de espera (opcional)
                .arg("0")  // 0 segundos
                .status()?;
            Ok(())
        }
        _ => println!("Opción no válida."), 
    }


}