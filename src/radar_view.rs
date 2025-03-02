use serde::{Serialize, Deserialize};

/// Représente un élément sur une cellule du radar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadarItem {
    /// Aucune information particulière, aucune entité
    Empty,
    /// Aucune information particulière, entité alliée
    Ally,
    /// Aucune information particulière, entité opposée
    Enemy,
    /// Aucune information particulière, entité hostile (monstre)
    Monster,
    /// Élément d'aide (indice), aucune entité
    Hint,
    /// Élément d'aide avec entité alliée
    HintWithAlly,
    /// Élément d'aide avec entité opposée
    HintWithEnemy,
    /// Élément d'aide avec entité hostile
    HintWithMonster,
    /// Objectif (sortie), aucune entité
    Goal,
    /// Objectif avec entité alliée
    GoalWithAlly,
    /// Objectif avec entité opposée
    GoalWithEnemy,
    /// Objectif avec entité hostile
    GoalWithMonster,
    /// Donnée non définie/invalide
    Undefined,
}

impl From<u8> for RadarItem {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => RadarItem::Empty,
            0b0001 => RadarItem::Ally,
            0b0010 => RadarItem::Enemy,
            0b0011 => RadarItem::Monster,
            0b0100 => RadarItem::Hint,
            0b0101 => RadarItem::HintWithAlly,
            0b0110 => RadarItem::HintWithEnemy,
            0b0111 => RadarItem::HintWithMonster,
            0b1000 => RadarItem::Goal,
            0b1001 => RadarItem::GoalWithAlly,
            0b1010 => RadarItem::GoalWithEnemy,
            0b1011 => RadarItem::GoalWithMonster,
            0b1111 => RadarItem::Undefined,
            _ => RadarItem::Undefined,
        }
    }
}

/// Représente un passage (ouvert, fermé ou non défini)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Passage {
    /// Passage non défini (non visible)
    Undefined,
    /// Passage ouvert
    Open,
    /// Passage fermé (mur)
    Wall,
}

impl From<u8> for Passage {
    fn from(value: u8) -> Self {
        match value {
            0 => Passage::Undefined,
            1 => Passage::Open,
            2 => Passage::Wall,
            _ => Passage::Undefined,
        }
    }
}

/// Représente une vue radar complète
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarView {
    /// Passages horizontaux (12 passages sur 4 lignes)
    pub horizontal_passages: [[Passage; 3]; 4],
    /// Passages verticaux (12 passages sur 3 lignes)
    pub vertical_passages: [[Passage; 4]; 3],
    /// Cellules (9 cellules sur 3 lignes)
    pub cells: [[RadarItem; 3]; 3],
}

/// Décode une vue radar à partir de sa représentation encodée
pub fn decode_radar_view(encoded: &str) -> Result<RadarView, String> {
    // Décodage base64 de la chaîne encodée
    let binary_data = decode_b64(encoded)?;

    if binary_data.len() < 11 {
        return Err("Données trop courtes pour une vue radar".to_string());
    }

    // Extraction des passages horizontaux (3 octets)
    let h_passages = extract_horizontal_passages(&binary_data[0..3]);

    // Extraction des passages verticaux (3 octets)
    let v_passages = extract_vertical_passages(&binary_data[3..6]);

    // Extraction des cellules (5 octets)
    let cells = extract_cells(&binary_data[6..11]);

    Ok(RadarView {
        horizontal_passages: h_passages,
        vertical_passages: v_passages,
        cells,
    })
}

/// Extrait les passages horizontaux des données binaires
fn extract_horizontal_passages(data: &[u8]) -> [[Passage; 3]; 4] {
    let mut result = [[Passage::Undefined; 3]; 4];

    // Conversion des octets en little endian
    let mut bits = 0u32;
    bits |= (data[2] as u32) << 16;
    bits |= (data[1] as u32) << 8;
    bits |= data[0] as u32;

    // Extraction des passages (2 bits par passage)
    for row in 0..4 {
        for col in 0..3 {
            let shift = (row * 6) + (col * 2);
            let passage_bits = (bits >> shift) & 0b11;
            result[row][col] = Passage::from(passage_bits as u8);
        }
    }

    result
}

/// Extrait les passages verticaux des données binaires
fn extract_vertical_passages(data: &[u8]) -> [[Passage; 4]; 3] {
    let mut result = [[Passage::Undefined; 4]; 3];

    // Conversion des octets en little endian
    let mut bits = 0u32;
    bits |= (data[2] as u32) << 16;
    bits |= (data[1] as u32) << 8;
    bits |= data[0] as u32;

    // Extraction des passages (2 bits par passage)
    for row in 0..3 {
        for col in 0..4 {
            let shift = (row * 8) + (col * 2);
            let passage_bits = (bits >> shift) & 0b11;
            result[row][col] = Passage::from(passage_bits as u8);
        }
    }

    result
}

/// Extrait les cellules des données binaires
fn extract_cells(data: &[u8]) -> [[RadarItem; 3]; 3] {
    let mut result = [[RadarItem::Undefined; 3]; 3];

    // Extraction des cellules (4 bits par cellule)
    let mut cell_index = 0;
    for i in 0..5 {
        let byte = data[i];

        // Premier item (4 bits de poids fort)
        if cell_index < 9 {
            let row = cell_index / 3;
            let col = cell_index % 3;
            result[row][col] = RadarItem::from((byte >> 4) & 0x0F);
            cell_index += 1;
        }

        // Deuxième item (4 bits de poids faible)
        if cell_index < 9 {
            let row = cell_index / 3;
            let col = cell_index % 3;
            result[row][col] = RadarItem::from(byte & 0x0F);
            cell_index += 1;
        }
    }

    result
}

/// Décode une chaîne encodée en b64 vers des données binaires
fn decode_b64(encoded: &str) -> Result<Vec<u8>, String> {
    // Vérification de la taille
    let len = encoded.len();
    if len % 4 == 1 {
        return Err("Taille invalide pour l'encodage b64".to_string());
    }

    let mut result = Vec::with_capacity((len * 3) / 4);
    let chars: Vec<char> = encoded.chars().collect();

    let mut i = 0;
    while i < len {
        // Traitement par groupe de 4 caractères (ou moins pour le dernier groupe)
        let remaining = len - i;
        let group_size = std::cmp::min(4, remaining);

        let mut values = [0u8; 4];
        for j in 0..group_size {
            values[j] = char_to_value(chars[i + j])?;
        }

        // Premier octet (toujours présent)
        let byte1 = (values[0] << 2) | (values[1] >> 4);
        result.push(byte1);

        // Deuxième octet (si au moins 3 caractères)
        if group_size >= 3 {
            let byte2 = ((values[1] & 0x0F) << 4) | (values[2] >> 2);
            result.push(byte2);
        }

        // Troisième octet (si 4 caractères)
        if group_size >= 4 {
            let byte3 = ((values[2] & 0x03) << 6) | values[3];
            result.push(byte3);
        }

        i += group_size;
    }

    Ok(result)
}

/// Convertit un caractère b64 en sa valeur numérique
fn char_to_value(c: char) -> Result<u8, String> {
    match c {
        'a'..='z' => Ok(c as u8 - b'a'),
        'A'..='Z' => Ok(c as u8 - b'A' + 26),
        '0'..='9' => Ok(c as u8 - b'0' + 52),
        '+' => Ok(62),
        '/' => Ok(63),
        _ => Err(format!("Caractère invalide dans l'encodage b64: {}", c)),
    }
}