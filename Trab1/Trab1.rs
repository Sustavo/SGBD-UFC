use std::io;

const MAX_CHARACTERS_PER_PAGE: u32 = 5;

struct Document {
    page_id: u32,
    seq: u32,
    tam: u32,
    content: String,
}

impl Document {
    fn new(page_id: u32, seq: u32, content: &str) -> Option<Self> {
        if content.len() > 5 {
            return None;
        }
        let tam = content.len() as u32;
        Some(Self {
            page_id,
            seq,
            tam,
            content: content.to_string(),
        })
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}] valor: {}",
            self.page_id, self.seq, self.tam, self.content
        )
    }   
}

fn main() {
    let mut vector: Vec<Document> = Vec::new();
    let mut char_count = 0;
    let mut doc_count = 0;
    let mut page_id = 0;
    let mut available_characters = MAX_CHARACTERS_PER_PAGE;

    loop {
        println!("Digite um número de 1 a 5:");
        println!("1 - Inserir");
        println!("2 - Deletar");
        println!("3 - Procurar");
        println!("4 - Full Scan");
        println!("5 - Sair");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");
        let choice = input.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                println!("Digite o conteúdo:");
                let mut input_one = String::new();
                io::stdin()
                    .read_line(&mut input_one)
                    .expect("Erro ao ler entrada");
                let content = input_one.trim();
                let content_len = content.chars().count() as u32;
                if content_len > available_characters {
                    // criar nova página e atualizar as variáveis
                    page_id += 1;
                    doc_count = 1;
                    available_characters = MAX_CHARACTERS_PER_PAGE;
                }
                match Document::new(page_id, doc_count, content) {
                    Some(doc) => {
                        vector.push(doc);
                        available_characters -= content_len;
                        doc_count += 1;
                    }
                    None => println!("Erro: conteúdo tem mais de 5 caracteres."),
                }
            }
            2 => {
                // implementar a remoção de um documento
                println!("Digite o número da página:");
                let mut input_one = String::new();
                io::stdin()
                .read_line(&mut input_one)
                .expect("Erro ao ler entrada");
                let page_id = input_one.trim().parse::<u32>().unwrap_or(0);
                println!("Digite a sequência do documento:");
                let mut input_two = String::new();
                io::stdin()
                    .read_line(&mut input_two)
                    .expect("Erro ao ler entrada");
                let seq = input_two.trim().parse::<u32>().unwrap_or(0);

                let mut found = false;
                for i in 0..vector.len() {
                    if vector[i].page_id == page_id && vector[i].seq == seq {
                        char_count -= vector[i].tam;
                        available_characters += vector[i].tam;
                        doc_count -= 1;
                        vector.remove(i);
                        found = true;
                        break;
                    }
                }
                if found {
                    println!("Documento removido.");
                } else {
                    println!("Documento não encontrado.");
                }
            }
            3 => {
                // implementar a busca de um documento pelo conteúdo
                println!("Digite o conteúdo do documento a ser procurado:");
                let mut input_one = String::new();
                io::stdin()
                    .read_line(&mut input_one)
                    .expect("Erro ao ler entrada");
                let content = input_one.trim();
    
                let mut found = false;
                for doc in &vector {
                    if doc.content == content {
                        println!("{}", doc);
                        found = true;
                    }
                }
                if !found {
                    println!("Documento não encontrado.");
                }
            }
    
            4 => {
                for doc in &vector {
                    if doc.page_id != page_id {
                        println!();
                        page_id = doc.page_id;
                    }
                    println!("{}", doc);
                }
                println!();
            }
            5 => {
                println!("Encerrando o programa...");
                break;
            }
            _ => {
                println!("Opção inválida!");
            }
        }
    }
}