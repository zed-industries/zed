use super::world::ItemId;

#[derive(Debug, Clone)]
pub struct Jogador {
    pub inventario: Vec<ItemId>,
    pub movimentos: u32,
    pub ai_ajudou: bool,
    pub diff_examinado: bool,
    pub vault_aberto: bool,
    pub venceu: bool,
}

impl Jogador {
    pub fn novo() -> Self {
        Self {
            inventario: Vec::new(),
            movimentos: 0,
            ai_ajudou: false,
            diff_examinado: false,
            vault_aberto: false,
            venceu: false,
        }
    }

    pub fn pegar(&mut self, item: ItemId) {
        if !self.inventario.contains(&item) {
            self.inventario.push(item);
        }
    }

    pub fn largar(&mut self, item: &ItemId) -> bool {
        if let Some(pos) = self.inventario.iter().position(|i| i == item) {
            self.inventario.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn tem_itens_para_vault(&self) -> bool {
        let necessarios = [
            ItemId::Cursor,
            ItemId::LspWand,
            ItemId::SemicolonKey,
            ItemId::AutosaveAmulet,
            ItemId::GitToken,
            ItemId::DiffPrism,
        ];
        necessarios.iter().all(|i| self.inventario.contains(i))
    }
}
