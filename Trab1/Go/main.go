package Go

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