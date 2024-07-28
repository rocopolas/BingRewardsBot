use rdev::{simulate, EventType, Key};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

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
        "despertar", "soñar", "recordar", "olvidar", "abrir", "cerrar", "conseguir", "lograr", "fallar", "intentar", "descubrir", "ocultar", "mostrar", "esconder", "encontrar", "perder", "obtener", "dar", "recibir", "enviar", "aceptar", "rechazar", "buscar", "encontrar", "perder", "observar", "mirar", "ver", "escuchar", "oír", "tocar", "sentir", "oler", "saborear", "gusto", "olor", "visión", "audición", "tacto", "sentimiento", "pensamiento", "idea", "concepto", "teoría", "hipótesis", "experimento", "prueba", "resultado", "análisis", "síntesis", "comprensión", "entendimiento", "conocimiento", "sabiduría", "inteligencia", "ingenio", "creatividad", "imaginación", "intuición", "percepción", "sensación", "emoción", "afecto", "amor", "cariño", "pasión", "ternura", "alegría", "felicidad", "placer", "contento", "satisfacción", "orgullo", "esperanza", "optimismo", "confianza", "calma", "tranquilidad", "serenidad", "paz", "armonía", "equilibrio", "estabilidad", "seguridad", "certeza", "convicción", "fe", "creencia", "ideal", "valor", "principio", "moral", "ética", "justicia", "derecho", "deber", "obligación", "responsabilidad", "compromiso", "lealtad", "fidelidad", "honor", "dignidad", "respeto", "tolerancia", "comprensión", "solidaridad", "cooperación", "colaboración", "ayuda", "apoyo", "protección", "defensa", "seguridad", "peligro", "riesgo", "desafío", "obstáculo", "dificultad", "problema", "solución", "respuesta", "explicación", "claridad", "precisión", "exactitud", "veracidad", "honestidad", "transparencia", "franqueza", "sinceridad", "lealtad", "fidelidad", "confianza", "seguridad", "certeza", "duda", "incertidumbre", "inseguridad", "temor", "miedo", "pánico", "terror", "angustia", "ansiedad", "preocupación", "inquietud", "desasosiego", "nerviosismo", "tensión", "estrés", "cansancio", "fatiga", "agotamiento", "debilidad", "fuerza", "vigor", "energía", "vitalidad", "salud", "bienestar", "equilibrio", "armonía", "estabilidad", "serenidad", "tranquilidad", "paz", "descanso", "relajación", "placer", "satisfacción", "felicidad", "alegría", "contento", "gozo", "júbilo", "éxtasis", "deleite", "placer", "satisfacción", "orgullo", "dignidad", "honor", "respeto", "consideración", "estima", "admiración", "aprecio", "valor", "mérito", "reconocimiento", "aprobación", "elogio", "alabanza", "agradecimiento", "gratitud", "satisfacción", "cumplimiento", "realización", "logro", "éxito", "triunfo", "victoria", "conquista", "dominio", "control", "poder", "autoridad", "influencia", "prestigio", "reputación", "fama", "notoriedad", "popularidad", "renombre", "celebridad", "distinción", "honor", "gloria", "fama", "reconocimiento", "respeto", "admiración", "consideración", "estima", "valor", "mérito", "importancia", "significado", "relevancia", "pertinencia", "utilidad", "eficacia", "eficiencia", "rentabilidad", "productividad", "beneficio", "ganancia", "ventaja", "oportunidad", "posibilidad", "probabilidad", "potencial", "capacidad", "habilidad", "talento", "genio", "maestría", "competencia", "pericia", "experiencia", "destreza", "habilidad", "capacidad", "competencia", "aptitud", "idoneidad", "calificación", "preparación", "formación", "educación", "instrucción", "aprendizaje", "enseñanza", "conocimiento", "sabiduría", "ciencia", "arte", "disciplina", "campo", "área", "especialidad", "tema", "materia", "asunto", "cuestión", "problema", "tema", "asunto", "caso", "situación", "circunstancia", "condición", "estado", "posición", "situación", "ubicación", "lugar", "sitio", "espacio", "región", "zona", "territorio", "país", "nación", "estado", "provincia", "municipio", "ciudad", "pueblo", "aldea", "comunidad", "barrio", "distrito", "zona", "sector", "área", "territorio", "región", "división", "subdivisión", "departamento", "sección", "unidad", "célula", "grupo", "conjunto", "colectivo", "equipo", "banda", "orquesta", "conjunto", "grupo", "equipo", "banda", "orquesta", "comité", "comisión", "delegación", "representación", "congreso", "asamblea", "parlamento", "consejo", "cámara", "senado", "diputados", "legislatura", "gobierno", "administración", "autoridad", "dirección", "jefatura", "liderazgo", "gestión", "gerencia", "administración", "dirección", "supervisión", "control", "vigilancia", "inspección", "evaluación", "análisis", "diagnóstico", "propuesta", "proyecto", "plan", "programa", "estrategia", "táctica", "operación", "acción", "actividad", "tarea", "trabajo", "oficio", "ocupación", "profesión", "empleo", "cargo", "puesto", "función", "responsabilidad", "obligación", "deber", "derecho", "privilegio", "beneficio", "ventaja", "oportunidad", "posibilidad", "probabilidad", "potencial", "capacidad", "habilidad", "talento", "genio", "maestría", "competencia", "pericia", "destreza", "habilidad", "capacidad"
    ];

    sleep(Duration::from_secs(4));
    let mut rng = thread_rng();

    for _ in 0..35 {
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
        sleep(Duration::from_millis(numero_aleatorio));
    }
}