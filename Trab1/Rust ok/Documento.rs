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

    fn update_seq(&mut self) {
        let mut i = 0;
        for doc in &mut self.docs {
            doc.did.seq = 1 ;
            i += 1;
        }
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

    // pub fn update_page_id(&mut self, pagina: &mut Pagina) {

    // }
}

fn formatar_erro(err: String) -> String {
    format!("Error: {}", err)
}

fn main() {
    let mut sgbd = SGBD::new();

    // Cria uma nova Página
    let mut pagina = match sgbd.primeira_pagina {
        Some(ref pagina) => (**pagina).clone(),
        None => {
            println!("Erro: Nenhuma página encontrada.");
            return;
        }
    };

    // Cria alguns Documentos de exemplo
    let documento1 = match Documento::new("AAAA") {
        Ok(doc) => Box::new(doc),
        Err(err) => {
            println!("{}", formatar_erro(err));
            return;
        }
    };

    let documento2 = match Documento::new("XXXX") {
        Ok(doc) => Box::new(doc),
        Err(err) => {
            println!("{}", formatar_erro(err));
            return;
        }
    };

    // Adiciona os Documentos à Página
    pagina.add_documento(documento1.clone());
    pagina.add_documento(documento2.clone());

    // Antes da atualização
    println!("Página antes da atualização: {:?}", pagina);
    println!();

    // Atualiza a sequência dos Documentos na Página
    pagina.update_seq();

    // Após a atualização
    println!("Página após a atualização: {:?}", pagina);
    println!();
}