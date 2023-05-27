#[derive(Debug, Clone)]
pub struct DID {
    page_id: i32,
    seq: i32,
    tam: i32,
}

#[derive(Debug, Clone)]
pub struct Documento {
    did: DID,
    dados: Vec<u8>,
}

impl Documento {
    pub fn new(dados: &str) -> Result<Documento, String> {
        let databytes = dados.as_bytes();
        if databytes.len() < 1 || databytes.len() > 5 {
            return Err(String::from("O Documento criado deve possuir de 1 a 5 bytes"));
        }
    
        let documento = Documento {
            did: DID {
                page_id: -1,
                seq: -1,
                tam: databytes.len() as i32,
            },
            dados: databytes.to_vec(),
        };
    
        Ok(documento)
    }
}

#[derive(Debug, Clone)]
pub struct Pagina {
    page_id: i32,
    arm_livre: i32,
    num_docs: i32,
    docs: Vec<Box<Documento>>,
    next : Option<Box<Pagina>>,
}

impl Pagina {
    pub fn new(page_id: i32) -> Pagina {
        Pagina {
            page_id,
            arm_livre: 5,
            num_docs: 0,
            docs: Vec::new(),
            next: None,
        }
    }

    pub fn add_documento(&mut self, mut doc: Box<Documento>) -> bool {
        if doc.dados.is_empty() {
            return false;
        }
        let num_docs = self.docs.len() as i32;
        doc.did.seq = num_docs;
        doc.did.page_id = self.page_id;

        self.num_docs += 1;
        self.arm_livre -= doc.did.tam;
        self.docs.push(doc);
        
        true
    }
}

#[derive(Debug, Clone)]
pub struct SGBD {
    primeira_pagina: Option<Box<Pagina>>,
    ultima_pagina: Option<Box<Pagina>>,
    num_paginas: i32,
}

impl SGBD {
    pub fn new() -> SGBD {
        let mut sgbd = SGBD {
            primeira_pagina: None,
            ultima_pagina: None,
            num_paginas: 0,
        };

        let pagina = Pagina::new(0); // Pagina 0
        sgbd.primeira_pagina = Some(Box::new(pagina.clone()));
        sgbd.ultima_pagina = Some(Box::new(pagina.clone()));
        sgbd.num_paginas = 1;

        sgbd
    }
}

fn formatar_erro(err: String) -> String {
    format!("Error: {}", err)
}

fn main() {
    let sgbd = SGBD::new();
    
    // Cria uma nova Página
    let mut pagina = match sgbd.primeira_pagina {
        Some(ref pagina) => (**pagina).clone(),
        None => {
            println!("Erro: Nenhuma página encontrada.");
            return;
        }
    };
    
    println!("Página inicial: {:?}", pagina);
    println!();
    
    // Cria um novo Documento
    let documento = match Documento::new("Gust") {
        Ok(doc) => Box::new(doc),
        Err(err) => {
            println!("{}", formatar_erro(err));
            return;
        }
    };

    // Adiciona o Documento à Página
    let adicionado = pagina.add_documento(documento);
    
    println!("Página após adicionar o documento: {:?}", pagina);
    println!();
    
    if adicionado {
        println!("Documento adicionado com sucesso!");
    } else {
        println!("Falha ao adicionar o documento.");
    }

    println!("SGBG: {:?}", sgbd);
}