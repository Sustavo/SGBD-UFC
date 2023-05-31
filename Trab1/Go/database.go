package Go

import (
	"bytes"
	"errors"
	"fmt"
)

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