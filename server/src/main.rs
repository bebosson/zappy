pub mod args;
pub mod cell;
pub mod gamer;
pub mod player;
mod ressources;
mod teams;
pub mod egg;
pub mod zappy;
use args::args::Args;




fn main() {
    /*
    parsing 
    
    init env/memory
    
    while (pas encore tous les clients) {
        envoie "bienvenue"
        recv les reponses {
            etablit la co
            register un client
        }
    }
    
    prend le timestamp
    
    while (pas encore de ctrl+c || pas encore de gagnant) {
        
        est ce que j'ai recu un paquet {
            est ce que le paquet est valide {
                enregistre les datas de la commande recu dans le bon player
                on craft le paquet gfx ??? 
            }
        }
        
        check si une des actions en attente a termine {
            on craft le paquet gfx ???
            on craft le paquet client
            on envoie la reponse
        }

        prend le timestamp + conversion timestamp vers tick

        update les actions/ticks des actions en attentes
    }
    */
}
