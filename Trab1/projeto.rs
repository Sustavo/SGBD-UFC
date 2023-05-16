use std::io::{self, BufRead};

struct Documento {
    // Define your Documento struct fields here
}

struct Pagina {
    // Define your Pagina struct fields here
}

struct SGBD {
    // Define your SGBD struct fields here
}

impl SGBD {
    fn new() -> Self {
        // Implement the new method for SGBD struct
    }

    fn insert(&mut self, data: &[u8]) -> bool {
        // Implement the insert method for SGBD struct
    }

    fn scan(&self) -> Vec<Documento> {
        // Implement the scan method for SGBD struct
    }

    fn seek(&self, content: &[u8]) -> Option<Documento> {
        // Implement the seek method for SGBD struct
    }

    fn delete(&mut self, content: &[u8]) -> Result<(), String> {
        // Implement the delete method for SGBD struct
    }
}

impl Pagina {
    fn atualizar_page_id(&mut self, page_id: i32) {
        // Implement the atualizar_page_id method for Pagina struct
    }
}

fn main() {
    let mut sgbd = SGBD::new();

    let reader = io::BufReader::new(io::stdin());
    println!("SGBD em execução...");
    for line in reader.lines() {
        println!("Escolha uma opção:");
        println!("1 - Inserir");
        println!("2 - Deletar");
        println!("3 - Procurar");
        println!("4 - FULL SCAN");
        println!("5 - Sair");

        let choice = line.unwrap().trim().to_string();

        match choice.as_str() {
            "1" => {
                println!("Digite o conteúdo do documento a ser inserido:");
                let content = {
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).unwrap();
                    content.trim().to_string()
                };
                if sgbd.insert(content.as_bytes()) {
                    println!("Documento inserido com sucesso");
                } else {
                    println!("\nNão foi possível inserir o documento");
                }
            }
            "2" => {
                println!("Digite o conteúdo do documento a ser deletado:");
                let content = {
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).unwrap();
                    content.trim().to_string()
                };
                if content.len() > 5 {
                    println!("Conteúdo do documento deve ter no máximo 5 caracteres");
                    continue;
                }
                if let Err(err) = sgbd.delete(content.as_bytes()) {
                    println!("Não foi possível deletar o documento: {}", err);
                } else {
                    println!("Documento deletado com sucesso");
                }
            }
            "3" => {
                println!("Digite o conteúdo do documento a ser procurado:");
                let content = {
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).unwrap();
                    content.trim().to_string()
                };
                if content.len() > 5 {
                    println!("Conteúdo do documento deve ter no máximo 5 caracteres");
                    continue;
                }
                if let Some(did) = sgbd.seek(content.as_bytes()) {
                    println!("Documento encontrado: {:?}", did);
                } else {
                    println!("Documento não encontrado");
                }
            }
            "4" => {
                let docs = sgbd.scan();
                println!("\nDocumentos armazenados no SGBD:");
                for doc in &docs {
                    println!("DID: {} - Caracteres Armazenados: '{}'", doc.did, String::from_utf8_lossy(&doc.dados));
                }
            }
            "5" => {
                println!("Saindo...");
                return;
            }
            _ => {
                println!("Opção inválida");
            }
        }
    }
}