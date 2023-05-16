use std::convert::TryFrom; 
use std::convert::TryInto;
use std::fmt;
use std::io;

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

        let tam = u32::try_from(content.len()).ok()?;
        println!("{}", tam);
        Some(Self {
            page_id,
            seq,
            tam,
            content: content.to_string(),
        })
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}] valor: {}",
            self.page_id, self.seq, self.tam, self.content
        )
    }
}

fn transform_len_array(array_transform: &mut [i32], content: &str) {
    array_transform.push(content.len().try_into().unwrap());
}

fn main() {
    let mut vector: Vec<Document> = Vec::new();
    let mut content_array: Vec<i32> = Vec::new();

    loop {
        println!("Digite um número de 1 a 5:");
        println!("1 - Inserir");
        println!("2 - Deletar");
        println!("3 - Procurar");
        println!("4 - Full Scan");
        println!("5 - Sair");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao fazer a escolha");
        let choose_number = input.trim().parse::<u32>().unwrap_or(0);

        match choose_number {
            1 => {
                println!("Insira o Valor:");
                let mut content_value = String::new();
                io::stdin().read_line(&mut content_value).expect("Erro ao ler o valor");
                let content_formated = content_value.trim();
                if content_formated.len() == 0 {
                    println!("O valor não pode ser vazio");
                    continue;
                }

                if let Some(doc) = Document::new(1, 1, content_formated) {
                    vector.push(doc);
                    transform_len_array(content_array, content_formated);
                } else {
                    println!("Falha ao criar o documento");
                }

            }
            2 => {

            }
            3 => {

            }
            4 => {
                for doc in &vector {
                    println!("{}", doc);
                }
            }
            5 => {
                break;
            }
            _ => {
                println!("Número inválido, tente novamente")
            }
        }
    }


}