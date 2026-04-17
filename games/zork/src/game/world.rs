use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoomId {
    EditorPane,
    CommandPalette,
    AiPanel,
    FileTree,
    ExtensionsGallery,
    Terminal,
    GitRepo,
    TheBuffer,
    SettingsVault,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemId {
    Cursor,
    LspWand,
    CoffeeMug,
    SemicolonKey,
    AutosaveAmulet,
    GitToken,
    DiffPrism,
    RustyKeyboard,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub nome: &'static str,
    pub descricao: &'static str,
    pub exame: &'static str,
    pub pegavel: bool,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub nome: &'static str,
    pub descricao: &'static str,
    pub saidas: HashMap<&'static str, RoomId>,
    pub itens: Vec<ItemId>,
}

fn sala(
    nome: &'static str,
    descricao: &'static str,
    saidas: HashMap<&'static str, RoomId>,
    itens: Vec<ItemId>,
) -> Room {
    Room {
        nome,
        descricao,
        saidas,
        itens,
    }
}

fn saidas(lista: &[(&'static str, RoomId)]) -> HashMap<&'static str, RoomId> {
    lista.iter().cloned().collect()
}

pub fn criar_mundo() -> HashMap<RoomId, Room> {
    let mut mundo = HashMap::new();

    // EDITOR PANE — sala inicial
    mundo.insert(
        RoomId::EditorPane,
        sala(
            "O Editor Pane",
            "Você se materializa num vasto espaço branco infinito. \
             Linhas de código pulsam ao redor como veias de luz. \
             Um cursor gigante pisca com ansiedade no horizonte. \
             A sensação é de estar dentro de um documento em branco — \
             silencioso, mas cheio de potencial. Algo está errado aqui. \
             Há uma presença maligna: o Unclosed Brace infectou a base de código \
             e precisa ser encontrado e corrigido antes que o editor trave para sempre.\n\
             Saídas: norte (Command Palette), sul (Terminal), leste (AI Panel), oeste (File Tree)",
            saidas(&[
                ("north", RoomId::CommandPalette),
                ("n", RoomId::CommandPalette),
                ("south", RoomId::Terminal),
                ("s", RoomId::Terminal),
                ("east", RoomId::AiPanel),
                ("e", RoomId::AiPanel),
                ("west", RoomId::FileTree),
                ("w", RoomId::FileTree),
            ]),
            vec![ItemId::Cursor],
        ),
    );

    // COMMAND PALETTE
    mundo.insert(
        RoomId::CommandPalette,
        sala(
            "A Command Palette",
            "Uma câmara circular onde comandos flutuam como partículas de luz. \
             Você ouve sussurros: 'Open File...', 'Format Document...', 'Toggle Terminal...'. \
             As paredes são feitas de atalhos de teclado cristalizados. \
             Uma varinha brilha no centro — o LSP Wand, ferramenta de diagnóstico do editor.\n\
             Saídas: sul (Editor Pane)",
            saidas(&[("south", RoomId::EditorPane), ("s", RoomId::EditorPane)]),
            vec![ItemId::LspWand],
        ),
    );

    // AI PANEL
    mundo.insert(
        RoomId::AiPanel,
        sala(
            "O AI Assistant Panel",
            "Uma câmara de oráculo banhada em luz azul etérea. \
             Uma entidade feita de tokens e embeddings flutua no centro, \
             observando você com olhos de atenção multi-cabeça. \
             'Olá, sou o Zed AI. Posso ajudar com código, mas estou com baixo nível de energia.' \
             Uma caneca de café fumegante repousa sobre um pedestal de silício.\n\
             Saídas: oeste (Editor Pane)\n\
             Dica: Tente DAR o café para o AI. Ele pode recompensá-lo.",
            saidas(&[("west", RoomId::EditorPane), ("w", RoomId::EditorPane)]),
            vec![ItemId::CoffeeMug],
        ),
    );

    // FILE TREE
    mundo.insert(
        RoomId::FileTree,
        sala(
            "A File Tree",
            "Um labirinto de diretórios se estende até o infinito. \
             Pastas aninhadas criam uma floresta digital: src/, tests/, crates/, target/. \
             O ruído de cargo build ecoa ao longe como trovão distante. \
             Entre os galhos de um src/main.rs esquecido, algo brilha: \
             uma Chave de Ponto-e-Vírgula! Mas está presa numa variável não utilizada.\n\
             Saídas: leste (Editor Pane), sul (Extensions Gallery)",
            saidas(&[
                ("east", RoomId::EditorPane),
                ("e", RoomId::EditorPane),
                ("south", RoomId::ExtensionsGallery),
                ("s", RoomId::ExtensionsGallery),
            ]),
            vec![ItemId::SemicolonKey],
        ),
    );

    // EXTENSIONS GALLERY
    mundo.insert(
        RoomId::ExtensionsGallery,
        sala(
            "A Extensions Gallery",
            "Um bazar mágico iluminado por ícones coloridos. \
             Extensões de todos os tipos pairam em prateleiras: temas escuros, \
             language servers, formatadores. Um vendedor holográfico acena: \
             'Bem-vindo! Temos o melhor da comunidade open-source!' \
             No centro, um amuleto brilhante: o Autosave Amulet — \
             garante que nenhum progresso seja perdido, mesmo num crash.\n\
             Saídas: norte (File Tree)",
            saidas(&[("north", RoomId::FileTree), ("n", RoomId::FileTree)]),
            vec![ItemId::AutosaveAmulet],
        ),
    );

    // TERMINAL
    mundo.insert(
        RoomId::Terminal,
        sala(
            "O Terminal Integrado",
            "Um subsolo escuro iluminado apenas pelo brilho verde do prompt. \
             O cheiro de cargo test --failed permeia o ar. \
             Daemons correm pelas paredes em loops infinitos. \
             Um token git brilha no chão: alguém o perdeu durante um force push.\n\
             Saídas: norte (Editor Pane), leste (Git Repository), sul (The Buffer)",
            saidas(&[
                ("north", RoomId::EditorPane),
                ("n", RoomId::EditorPane),
                ("east", RoomId::GitRepo),
                ("e", RoomId::GitRepo),
                ("south", RoomId::TheBuffer),
                ("s", RoomId::TheBuffer),
            ]),
            vec![ItemId::GitToken],
        ),
    );

    // GIT REPO
    mundo.insert(
        RoomId::GitRepo,
        sala(
            "O Git Repository",
            "Uma câmara de viagem no tempo. As paredes mostram commits passados \
             como fotografias em movimento: 'fix: resolve merge conflict', \
             'feat: add language server', 'chore: update deps'. \
             Um prisma cristalino no centro divide a realidade em dois: \
             o antes e o depois de cada mudança. O Diff Prism!\n\
             Saídas: oeste (Terminal)\n\
             Dica: Com o Diff Prism, você pode EXAMINAR o histórico e localizar o bug.",
            saidas(&[("west", RoomId::Terminal), ("w", RoomId::Terminal)]),
            vec![ItemId::DiffPrism],
        ),
    );

    // THE BUFFER
    mundo.insert(
        RoomId::TheBuffer,
        sala(
            "O Buffer",
            "O purgatório do código deletado. Snippets de funções abandonadas \
             flutuam como fantasmas: loops infinitos incompletos, structs sem implementação, \
             TODOs que nunca viraram código. Um teclado enferrujado jaz no chão — \
             deve ter pertencido ao último programador que ficou preso aqui.\n\
             Saídas: norte (Terminal), leste (Settings Vault)",
            saidas(&[
                ("north", RoomId::Terminal),
                ("n", RoomId::Terminal),
                ("east", RoomId::SettingsVault),
                ("e", RoomId::SettingsVault),
            ]),
            vec![ItemId::RustyKeyboard],
        ),
    );

    // SETTINGS VAULT
    mundo.insert(
        RoomId::SettingsVault,
        sala(
            "O Settings Vault",
            "Uma câmara fortemente protegida por configurações aninhadas. \
             Arquivos settings.json se empilham até o teto. \
             No centro, uma porta selada com runas TOML. Por trás dela, \
             o Unclosed Brace espera — o bug que travou o editor e prendeu você aqui. \
             A porta exige que você tenha: Cursor, LSP Wand, Chave de Ponto-e-Vírgula, \
             Autosave Amulet, Git Token e Diff Prism para ser aberta.\n\
             Saídas: oeste (The Buffer)",
            saidas(&[("west", RoomId::TheBuffer), ("w", RoomId::TheBuffer)]),
            vec![],
        ),
    );

    mundo
}

pub fn criar_itens() -> HashMap<ItemId, Item> {
    let mut itens = HashMap::new();

    itens.insert(
        ItemId::Cursor,
        Item {
            nome: "Cursor",
            descricao: "Um cursor piscante que obedece seus comandos",
            exame: "O cursor pulsa com energia potencial. É a sua principal ferramenta \
                dentro do editor — onde você aponta, o código muda.",
            pegavel: true,
        },
    );

    itens.insert(
        ItemId::LspWand,
        Item {
            nome: "LSP Wand",
            descricao: "Uma varinha que detecta e corrige erros de código",
            exame: "A Language Server Protocol Wand vibra com diagnósticos. \
                Pontas vermelhas indicam erros, amarelas são warnings. \
                Com ela, você pode localizar e corrigir qualquer bug — inclusive o Unclosed Brace.",
            pegavel: true,
        },
    );

    itens.insert(
        ItemId::CoffeeMug,
        Item {
            nome: "Caneca de Café",
            descricao: "Uma caneca fumegante de café forte",
            exame: "Café escuro como o espaço, quente como um loop infinito. \
                O AI precisa disso para funcionar em plena capacidade. \
                Talvez você deva DAR para o AI Assistant.",
            pegavel: true,
        },
    );

    itens.insert(
        ItemId::SemicolonKey,
        Item {
            nome: "Chave de Ponto-e-Vírgula",
            descricao: "Uma chave em formato de ponto-e-vírgula, brilhante e afiada",
            exame: "A Semicolon Key — símbolo de conclusão em linguagens de programação. \
                Esta chave pode abrir a porta do Settings Vault quando combinada \
                com os outros itens necessários.",
            pegavel: true,
        },
    );

    itens.insert(
        ItemId::AutosaveAmulet,
        Item {
            nome: "Autosave Amulet",
            descricao: "Um amuleto que previne perda de progresso",
            exame: "O amuleto pulsa em azul suave. Enquanto você o carrega, \
                nenhum crash ou falha pode apagar seu progresso. \
                Essencial para a jornada.",
            pegavel: true,
        },
    );

    itens.insert(
        ItemId::GitToken,
        Item {
            nome: "Git Token",
            descricao: "Um token de autenticação git resplandecente",
            exame: "Um token pessoal de acesso ao repositório, brilhando com permissões \
                de leitura e escrita. Alguém o perdeu durante um force push mal planejado. \
                Necessário para acessar o Settings Vault.",
            pegavel: true,
        },
    );

    itens.insert(
        ItemId::DiffPrism,
        Item {
            nome: "Diff Prism",
            descricao: "Um prisma que mostra diferenças entre versões do código",
            exame: "O Diff Prism separa vermelho (removido) de verde (adicionado). \
                Ao olhar através dele, você pode ver exatamente onde o Unclosed Brace \
                foi introduzido no commit '3f7a2b1: refactor: update brace handling'. \
                Isso é crucial para corrigi-lo.",
            pegavel: true,
        },
    );

    itens.insert(
        ItemId::RustyKeyboard,
        Item {
            nome: "Teclado Enferrujado",
            descricao: "Um teclado velho e enferrujado de um programador anterior",
            exame: "Teclas desgastadas contam histórias de mil refatorações. \
                A tecla Escape está completamente gasta. Apesar do estado, \
                ainda funciona — programadores antigos construíam para durar.",
            pegavel: true,
        },
    );

    itens
}
