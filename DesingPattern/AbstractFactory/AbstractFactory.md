# Abstract Factory.
## Intent.
Fornecer uma interface para a criação de famílias de objetos relacionados ou dependentes sem especificar suas classes concretas é um princípio de design conhecido como Abstract Factory (Fábrica Abstrata). A Abstract Factory é um padrão de design que permite criar objetos relacionados sem depender de classes específicas.

A ideia por trás do padrão Abstract Factory é ter uma interface abstrata (ou classe abstrata) chamada de fábrica, que define métodos para criar objetos de uma família relacionada. Cada implementação concreta da fábrica é responsável por criar objetos específicos dessa família. Dessa forma, a criação de objetos ocorre através da chamada dos métodos da fábrica, em vez de instanciar classes diretamente.

A utilização do padrão Abstract Factory permite isolar o código que cria objetos da implementação concreta desses objetos. Isso facilita a troca de famílias de objetos relacionados, permitindo que você substitua a fábrica concreta por outra sem modificar o código que usa esses objetos.

Em resumo, o padrão Abstract Factory oferece uma maneira flexível de criar famílias de objetos relacionados, fornecendo uma interface comum para criar esses objetos sem depender de classes concretas. Isso promove o princípio da programação orientada a interfaces e permite que você projete sistemas que sejam mais flexíveis e extensíveis.

### O padrão Abstract Factory é aplicável nos seguintes cenários:

   - Quando um sistema deve ser independente de como seus produtos são criados, compostos e representados: O padrão Abstract Factory permite abstrair o processo de criação de objetos, garantindo que o sistema não dependa de classes ou implementações específicas. Isso promove o desacoplamento e aprimora a manutenibilidade e a flexibilidade.

   -  Quando um sistema precisa ser configurado com uma das várias famílias de produtos: O padrão Abstract Factory possibilita a configuração de diferentes famílias de produtos, permitindo que o sistema utilize uma família específica de objetos relacionados. Isso facilita a troca entre famílias de produtos sem modificar o código do sistema.

   - Quando uma família de objetos relacionados é projetada para ser usada em conjunto e você precisa impor essa restrição: O padrão Abstract Factory garante que os objetos de uma família específica sejam criados de forma coerente e possam ser usados juntos de maneira consistente. Isso ajuda a evitar erros de incompatibilidade entre os objetos relacionados.

   - Quando você deseja fornecer uma biblioteca de classes de produtos e deseja expor apenas suas interfaces, não suas implementações: O padrão Abstract Factory permite que você defina interfaces comuns para as famílias de produtos, ocultando as implementações específicas. Isso ajuda a promover a encapsulação e a fornecer uma abstração limpa para os usuários da biblioteca.

Em resumo, o padrão Abstract Factory é útil quando você precisa criar objetos de famílias relacionadas de maneira independente, configurar o sistema com uma família de produtos específica, garantir o uso coeso de objetos relacionados ou fornecer uma biblioteca de produtos com interfaces claras.
