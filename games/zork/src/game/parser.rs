/// Comandos suportados pelo parser
#[derive(Debug, Clone, PartialEq)]
pub enum Comando {
    Ir(String),
    Pegar(String),
    Largar(String),
    Examinar(Option<String>),
    Inventario,
    Olhar,
    Dar(String, String),
    Usar(String, Option<String>),
    Abrir(String),
    Ajuda,
    Sair,
    Desconhecido(String),
}

pub fn parsear(entrada: &str) -> Comando {
    let entrada = entrada.trim().to_lowercase();
    let palavras: Vec<&str> = entrada.split_whitespace().collect();

    if palavras.is_empty() {
        return Comando::Olhar;
    }

    let verbo = palavras[0];
    let resto = palavras[1..].join(" ");

    match verbo {
        "ir" | "go" | "andar" | "mover" | "n" | "s" | "e" | "w" | "north" | "south" | "east"
        | "west" | "norte" | "sul" | "leste" | "oeste" => {
            if matches!(verbo, "n" | "north" | "norte") {
                Comando::Ir("north".to_string())
            } else if matches!(verbo, "s" | "south" | "sul") {
                Comando::Ir("south".to_string())
            } else if matches!(verbo, "e" | "east" | "leste") {
                Comando::Ir("east".to_string())
            } else if matches!(verbo, "w" | "west" | "oeste") {
                Comando::Ir("west".to_string())
            } else if resto.is_empty() {
                Comando::Desconhecido("Para onde?".to_string())
            } else {
                let direcao = mapear_direcao(&resto);
                Comando::Ir(direcao)
            }
        }
        "pegar" | "take" | "get" | "apanhar" | "coletar" => {
            if resto.is_empty() {
                Comando::Desconhecido("Pegar o quê?".to_string())
            } else {
                Comando::Pegar(resto)
            }
        }
        "largar" | "drop" | "soltar" | "jogar" => {
            if resto.is_empty() {
                Comando::Desconhecido("Largar o quê?".to_string())
            } else {
                Comando::Largar(resto)
            }
        }
        "examinar" | "examine" | "x" | "olhar" | "look" | "ver" | "inspecionar" | "inspect"
        | "observar" => {
            if (verbo == "look" || verbo == "olhar") && palavras.len() == 1 {
                return Comando::Olhar;
            }
            if resto.is_empty() {
                Comando::Examinar(None)
            } else {
                Comando::Examinar(Some(resto))
            }
        }
        "inventario" | "inventory" | "i" | "inv" | "mochila" | "itens" => Comando::Inventario,
        "dar" | "give" | "entregar" => {
            // "dar X para Y" ou "give X to Y"
            let partes: Vec<&str> = if entrada.contains(" para ") {
                entrada.splitn(2, " para ").collect()
            } else if entrada.contains(" to ") {
                entrada.splitn(2, " to ").collect()
            } else {
                vec![&entrada]
            };

            if partes.len() == 2 {
                let item_raw = partes[0]
                    .trim_start_matches("dar ")
                    .trim_start_matches("give ")
                    .trim_start_matches("entregar ")
                    .trim()
                    .to_string();
                let alvo = partes[1].trim().to_string();
                Comando::Dar(item_raw, alvo)
            } else {
                Comando::Desconhecido("Sintaxe: dar [item] para [alvo]".to_string())
            }
        }
        "usar" | "use" | "utilizar" => {
            if resto.is_empty() {
                Comando::Desconhecido("Usar o quê?".to_string())
            } else if resto.contains(" em ") || resto.contains(" on ") || resto.contains(" no ") {
                let sep = if resto.contains(" em ") {
                    " em "
                } else if resto.contains(" no ") {
                    " no "
                } else {
                    " on "
                };
                let partes: Vec<&str> = resto.splitn(2, sep).collect();
                Comando::Usar(
                    partes[0].trim().to_string(),
                    Some(partes[1].trim().to_string()),
                )
            } else {
                Comando::Usar(resto, None)
            }
        }
        "abrir" | "open" => {
            if resto.is_empty() {
                Comando::Desconhecido("Abrir o quê?".to_string())
            } else {
                Comando::Abrir(resto)
            }
        }
        "ajuda" | "help" | "?" | "h" => Comando::Ajuda,
        "sair" | "quit" | "exit" | "q" => Comando::Sair,
        _ => Comando::Desconhecido(format!("Não entendo '{}'.", verbo)),
    }
}

fn mapear_direcao(texto: &str) -> String {
    match texto {
        "n" | "norte" | "north" => "north".to_string(),
        "s" | "sul" | "south" => "south".to_string(),
        "e" | "leste" | "east" => "east".to_string(),
        "w" | "o" | "oeste" | "west" => "west".to_string(),
        outro => outro.to_string(),
    }
}
