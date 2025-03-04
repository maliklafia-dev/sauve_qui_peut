```markdown
# 🏆 Sauve Qui Peut - Client TCP en Rust 🦀

## 📌 Description
*Sauve qui peut* est un client Rust conçu pour interagir avec un *serveur TCP* dans le cadre d’un jeu de l'abirinthe où l'objectif est d'en sortir.  
Il permet d’enregistrer une équipe, d’inscrire un joueur et de communiquer avec le serveur via des *messages JSON* pour effectuer des actions comme se déplacer devant, à droite, à gauche et derrière.

## 🚀 Fonctionnalités

- Connexion à un serveur TCP
- Enregistrement d'une équipe avec un nom unique
- Inscription d'un joueur avec un token d'enregistrement
- Envoi et réception de messages JSON
- Analyse des données du radar (RadarView) et des objets (ItemsView)
- Déplacements en fonction des informations du radar
- Tests unitaires avec un serveur mocké

## 🛠️ Implémentation des fonctionnalités principales

### 🔹 1. Enregistrement d'une équipe (register_team)

```rust
fn register_team(address: &str) -> Result<String, String> {
    let mut stream = TcpStream::connect(address)
        .map_err(|e| format!("Erreur de connexion: {}", e))?;
    
    let register_team_msg = serde_json::json!({
        "RegisterTeam": { "name": "gandhi_wise" }
    });
    
    send_message(&mut stream, &register_team_msg)?;
    let response = receive_message(&mut stream)?;
    
    response.get("RegisterTeamResult")
        .and_then(|r| r.get("Ok"))
        .and_then(|ok| ok.get("registration_token"))
        .and_then(|t| t.as_str())
        .map(String::from)
        .ok_or("Token d'enregistrement invalide".to_string())
}
```

### 🔹 2. Inscription d'un joueur (subscribe_player)

```rust
fn subscribe_player(address: &str, token: &str) -> Result<TcpStream, String> {
    let mut stream = TcpStream::connect(address)
        .map_err(|e| format!("Erreur de connexion: {}", e))?;
    
    let subscribe_player_msg = serde_json::json!({
        "SubscribePlayer": {
            "name": "gandhi_wise_player",
            "registration_token": token
        }
    });
    
    send_message(&mut stream, &subscribe_player_msg)?;
    let response = receive_message(&mut stream)?;
    
    if response.get("SubscribePlayerResult").is_some() {
        Ok(stream)
    } else {
        Err("Erreur d'inscription du joueur".to_string())
    }
}
```

### 🔹 3. Envoi et réception des messages JSON

#### ✅ Envoi d'un message (send_message)

```rust
fn send_message(stream: &mut TcpStream, message: &Value) -> Result<(), String> {
    let json = serde_json::to_string(message)
        .map_err(|e| format!("Erreur de sérialisation: {}", e))?;
    
    let message_size = json.len() as u32;
    stream.write_all(&message_size.to_le_bytes())
        .map_err(|e| format!("Erreur d'envoi de la taille: {}", e))?;
    stream.write_all(json.as_bytes())
        .map_err(|e| format!("Erreur d'envoi du message: {}", e))?;
    
    stream.flush()
        .map_err(|e| format!("Erreur de flush: {}", e))?;
    Ok(())
}
```

#### ✅ Réception d'un message (receive_message)

```rust
fn receive_message(stream: &mut TcpStream) -> Result<Value, String> {
    let mut size_buffer = [0u8; 4];
    stream.read_exact(&mut size_buffer)
        .map_err(|e| format!("Erreur de lecture de la taille: {}", e))?;
    let message_size = u32::from_le_bytes(size_buffer) as usize;
    
    let mut message_buffer = vec![0u8; message_size];
    stream.read_exact(&mut message_buffer)
        .map_err(|e| format!("Erreur de lecture du message: {}", e))?;
    
    let message_str = String::from_utf8(message_buffer)
        .map_err(|e| format!("Erreur de conversion UTF-8: {}", e))?;
    
    serde_json::from_str(&message_str)
        .map_err(|e| format!("Erreur de désérialisation: {}", e))
}
```

### 🔹 4. Décodage des données du radar (RadarView)

#### ✅ Décodage du radar (decode_radar_view)

```rust
fn decode_radar_view(encoded: &str) -> Result<RadarView, String> {
    let binary_data = decode_b64(encoded)?;
    if binary_data.len() < 11 {
        return Err("Données trop courtes pour une vue radar".to_string());
    }
    let h_passages = extract_horizontal_passages(&binary_data[0..3]);
    let v_passages = extract_vertical_passages(&binary_data[3..6]);
    let cells = extract_cells(&binary_data[6..11]);
    Ok(RadarView {
        horizontal_passages: h_passages,
        vertical_passages: v_passages,
        cells,
    })
}
```

#### ✅ Décodage d'un objet (decode_b64)

```rust
fn decode_b64(encoded: &str) -> Result<Vec<u8>, String> {
    base64::decode(encoded).map_err(|e| format!("Erreur de décodage Base64: {}", e))
}
```

### 🔹 5. Déplacements et gestion du jeu (game_loop)

```rust
fn game_loop(stream: &mut TcpStream) {
    let mut current_direction = "Front";
    
    loop {
        let message = match receive_message(stream) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Erreur lors de la réception d'un message: {}", e);
                break;
            }
        };

        // Vérifier si le serveur envoie une vue radar
        if message.get("RadarView").is_some() {
            println!("📡 Vue radar reçue, déplacement vers {}", current_direction);
            let move_action = serde_json::json!({
                "Action": { "MoveTo": current_direction }
            });
            if let Err(e) = send_message(stream, &move_action) {
                println!("Erreur lors de l'envoi d'une action: {}", e);
                break;
            }
        } else if message.get("ActionError").is_some() {
            println!("❌ Erreur d'action reçue, changement de direction");
            current_direction = match current_direction {
                "Front" => "Right",
                "Right" => "Back",
                "Back" => "Left",
                "Left" => "Front",
                _ => "Front"
            };
        }
    }
}
```

## 🧪 Tests unitaires

```rust
#[test]
fn test_register_team() {
    let server_address = "127.0.0.1:8888";
    let result = register_team(server_address);
    assert!(result.is_ok(), "L'enregistrement de l'équipe a échoué");
}
```

```rust
#[test]
fn test_send_message() {
    let server_address = "127.0.0.1:8888";
    let mut stream = TcpStream::connect(server_address).unwrap();
    let message = serde_json::json!({ "test": "message" });
    let result = send_message(&mut stream, &message);
    assert!(result.is_ok(), "L'envoi du message a échoué");
}
```
### 🚀 Lancer le client 
Cargo run

### 🚀 Lancer les tests
Cargo test
