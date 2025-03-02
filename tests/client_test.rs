use maze_client::client::{register_team, subscribe_player, send_message, receive_message};
use std::net::TcpStream;
use std::thread;
use serde_json::json;
mod mock_server;
use crate::mock_server::*;
use std::io::Read;


#[test]
fn test_register_team() {
    let server = start_mock_server();
    let server_address = server.local_addr().unwrap().to_string();

    thread::spawn(move || {
        let (stream, _) = server.accept().unwrap();
        handle_mock_register_team(stream);
    });

    let result = register_team(&server_address);
    assert_eq!(result.unwrap(), "MOCK_TOKEN_1234");
}

#[test]
fn test_subscribe_player() {
    let server = start_mock_server();
    let server_address = server.local_addr().unwrap().to_string();

    thread::spawn(move || {
        let (stream, _) = server.accept().unwrap();
        handle_mock_subscribe_player(stream);
    });

    let result = subscribe_player(&server_address, "MOCK_TOKEN_1234");

    // ✅ Ajout du debug
    if let Err(ref e) = result {
        println!("❌ Erreur lors de l'inscription du joueur: {}", e);
    }

    assert!(result.is_ok());
}

#[test]
fn test_receive_message_radar_view() {
    let server = start_mock_server();
    let server_address = server.local_addr().unwrap().to_string();

    thread::spawn(move || {
        let (stream, _) = server.accept().unwrap();
        handle_mock_radar_view(stream);
    });

    let mut stream = TcpStream::connect(&server_address).unwrap();
    let response = receive_message(&mut stream).unwrap();
    assert!(response.get("RadarView").is_some());
}

#[test]
fn test_send_message() {
    let server = start_mock_server();
    let server_address = server.local_addr().unwrap().to_string();

    thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();

        // 🔍 Lire la taille du message envoyé
        let mut size_buffer = [0u8; 4];
        stream.read_exact(&mut size_buffer).unwrap();
        let message_size = u32::from_le_bytes(size_buffer) as usize;

        // 🔍 Lire le message lui-même
        let mut message_buffer = vec![0u8; message_size];
        stream.read_exact(&mut message_buffer).unwrap();
        let message_str = String::from_utf8(message_buffer).unwrap();
        let message: serde_json::Value = serde_json::from_str(&message_str).unwrap();

        println!("📥 Message reçu : {}", message_str);

        // ✅ Vérifie que le message reçu correspond à ce qui a été envoyé
        assert_eq!(message["TestKey"], "TestValue");
    });

    let mut stream = TcpStream::connect(&server_address).unwrap();
    let test_message = json!({"TestKey": "TestValue"});

    // ✅ Teste l'envoi du message
    let result = send_message(&mut stream, &test_message);

    // ✅ Vérifie que `send_message` s'est bien exécuté
    assert!(result.is_ok());
}

