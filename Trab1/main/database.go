package main

import (
	"errors"
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