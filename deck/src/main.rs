/* 
Este programa define uma estrutura para um baralho de cartas 
e um método para embaralhar as cartas.

1. Importa o módulo rand e as funções thread_rng e SliceRandom do crate rand, 
que são usadas para gerar números aleatórios e 
para embaralhar uma sequência, respectivamente.

2. Define a estrutura Deck que contém um vetor de strings, 
representando as cartas.

3. Implementação do Deck:
   - O método new cria um novo baralho de cartas. 
   Ele itera sobre os naipes (suits) e valores (values), 
   cria uma string para cada combinação e adiciona ao vetor cards.
   - O método shuffle está definido, mas não implementado.

4. A função main:
   - Cria uma nova instância de Deck usando Deck::new.
   - Chama o método shuffle (embora não faça nada porque está vazio).
   - Imprime o baralho criado.
*/

use rand::{thread_rng, seq::SliceRandom};


#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {     
        
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];    
        

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        Deck { cards }
        }

    
        fn shuffle(&mut self) {
            let mut rng = thread_rng(); 
            self.cards.shuffle(&mut rng);
            
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();
    // Probably need to add error handling!!!!
    let cards = deck.deal(3);
    

    println!("Heres your hand: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
