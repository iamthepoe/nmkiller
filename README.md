# nmkiller
NodeModulesKiller é uma CLI feita para exterminar suas pastas de node_modules.

<p><code>nmkiller [path]</code> para deletar todas as pastas de um diretório específico.</p>
<p><code>nmkiller --purge-all</code> para deletar todas as pastas node_modules do sistema.</p>


## Build

É necessário ter o <a href="https://www.rust-lang.org/tools/install" target="_blank">Rust</a> instalado.

*Clone o repositório e acesse sua pasta*

```bash
$ git clone https://github.com/iamthepoe/nmkiller && cd nmkiller
```

*Siga os passos*

```bash

#Vá até a pasta src
cd src/

# Execute o comando
cargo build --release
```

Após a compilação, o executável estará disponível no diret <code>target/release</code> com o nome de "nmkiller".
