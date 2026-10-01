# 🌦️ Tempus

> Previsão do tempo moderna, fluida e com curva horária de 24 horas no terminal.  
> *Modern, ultra-fast terminal weather app with 14-day forecasts and 24h hourly curve.*

[![CI](https://github.com/johnbatistalinux/tempus/actions/workflows/ci.yml/badge.svg)](https://github.com/johnbatistalinux/tempus/actions)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust 2021](https://img.shields.io/badge/Rust-2021-orange.svg)
![Platform](https://img.shields.io/badge/Platform-Linux-blue.svg)

---

**Tempus** é um aplicativo de terminal (TUI) para previsão do tempo desenvolvido em **Rust** utilizando **Ratatui** e **Tokio**. Ele combina dados meteorológicos em tempo real com uma interface sofisticada, navegação dia a dia interativa e gráficos térmicos detalhados.

---

## ✨ Recursos

- ⚡ **Assíncrono & Instantâneo:** Todas as requisições à API ocorrem em segundo plano de forma não bloqueante.
- 📅 **Previsão de 14 Dias:** Tabela com mínimas, máximas, condições, volume de chuva e ventos.
- 📊 **Curva Horária 24h Dinâmica:** Gráfico térmico hora a hora (`00h` até `23h`) para qualquer um dos 14 dias selecionados.
- ☀️ **Emojis de Condição do Céu:** Emojis modernos e coloridos (`☀️`, `🌙`, `⛅`, `🌧️`, `⛈️`, `🌫️`, `❄️`) posicionados acima de cada hora, respeitando o ciclo solar local (`is_day`).
- 🧭 **Navegação Fluida entre Dias:** Percorra os 14 dias previstos usando as setas ou teclas Vim (`←/→`, `↑/↓`, `h/j/k/l`, `0/Home`).
- 🔍 **Busca Global de Cidades:** Busca rápida com geocodificação mundial integrada.
- 💾 **Persistência Inteligente:** Lembra e restaura automaticamente a última cidade consultada (`~/.config/tempus/last_city.json`).
- 🎨 **Design Sofisticado:** Paleta de alto contraste otimizada para terminais modernos com bordas arredondadas e barras de progresso simétricas.

---

## 🎮 Atalhos do Teclado

| Teclas | Ação |
| :--- | :--- |
| `→` ou `↓` ou `l` ou `j` | Avançar para o próximo dia na previsão |
| `←` ou `↑` ou `h` ou `k` | Retornar para o dia anterior |
| `0` ou `Home` | Voltar instantaneamente para a previsão de **Hoje** |
| `s` | Buscar nova cidade interativamente |
| `r` | Recarregar dados meteorológicos |
| `q` ou `Esc` | Sair do aplicativo |
| `Ctrl+C` | Encerramento imediato e seguro |

---

## 📦 Instalação

### Pré-requisitos

- **Rust & Cargo** (versão 1.75 ou superior): [rustup.rs](https://rustup.rs)

### Compilação e Instalação Local

1. Clone o repositório:
   ```bash
   git clone https://github.com/johnbatistalinux/tempus.git
   cd tempus
   ```

2. Instale diretamente com o Makefile (compila em modo release e instala em `~/.local/bin/tempus`):
   ```bash
   make install
   ```

> **Nota:** Certifique-se de que `~/.local/bin` está no seu `$PATH`.

3. Ou instale via `cargo`:
   ```bash
   cargo install --path .
   ```

---

## 🚀 Como Usar

```bash
# Inicia com a última cidade consultada (ou Curitiba por padrão):
tempus

# Abre diretamente a previsão de uma cidade:
tempus "São Paulo"
tempus Florianopolis
tempus Paris
tempus Tokyo

# Exibe as opções de ajuda:
tempus --help
```

---

## 🏗️ Estrutura do Código

```
src/
├── api/            # Cliente HTTP assíncrono para Open-Meteo (previsão e geocodificação)
├── models/         # Modelos de dados fortemente tipados (cidades, clima atual, horários, diários)
├── widgets/        # Componentes de renderização TUI
│   ├── current.rs        # Cartela de clima atual e temperatura em blocos
│   ├── details.rs        # Cartela de detalhes (umidade, vento, nascer/pôr do sol, UV)
│   ├── chart.rs          # Gráfico de barras térmicas 24h e gráfico de chuva 14 dias
│   └── forecast_table.rs # Tabela de previsão estendida de 14 dias com seleção ativa
├── app.rs          # Gerenciamento de estado da aplicação (navegação, busca, seleção)
├── config.rs       # Persistência de configuração local (XDG compliant)
├── ui.rs           # Layout mestre, barra de status superior e rodapé
├── wmo.rs          # Mapeamento oficial de códigos WMO para descrições e emojis
└── main.rs         # Loop de eventos crossterm, argumentos CLI e ciclo de vida
```

---

## 🧪 Testes e Qualidade

O projeto mantém **100%** de conformidade com as diretrizes do Rust:

```bash
# Executar a suíte de testes unitários:
cargo test

# Executar o linter com verificação estrita:
cargo clippy -- -D warnings

# Checar formatação do código:
cargo fmt --check
```

---

## 🛰️ Fontes de Dados & Créditos

- Dados meteorológicos fornecidos gratuitamente pela [Open-Meteo](https://open-meteo.com) sob licença [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
- Interface TUI construída com [Ratatui](https://ratatui.rs).

---

## 📄 Licença

Distribuído sob a licença **MIT**. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
