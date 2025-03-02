use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::Value;

// Constantes
const TEAM_NAME: &str = "gandhi_wise";

// Fonction pour enregistrer une équipe et récupérer un token
pub fn register_team(address: &str) -> Result<String, String> {
    let mut stream = TcpStream::connect(address)
        .map_err(|e| format!("Erreur de connexion: {}", e))?;
    println!("Connexion établie pour l'équipe");

    let register_team_msg = serde_json::json!({
        "RegisterTeam": {
            "name": TEAM_NAME
        }
    });

    send_message(&mut stream, &register_team_msg)?;
    let response = receive_message(&mut stream)?;

    let token = response.get("RegisterTeamResult")
        .and_then(|result| result.get("Ok"))
        .and_then(|ok| ok.get("registration_token"))
        .and_then(|token| token.as_str())
        .ok_or("Token d'enregistrement invalide")?
        .to_string();

    println!("Token reçu: {}", token);
    Ok(token)
}

// Fonction pour inscrire un joueur avec le token récupéré
pub fn subscribe_player(address: &str, token: &str) -> Result<TcpStream, String> {
    let mut stream = TcpStream::connect(address)
        .map_err(|e| format!("Erreur de connexion: {}", e))?;
    println!("Connexion établie pour le joueur");

    let subscribe_player_msg = serde_json::json!({
        "SubscribePlayer": {
            "name": format!("{}_player", TEAM_NAME),
            "registration_token": token
        }
    });

    send_message(&mut stream, &subscribe_player_msg)?;
    let _response = receive_message(&mut stream)?;

    Ok(stream)
}

// Fonction pour envoyer un message au serveur
pub fn send_message(stream: &mut TcpStream, message: &Value) -> Result<(), String> {
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

// Fonction pour recevoir un message du serveur
pub fn receive_message(stream: &mut TcpStream) -> Result<Value, String> {
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

