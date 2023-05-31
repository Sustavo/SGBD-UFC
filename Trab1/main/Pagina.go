package main

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