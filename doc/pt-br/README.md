# Sobre o projeto
O projeto Hostel By Pass é um microserviço escrito em Rust e no framework Axum, projetado para fornecer várias funcionalidades úteis para o gerenciamento de reservas de quartos. O microserviço utiliza o MongoDB como seu banco de dados. Ele foi criado para facilitar o gerenciamento de reservas.

## Arquitetura
A arquitetura deste projeto foi projetada com o objetivo de facilitar a interconexão de componentes e simplificar a adição de novos recursos. Isso torna mais fácil e seguro para desenvolvedores e para o negócio realizar manutenções e implementar novas ideias rapidamente. A estrutura do projeto é organizada em camadas, o que significa que os diferentes elementos se comunicam entre si de forma eficiente. Cada componente foi criado com um propósito específico, garantindo que cada parte do projeto cumpra sua função.

### Controller
Os controladores são componentes criados para lidar com validações e gerenciar o fluxo de solicitações de serviço diretamente. Eles podem utilizar métodos importantes como os do repositório. Neste projeto, cada controlador lida com apenas um caso de uso e suas validações são mantidas privadas.

### Entidade
Uma Entidade é como um componente criado para representar um objeto ou recurso dentro de um sistema. Cada entidade tem seus atributos específicos e é responsável por encapsular dados e operações relacionadas a esses objetos, fornecendo uma maneira organizada e eficiente de interagir com o sistema.

### Repositório
Um "repositório" é como uma abstração de banco de dados onde armazenamos e gerenciamos operações essenciais. Cada "repositório" tem um propósito específico e fornece métodos para adicionar, atualizar e recuperar operações. Eles ajudam a manter o código organizado e simplificar a manipulação de operações, mantendo-as seguras e acessíveis.

### Serviços
Os serviços são abstrações de bibliotecas ou interfaces com métodos fáceis de usar, projetados para evitar o acoplamento direto de pacotes de terceiros dentro da aplicação. Eles atuam como uma camada intermediária, facilitando para os desenvolvedores o acesso a recursos externos sem lidar diretamente com a complexidade desses pacotes.

### Rota
Rotas são como caminhos em um sistema, orientando solicitações para o destino correto. Elas desempenham um papel crucial no tratamento de solicitações recebidas e direcionando-as para os controladores apropriados, que são responsáveis por executar ações ou operações específicas. Essencialmente, as rotas atuam como os controladores de tráfego de uma aplicação web, garantindo que as solicitações sejam processadas corretamente e respondidas de acordo com a lógica e funcionalidade definidas.

### Auxiliar
Os auxiliares servem como assistentes confiáveis dentro de um sistema, oferecendo soluções práticas para tarefas comuns em diferentes áreas. Esses métodos geralmente são estáticos, o que significa que não requerem uma instância de um objeto para serem usados, tornando-os facilmente acessíveis de qualquer lugar dentro da aplicação. Sua versatilidade reside em sua capacidade de realizar diversas funções, como formatação de dados, manipulação de cálculos ou validação de entradas. Ao encapsular essas funcionalidades em módulos reutilizáveis, os auxiliares promovem a reutilização de código, a manutenibilidade e a eficiência geral no desenvolvimento de software.

# Requisitos do Ambiente
Para usar a aplicação que utiliza `Docker` e `Rust`, é necessário ter um sistema operacional compatível, como Linux, e ter o `Docker` instalado e configurado. Os usuários devem ter um entendimento básico do Docker e seus conceitos, bem como um ambiente de desenvolvimento `Rust` configurado com o compilador `Rustc` e o gerenciador de pacotes `Cargo`. Após clonar o repositório da aplicação, o código Rust deve ser compilado usando `Cargo`. Em seguida, os usuários devem criar uma imagem Docker a partir do Dockerfile fornecido no repositório e executar um contêiner Docker usando o comando `docker run`. Após a execução do contêiner, os usuários podem testar e validar a funcionalidade da aplicação. É importante monitorar e manter a aplicação, realizando manutenção regular do sistema operacional, da imagem Docker e da própria aplicação para garantir operação contínua e confiável.

## Por que MongoDB?
O uso do MongoDB ao lado do Rust traz vários benefícios, especialmente em um contexto de microservices. O `MongoDB` é um banco de dados `NoSQL` altamente escalável e flexível, enquanto o `Rust` é uma linguagem de programação conhecida por sua segurança e eficiência. A combinação dessas tecnologias permite uma integração perfeita entre o código Rust e o banco de dados, facilitando a manipulação e o armazenamento de dados. Além disso, o formato BSON do MongoDB, que é semelhante ao JSON, oferece uma maior compatibilidade com dados estruturados no formato JSON, comuns em arquiteturas de microservices. Isso facilita a troca de dados entre os serviços de forma mais fácil e eficiente. Além disso, a flexibilidade do MongoDB em relação a mudanças nas regras de negócio é outra vantagem significativa, permitindo ajustes e adaptações rápidas conforme necessário sem comprometer a integridade dos dados ou a estabilidade do sistema. Em resumo, a combinação de MongoDB e Rust fornece uma base sólida e flexível para o desenvolvimento de microservices robustos e escaláveis.

# Json
O uso do JSON oferece vários benefícios, principalmente porque é o formato mais comumente usado para enviar solicitações. Sua simplicidade e legibilidade o tornam ideal para transmitir dados entre sistemas, seja entre um cliente e um servidor ou entre microservices em uma arquitetura distribuída. A adoção generalizada do JSON garante compatibilidade e interoperabilidade entre diferentes plataformas e linguagens de programação, facilitando a comunicação e integração perfeitas entre vários componentes de um sistema.
- Exemplo JSON:
```json
{
    "name": "Nome do Hóspede",
    "email": "email@example.com",
    "room_number": 9999,
    "check_in_date": "00/00/0000",
    "check_out_date": "00/00/0000",
    "num_guests": 0
}
```
# Tratamento de Exceção

O tratamento de erros é gerenciado dentro dos controladores, então em caso de qualquer ocorrência de erro, a resposta é um JSON descrevendo o erro acompanhado de um status de "Requisição Inválida". Esse processo garante uma comunicação clara entre o servidor e o cliente, tornando a identificação e resolução de problemas mais eficientes.

- JSON:

```json

 {
  "error": {
    "code": 500,
    "message": "Mensagem de erro"
  }
}
```
# Desempenho

Usar `Rust` como a linguagem para criar microservices traz benefícios significativos. `Rust` é conhecido por sua segurança e velocidade, oferecendo robustas características de segurança de memória que previnem erros de programação comuns como desreferenciamento de `null pointer` e  `buffer overflow`. Isso garante a confiabilidade e estabilidade dos microservices mesmo sob cargas pesadas. Além disso, o desempenho do Rust é excepcional, tornando-o adequado para aplicações de alta performance. Outra vantagem é o `Cargo`, o `gerenciador de pacotes` do Rust, que é multiplataforma e simplifica o gerenciamento de dependências, permitindo o desenvolvimento e implantação eficientes de microservices em diferentes ambientes.

# Licença

O projeto Hostel By Pass utiliza a Licença MIT, uma licença de software aberta e permissiva que fornece aos desenvolvedores grande liberdade para usar, modificar e distribuir o código-fonte.