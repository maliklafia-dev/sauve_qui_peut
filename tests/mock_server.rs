use std::net::{TcpListener, TcpStream};
use std::io::{ Write};
use serde_json::json;

/// Démarre un serveur TCP mocké avec un port dynamique
pub fn start_mock_server() -> TcpListener {
    TcpListener::bind("127.0.0.1:0").expect("Impossible de démarrer le serveur mock") // ✅ Port dynamique
}

/// Gère une connexion mockée pour `subscribe_player`
pub fn handle_mock_subscribe_player(mut stream: TcpStream) {
    let response = json!({
        "SubscribePlayerResult": "Success"
    }).to_string();

    send_mock_response(&mut stream, response);
}

/// Gère une connexion mockée pour `receive_message` (RadarView)
pub fn handle_mock_radar_view(mut stream: TcpStream) {
    let response = json!({ "RadarView": {} }).to_string();
    send_mock_response(&mut stream, response);
}

/// Envoie une réponse JSON au client
fn send_mock_response(stream: &mut TcpStream, response: String) {
    let size = response.len() as u32;

    println!("📤 Envoi d'une réponse mockée : {}", response);

    stream.write_all(&size.to_le_bytes()).unwrap();
    stream.write_all(response.as_bytes()).unwrap();

    // ✅ S'assurer que toutes les données sont envoyées avant de fermer la connexion
    stream.flush().unwrap();

    println!("✅ Réponse envoyée avec succès !");
}

/// Gère une connexion mockée pour `register_team`
pub fn handle_mock_register_team(mut stream: TcpStream) {
    let response = json!({
        "RegisterTeamResult": {
            "Ok": { "registration_token": "MOCK_TOKEN_1234" }
        }
    }).to_string();

    send_mock_response(&mut stream, response);
}