pub mod parser;
pub mod player;
pub mod world;

use parser::{parsear, Comando};
use player::Jogador;
use std::collections::HashMap;
use world::{Item, ItemId, Room, RoomId};

pub struct EstadoJogo {
    pub salas: HashMap<RoomId, Room>,
    pub itens: HashMap<ItemId, Item>,
    pub sala_atual: RoomId,
    pub jogador: Jogador,
    pub mensagens: Vec<String>,
    pub rodando: bool,
    pub entrada_atual: String,
}

impl EstadoJogo {
    pub fn novo() -> Self {
        let salas = world::criar_mundo();
        let itens = world::criar_itens();

        let mut estado = Self {
            salas,
            itens,
            sala_atual: RoomId::EditorPane,
            jogador: Jogador::novo(),
            mensagens: Vec::new(),
            rodando: true,
            entrada_atual: String::new(),
        };

        estado.mensagens.push(BANNER.to_string());
        estado.mensagens.push(INTRODUCAO.to_string());
        estado.descrever_sala();

        estado
    }

    pub fn processar_entrada(&mut self) {
        let texto = self.entrada_atual.trim().to_string();
        if texto.is_empty() {
            return;
        }

        self.mensagens.push(format!("> {}", texto));
        let comando = parsear(&texto);
        self.executar(comando);
        self.jogador.movimentos += 1;
        self.entrada_atual.clear();
    }

    fn executar(&mut self, cmd: Comando) {
        match cmd {
            Comando::Olhar => self.descrever_sala(),
            Comando::Ir(dir) => self.mover(dir),
            Comando::Pegar(nome) => self.pegar_item(&nome),
            Comando::Largar(nome) => self.largar_item(&nome),
            Comando::Examinar(alvo) => self.examinar(alvo),
            Comando::Inventario => self.mostrar_inventario(),
            Comando::Dar(item, alvo) => self.dar_item(&item, &alvo),
            Comando::Usar(item, alvo) => self.usar_item(&item, alvo.as_deref()),
            Comando::Abrir(alvo) => self.abrir(&alvo),
            Comando::Ajuda => self.mostrar_ajuda(),
            Comando::Sair => {
                self.msg("Saindo do Zork-Zed. Até a próxima aventura!");
                self.rodando = false;
            }
            Comando::Desconhecido(err) => self.msg(&err),
        }
    }

    fn mover(&mut self, direcao: String) {
        let sala = &self.salas[&self.sala_atual];
        if let Some(destino) = sala.saidas.get(direcao.as_str()).cloned() {
            self.sala_atual = destino;
            self.descrever_sala();
        } else {
            self.msg("Você não pode ir nessa direção.");
        }
    }

    fn pegar_item(&mut self, nome: &str) {
        let sala = self.salas.get_mut(&self.sala_atual).unwrap();
        let nome_lower = nome.to_lowercase();

        let pos = sala.itens.iter().position(|id| {
            let item = &self.itens[id];
            item.nome.to_lowercase().contains(&nome_lower)
        });

        if let Some(idx) = pos {
            let item_id = sala.itens[idx].clone();
            let item = &self.itens[&item_id];
            if item.pegavel {
                let nome_item = item.nome.to_string();
                sala.itens.remove(idx);
                self.jogador.pegar(item_id);
                self.msg(&format!("Você pegou: {}.", nome_item));
            } else {
                self.msg("Você não consegue pegar isso.");
            }
        } else {
            self.msg(&format!("Não há '{}' aqui.", nome));
        }
    }

    fn largar_item(&mut self, nome: &str) {
        let nome_lower = nome.to_lowercase();
        let item_id = self
            .jogador
            .inventario
            .iter()
            .find(|id| self.itens[*id].nome.to_lowercase().contains(&nome_lower))
            .cloned();

        if let Some(id) = item_id {
            let nome_item = self.itens[&id].nome.to_string();
            self.jogador.largar(&id);
            self.salas.get_mut(&self.sala_atual).unwrap().itens.push(id);
            self.msg(&format!("Você largou: {}.", nome_item));
        } else {
            self.msg(&format!("Você não tem '{}'.", nome));
        }
    }

    fn examinar(&mut self, alvo: Option<String>) {
        if let Some(nome) = alvo {
            let nome_lower = nome.to_lowercase();

            // Verificar no inventário primeiro
            let item_inv = self
                .jogador
                .inventario
                .iter()
                .find(|id| self.itens[*id].nome.to_lowercase().contains(&nome_lower))
                .cloned();

            if let Some(id) = item_inv {
                let exame = self.itens[&id].exame.to_string();
                // Evento especial: examinar Diff Prism
                if id == ItemId::DiffPrism {
                    self.jogador.diff_examinado = true;
                }
                self.msg(&exame);
                return;
            }

            // Verificar na sala
            let sala = &self.salas[&self.sala_atual];
            let item_sala = sala
                .itens
                .iter()
                .find(|id| self.itens[*id].nome.to_lowercase().contains(&nome_lower))
                .cloned();

            if let Some(id) = item_sala {
                let exame = self.itens[&id].exame.to_string();
                if id == ItemId::DiffPrism {
                    self.jogador.diff_examinado = true;
                }
                self.msg(&exame);
            } else {
                self.msg(&format!("Você examina '{}'. Nada de especial.", nome));
            }
        } else {
            self.descrever_sala();
        }
    }

    fn mostrar_inventario(&mut self) {
        if self.jogador.inventario.is_empty() {
            self.msg("Seu inventário está vazio.");
        } else {
            self.msg("Você carrega:");
            let nomes: Vec<String> = self
                .jogador
                .inventario
                .iter()
                .map(|id| format!("  - {}", self.itens[id].nome))
                .collect();
            for nome in nomes {
                self.msg(&nome);
            }
        }
    }

    fn dar_item(&mut self, nome_item: &str, alvo: &str) {
        let nome_lower = nome_item.to_lowercase();
        let alvo_lower = alvo.to_lowercase();

        let item_id = self
            .jogador
            .inventario
            .iter()
            .find(|id| self.itens[*id].nome.to_lowercase().contains(&nome_lower))
            .cloned();

        if item_id.is_none() {
            self.msg(&format!("Você não tem '{}'.", nome_item));
            return;
        }

        let id = item_id.unwrap();

        // Dar café para o AI
        if id == ItemId::CoffeeMug
            && self.sala_atual == RoomId::AiPanel
            && (alvo_lower.contains("ai")
                || alvo_lower.contains("oracle")
                || alvo_lower.contains("assistant"))
        {
            self.jogador.largar(&id);
            self.jogador.ai_ajudou = true;
            self.msg(
                "O AI recebe o café e seus olhos acendem com brilho de atenção renovada.\n\
                'Obrigado! Agora posso pensar claramente. Ouça: o Unclosed Brace está \
                trancado no Settings Vault. Para abri-lo, você precisa de 6 itens:\n\
                Cursor, LSP Wand, Chave de Ponto-e-Vírgula, Autosave Amulet, \
                Git Token e Diff Prism. Colete todos e use-os na porta do Vault!'",
            );
        } else {
            self.msg(&format!(
                "'{}' não parece interessado em receber isso.",
                alvo
            ));
        }
    }

    fn usar_item(&mut self, nome_item: &str, alvo: Option<&str>) {
        let nome_lower = nome_item.to_lowercase();

        let item_id = self
            .jogador
            .inventario
            .iter()
            .find(|id| self.itens[*id].nome.to_lowercase().contains(&nome_lower))
            .cloned();

        if item_id.is_none() {
            self.msg(&format!("Você não tem '{}'.", nome_item));
            return;
        }

        let id = item_id.unwrap();

        match (&id, self.sala_atual.clone()) {
            // Usar LSP Wand no Unclosed Brace no Vault aberto
            (ItemId::LspWand, RoomId::SettingsVault) if self.jogador.vault_aberto => {
                self.vitoria();
            }
            // Usar Diff Prism no Git Repo
            (ItemId::DiffPrism, RoomId::GitRepo) => {
                self.jogador.diff_examinado = true;
                self.msg(
                    "Você ergue o Diff Prism e olha através dele para as paredes do Git Repo.\n\
                    O commit '3f7a2b1: refactor: update brace handling' pulsa em vermelho.\n\
                    Você vê exatamente onde o Unclosed Brace foi introduzido: \
                    alguém deletou um '}' na linha 42 de src/editor/core.rs durante uma refatoração.\n\
                    Agora você sabe onde e como consertar. O LSP Wand pode fazer isso!",
                );
            }
            // Usar Chave no Vault
            (ItemId::SemicolonKey, RoomId::SettingsVault) => {
                if self.jogador.tem_itens_para_vault() {
                    self.jogador.vault_aberto = true;
                    self.msg(
                        "Você insere a Chave de Ponto-e-Vírgula na fechadura TOML.\n\
                        A porta TREME. Os outros itens em seu inventário ressoam com poder.\n\
                        Com um CLIQUE satisfatório, a porta se abre!\n\n\
                        À sua frente: o Unclosed Brace, uma chave '}' vermelha e pulsante, \
                        suspensa no ar como um erro de compilação materializado.\n\
                        Use o LSP Wand para corrigi-lo!",
                    );
                } else {
                    self.msg(
                        "A fechadura reage à chave, mas você não tem todos os itens necessários.\n\
                        Você precisa de: Cursor, LSP Wand, Chave de Ponto-e-Vírgula, \
                        Autosave Amulet, Git Token e Diff Prism.",
                    );
                }
            }
            _ => {
                let alvo_str = alvo.unwrap_or("aqui");
                self.msg(&format!(
                    "Você usa {} em {}. Nada de especial acontece.",
                    self.itens[&id].nome, alvo_str
                ));
            }
        }
    }

    fn abrir(&mut self, alvo: &str) {
        let alvo_lower = alvo.to_lowercase();
        if (alvo_lower.contains("vault")
            || alvo_lower.contains("porta")
            || alvo_lower.contains("door"))
            && self.sala_atual == RoomId::SettingsVault
        {
            if self.jogador.tem_itens_para_vault() {
                self.executar(Comando::Usar(
                    "chave".to_string(),
                    Some("vault".to_string()),
                ));
            } else {
                self.msg(
                    "A porta está selada com runas TOML. Você sente que precisa dos \
                    6 itens especiais para abri-la.",
                );
            }
        } else {
            self.msg(&format!("Você não pode abrir '{}' aqui.", alvo));
        }
    }

    fn mostrar_ajuda(&mut self) {
        self.msg(
            "═══ COMANDOS DISPONÍVEIS ═══\n\
            \n\
            MOVIMENTO:\n\
              ir norte/sul/leste/oeste  (ou: n/s/e/w)\n\
            \n\
            OBJETOS:\n\
              pegar [item]      - Adicionar ao inventário\n\
              largar [item]     - Soltar na sala atual\n\
              examinar [item]   - Inspecionar item ou sala\n\
              usar [item]       - Usar um item\n\
              usar [item] em [alvo]\n\
              dar [item] para [alvo]\n\
            \n\
            INFORMAÇÕES:\n\
              inventario (i)   - Ver seus itens\n\
              olhar (look)     - Descrever a sala atual\n\
              ajuda (help)     - Esta mensagem\n\
              sair (quit)      - Encerrar o jogo\n\
            \n\
            DICA: Explore todas as salas, colete os 6 itens\n\
            especiais e encontre o Settings Vault!",
        );
    }

    fn descrever_sala(&mut self) {
        // Clonar dados da sala antes de chamar self.msg (evita borrow conflict)
        let (nome, desc, itens_sala) = {
            let sala = &self.salas[&self.sala_atual];
            (
                sala.nome.to_string(),
                sala.descricao.to_string(),
                sala.itens.clone(),
            )
        };

        self.msg(&format!("━━━ {} ━━━", nome));
        self.msg(&desc);

        if !itens_sala.is_empty() {
            self.msg("\nItens visíveis:");
            let descricoes: Vec<String> = itens_sala
                .iter()
                .map(|id| {
                    let item = &self.itens[id];
                    format!("  • {} — {}", item.nome, item.descricao)
                })
                .collect();
            for d in descricoes {
                self.msg(&d);
            }
        }
    }

    fn vitoria(&mut self) {
        self.msg(VITORIA);
        self.jogador.venceu = true;
        self.rodando = false;
    }

    fn msg(&mut self, texto: &str) {
        for linha in texto.lines() {
            self.mensagens.push(linha.to_string());
        }
    }
}

const BANNER: &str = r#"
 ███████╗███████╗██████╗      ███████╗███████╗██████╗
 ╚══███╔╝██╔════╝██╔══██╗     ╚════██║██╔════╝██╔══██╗
   ███╔╝ █████╗  ██║  ██║         ██╔╝█████╗  ██║  ██║
  ███╔╝  ██╔══╝  ██║  ██║        ██╔╝ ██╔══╝  ██║  ██║
 ███████╗███████╗██████╔╝        ██║  ███████╗██████╔╝
 ╚══════╝╚══════╝╚═════╝         ╚═╝  ╚══════╝╚═════╝

        Uma Aventura de Texto dentro do Editor Zed
        Versão 0.1.0 — "The Unclosed Brace"
"#;

const INTRODUCAO: &str = "\
Você abre um arquivo no Zed e... algo dá errado.\n\
Um flash de luz. Um crash inesperado. E de repente,\n\
você está DENTRO do editor.\n\
\n\
O editor está infectado pelo UNCLOSED BRACE — um bug antigo que\n\
consome memória, trava processos e prende almas incautas.\n\
\n\
Para escapar, você precisa encontrar e corrigir o bug.\n\
Explore o editor. Colete os itens. Salve o código.\n\
\n\
(Digite 'ajuda' para ver os comandos disponíveis)\n";

const VITORIA: &str = "\n\
★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★\n\
\n\
Você ergue o LSP Wand em direção ao Unclosed Brace.\n\
A varinha vibra, diagnostica, e com um flash de luz azul...\n\
\n\
  3f7a2b1 | src/editor/core.rs | linha 42\n\
  - if condition {\n\
  -     process();\n\
  + if condition {\n\
  +     process();\n\
  + }   ← ADICIONADO!\n\
\n\
O '}' materializa-se. O código compila.\n\
O editor respira novamente.\n\
\n\
Um portal de luz se abre. Você é ejetado de volta\n\
ao mundo real, com os dedos ainda no teclado.\n\
O Zed está rodando perfeitamente. A base de código salva.\n\
\n\
PARABÉNS! Você venceu o ZORK-ZED!\n\
★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★ ★\n";
