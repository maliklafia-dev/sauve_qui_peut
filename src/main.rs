mod client;

use std::env;
use std::net::TcpStream;
use client::{register_team, subscribe_player, send_message, receive_message};

const DEFAULT_PORT: u16 = 8778;

fn main() {
    let args: Vec<String> = env::args().collect();

    // ✅ Correction : Stocker d'abord "localhost".to_string() dans une variable
    let default_address = "localhost".to_string();
    let server_address = args.get(1).unwrap_or(&default_address);

    let full_address = format!("{}:{}", server_address, DEFAULT_PORT);

    println!("Connexion au serveur: {}", full_address);

    let token = match register_team(&full_address) {
        Ok(token) => token,
        Err(e) => {
            println!("Erreur d'enregistrement: {}", e);
            return;
        }
    };

    match subscribe_player(&full_address, &token) {
        Ok(mut stream) => {
            println!("Joueur inscrit !");
            game_loop(&mut stream);
        }
        Err(e) => println!("Erreur d'inscription: {}", e),
    }
}

// Fonction de jeu fictive pour éviter une erreur de compilation
fn game_loop(stream: &mut TcpStream) {
    let mut current_direction = "Front";

    loop {
        // 🔍 Debug : Afficher les messages reçus
        let message = match receive_message(stream) {
            Ok(msg) => {
                println!("🔍 Message reçu : {:?}", msg);
                msg
            },
            Err(e) => {
                println!("❌ Erreur lors de la réception d'un message: {}", e);
                break;
            }
        };

        // 📌 Vérification des messages du serveur
        if message.get("RadarView").is_some() {
            println!("🚀 Déplacement vers : {}", current_direction);
            let move_action = serde_json::json!({
                "Action": { "MoveTo": current_direction }
            });

            if let Err(e) = send_message(stream, &move_action) {
                println!("❌ Erreur lors de l'envoi d'une action: {}", e);
                break;
            }
        } else if message.get("ActionError").is_some() {
            println!("⚠️ Erreur d'action reçue, changement de direction");
            current_direction = match current_direction {
                "Front" => "Right",
                "Right" => "Back",
                "Back" => "Left",
                "Left" => "Front",
                _ => "Front"
            };
        } else {
            println!("ℹ️ Message inconnu reçu, ignoré...");
        }
    }
}

