package main

import (
	"bytes"
	"errors"
	"fmt"
	"bufio"
	"os"
)

type DID struct {
	page_id int
	seq     int
	tam     int
}

type Documento struct {
	did   DID  
	dados []byte
}

func DocNew(dados []byte) (*Documento, error) {
	if len(dados) < 1 || len(dados) > 5 {
		return nil, errors.New("Os dados do documento devem ter entre 1 e 5 caracteres")
	}
	documento := &Documento{
		did: DID{
			page_id: -1,
			seq:     -1,
			tam:     len(dados),
		},
		dados: make([]byte, len(dados)),
	}

	copy(documento.dados, dados)

	return documento, nil
}

type Pagina struct {
	page_id   int
	arm_livre int
	num_doc  int
	Docs      []*Documento
	next      *Pagina 
}

func PagNew(page_id int) *Pagina {
	return &Pagina{
		page_id:   page_id,
		arm_livre: 5,
		num_doc:  0,
		Docs:      []*Documento{},
		next:      nil,
	}
}

func (p *Pagina) AddDoc(doc *Documento) bool { 
  if doc == nil{
    return false
  }
	p.num_doc += 1
	seqValue := p.num_doc - 1
	
	doc.did.seq = seqValue
	doc.did.page_id = p.page_id

	p.Docs = append(p.Docs, doc)
	p.arm_livre = p.arm_livre - doc.did.tam
	return true
}

func(pagina *Pagina) RecalculateSeq() { 
	i := 0
	for _, doc := range pagina.Docs {
		doc.did.seq = i
		i += 1
	}
}

func (p *Pagina) recursiveUpdatePageId(pageID int) {
	for _, doc := range p.Docs {
		doc.did.page_id = pageID
	}
	if p.next != nil {
		p.next.recursiveUpdatePageId(pageID + 1)
	}
}

type DataBase struct {
	FirstPage *Pagina
	LastPage   *Pagina
	QntPages     int
}


func NewDataBase() *DataBase {
	DataBase := &DataBase{}
	pagina := PagNew(0) 
	DataBase.FirstPage = pagina
	DataBase.LastPage = pagina
	DataBase.QntPages = 1
	return DataBase
}

func (s *DataBase) updatePageId(pagina *Pagina) {
	if pagina == nil {
		return
	}
	pageID := pagina.page_id
	pageID--

	for p := pagina; p != nil; p = p.next {
		p.page_id = pageID
		p.recursiveUpdatePageId(pageID)
		pageID++
	}
}


func (s *DataBase) Insert(data []byte) bool {
	doc, err := DocNew(data)

	if err != nil {
		fmt.Println(err)
		return false
	}

	paginaAtual := s.FirstPage
	for {
		if paginaAtual.arm_livre >= doc.did.tam {
			paginaAtual.AddDoc(doc)
			return true
		}
		if paginaAtual.next == nil {
			break
		}
		paginaAtual = paginaAtual.next
	}

	if s.QntPages < 20 {
		pagina := PagNew(s.QntPages)
		pagina.AddDoc(doc)
		s.LastPage.next = pagina
		s.LastPage = pagina
		s.QntPages++
		return true
	}

  fmt.Println("O banco de dados está sem espaço, não é possível inserir esse documento!")
	return false
}

func (s *DataBase) Delete(content []byte) error {
	paginaAtual := s.FirstPage
	var paginaAnterior *Pagina

	for {

		for i, doc := range paginaAtual.Docs {

			if bytes.Equal(doc.dados, content) {
				paginaAtual.Docs = append(paginaAtual.Docs[:i], paginaAtual.Docs[i+1:]...)
				paginaAtual.num_doc--
        
				if paginaAtual.num_doc == 0 {
					if paginaAnterior == nil {
						s.FirstPage = paginaAtual.next
					} else {
						paginaAnterior.next = paginaAtual.next
					}
					if paginaAtual == s.LastPage {
						s.LastPage = paginaAnterior
					}
					s.QntPages--
					s.updatePageId(paginaAtual.next)
				} else {

					paginaAtual.arm_livre = paginaAtual.arm_livre + len(doc.dados)
					paginaAtual.RecalculateSeq()
				}
				return nil
			}
		}
		if paginaAtual.next == nil {
			break
		}
		paginaAnterior = paginaAtual
		paginaAtual = paginaAtual.next
	}
	return errors.New("Documento não encontrado")
}

func (s *DataBase) Scan() []*Documento {

	var docs []*Documento
	paginaAtual := s.FirstPage
	for {
		for _, doc := range paginaAtual.Docs {
			docs = append(docs, doc)
		}
		if paginaAtual.next == nil {
			break
		}
		paginaAtual = paginaAtual.next
	}
	return docs
}

func (s *DataBase) Seek(content []byte) (DID, error) {
  
	paginaAtual := s.FirstPage
	for {
		for _, doc := range paginaAtual.Docs {
			if bytes.Equal(doc.dados, content) {
				return doc.did, nil
			}
		}
		if paginaAtual.next == nil {
			break
		}
		paginaAtual = paginaAtual.next
	}
	return DID{}, errors.New("Não foi possível achar o documento")
}

func main() {
	sgbd := NewDataBase()

	reader := bufio.NewReader(os.Stdin)
	for {
		fmt.Println("Menu:")
		fmt.Println("1 - Insert")
		fmt.Println("2 - Delete")
		fmt.Println("3 - Seek")
		fmt.Println("4 - Scan")
		fmt.Println("5 - Exit")
    fmt.Println("")

		choice, _ := reader.ReadString('\n')
		choice = choice[:len(choice)-1]

		switch choice {
		case "1":
			fmt.Println("Insira o valor:")
			content, _ := reader.ReadString('\n')
			content = content[:len(content)-1]
			if sgbd.Insert([]byte(content)) {
				fmt.Println("sucesso na inserção")
			} else {
				fmt.Println("\nNão foi possível inserir")
			}
		case "2":
			fmt.Println("Digite o conteúdo a ser deletado:")
			content, _ := reader.ReadString('\n')
			content = content[:len(content)-1]
			if len(content) > 5 {
				fmt.Println("Não existem conteúdos acima de 5 letras")
				continue
			}
			if err := sgbd.Delete([]byte(content)); err == nil {
				fmt.Println("Conteúdo deletado com sucesso")
			} else {
				fmt.Println("Não foi possível deletar o conteúdo:", err)
			}
		case "3":
			fmt.Println("Digite o conteúdo a ser procurado:")
			content, _ := reader.ReadString('\n')
			content = content[:len(content)-1]
			if len(content) > 5 {
				fmt.Println("Não existem conteúdos acima de 5 letras")
				continue
			}
			if did, err := sgbd.Seek([]byte(content)); err == nil {
				fmt.Println("Conteúdo encontrado:", did)
			} else {
				fmt.Println("Conteúdo não encontrado")
			}
		case "4":
			docs := sgbd.Scan()
      
			fmt.Println("\nConteúdos armazenados no SGBD:")
      fmt.Printf("Page Seq Tam Valor\n")
			for _, doc := range docs {
				fmt.Printf("%d    %d   %d   %s\n\n", doc.did.page_id ,doc.did.seq, doc.did.tam,  doc.dados)
			}
		case "5":
			fmt.Println("Saindo...")
			return
		default:
			fmt.Println("Opção inválida")
		}
	}
}