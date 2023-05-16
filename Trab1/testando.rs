use std::io::{self, BufRead};

struct DID {
    page_id: i32,
    seq: i32,
    tam: i32,
}

struct Documento {
    did: DID,
    dados: Vec<u8>,
}

fn new_documento(dados: &[u8]) -> Result<Documento, String> {
    if dados.len() < 1 || dados.len() > 5 {
        return Err(String::from("Os dados do documento devem ter entre 1 e 5 caracteres"));
    }

    let documento = Documento {
        did: DID {
            page_id: -1,
            seq: -1,
            tam: dados.len() as i32,
        },
        dados: dados.to_vec(),
    };

    Ok(documento)
}

struct Pagina {
    page_id: i32,
    arm_livre: i32,
    num_docs: i32,
    docs: Vec<Box<Documento>>,
    next: Option<Box<Pagina>>,
}

fn new_pagina(page_id: i32) -> Box<Pagina> {
    Box::new(Pagina {
        page_id,
        arm_livre: 5,
        num_docs: 0,
        docs: Vec::new(),
        next: None,
    })
}

impl Pagina {
    fn find_documento_by_tam(&self, tam: i32) -> Option<&Documento> {
        for doc in &self.docs {
            if doc.did.tam == tam {
                return Some(doc);
            }
        }
        None
    }
}

impl Pagina {
    fn add_documento(&mut self, doc: Box<Documento>) -> bool {
        self.num_docs += 1;
        doc.did.seq = self.num_docs - 1;
        doc.did.page_id = self.page_id;
        self.docs.push(doc);
        self.arm_livre -= doc.did.tam;
        true
    }
}

struct SGBD {
    primeira_pagina: Option<Box<Pagina>>,
    ultima_pagina: Option<Box<Pagina>>,
    num_paginas: i32,
}

fn new_sgbd() -> Box<SGBD> {
    let mut sgbd = Box::new(SGBD {
        primeira_pagina: None,
        ultima_pagina: None,
        num_paginas: 0,
    });

    let pagina = new_pagina(0); // pagina 0
    sgbd.primeira_pagina = Some(pagina.clone());
    sgbd.ultima_pagina = Some(pagina);
    sgbd.num_paginas = 1;

    sgbd
}

impl SGBD {
    fn insert(&mut self, data: Vec<u8>) -> bool {
        let doc_result = NewDocumento(data);
        if let Err(err) = doc_result {
            println!("{}", err);
            return false;
        }
        let doc = doc_result.unwrap();
        let mut pagina_atual = self.primeira_pagina.as_mut();

        loop {
            if let Some(pagina) = pagina_atual {
                if pagina.arm_livre >= doc.did.tam {
                    pagina.add_documento(Box::new(doc));
                    return true;
                }
                pagina_atual = pagina.next.as_mut();
            } else {
                break;
            }
        }

        if self.num_paginas < 20 {
            let pagina = new_pagina(self.num_paginas);
            pagina.add_documento(Box::new(doc));
            self.ultima_pagina.as_mut().unwrap().next = Some(pagina.clone());
            self.ultima_pagina = Some(pagina);
            self.num_paginas += 1;
            return true;
        }

        false
    }
}

impl SGBD {
    fn scan(&self) -> Vec<Box<Documento>> {
        let mut docs = Vec::new();
        let mut pagina_atual = self.primeira_pagina.as_ref();

        while let Some(pagina) = pagina_atual {
            for doc in &pagina.docs {
                docs.push(doc.clone());
            }
            pagina_atual = pagina.next.as_ref();
        }

        docs
    }
}

impl SGBD {
    fn seek(&self, content: &[u8]) -> Result<DID, String> {
        let mut pagina_atual = self.primeira_pagina.as_ref();

        while let Some(pagina) = pagina_atual {
            for doc in &pagina.docs {
                if doc.dados == content {
                    return Ok(doc.did.clone());
                }
            }
            pagina_atual = pagina.next.as_ref();
        }

        Err("Documento não encontrado".to_string())
    }
}

impl SGBD {
    fn delete(&mut self, content: &[u8]) -> Result<(), String> {
        let mut pagina_atual = self.primeira_pagina.as_mut();
        let mut pagina_anterior: Option<&mut Pagina> = None;

        while let Some(pagina) = pagina_atual {
            let mut i = 0;
            let mut found = false;

            while i < pagina.docs.len() {
                if pagina.docs[i].dados == content {
                    pagina.docs.remove(i);
                    pagina.num_docs -= 1;

                    if pagina.num_docs == 0 {
                        if let Some(pagina_ant) = pagina_anterior {
                            pagina_ant.next = pagina.next.take();
                        } else {
                            self.primeira_pagina = pagina.next.take();
                        }

                        if pagina as *const Pagina == self.ultima_pagina.as_ref().unwrap() as *const Pagina {
                            self.ultima_pagina = pagina_anterior;
                        }

                        self.num_paginas -= 1;
                        update_page_id(pagina.next.as_mut());
                    } else {
                        pagina.arm_livre += content.len();
                        let new_doc_result = self.teste(content.len());

                        if let Ok(new_docs) = new_doc_result {
                            for (km, docmm) in new_docs.iter().enumerate() {
                                if km < new_docs.len() - 1 {
                                    self.teste2(&docmm.dados);
                                    self.insert(docmm.dados.clone());
                                } else {
                                    self.delete(docmm.dados)?;
                                    self.insert(docmm.dados.clone());
                                }
                            }
                        }
                        update_seq(pagina);
                    }

                    found = true;
                    break;
                }

                i += 1;
            }

            if found {
                return Ok(());
            }

            pagina_anterior = Some(pagina);
            pagina_atual = pagina.next.as_mut();
        }

        Err("Documento não encontrado".to_string())
    }
}

impl SGBD {
    fn teste2(&mut self, content: &[u8]) -> Result<(), String> {
        let mut pagina_atual = self.primeira_pagina.as_mut();
        let mut pagina_anterior: Option<&mut Pagina> = None;

        while let Some(pagina) = pagina_atual {
            let mut i = 0;
            let mut found = false;

            while i < pagina.docs.len() {
                if pagina.docs[i].dados == content {
                    pagina.docs.remove(i);
                    pagina.num_docs -= 1;

                    if pagina.num_docs == 0 {
                        if let Some(pagina_ant) = pagina_anterior {
                            pagina_ant.next = pagina.next.take();
                        } else {
                            self.primeira_pagina = pagina.next.take();
                        }

                        if pagina as *const Pagina == self.ultima_pagina.as_ref().unwrap() as *const Pagina {
                            self.ultima_pagina = pagina_anterior;
                        }

                        self.num_paginas -= 1;
                        update_page_id(pagina.next.as_mut());
                    } else {
                        pagina.arm_livre += pagina.docs[i].dados.len();
                        update_seq(pagina);
                    }

                    found = true;
                    break;
                }

                i += 1;
            }

            if found {
                return Ok(());
            }

            pagina_anterior = Some(pagina);
            pagina_atual = pagina.next.as_mut();
        }

        Err("Documento não encontrado".to_string())
    }
}

impl SGBD {
    fn find_document_by_size(&self, size: usize) -> Result<&Documento, String> {
        let mut pagina_atual = self.primeira_pagina.as_ref();

        while let Some(pagina) = pagina_atual {
            for doc in &pagina.docs {
                if doc.did.tam == size {
                    return Ok(doc);
                }
            }

            if pagina.next.is_none() {
                return Err("Não há documentos com o tamanho especificado no SGBD".to_string());
            }

            pagina_atual = pagina.next.as_ref();
        }

        Err("Não há documentos com o tamanho especificado no SGBD".to_string())
    }
}

impl SGBD {
    fn teste(&self, size: usize) -> Result<Vec<&Documento>, String> {
        let mut pagina_atual = self.primeira_pagina.as_ref();
        let mut docs: Vec<&Documento> = Vec::new();
        let mut size2 = size;

        while let Some(pagina) = pagina_atual {
            for doc in &pagina.docs {
                if doc.did.tam == size {
                    docs.clear();
                    docs.push(doc);
                    return Ok(docs);
                } else if size2 >= doc.did.tam {
                    size2 -= doc.did.tam;
                    docs.push(doc);
                    if size2 == 0 {
                        return Ok(docs);
                    }
                }
            }

            if pagina.next.is_none() {
                return Err("Não há documentos com o tamanho especificado no SGBD".to_string());
            }

            pagina_atual = pagina.next.as_ref();
        }

        Err("Não há documentos com o tamanho especificado no SGBD".to_string())
    }
}

fn update_seq(pagina: &mut Pagina) {
    let mut i = 0;
    for doc in &mut pagina.docs {
        doc.did.seq = i;
        i += 1;
    }
}

fn update_page_id(pagina: Option<&mut Pagina>) {
    // Inicializa o contador de page_id's
    let mut page_id = match pagina {
        Some(p) => p.page_id,
        None => return,
    };
    page_id -= 1;

    // Percorre todas as páginas a partir da página atual
    let mut current_pagina = pagina;
    while let Some(p) = current_pagina {
        // Atualiza o page_id da página
        p.page_id = page_id;
        p.atualizar_page_id(page_id);

        // Incrementa o contador de page_id's
        page_id += 1;

        // Avança para a próxima página
        current_pagina = p.next.as_mut();
    }
}

impl Pagina {
    fn atualizar_page_id(&mut self, page_id: i32) {
        for doc in &mut self.docs {
            doc.did.page_id = page_id;
        }
        if let Some(ref mut next) = self.next {
            next.atualizar_page_id(page_id + 1);
        }
    }
}

fn main() {
    let mut sgbd = new_sgbd();

    println!("SGBD em execução...");
    loop {
        println!("Escolha uma opção:");
        println!("1 - Inserir");
        println!("2 - Deletar");
        println!("3 - Procurar");
        println!("4 - FULL SCAN");
        println!("5 - Sair");

        let mut choice = String::new();
        io::stdin().lock().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Digite o conteúdo do documento a ser inserido:");
                let mut content = String::new();
                io::stdin().lock().read_line(&mut content).unwrap();
                content = content.trim().to_string();
                if sgbd.insert(content.as_bytes()) {
                    println!("Documento inserido com sucesso");
                } else {
                    println!("\nNão foi possível inserir o documento");
                }
            }
            "2" => {
                println!("Digite o conteúdo do documento a ser deletado:");
                let mut content = String::new();
                io::stdin().lock().read_line(&mut content).unwrap();
                content = content.trim().to_string();
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
                let mut content = String::new();
                io::stdin().lock().read_line(&mut content).unwrap();
                content = content.trim().to_string();
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
                for doc in docs {
                    println!("DID: {} - Caracteres Armazenados: '{}'", doc.did, String::from_utf8_lossy(doc.dados));
                }
            }
            "5" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida"),
        }
    }
}

fn new_sgbd() -> SGBD {
    let primeira_pagina = new_pagina(0); // pagina 0
    let sgbd = SGBD {
        primeira_pagina: Some(primeira_pagina.clone()),
        ultima_pagina: Some(primeira_pagina),
        num_paginas: 1,
    };
    sgbd
}